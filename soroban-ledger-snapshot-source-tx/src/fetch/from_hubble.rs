//! Ledger entry lookups backed by the Stellar [Hubble] BigQuery dataset.
//!
//! Hubble is a public BigQuery dataset, maintained by the Stellar Development
//! Foundation, containing a per-ledger change log of Stellar ledger entries.
//! Because it supports random access by ledger key, it can answer the
//! "state as of ledger N" question that this crate otherwise answers by
//! downloading and linearly scanning entire history-archive bucket files.
//!
//! [Hubble]: https://developers.stellar.org/docs/data/analytics/hubble
//!
//! # What this module can and cannot answer
//!
//! Hubble stores decoded, flattened columns for most ledger entry types, and
//! raw XDR for almost none of them. A `LedgerEntry` can therefore only be
//! reconstructed *exactly* for the entry types whose full contents survive
//! Hubble's transformation:
//!
//! * [`LedgerKey::ContractData`] — the `contract_data` table carries a
//!   `contract_data_xdr` column holding the base64 XDR of the
//!   `ContractDataEntry` (the inner `LedgerEntryData::ContractData` payload).
//! * [`LedgerKey::Ttl`] — a `TtlEntry` is exactly `(key_hash,
//!   live_until_ledger_seq)`, and the `ttl` table stores both columns.
//!
//! Every other key type resolves to [`Lookup::Unsupported`], so callers fall
//! back to an authoritative source rather than receiving a lossy answer. In
//! particular `contract_code` is *not* supported: the `contract_code` table
//! records only static analysis metrics (`n_instructions`, `n_functions`, …)
//! and deliberately omits the Wasm bytes, so the entry cannot be rebuilt.
//!
//! See the crate README for the full list of limitations.
//!
//! # Granularity
//!
//! Hubble's finest granularity is a whole ledger: state tables have a
//! `ledger_sequence` column but no transaction index, so Hubble alone cannot
//! express "state as of just before transaction T within ledger N". That is
//! not a limitation in practice for this crate, because transaction-granular
//! state is produced by replaying ledger-close meta (phases 1-2 of
//! [`crate::fetch::LedgerEntryFetcher`]); Hubble is only consulted for the
//! checkpoint fallback, which is ledger-granular by construction.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use soroban_sdk::xdr::{
    ContractDataEntry, Hash, LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey,
    LedgerKeyContractData, Limits, ReadXdr, ScVal, TtlEntry, WriteXdr,
};

/// Error type for Hubble lookups.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("xdr: {0}")]
    Xdr(#[from] soroban_sdk::xdr::Error),
    #[error("obtaining access token: {0}")]
    AccessToken(String),
    #[error("bigquery: {0}")]
    BigQuery(String),
    #[error("bigquery query did not complete within the configured timeout")]
    QueryIncomplete,
    #[error("invalid identifier {0:?}: expected only ASCII letters, digits, '-' or '_'")]
    InvalidIdentifier(String),
    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),
    #[error(
        "hubble returned an entry whose key does not match the requested key; \
         this indicates a malformed query or a corrupted dataset"
    )]
    KeyMismatch,
}

/// Outcome of a Hubble lookup for a single ledger key.
///
/// Distinguishing "Hubble cannot answer this" from "Hubble says the entry does
/// not exist" is what lets callers keep an authoritative fallback for the
/// former without paying for it on the latter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    /// Hubble cannot answer authoritatively for this key. The caller must
    /// consult another source. Returned for unsupported key types, and for
    /// "no rows" when the query was restricted to a `closed_at` window (in
    /// which case an absent row may simply lie outside the window).
    Unsupported,
    /// Hubble answered authoritatively: the entry exists with this value.
    ///
    /// Boxed because a `LedgerEntry` is much larger than the other variants.
    Found(Box<LedgerEntry>),
    /// Hubble answered authoritatively: the entry does not exist at the
    /// requested ledger (it was never created, or was deleted at or before
    /// it).
    Absent,
}

/// Supplies the OAuth 2.0 access token used to authenticate BigQuery requests.
///
/// Credentials are never read from, or written to, this crate's caches, and no
/// default credential lookup is performed implicitly; the caller decides where
/// a token comes from. A token for the current `gcloud` login can be obtained
/// with `gcloud auth print-access-token`, and a token minted for a service
/// account by the usual Google auth libraries. The required scope is
/// `https://www.googleapis.com/auth/bigquery.readonly`.
pub trait AccessTokenSource: Send + Sync {
    /// Return a currently-valid OAuth 2.0 access token.
    fn access_token(&self) -> Result<String, Error>;
}

/// An access token supplied directly by the caller.
///
/// The token is held in memory only, and is never logged or persisted by this
/// crate.
pub struct StaticAccessToken(String);

impl StaticAccessToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }
}

impl AccessTokenSource for StaticAccessToken {
    fn access_token(&self) -> Result<String, Error> {
        Ok(self.0.clone())
    }
}

/// An access token read from an environment variable on each request.
///
/// Reading per request (rather than once at construction) means a token
/// refreshed out-of-band in the environment is picked up without rebuilding
/// the source.
pub struct EnvAccessToken(String);

impl EnvAccessToken {
    /// The environment variable consulted by [`EnvAccessToken::default`].
    pub const DEFAULT_VAR: &'static str = "GOOGLE_OAUTH_ACCESS_TOKEN";

    pub fn new(var: impl Into<String>) -> Self {
        Self(var.into())
    }
}

impl Default for EnvAccessToken {
    fn default() -> Self {
        Self::new(Self::DEFAULT_VAR)
    }
}

impl AccessTokenSource for EnvAccessToken {
    fn access_token(&self) -> Result<String, Error> {
        // Deliberately does not include the `VarError` payload: its
        // `NotUnicode` variant renders the variable's *value*, which would
        // write the token itself into an error that callers log.
        match std::env::var(&self.0) {
            Ok(token) => Ok(token),
            Err(std::env::VarError::NotPresent) => {
                Err(Error::AccessToken(format!("${} is not set", self.0)))
            }
            Err(std::env::VarError::NotUnicode(_)) => Err(Error::AccessToken(format!(
                "${} is not valid unicode",
                self.0
            ))),
        }
    }
}

/// Transport used to issue the BigQuery `jobs.query` HTTP request.
///
/// Abstracted so the query construction and response handling above it can be
/// exercised without network access.
pub trait Transport: Send + Sync {
    /// POST `body` (a JSON `QueryRequest`) to `url` with `access_token` as a
    /// bearer credential, returning the raw response body.
    fn post(&self, url: &str, access_token: &str, body: &str) -> Result<String, Error>;
}

/// The default [`Transport`], backed by a blocking `reqwest` client.
pub struct ReqwestTransport;

impl Transport for ReqwestTransport {
    fn post(&self, url: &str, access_token: &str, body: &str) -> Result<String, Error> {
        let response = reqwest::blocking::Client::new()
            .post(url)
            .bearer_auth(access_token)
            .header("content-type", "application/json")
            .body(body.to_string())
            .send()?;
        // Read the body before checking the status: BigQuery reports errors
        // (quota, permission, bad query) as a JSON `error` object, which
        // carries a far more actionable message than the status code alone.
        let status = response.status();
        let text = response.text()?;
        if !status.is_success() && text.is_empty() {
            return Err(Error::BigQuery(format!("got status code {status}")));
        }
        Ok(text)
    }
}

/// Where the Hubble dataset lives, and the limits placed on queries against
/// it.
#[derive(Debug, Clone)]
pub struct HubbleConfig {
    /// The caller's own Google Cloud project, which is billed for query
    /// compute. Hubble's *storage* is paid for by the Stellar Development
    /// Foundation, but every query is billed to the caller, so there is no
    /// default for this.
    pub billing_project_id: String,
    /// Google Cloud project hosting the dataset. Defaults to `crypto-stellar`.
    pub dataset_project_id: String,
    /// BigQuery dataset name. Defaults to `crypto_stellar`, the bronze layer
    /// holding the per-ledger change log.
    pub dataset: String,
    /// Dataset location, sent with the job so BigQuery does not have to infer
    /// it. Defaults to `US`.
    pub location: String,
    /// BigQuery API endpoint. Overridable for testing against an emulator.
    pub endpoint: String,
    /// Hard cap on bytes billed for a single query. BigQuery *fails* a query
    /// that would exceed this rather than running it, making this the
    /// authoritative protection against an accidentally expensive scan. There
    /// is deliberately no way to express "no limit".
    pub maximum_bytes_billed: u64,
    /// Server-side timeout for a single query, in milliseconds.
    pub timeout_ms: u32,
    /// Optional inclusive lower bound on `closed_at`, as an RFC 3339
    /// timestamp.
    ///
    /// Hubble's state tables are partitioned by month on `closed_at`, so a
    /// lower bound is what actually prunes partitions and therefore what
    /// actually reduces cost. It is optional because it also *weakens* the
    /// answer: with a lower bound set, "no rows" no longer proves the entry
    /// does not exist (it may simply predate the window), so a miss is
    /// reported as [`Lookup::Unsupported`] and the caller falls back.
    pub closed_at_from: Option<String>,
    /// Optional exclusive upper bound on `closed_at`, as an RFC 3339
    /// timestamp. Safe to set without weakening the answer, since no change at
    /// or below the requested ledger can have closed after the ledger did.
    pub closed_at_to: Option<String>,
    /// How many days back to look when establishing how far Hubble has
    /// ingested.
    ///
    /// Only needs to exceed Hubble's worst-case ingestion lag; a larger value
    /// merely scans more monthly partitions. Interpolated into SQL rather than
    /// passed as a parameter, which is safe because a `u32` renders as digits
    /// only.
    pub coverage_lookback_days: u32,
}

impl HubbleConfig {
    /// Configuration for the public mainnet Hubble dataset, billed to
    /// `billing_project_id`.
    ///
    /// Only mainnet data is published publicly; there is no public testnet or
    /// futurenet Hubble dataset, and no network discriminator column, so a
    /// caller pointing this at a non-mainnet network must override
    /// [`HubbleConfig::dataset_project_id`] and [`HubbleConfig::dataset`] to
    /// name their own dataset.
    pub fn mainnet(billing_project_id: impl Into<String>) -> Self {
        Self {
            billing_project_id: billing_project_id.into(),
            dataset_project_id: "crypto-stellar".to_string(),
            dataset: "crypto_stellar".to_string(),
            location: "US".to_string(),
            endpoint: "https://bigquery.googleapis.com".to_string(),
            // 8 GiB. Enough for a clustered point lookup over a bounded set of
            // partitions, small enough that a mistake fails fast instead of
            // scanning the whole table.
            maximum_bytes_billed: 8 * 1024 * 1024 * 1024,
            timeout_ms: 60_000,
            closed_at_from: None,
            closed_at_to: None,
            coverage_lookback_days: 30,
        }
    }

    fn query_url(&self) -> Result<String, Error> {
        validate_identifier(&self.billing_project_id)?;
        Ok(format!(
            "{}/bigquery/v2/projects/{}/queries",
            self.endpoint.trim_end_matches('/'),
            self.billing_project_id,
        ))
    }
}

/// Reject anything that is not a plain BigQuery identifier.
///
/// Project, dataset, and table names cannot be supplied as query parameters —
/// they are interpolated into the SQL text — so they are validated against a
/// conservative allowlist first. Everything that varies per lookup (ledger
/// key, ledger sequence, timestamps) is passed as a named query parameter and
/// never interpolated.
fn validate_identifier(identifier: &str) -> Result<(), Error> {
    let valid = !identifier.is_empty()
        && identifier
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'));
    if valid {
        Ok(())
    } else {
        Err(Error::InvalidIdentifier(identifier.to_string()))
    }
}

/// A named BigQuery query parameter.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct QueryParameter {
    name: String,
    #[serde(rename = "parameterType")]
    parameter_type: ParameterType,
    #[serde(rename = "parameterValue")]
    parameter_value: ParameterValue,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct ParameterType {
    #[serde(rename = "type")]
    type_: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct ParameterValue {
    value: String,
}

impl QueryParameter {
    fn new(name: &str, type_: &'static str, value: impl Into<String>) -> Self {
        Self {
            name: name.to_string(),
            parameter_type: ParameterType { type_ },
            parameter_value: ParameterValue {
                value: value.into(),
            },
        }
    }
}

/// A fully-formed, parameterized BigQuery query.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Query {
    sql: String,
    parameters: Vec<QueryParameter>,
    /// Whether "no rows" proves the entry does not exist. False when the query
    /// was restricted to a `closed_at` window that may not cover the entry's
    /// last modification.
    absence_is_authoritative: bool,
}
/// Hex-encode the SHA-256 of a ledger key's XDR, matching the
/// `ledger_key_hash` column produced by stellar-etl.
fn ledger_key_hash(key: &LedgerKey) -> Result<String, Error> {
    Ok(hex::encode(Sha256::digest(key.to_xdr(Limits::none())?)))
}

/// Build the query answering "value of `key` as of the end of `ledger`",
/// together with the table it reads, or `None` if Hubble cannot answer for
/// this key type.
///
/// Rows are ordered by `ledger_sequence` descending so the most recent change
/// at or before `ledger` wins. Within a single ledger, `ledger_entry_change`
/// value 3 ("state") is the *pre-image* of the entry rather than a mutation,
/// so it is ordered last to ensure the post-ledger value is selected when both
/// a state row and a create/update/delete row exist for the same ledger.
fn query_for_key(
    key: &LedgerKey,
    ledger: u32,
    config: &HubbleConfig,
) -> Result<Option<(&'static str, Query)>, Error> {
    validate_identifier(&config.dataset_project_id)?;
    validate_identifier(&config.dataset)?;

    let (table, columns, key_column, key_value) = match key {
        // Soroban auth nonces are contract data entries keyed by
        // `ScVal::LedgerKeyNonce`. stellar-etl discards them before loading, so
        // the `contract_data` table never holds a row for one and a query would
        // return "no rows" — which must not be mistaken for proof that the
        // nonce is unused.
        LedgerKey::ContractData(LedgerKeyContractData {
            key: ScVal::LedgerKeyNonce(_),
            ..
        }) => return Ok(None),
        LedgerKey::ContractData(_) => (
            "contract_data",
            "contract_data_xdr, last_modified_ledger, deleted",
            "ledger_key_hash",
            ledger_key_hash(key)?,
        ),
        // The `ttl` table keys on the TTL entry's `key_hash` directly — the
        // hash of the *underlying* entry's key — not on a hash of the
        // `LedgerKeyTtl` itself, so it is taken from the key rather than
        // computed.
        LedgerKey::Ttl(ttl) => (
            "ttl",
            "live_until_ledger_seq, last_modified_ledger, deleted",
            "key_hash",
            hex::encode(ttl.key_hash.0),
        ),
        _ => return Ok(None),
    };

    let mut parameters = vec![
        QueryParameter::new("key", "STRING", key_value),
        QueryParameter::new("ledger", "INT64", ledger.to_string()),
    ];
    let mut predicates = String::new();
    if let Some(from) = &config.closed_at_from {
        predicates.push_str("\n    AND closed_at >= @closed_at_from");
        parameters.push(QueryParameter::new("closed_at_from", "TIMESTAMP", from));
    }
    if let Some(to) = &config.closed_at_to {
        predicates.push_str("\n    AND closed_at < @closed_at_to");
        parameters.push(QueryParameter::new("closed_at_to", "TIMESTAMP", to));
    }

    let sql = format!(
        "SELECT {columns}\n  \
         FROM `{project}.{dataset}.{table}`\n  \
         WHERE {key_column} = @key\n    \
         AND ledger_sequence <= @ledger{predicates}\n  \
         ORDER BY ledger_sequence DESC, IF(ledger_entry_change = 3, 0, 1) DESC\n  \
         LIMIT 1",
        project = config.dataset_project_id,
        dataset = config.dataset,
    );

    Ok(Some((
        table,
        Query {
            sql,
            parameters,
            absence_is_authoritative: config.closed_at_from.is_none(),
        },
    )))
}

/// Build the query establishing how far Hubble has ingested `table`.
///
/// Coverage is established from the same table the answer will come from.
/// Hubble's history tables and state tables are loaded by independent
/// pipelines, so a high-water mark taken from `history_ledgers` would not
/// prove that `contract_data` or `ttl` had caught up.
///
/// `MAX(ledger_sequence)` is restricted to the most recent
/// [`HubbleConfig::coverage_lookback_days`] so it prunes to a couple of
/// monthly partitions instead of scanning the full table.
fn coverage_query(table: &str, config: &HubbleConfig) -> Result<Query, Error> {
    validate_identifier(&config.dataset_project_id)?;
    validate_identifier(&config.dataset)?;
    validate_identifier(table)?;
    Ok(Query {
        sql: format!(
            "SELECT MAX(ledger_sequence) AS max_ledger\n  \
             FROM `{project}.{dataset}.{table}`\n  \
             WHERE closed_at >= TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL {days} DAY)",
            project = config.dataset_project_id,
            dataset = config.dataset,
            days = config.coverage_lookback_days,
        ),
        parameters: Vec::new(),
        absence_is_authoritative: true,
    })
}

/// The JSON body of a BigQuery `jobs.query` request.
#[derive(serde::Serialize)]
struct QueryRequest<'a> {
    query: &'a str,
    #[serde(rename = "useLegacySql")]
    use_legacy_sql: bool,
    #[serde(rename = "parameterMode")]
    parameter_mode: &'static str,
    #[serde(rename = "queryParameters")]
    query_parameters: &'a [QueryParameter],
    #[serde(rename = "maximumBytesBilled")]
    maximum_bytes_billed: String,
    #[serde(rename = "timeoutMs")]
    timeout_ms: u32,
    #[serde(rename = "maxResults")]
    max_results: u32,
    location: &'a str,
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    #[serde(default)]
    error: Option<ApiError>,
    #[serde(rename = "jobComplete", default)]
    job_complete: bool,
    #[serde(default)]
    schema: Option<Schema>,
    #[serde(default)]
    rows: Option<Vec<Row>>,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct Schema {
    #[serde(default)]
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Row {
    #[serde(default)]
    f: Vec<Cell>,
}

#[derive(Debug, Deserialize)]
struct Cell {
    #[serde(default)]
    v: Option<serde_json::Value>,
}

/// A single result row, addressable by column name.
///
/// BigQuery returns every scalar as a JSON string (or null) in a positional
/// `f`/`v` array, with names carried separately in the schema, so the two are
/// zipped here into something that can be read by name.
struct ResultRow {
    columns: Vec<(String, Option<String>)>,
}

impl ResultRow {
    fn get(&self, name: &str) -> Result<&str, Error> {
        match self.columns.iter().find(|(n, _)| n == name) {
            Some((_, Some(v))) => Ok(v),
            Some((_, None)) => Err(Error::UnexpectedResponse(format!("column {name} is null"))),
            None => Err(Error::UnexpectedResponse(format!("missing column {name}"))),
        }
    }

    fn get_u32(&self, name: &str) -> Result<u32, Error> {
        self.get(name)?
            .parse()
            .map_err(|e| Error::UnexpectedResponse(format!("column {name}: {e}")))
    }

    fn get_bool(&self, name: &str) -> Result<bool, Error> {
        match self.get(name)? {
            "true" => Ok(true),
            "false" => Ok(false),
            other => Err(Error::UnexpectedResponse(format!(
                "column {name}: expected a boolean, got {other:?}"
            ))),
        }
    }
}

/// Parse a `jobs.query` response into at most one row.
fn parse_response(body: &str) -> Result<Option<ResultRow>, Error> {
    let response: QueryResponse = serde_json::from_str(body)?;
    if let Some(error) = response.error {
        return Err(Error::BigQuery(error.message));
    }
    // An incomplete job means the results are not in this response and would
    // have to be collected via `jobs.getQueryResults`. Rather than silently
    // treating that as "no rows" — which would be indistinguishable from a
    // genuine absence and could be cached as a wrong answer — surface it.
    if !response.job_complete {
        return Err(Error::QueryIncomplete);
    }
    let Some(row) = response.rows.unwrap_or_default().into_iter().next() else {
        return Ok(None);
    };
    let names = response.schema.map(|s| s.fields).unwrap_or_default();
    if names.len() != row.f.len() {
        return Err(Error::UnexpectedResponse(format!(
            "schema has {} fields but row has {} cells",
            names.len(),
            row.f.len(),
        )));
    }
    let columns = names
        .into_iter()
        .zip(row.f)
        .map(|(field, cell)| {
            let value = match cell.v {
                None | Some(serde_json::Value::Null) => None,
                Some(serde_json::Value::String(s)) => Some(s),
                Some(other) => Some(other.to_string()),
            };
            (field.name, value)
        })
        .collect();
    Ok(Some(ResultRow { columns }))
}

/// Rebuild a `LedgerEntry` for `key` from a result row.
fn entry_from_row(key: &LedgerKey, row: &ResultRow) -> Result<LedgerEntry, Error> {
    let data = match key {
        LedgerKey::ContractData(_) => LedgerEntryData::ContractData(
            ContractDataEntry::from_xdr_base64(row.get("contract_data_xdr")?, Limits::none())?,
        ),
        LedgerKey::Ttl(ttl) => LedgerEntryData::Ttl(TtlEntry {
            key_hash: Hash(ttl.key_hash.0),
            live_until_ledger_seq: row.get_u32("live_until_ledger_seq")?,
        }),
        _ => return Err(Error::KeyMismatch),
    };
    let entry = LedgerEntry {
        data,
        last_modified_ledger_seq: row.get_u32("last_modified_ledger")?,
        // Hubble stores the LedgerEntry's extension (sponsorship) fields
        // nowhere, so it cannot be recovered. V0 is used, matching what the
        // RPC source in this crate already does; for contract tests this has
        // no material impact, as sponsorship does not apply to contract data
        // or TTL entries.
        ext: LedgerEntryExt::V0,
    };
    // Defence in depth: a mis-built query or a hash collision would otherwise
    // hand the host an entry for a different key, silently corrupting a test.
    if entry.to_key() != *key {
        return Err(Error::KeyMismatch);
    }
    Ok(entry)
}

/// Reads historical ledger entries from the Hubble BigQuery dataset.
pub struct HubbleSource {
    config: HubbleConfig,
    token: Box<dyn AccessTokenSource>,
    transport: Box<dyn Transport>,
    /// Highest ledger Hubble has ingested, per table, resolved once each and
    /// then reused.
    max_ingested_ledger: std::sync::Mutex<std::collections::BTreeMap<&'static str, Option<u32>>>,
}

impl HubbleSource {
    /// Create a source that reads tokens from the environment (see
    /// [`EnvAccessToken`]) and issues requests over HTTP.
    pub fn new(config: HubbleConfig) -> Self {
        Self::with_token(config, Box::new(EnvAccessToken::default()))
    }

    /// Create a source with an explicit access token source.
    pub fn with_token(config: HubbleConfig, token: Box<dyn AccessTokenSource>) -> Self {
        Self {
            config,
            token,
            transport: Box::new(ReqwestTransport),
            max_ingested_ledger: std::sync::Mutex::new(std::collections::BTreeMap::new()),
        }
    }

    /// Replace the HTTP transport, primarily so lookups can be exercised
    /// without network access.
    pub fn with_transport(mut self, transport: Box<dyn Transport>) -> Self {
        self.transport = transport;
        self
    }

    /// Whether Hubble has ingested `table` up to and including `ledger`.
    ///
    /// The high-water mark is resolved at most once per table, since it only
    /// ever moves forward and re-querying it per key would multiply the cost
    /// of every lookup. A failed query is deliberately not memoized, so a
    /// transient error does not disable the source for the rest of the run.
    fn covers(&self, table: &'static str, ledger: u32) -> Result<bool, Error> {
        // Look the cached mark up and release the lock before doing any I/O.
        // Two threads racing here cost one duplicate query, which is far
        // cheaper than holding a lock across a network round trip.
        if let Some(max) = self.lock_coverage().get(table).copied() {
            return Ok(max.is_some_and(|max| max >= ledger));
        }
        let row = self.run(&coverage_query(table, &self.config)?)?;
        // `MAX` over no rows yields a single NULL row, so both "no rows" and a
        // null cell mean the lookback window found nothing, and coverage is
        // therefore unproven.
        let max = row.and_then(|row| row.get_u32("max_ledger").ok());
        tracing::debug!(table, max_ingested_ledger = ?max, "hubble coverage");
        self.lock_coverage().insert(table, max);
        Ok(max.is_some_and(|max| max >= ledger))
    }

    fn lock_coverage(
        &self,
    ) -> std::sync::MutexGuard<'_, std::collections::BTreeMap<&'static str, Option<u32>>> {
        self.max_ingested_ledger
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Execute `query`, returning at most one row.
    fn run(&self, query: &Query) -> Result<Option<ResultRow>, Error> {
        let body = serde_json::to_string(&QueryRequest {
            query: &query.sql,
            use_legacy_sql: false,
            parameter_mode: "NAMED",
            query_parameters: &query.parameters,
            maximum_bytes_billed: self.config.maximum_bytes_billed.to_string(),
            timeout_ms: self.config.timeout_ms,
            max_results: 1,
            location: &self.config.location,
        })?;
        let response = self.transport.post(
            &self.config.query_url()?,
            &self.token.access_token()?,
            &body,
        )?;
        parse_response(&response)
    }

    /// Look up the value of `key` as of the end of `ledger`.
    pub fn get(&self, key: &LedgerKey, ledger: u32) -> Result<Lookup, Error> {
        let Some((table, query)) = query_for_key(key, ledger, &self.config)? else {
            tracing::debug!(ledger, "hubble cannot serve this key type");
            return Ok(Lookup::Unsupported);
        };

        // Hubble is loaded in intraday batches, so it can lag the network. Any
        // answer about a ledger past what it has ingested is unsound in both
        // directions: an entry created after the high-water mark looks like it
        // never existed, and one updated or deleted after it reads back stale.
        // Neither is distinguishable from a correct answer at the call site,
        // and the caller caches results persistently, so establish coverage
        // before trusting anything.
        if !self.covers(table, ledger)? {
            tracing::debug!(ledger, table, "hubble has not ingested this ledger yet");
            return Ok(Lookup::Unsupported);
        }

        let Some(row) = self.run(&query)? else {
            return Ok(if query.absence_is_authoritative {
                Lookup::Absent
            } else {
                // The search was bounded by `closed_at_from`, so the entry may
                // simply have last changed before the window. Absence is not
                // proven; defer to the caller's fallback.
                Lookup::Unsupported
            });
        };

        // `deleted` marks the row as the entry's removal, so the entry does not
        // exist as of this ledger.
        if row.get_bool("deleted")? {
            return Ok(Lookup::Absent);
        }
        Ok(Lookup::Found(Box::new(entry_from_row(key, &row)?)))
    }
}

#[cfg(test)]
mod test {
    use super::{
        coverage_query, entry_from_row, ledger_key_hash, parse_response, query_for_key,
        validate_identifier, Error, HubbleConfig, HubbleSource, Lookup, StaticAccessToken,
        Transport,
    };
    use soroban_sdk::xdr::{
        AccountId, ContractDataDurability, ContractDataEntry, ContractId, ExtensionPoint, Hash,
        LedgerEntryData, LedgerEntryExt, LedgerKey, LedgerKeyAccount, LedgerKeyContractCode,
        LedgerKeyContractData, LedgerKeyTtl, Limits, PublicKey, ScAddress, ScNonceKey, ScVal,
        Uint256, WriteXdr,
    };
    use std::sync::{Arc, Mutex};

    fn config() -> HubbleConfig {
        HubbleConfig::mainnet("my-billing-project")
    }

    fn contract_data_key() -> LedgerKey {
        LedgerKey::ContractData(LedgerKeyContractData {
            contract: ScAddress::Contract(ContractId(Hash([3u8; 32]))),
            key: ScVal::I32(7),
            durability: ContractDataDurability::Persistent,
        })
    }

    fn contract_data_entry() -> ContractDataEntry {
        ContractDataEntry {
            ext: ExtensionPoint::V0,
            contract: ScAddress::Contract(ContractId(Hash([3u8; 32]))),
            key: ScVal::I32(7),
            durability: ContractDataDurability::Persistent,
            val: ScVal::U32(42),
        }
    }

    fn ttl_key() -> LedgerKey {
        LedgerKey::Ttl(LedgerKeyTtl {
            key_hash: Hash([9u8; 32]),
        })
    }

    /// A transport that records the requests it was given and replays canned
    /// responses in order, so lookups run entirely offline.
    struct MockTransport {
        responses: Mutex<std::collections::VecDeque<String>>,
        seen: Mutex<Vec<(String, String, String)>>,
    }

    /// A coverage response reporting that Hubble has ingested up to `ledger`.
    fn coverage_response(ledger: u32) -> String {
        format!(
            r#"{{"jobComplete":true,"schema":{{"fields":[{{"name":"max_ledger"}}]}},
                "rows":[{{"f":[{{"v":"{ledger}"}}]}}]}}"#
        )
    }

    impl MockTransport {
        /// Returns a handle the test keeps, so requests can be inspected after
        /// the source (which takes ownership of a `Box<dyn Transport>`) has run.
        fn new<I: IntoIterator<Item = S>, S: Into<String>>(responses: I) -> Arc<Self> {
            Arc::new(Self {
                responses: Mutex::new(responses.into_iter().map(Into::into).collect()),
                seen: Mutex::new(Vec::new()),
            })
        }

        fn requests(&self) -> Vec<(String, String, String)> {
            self.seen.lock().unwrap().clone()
        }
    }

    impl Transport for Arc<MockTransport> {
        fn post(&self, url: &str, access_token: &str, body: &str) -> Result<String, Error> {
            self.seen.lock().unwrap().push((
                url.to_string(),
                access_token.to_string(),
                body.to_string(),
            ));
            Ok(self
                .responses
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected extra request"))
        }
    }

    fn source_with(config: HubbleConfig, transport: Arc<MockTransport>) -> HubbleSource {
        HubbleSource::with_token(config, Box::new(StaticAccessToken::new("test-token")))
            .with_transport(Box::new(transport))
    }

    /// A source whose coverage check succeeds and whose key query returns
    /// `response`.
    fn source(response: &str) -> HubbleSource {
        source_with(
            config(),
            MockTransport::new([coverage_response(u32::MAX), response.to_string()]),
        )
    }

    fn contract_data_response(val_xdr: &str, last_modified: u32, deleted: bool) -> String {
        format!(
            r#"{{"jobComplete":true,
                "schema":{{"fields":[
                  {{"name":"contract_data_xdr"}},
                  {{"name":"last_modified_ledger"}},
                  {{"name":"deleted"}}]}},
                "rows":[{{"f":[{{"v":"{val_xdr}"}},{{"v":"{last_modified}"}},{{"v":"{deleted}"}}]}}]}}"#
        )
    }

    // --- key conversion -------------------------------------------------

    #[test]
    fn ledger_key_hash_is_hex_sha256_of_key_xdr() {
        // stellar-etl computes `ledger_key_hash` as
        // hex(sha256(MarshalBinary(ledgerKey))); this pins the same encoding.
        let key = contract_data_key();
        let expected = {
            use sha2::{Digest, Sha256};
            hex::encode(Sha256::digest(key.to_xdr(Limits::none()).unwrap()))
        };
        let actual = ledger_key_hash(&key).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 64);
        assert!(actual
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }

    #[test]
    fn ttl_queries_use_the_inner_key_hash_not_a_hash_of_the_ttl_key() {
        // The `ttl` table's `key_hash` column is the hash of the entry the TTL
        // refers to, which is exactly `LedgerKeyTtl::key_hash`. Hashing the
        // `LedgerKeyTtl` XDR instead would never match a row.
        let key = ttl_key();
        let query = query_for_key(&key, 100, &config()).unwrap().unwrap().1;
        let key_param = query.parameters.iter().find(|p| p.name == "key").unwrap();
        assert_eq!(key_param.parameter_value.value, hex::encode([9u8; 32]));
        assert_ne!(
            key_param.parameter_value.value,
            ledger_key_hash(&key).unwrap()
        );
    }

    // --- query generation and parameterization ---------------------------

    #[test]
    fn contract_data_query_is_parameterized_and_bounded() {
        let query = query_for_key(&contract_data_key(), 12345, &config())
            .unwrap()
            .unwrap()
            .1;
        assert!(query
            .sql
            .contains("FROM `crypto-stellar.crypto_stellar.contract_data`"));
        assert!(query.sql.contains("WHERE ledger_key_hash = @key"));
        assert!(query.sql.contains("AND ledger_sequence <= @ledger"));
        assert!(query.sql.contains("LIMIT 1"));
        // No literal values may be interpolated into the SQL text; everything
        // that varies is a named parameter.
        assert!(!query.sql.contains("12345"));
        assert!(!query
            .sql
            .contains(&ledger_key_hash(&contract_data_key()).unwrap()));

        let names: Vec<_> = query.parameters.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["key", "ledger"]);
        let ledger = &query.parameters[1];
        assert_eq!(ledger.parameter_type.type_, "INT64");
        assert_eq!(ledger.parameter_value.value, "12345");
        assert_eq!(query.parameters[0].parameter_type.type_, "STRING");
    }

    #[test]
    fn state_rows_are_ordered_last_within_a_ledger() {
        // ledger_entry_change 3 is the pre-image "state" row; the post-ledger
        // value must win when both exist at the same ledger_sequence.
        let query = query_for_key(&contract_data_key(), 1, &config())
            .unwrap()
            .unwrap()
            .1;
        assert!(query
            .sql
            .contains("ORDER BY ledger_sequence DESC, IF(ledger_entry_change = 3, 0, 1) DESC"));
    }

    #[test]
    fn closed_at_bounds_become_timestamp_parameters() {
        let mut config = config();
        config.closed_at_from = Some("2024-01-01T00:00:00Z".to_string());
        config.closed_at_to = Some("2024-03-01T00:00:00Z".to_string());
        let query = query_for_key(&contract_data_key(), 1, &config)
            .unwrap()
            .unwrap()
            .1;
        assert!(query.sql.contains("AND closed_at >= @closed_at_from"));
        assert!(query.sql.contains("AND closed_at < @closed_at_to"));
        assert!(!query.sql.contains("2024-01-01"));
        let from = query
            .parameters
            .iter()
            .find(|p| p.name == "closed_at_from")
            .unwrap();
        assert_eq!(from.parameter_type.type_, "TIMESTAMP");
        assert_eq!(from.parameter_value.value, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn absence_is_only_authoritative_without_a_lower_bound() {
        let query = query_for_key(&contract_data_key(), 1, &config())
            .unwrap()
            .unwrap()
            .1;
        assert!(query.absence_is_authoritative);

        let mut bounded = config();
        bounded.closed_at_from = Some("2024-01-01T00:00:00Z".to_string());
        let query = query_for_key(&contract_data_key(), 1, &bounded)
            .unwrap()
            .unwrap()
            .1;
        assert!(!query.absence_is_authoritative);

        // An upper bound alone cannot hide an older change, so it is safe.
        let mut upper_only = config();
        upper_only.closed_at_to = Some("2024-01-01T00:00:00Z".to_string());
        let query = query_for_key(&contract_data_key(), 1, &upper_only)
            .unwrap()
            .unwrap()
            .1;
        assert!(query.absence_is_authoritative);
    }

    #[test]
    fn unsupported_key_types_produce_no_query() {
        // contract_code carries no Wasm bytes in Hubble, and classic entry
        // types carry no XDR at all, so neither can be reconstructed.
        for key in [
            LedgerKey::ContractCode(LedgerKeyContractCode {
                hash: Hash([0u8; 32]),
            }),
            LedgerKey::Account(LedgerKeyAccount {
                account_id: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0u8; 32]))),
            }),
        ] {
            assert!(query_for_key(&key, 1, &config()).unwrap().is_none());
        }
    }

    // --- identifier validation (SQL injection) ---------------------------

    #[test]
    fn identifiers_reject_injection() {
        assert!(validate_identifier("crypto-stellar").is_ok());
        assert!(validate_identifier("crypto_stellar").is_ok());
        for bad in [
            "",
            "a`b",
            "a b",
            "a.b",
            "crypto_stellar` UNION SELECT * FROM `secret",
            "a'b",
            "a;b",
        ] {
            assert!(
                matches!(validate_identifier(bad), Err(Error::InvalidIdentifier(_))),
                "{bad:?} should be rejected",
            );
        }
    }

    #[test]
    fn injected_dataset_name_is_rejected_before_a_query_is_built() {
        let mut config = config();
        config.dataset = "crypto_stellar` UNION SELECT * FROM `other".to_string();
        assert!(matches!(
            query_for_key(&contract_data_key(), 1, &config),
            Err(Error::InvalidIdentifier(_)),
        ));
    }

    #[test]
    fn injected_billing_project_is_rejected_before_a_request_is_sent() {
        let mut config = config();
        config.billing_project_id = "proj/../../other".to_string();
        assert!(matches!(
            config.query_url(),
            Err(Error::InvalidIdentifier(_))
        ));
    }

    // --- request construction --------------------------------------------

    #[test]
    fn request_carries_cost_and_result_caps_and_disables_legacy_sql() {
        let mock = MockTransport::new([
            coverage_response(u32::MAX),
            contract_data_response(
                &contract_data_entry().to_xdr_base64(Limits::none()).unwrap(),
                100,
                false,
            ),
        ]);
        let source = source_with(config(), mock.clone());
        source.get(&contract_data_key(), 500).unwrap();

        let requests = mock.requests();
        assert_eq!(requests.len(), 2);
        let (url, token, body) = &requests[1];
        assert_eq!(
            url,
            "https://bigquery.googleapis.com/bigquery/v2/projects/my-billing-project/queries",
        );
        assert_eq!(token, "test-token");

        let request: serde_json::Value = serde_json::from_str(body).unwrap();
        assert_eq!(request["useLegacySql"], false);
        assert_eq!(request["parameterMode"], "NAMED");
        assert_eq!(request["maxResults"], 1);
        assert_eq!(request["location"], "US");
        assert_eq!(
            request["maximumBytesBilled"],
            (8u64 * 1024 * 1024 * 1024).to_string()
        );
        assert_eq!(request["queryParameters"].as_array().unwrap().len(), 2);
        // The ledger bound must reach BigQuery as a typed parameter, not as
        // text spliced into the query.
        assert_eq!(request["queryParameters"][1]["name"], "ledger");
        assert_eq!(
            request["queryParameters"][1]["parameterValue"]["value"],
            "500"
        );
        assert!(!request["query"].as_str().unwrap().contains("500"));
    }

    // --- ingestion coverage ----------------------------------------------

    #[test]
    fn ledgers_past_hubbles_high_water_mark_are_unsupported() {
        // Hubble is loaded in intraday batches. Answering for a ledger it has
        // not ingested would report a not-yet-loaded entry as absent, and the
        // caller caches that persistently.
        let mock = MockTransport::new([coverage_response(1000)]);
        let source = source_with(config(), mock.clone());
        assert_eq!(
            source.get(&contract_data_key(), 1001).unwrap(),
            Lookup::Unsupported,
        );
        // Only the coverage query runs; the key query is never issued.
        assert_eq!(mock.requests().len(), 1);
    }

    #[test]
    fn ledgers_at_the_high_water_mark_are_covered() {
        let source = source_with(
            config(),
            MockTransport::new([
                coverage_response(1000),
                r#"{"jobComplete":true,"schema":{"fields":[]},"rows":[]}"#.to_string(),
            ]),
        );
        assert_eq!(
            source.get(&contract_data_key(), 1000).unwrap(),
            Lookup::Absent,
        );
    }

    #[test]
    fn unknown_coverage_is_unsupported() {
        // `MAX` over an empty lookback window yields a null cell; coverage is
        // then unproven and must not be assumed.
        let response = r#"{"jobComplete":true,
            "schema":{"fields":[{"name":"max_ledger"}]},
            "rows":[{"f":[{"v":null}]}]}"#;
        let source = source_with(config(), MockTransport::new([response]));
        assert_eq!(
            source.get(&contract_data_key(), 1).unwrap(),
            Lookup::Unsupported,
        );
    }

    #[test]
    fn coverage_is_resolved_once_and_reused() {
        // Re-querying the high-water mark per key would multiply the cost of
        // every lookup.
        let empty = r#"{"jobComplete":true,"schema":{"fields":[]},"rows":[]}"#.to_string();
        let mock = MockTransport::new([coverage_response(u32::MAX), empty.clone(), empty.clone()]);
        let source = source_with(config(), mock.clone());
        source.get(&contract_data_key(), 1).unwrap();
        source.get(&contract_data_key(), 2).unwrap();
        // One coverage query plus one query per lookup.
        assert_eq!(mock.requests().len(), 3);
    }

    #[test]
    fn coverage_query_reads_the_same_table_the_answer_comes_from() {
        // Hubble's history and state tables are loaded by independent
        // pipelines, so coverage taken from `history_ledgers` would not prove
        // `contract_data` had caught up. It also uses a different column name
        // (`sequence`), which would make the query fail outright.
        for table in ["contract_data", "ttl"] {
            let query = coverage_query(table, &config()).unwrap();
            assert!(
                query
                    .sql
                    .contains(&format!("FROM `crypto-stellar.crypto_stellar.{table}`")),
                "{}",
                query.sql,
            );
            assert!(query
                .sql
                .contains("SELECT MAX(ledger_sequence) AS max_ledger"));
            assert!(query.sql.contains(
                "WHERE closed_at >= TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 30 DAY)"
            ));
            assert!(!query.sql.contains("history_ledgers"));
        }
    }

    #[test]
    fn key_queries_and_coverage_queries_agree_on_the_table() {
        // The coverage gate is only sound if it measures the table the key
        // query reads.
        for (key, table) in [(contract_data_key(), "contract_data"), (ttl_key(), "ttl")] {
            let (actual, _) = query_for_key(&key, 1, &config()).unwrap().unwrap();
            assert_eq!(actual, table);
        }
    }

    // --- response handling -----------------------------------------------

    #[test]
    fn found_contract_data_is_reconstructed_exactly() {
        let entry = contract_data_entry();
        let response =
            contract_data_response(&entry.to_xdr_base64(Limits::none()).unwrap(), 777, false);
        let got = source(&response).get(&contract_data_key(), 1000).unwrap();
        let Lookup::Found(ledger_entry) = got else {
            panic!("expected Found, got {got:?}");
        };
        assert_eq!(ledger_entry.data, LedgerEntryData::ContractData(entry));
        assert_eq!(ledger_entry.last_modified_ledger_seq, 777);
        assert!(matches!(ledger_entry.ext, LedgerEntryExt::V0));
        assert_eq!(ledger_entry.to_key(), contract_data_key());
    }

    #[test]
    fn found_ttl_is_reconstructed_exactly() {
        let response = r#"{"jobComplete":true,
            "schema":{"fields":[
              {"name":"live_until_ledger_seq"},
              {"name":"last_modified_ledger"},
              {"name":"deleted"}]},
            "rows":[{"f":[{"v":"5000"},{"v":"4000"},{"v":"false"}]}]}"#;
        let got = source(response).get(&ttl_key(), 4500).unwrap();
        let Lookup::Found(entry) = got else {
            panic!("expected Found, got {got:?}");
        };
        let LedgerEntryData::Ttl(ttl) = &entry.data else {
            panic!("expected a TTL entry");
        };
        assert_eq!(ttl.key_hash, Hash([9u8; 32]));
        assert_eq!(ttl.live_until_ledger_seq, 5000);
        assert_eq!(entry.last_modified_ledger_seq, 4000);
        assert_eq!(entry.to_key(), ttl_key());
    }

    #[test]
    fn deleted_rows_report_the_entry_as_absent() {
        // A deletion row must resolve to Absent rather than to the (stale)
        // value the row still carries.
        let response = contract_data_response(
            &contract_data_entry().to_xdr_base64(Limits::none()).unwrap(),
            777,
            true,
        );
        assert_eq!(
            source(&response).get(&contract_data_key(), 1000).unwrap(),
            Lookup::Absent,
        );
    }

    #[test]
    fn no_rows_is_absent_when_unbounded() {
        let response = r#"{"jobComplete":true,"schema":{"fields":[]},"rows":[]}"#;
        assert_eq!(
            source(response).get(&contract_data_key(), 1000).unwrap(),
            Lookup::Absent,
        );
    }

    #[test]
    fn no_rows_is_unsupported_when_a_lower_bound_was_applied() {
        // With a `closed_at` lower bound the query cannot see older changes,
        // so a miss must not be reported as proof of absence.
        let mut config = config();
        config.closed_at_from = Some("2024-01-01T00:00:00Z".to_string());
        let source = source_with(
            config,
            MockTransport::new([
                coverage_response(u32::MAX),
                r#"{"jobComplete":true,"schema":{"fields":[]},"rows":[]}"#.to_string(),
            ]),
        );
        assert_eq!(
            source.get(&contract_data_key(), 1000).unwrap(),
            Lookup::Unsupported,
        );
    }

    #[test]
    fn unsupported_key_types_short_circuit_without_a_request() {
        let mock = MockTransport::new(Vec::<String>::new());
        let source = source_with(config(), mock.clone());
        let key = LedgerKey::ContractCode(LedgerKeyContractCode {
            hash: Hash([0u8; 32]),
        });
        assert_eq!(source.get(&key, 1).unwrap(), Lookup::Unsupported);
        // No query is issued — not even the coverage check — so an
        // unsupported key type costs nothing.
        assert!(mock.requests().is_empty());
    }

    #[test]
    fn nonce_keys_are_unsupported_and_cost_nothing() {
        // stellar-etl discards contract data keyed by a nonce before loading,
        // so Hubble never has a row for one. Treating that miss as proof the
        // nonce is unused would silently change replay semantics for
        // `require_auth`.
        let mock = MockTransport::new(Vec::<String>::new());
        let source = source_with(config(), mock.clone());
        let key = LedgerKey::ContractData(LedgerKeyContractData {
            contract: ScAddress::Contract(ContractId(Hash([3u8; 32]))),
            key: ScVal::LedgerKeyNonce(ScNonceKey { nonce: 1234 }),
            durability: ContractDataDurability::Temporary,
        });
        assert!(query_for_key(&key, 1, &config()).unwrap().is_none());
        assert_eq!(source.get(&key, 1).unwrap(), Lookup::Unsupported);
        assert!(mock.requests().is_empty());
    }

    #[test]
    fn api_errors_are_surfaced() {
        let response = r#"{"error":{"message":"Query exceeded limit for bytes billed"}}"#;
        let err = source(response).get(&contract_data_key(), 1).unwrap_err();
        assert!(matches!(err, Error::BigQuery(m) if m.contains("bytes billed")));
    }

    #[test]
    fn incomplete_jobs_are_surfaced_rather_than_read_as_absence() {
        let response = r#"{"jobComplete":false}"#;
        let err = source(response).get(&contract_data_key(), 1).unwrap_err();
        assert!(matches!(err, Error::QueryIncomplete));
    }

    #[test]
    fn schema_row_length_mismatch_is_rejected() {
        let response = r#"{"jobComplete":true,
            "schema":{"fields":[{"name":"a"},{"name":"b"}]},
            "rows":[{"f":[{"v":"1"}]}]}"#;
        assert!(matches!(
            parse_response(response),
            Err(Error::UnexpectedResponse(_)),
        ));
    }

    #[test]
    fn entry_for_a_mismatched_key_is_rejected() {
        // Simulate a row whose XDR decodes to a different contract than the
        // key that was asked for.
        let other = ContractDataEntry {
            contract: ScAddress::Contract(ContractId(Hash([8u8; 32]))),
            ..contract_data_entry()
        };
        let response =
            contract_data_response(&other.to_xdr_base64(Limits::none()).unwrap(), 1, false);
        let err = source(&response).get(&contract_data_key(), 1).unwrap_err();
        assert!(matches!(err, Error::KeyMismatch));
    }

    #[test]
    fn null_columns_are_rejected() {
        let response = r#"{"jobComplete":true,
            "schema":{"fields":[
              {"name":"contract_data_xdr"},
              {"name":"last_modified_ledger"},
              {"name":"deleted"}]},
            "rows":[{"f":[{"v":null},{"v":"1"},{"v":"false"}]}]}"#;
        let row = parse_response(response).unwrap().unwrap();
        assert!(matches!(
            entry_from_row(&contract_data_key(), &row),
            Err(Error::UnexpectedResponse(_)),
        ));
    }

    // --- configuration ----------------------------------------------------

    #[test]
    fn mainnet_config_points_at_the_public_dataset() {
        let config = config();
        assert_eq!(config.dataset_project_id, "crypto-stellar");
        assert_eq!(config.dataset, "crypto_stellar");
        assert_eq!(config.location, "US");
        assert_eq!(
            config.query_url().unwrap(),
            "https://bigquery.googleapis.com/bigquery/v2/projects/my-billing-project/queries",
        );
        // Billing is always the caller's own project; there is no default.
        assert_eq!(config.billing_project_id, "my-billing-project");
    }
}
