//! A small BigQuery client for the public Stellar Hubble dataset.
//!
//! Hubble's `contract_data` table is a history of changes, not a transaction
//! ordered snapshot. This module therefore only answers "latest known contract
//! data at or before ledger N" and intentionally does not accept a transaction
//! hash. Callers must retain the meta/history-archive path for transaction
//! boundaries and for unsupported ledger-key types.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use soroban_sdk::xdr::{
    ContractDataEntry, LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, Limits, ReadXdr,
    WriteXdr,
};
use std::io::Read;

const DEFAULT_PROJECT_ID: &str = "crypto-stellar";
const DEFAULT_DATASET_ID: &str = "crypto_stellar";
const BIGQUERY_API_ROOT: &str = "https://bigquery.googleapis.com/bigquery/v2/projects";

/// Network identity understood by the public Hubble dataset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HubbleNetwork {
    /// The public `crypto-stellar.crypto_stellar` dataset.
    Mainnet,
    /// Hubble does not publish a network discriminator or documented reset
    /// partition for testnet, so testnet queries are rejected rather than
    /// risking a result from the wrong epoch.
    Testnet { reset_ledger: u32 },
}

/// Result of a historical contract-data lookup.
#[derive(Debug, PartialEq, Eq)]
pub enum ContractDataLookup {
    /// The target ledger is not present in Hubble, so the caller should use
    /// its authoritative fallback.
    Missing,
    /// The key's latest change at or before the target ledger was a deletion.
    Deleted,
    /// The key's latest change at or before the target ledger is live.
    Live(LedgerEntry),
}

/// Configuration for querying Hubble through the BigQuery REST API.
#[derive(Clone)]
pub struct HubbleConfig {
    /// GCP project billed for the BigQuery query job.
    pub project_id: String,
    /// GCP project that owns the public Hubble dataset.
    pub dataset_project_id: String,
    pub dataset_id: String,
    pub access_token: String,
    pub network: HubbleNetwork,
    pub endpoint: String,
    /// Highest ledger for which the caller has verified Hubble state-table
    /// completeness. Without this, lookups safely fall back to the archive.
    pub state_watermark: Option<u32>,
}

impl std::fmt::Debug for HubbleConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HubbleConfig")
            .field("project_id", &self.project_id)
            .field("dataset_project_id", &self.dataset_project_id)
            .field("dataset_id", &self.dataset_id)
            .field("access_token", &"<redacted>")
            .field("network", &self.network)
            .field("endpoint", &self.endpoint)
            .field("state_watermark", &self.state_watermark)
            .finish()
    }
}

impl HubbleConfig {
    /// Configure the documented public mainnet dataset.
    ///
    /// `project_id` is the caller's billed/query project, not the public
    /// dataset owner (`crypto-stellar`).
    pub fn mainnet(project_id: impl Into<String>, access_token: impl Into<String>) -> Self {
        Self {
            project_id: project_id.into(),
            dataset_project_id: DEFAULT_PROJECT_ID.to_string(),
            dataset_id: DEFAULT_DATASET_ID.to_string(),
            access_token: access_token.into(),
            network: HubbleNetwork::Mainnet,
            endpoint: format!("{BIGQUERY_API_ROOT}/{DEFAULT_PROJECT_ID}/queries"),
            state_watermark: None,
        }
    }

    /// Return a copy using a different BigQuery dataset.
    pub fn with_dataset(
        mut self,
        dataset_project_id: impl Into<String>,
        dataset_id: impl Into<String>,
    ) -> Self {
        self.dataset_project_id = dataset_project_id.into();
        self.dataset_id = dataset_id.into();
        self.endpoint = format!("{BIGQUERY_API_ROOT}/{}/queries", self.project_id);
        self
    }

    /// Return a copy for a caller-supplied network identity.
    pub fn with_network(mut self, network: HubbleNetwork) -> Self {
        self.network = network;
        self
    }

    /// Return a copy using a mock or proxy BigQuery endpoint.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Return a copy enabled through the caller's verified state-table
    /// completeness watermark.
    pub fn with_state_watermark(mut self, ledger: u32) -> Self {
        self.state_watermark = Some(ledger);
        self
    }
}

/// The parameterized query and values sent to BigQuery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryRequest {
    pub query: String,
    pub parameters: Vec<QueryParameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryParameter {
    pub name: String,
    pub type_name: String,
    pub value: String,
}

/// Errors returned by the Hubble prototype.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hubble does not publish a network discriminator for {network:?} (reset ledger {reset_ledger})")]
    UnsupportedNetwork {
        network: HubbleNetwork,
        reset_ledger: u32,
    },
    #[error("Hubble contract_data lookup does not support ledger key {0:?}")]
    UnsupportedLedgerKey(LedgerKey),
    #[error("invalid BigQuery identifier {0:?}")]
    InvalidIdentifier(String),
    #[error("BigQuery request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("BigQuery returned HTTP status {0}")]
    HttpStatus(reqwest::StatusCode),
    #[error("BigQuery response JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("BigQuery query failed: {0}")]
    BigQuery(String),
    #[error("BigQuery response row is malformed: {0}")]
    Row(String),
    #[error("Hubble state-table completeness watermark is required")]
    MissingStateWatermark,
    #[error("Hubble state-table watermark {watermark} is before ledger {ledger}")]
    StaleState { ledger: u32, watermark: u32 },
    #[error(
        "Hubble returned more than one change for ledger {0}; transaction order is unavailable"
    )]
    AmbiguousLedger(u32),
    #[error("contract data XDR failed: {0}")]
    Xdr(#[from] soroban_sdk::xdr::Error),
}

/// Build a bounded, parameterized lookup for the latest contract-data change
/// at or before `ledger`.
pub fn contract_data_query(
    config: &HubbleConfig,
    key: &LedgerKey,
    ledger: u32,
) -> Result<QueryRequest, Error> {
    if let HubbleNetwork::Testnet { reset_ledger } = config.network {
        return Err(Error::UnsupportedNetwork {
            network: config.network.clone(),
            reset_ledger,
        });
    }

    if !matches!(key, LedgerKey::ContractData(_)) {
        return Err(Error::UnsupportedLedgerKey(key.clone()));
    }

    match config.state_watermark {
        Some(watermark) if ledger <= watermark => {}
        Some(watermark) => return Err(Error::StaleState { ledger, watermark }),
        None => return Err(Error::MissingStateWatermark),
    }

    let dataset = qualified_table(
        &config.dataset_project_id,
        &config.dataset_id,
        "contract_data",
    )?;
    let ledgers = qualified_table(
        &config.dataset_project_id,
        &config.dataset_id,
        "history_ledgers",
    )?;
    let key_hash = ledger_key_hash(key)?;
    Ok(QueryRequest {
        query: format!(
            "WITH target_ledger AS ( \
                 SELECT closed_at \
                 FROM `{ledgers}` \
                 WHERE sequence = @ledger_sequence \
                 LIMIT 1 \
             ) \
             SELECT c.ledger_sequence, c.last_modified_ledger, c.deleted, c.contract_data_xdr \
             FROM `{dataset}` AS c \
             JOIN target_ledger AS t ON c.closed_at <= t.closed_at \
             WHERE c.ledger_key_hash = @ledger_key_hash \
               AND c.ledger_sequence <= @ledger_sequence \
               AND c.last_modified_ledger <= @ledger_sequence \
             ORDER BY c.ledger_sequence DESC \
             LIMIT 2"
        ),
        parameters: vec![
            QueryParameter {
                name: "ledger_key_hash".to_string(),
                type_name: "STRING".to_string(),
                value: key_hash,
            },
            QueryParameter {
                name: "ledger_sequence".to_string(),
                type_name: "INT64".to_string(),
                value: ledger.to_string(),
            },
        ],
    })
}

fn qualified_table(project_id: &str, dataset_id: &str, table: &str) -> Result<String, Error> {
    for identifier in [project_id, dataset_id, table] {
        if identifier.is_empty()
            || !identifier
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(Error::InvalidIdentifier(identifier.to_string()));
        }
    }
    Ok(format!("{project_id}.{dataset_id}.{table}"))
}

fn ledger_key_hash(key: &LedgerKey) -> Result<String, Error> {
    Ok(hex::encode(Sha256::digest(&key.to_xdr(Limits::none())?)))
}

/// BigQuery REST client. The access token is supplied by the caller so this
/// crate does not impose a credential-discovery dependency.
#[derive(Clone, Debug)]
pub struct HubbleClient {
    config: HubbleConfig,
    client: reqwest::blocking::Client,
}

impl HubbleClient {
    pub fn new(config: HubbleConfig) -> Self {
        Self {
            config,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn network(&self) -> &HubbleNetwork {
        &self.config.network
    }

    /// Query the latest known contract-data row at or before `ledger`.
    ///
    /// The caller should retain its history-archive fallback because Hubble is
    /// batch-updated and does not provide a completeness watermark beyond the
    /// target-ledger existence check.
    pub fn contract_data(&self, key: &LedgerKey, ledger: u32) -> Result<ContractDataLookup, Error> {
        let request = contract_data_query(&self.config, key, ledger)?;
        let body = json!({
            "query": request.query,
            "useLegacySql": false,
            "parameterMode": "NAMED",
            "timeoutMs": 10_000,
            "maxResults": 2,
            "queryParameters": request.parameters.iter().map(|parameter| json!({
                "name": parameter.name,
                "parameterType": { "type": parameter.type_name },
                "parameterValue": { "value": parameter.value },
            })).collect::<Vec<_>>(),
        });

        let response = self
            .client
            .post(&self.config.endpoint)
            .bearer_auth(&self.config.access_token)
            .json(&body)
            .send()?;
        if !response.status().is_success() {
            return Err(Error::HttpStatus(response.status()));
        }
        parse_lookup_response(response)
    }
}

/// Parse a BigQuery response without making a network request.
pub fn parse_response<R: Read>(reader: R) -> Result<Option<LedgerEntry>, Error> {
    Ok(match parse_lookup_response(reader)? {
        ContractDataLookup::Live(entry) => Some(entry),
        ContractDataLookup::Missing | ContractDataLookup::Deleted => None,
    })
}

/// Parse a BigQuery response while preserving a deletion tombstone.
pub fn parse_lookup_response<R: Read>(reader: R) -> Result<ContractDataLookup, Error> {
    let response: BigQueryResponse = serde_json::from_reader(reader)?;
    if let Some(error) = response.error {
        return Err(Error::BigQuery(error.message));
    }
    if let Some(errors) = response.errors {
        if !errors.is_empty() {
            return Err(Error::BigQuery(
                errors
                    .into_iter()
                    .map(|error| error.message)
                    .collect::<Vec<_>>()
                    .join("; "),
            ));
        }
    }
    if response.job_complete == Some(false) {
        return Err(Error::BigQuery(
            "BigQuery query did not complete".to_string(),
        ));
    }

    let rows = response.rows.unwrap_or_default();
    let Some(row) = rows.first() else {
        return Ok(ContractDataLookup::Missing);
    };
    if row.f.len() != 4 {
        return Err(Error::Row(format!(
            "expected four selected fields, got {}",
            row.f.len()
        )));
    }

    let ledger_sequence = value_string(&row.f[0], "ledger_sequence")?
        .parse::<u32>()
        .map_err(|error| Error::Row(format!("invalid ledger_sequence: {error}")))?;
    let last_modified_ledger_seq = value_string(&row.f[1], "last_modified_ledger")?
        .parse::<u32>()
        .map_err(|error| Error::Row(format!("invalid last_modified_ledger: {error}")))?;
    let deleted = value_string(&row.f[2], "deleted")?
        .parse::<bool>()
        .map_err(|error| Error::Row(format!("invalid deleted flag: {error}")))?;
    if let Some(next_row) = rows.get(1) {
        if next_row.f.len() != 4 {
            return Err(Error::Row(format!(
                "expected four selected fields, got {}",
                next_row.f.len()
            )));
        }
        let next_ledger_sequence = value_string(&next_row.f[0], "ledger_sequence")?
            .parse::<u32>()
            .map_err(|error| Error::Row(format!("invalid ledger_sequence: {error}")))?;
        if next_ledger_sequence == ledger_sequence {
            return Err(Error::AmbiguousLedger(ledger_sequence));
        }
    }
    if deleted {
        return Ok(ContractDataLookup::Deleted);
    }
    let xdr = value_string(&row.f[3], "contract_data_xdr")?;
    let contract_data = ContractDataEntry::from_xdr_base64(xdr, Limits::none())?;
    Ok(ContractDataLookup::Live(LedgerEntry {
        data: LedgerEntryData::ContractData(contract_data),
        last_modified_ledger_seq,
        ext: LedgerEntryExt::V0,
    }))
}

fn value_string<'a>(field: &'a BigQueryField, name: &str) -> Result<&'a str, Error> {
    field
        .v
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Row(format!("{name} is missing or null")))
}

#[derive(serde::Deserialize)]
struct BigQueryResponse {
    rows: Option<Vec<BigQueryRow>>,
    error: Option<BigQueryError>,
    errors: Option<Vec<BigQueryError>>,
    #[serde(rename = "jobComplete")]
    job_complete: Option<bool>,
}

#[derive(serde::Deserialize)]
struct BigQueryError {
    message: String,
}

#[derive(serde::Deserialize)]
struct BigQueryRow {
    f: Vec<BigQueryField>,
}

#[derive(serde::Deserialize)]
struct BigQueryField {
    v: Value,
}

#[cfg(test)]
mod test {
    use super::{
        contract_data_query, parse_lookup_response, parse_response, ContractDataLookup, Error,
        HubbleConfig, HubbleNetwork,
    };
    use serde_json::json;
    use soroban_sdk::xdr::{
        ContractDataDurability, ContractDataEntry, ContractId, ExtensionPoint, Hash, LedgerKey,
        LedgerKeyContractData, Limits, ScAddress, ScVal, WriteXdr,
    };
    use std::io::Cursor;

    fn key() -> LedgerKey {
        LedgerKey::ContractData(LedgerKeyContractData {
            contract: ScAddress::Contract(ContractId(Hash([7; 32]))),
            key: ScVal::Symbol(soroban_sdk::xdr::ScSymbol(
                soroban_sdk::xdr::StringM::try_from("key").unwrap(),
            )),
            durability: ContractDataDurability::Persistent,
        })
    }

    #[test]
    fn query_is_parameterized_and_bounded() {
        let config = HubbleConfig::mainnet("billing-project", "token").with_state_watermark(123);
        let request = contract_data_query(&config, &key(), 123).unwrap();
        assert!(request
            .query
            .contains("`crypto-stellar.crypto_stellar.contract_data`"));
        assert!(request
            .query
            .contains("ledger_sequence <= @ledger_sequence"));
        assert!(request
            .query
            .contains("FROM `crypto-stellar.crypto_stellar.history_ledgers`"));
        assert!(request.query.contains("ORDER BY c.ledger_sequence DESC"));
        assert!(request.query.contains("LIMIT 2"));
        assert_eq!(request.parameters[1].value, "123");
        assert!(!request.query.contains("token"));
        assert_eq!(config.project_id, "billing-project");
    }

    #[test]
    fn query_requires_a_verified_state_watermark() {
        let config = HubbleConfig::mainnet("billing-project", "token");
        assert!(matches!(
            contract_data_query(&config, &key(), 123),
            Err(Error::MissingStateWatermark)
        ));

        let config = config.with_state_watermark(122);
        assert!(matches!(
            contract_data_query(&config, &key(), 123),
            Err(Error::StaleState {
                ledger: 123,
                watermark: 122
            })
        ));
    }

    #[test]
    fn testnet_reset_is_rejected_without_a_dataset_partition() {
        let config = HubbleConfig::mainnet("billing-project", "token")
            .with_state_watermark(100)
            .with_network(HubbleNetwork::Testnet { reset_ledger: 42 });
        let error = contract_data_query(&config, &key(), 100).unwrap_err();
        assert!(matches!(
            error,
            Error::UnsupportedNetwork {
                reset_ledger: 42,
                ..
            }
        ));
    }

    #[test]
    fn unsupported_key_is_not_silently_mapped_to_contract_data() {
        let config = HubbleConfig::mainnet("billing-project", "token").with_state_watermark(100);
        let error = contract_data_query(
            &config,
            &LedgerKey::ContractCode(soroban_sdk::xdr::LedgerKeyContractCode {
                hash: Hash([1; 32]),
            }),
            100,
        )
        .unwrap_err();
        assert!(matches!(error, Error::UnsupportedLedgerKey(_)));
    }

    #[test]
    fn deleted_row_is_absent() {
        let response = json!({
            "rows": [{
                "f": [
                    {"v": "20"},
                    {"v": "18"},
                    {"v": "true"},
                    {"v": null}
                ]
            }]
        });
        assert_eq!(
            parse_lookup_response(Cursor::new(response.to_string())).unwrap(),
            ContractDataLookup::Deleted
        );
    }

    #[test]
    fn same_ledger_rows_are_rejected_as_ambiguous() {
        let response = json!({
            "rows": [
                {"f": [{"v": "20"}, {"v": "20"}, {"v": "false"}, {"v": "ignored"}]},
                {"f": [{"v": "20"}, {"v": "19"}, {"v": "true"}, {"v": null}]}
            ]
        });
        assert!(matches!(
            parse_lookup_response(Cursor::new(response.to_string())),
            Err(Error::AmbiguousLedger(20))
        ));
    }

    #[test]
    fn live_row_decodes_contract_data_xdr() {
        let data = ContractDataEntry {
            ext: ExtensionPoint::V0,
            contract: ScAddress::Contract(ContractId(Hash([7; 32]))),
            key: ScVal::U32(1),
            durability: ContractDataDurability::Persistent,
            val: ScVal::U32(2),
        };
        let response = json!({
            "rows": [{
                "f": [
                    {"v": "20"},
                    {"v": "18"},
                    {"v": "false"},
                    {"v": data.to_xdr_base64(Limits::none()).unwrap()}
                ]
            }]
        });
        let entry = parse_response(Cursor::new(response.to_string()))
            .unwrap()
            .unwrap();
        assert_eq!(entry.last_modified_ledger_seq, 18);
        assert!(matches!(
            entry.data,
            soroban_sdk::xdr::LedgerEntryData::ContractData(_)
        ));
    }

    #[test]
    fn query_errors_are_returned() {
        let response = r#"{"error":{"message":"access denied"}}"#;
        assert!(matches!(
            parse_response(Cursor::new(response)),
            Err(Error::BigQuery(message)) if message == "access denied"
        ));
    }

    #[test]
    fn query_job_errors_and_incomplete_jobs_are_returned() {
        let response = r#"{"errors":[{"message":"invalid query"}]}"#;
        assert!(matches!(
            parse_response(Cursor::new(response)),
            Err(Error::BigQuery(message)) if message == "invalid query"
        ));

        let response = r#"{"jobComplete":false}"#;
        assert!(matches!(
            parse_response(Cursor::new(response)),
            Err(Error::BigQuery(message)) if message == "BigQuery query did not complete"
        ));
    }
}
