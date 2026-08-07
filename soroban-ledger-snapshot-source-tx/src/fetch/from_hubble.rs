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
    LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, Limits, ReadXdr, WriteXdr,
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

/// Configuration for querying Hubble through the BigQuery REST API.
#[derive(Clone)]
pub struct HubbleConfig {
    pub project_id: String,
    pub dataset_id: String,
    pub access_token: String,
    pub network: HubbleNetwork,
    pub endpoint: String,
}

impl std::fmt::Debug for HubbleConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HubbleConfig")
            .field("project_id", &self.project_id)
            .field("dataset_id", &self.dataset_id)
            .field("access_token", &"<redacted>")
            .field("network", &self.network)
            .field("endpoint", &self.endpoint)
            .finish()
    }
}

impl HubbleConfig {
    /// Configure the documented public mainnet dataset.
    pub fn mainnet(access_token: impl Into<String>) -> Self {
        Self {
            project_id: DEFAULT_PROJECT_ID.to_string(),
            dataset_id: DEFAULT_DATASET_ID.to_string(),
            access_token: access_token.into(),
            network: HubbleNetwork::Mainnet,
            endpoint: format!("{BIGQUERY_API_ROOT}/{DEFAULT_PROJECT_ID}/queries"),
        }
    }

    /// Return a copy using a different BigQuery dataset.
    pub fn with_dataset(
        mut self,
        project_id: impl Into<String>,
        dataset_id: impl Into<String>,
    ) -> Self {
        self.project_id = project_id.into();
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

    let table = qualified_table(&config.project_id, &config.dataset_id, "contract_data")?;
    let key_hash = ledger_key_hash(key)?;
    Ok(QueryRequest {
        query: format!(
            "SELECT ledger_sequence, last_modified_ledger, deleted, contract_data_xdr \
             FROM `{table}` \
             WHERE ledger_key_hash = @ledger_key_hash \
               AND ledger_sequence <= @ledger_sequence \
             ORDER BY ledger_sequence DESC, deleted DESC \
             LIMIT 1"
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

    /// Query the latest known contract-data row at or before `ledger`.
    ///
    /// `None` means no usable row was returned. The caller should retain its
    /// history-archive fallback because Hubble is batch-updated and does not
    /// provide a completeness watermark in this query.
    pub fn contract_data(
        &self,
        key: &LedgerKey,
        ledger: u32,
    ) -> Result<Option<LedgerEntry>, Error> {
        let request = contract_data_query(&self.config, key, ledger)?;
        let body = json!({
            "query": request.query,
            "useLegacySql": false,
            "parameterMode": "NAMED",
            "timeoutMs": 10_000,
            "maxResults": 1,
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
        parse_response(response)
    }
}

/// Parse a BigQuery response without making a network request.
pub fn parse_response<R: Read>(reader: R) -> Result<Option<LedgerEntry>, Error> {
    let response: BigQueryResponse = serde_json::from_reader(reader)?;
    if let Some(error) = response.error {
        return Err(Error::BigQuery(error.message));
    }

    let Some(row) = response.rows.and_then(|mut rows| rows.pop()) else {
        return Ok(None);
    };
    if row.f.len() != 4 {
        return Err(Error::Row(format!(
            "expected four selected fields, got {}",
            row.f.len()
        )));
    }

    let _ledger_sequence = value_string(&row.f[0], "ledger_sequence")?
        .parse::<u32>()
        .map_err(|error| Error::Row(format!("invalid ledger_sequence: {error}")))?;
    let last_modified_ledger_seq = value_string(&row.f[1], "last_modified_ledger")?
        .parse::<u32>()
        .map_err(|error| Error::Row(format!("invalid last_modified_ledger: {error}")))?;
    let deleted = value_string(&row.f[2], "deleted")?
        .parse::<bool>()
        .map_err(|error| Error::Row(format!("invalid deleted flag: {error}")))?;
    if deleted {
        return Ok(None);
    }
    let xdr = value_string(&row.f[3], "contract_data_xdr")?;
    let data = LedgerEntryData::from_xdr_base64(xdr, Limits::none())?;
    Ok(Some(LedgerEntry {
        data,
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
    use super::{contract_data_query, parse_response, Error, HubbleConfig, HubbleNetwork};
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
        let config = HubbleConfig::mainnet("token");
        let request = contract_data_query(&config, &key(), 123).unwrap();
        assert!(request
            .query
            .contains("`crypto-stellar.crypto_stellar.contract_data`"));
        assert!(request
            .query
            .contains("ledger_sequence <= @ledger_sequence"));
        assert!(request
            .query
            .contains("ORDER BY ledger_sequence DESC, deleted DESC"));
        assert!(request.query.contains("LIMIT 1"));
        assert_eq!(request.parameters[1].value, "123");
        assert!(!request.query.contains("token"));
    }

    #[test]
    fn testnet_reset_is_rejected_without_a_dataset_partition() {
        let config = HubbleConfig::mainnet("token")
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
        let config = HubbleConfig::mainnet("token");
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
        assert!(parse_response(Cursor::new(response.to_string()))
            .unwrap()
            .is_none());
    }

    #[test]
    fn live_row_decodes_contract_data_xdr() {
        let data = soroban_sdk::xdr::LedgerEntryData::ContractData(ContractDataEntry {
            ext: ExtensionPoint::V0,
            contract: ScAddress::Contract(ContractId(Hash([7; 32]))),
            key: ScVal::U32(1),
            durability: ContractDataDurability::Persistent,
            val: ScVal::U32(2),
        });
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
}
