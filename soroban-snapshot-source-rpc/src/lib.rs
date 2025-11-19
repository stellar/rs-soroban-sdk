use serde_json::json;
use soroban_sdk::{
    testutils::{HostError, SnapshotSource, SnapshotSourceInput},
    xdr::{
        LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, Limits, ReadXdr, ScErrorCode,
        ScErrorType, WriteXdr,
    },
};
use std::rc::Rc;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http")]
    Http(#[from] reqwest::Error),
    #[error("serde")]
    Serde(#[from] serde_json::Error),
    #[error("rpc error: {0}")]
    Rpc(String),
}

/// Snapshot source that fetches ledger entries from stellar-rpc
pub struct RpcSnapshotSource {
    client: reqwest::blocking::Client,
    rpc_url: String,
}

impl RpcSnapshotSource {
    /// Create a new RPC snapshot source
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            rpc_url: rpc_url.to_string(),
        }
    }

    /// Fetch a single ledger entry from the RPC
    fn fetch_ledger_entry(
        &self,
        key: &LedgerKey,
    ) -> Result<Option<(LedgerEntry, Option<u32>)>, Error> {
        // Convert key to base64 XDR for the RPC call
        let key_xdr = key
            .to_xdr_base64(Limits::none())
            .map_err(|e| Error::Rpc(format!("XDR encoding error: {}", e)))?;

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getLedgerEntries".to_string(),
            params: json!({
                "keys": [key_xdr]
            }),
        };

        let response: RpcResponse<GetLedgerEntriesResponse> = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()?
            .json()?;

        if let Some(error) = response.error {
            return Err(Error::Rpc(error.message));
        }

        let result = response
            .result
            .ok_or_else(|| Error::Rpc("No result in response".to_string()))?;

        // Process the response
        if let Some(entry) = result.entries.into_iter().next() {
            let data = LedgerEntryData::from_xdr_base64(&entry.xdr, Limits::none())
                .map_err(|e| Error::Rpc(format!("XDR decode error: {}", e)))?;

            let ledger_entry = LedgerEntry {
                data,
                last_modified_ledger_seq: entry.last_modified_ledger_seq,
                // The RPC does not expose the extension information, so setting this to v0 and for
                // contract tests this should have no material impact.
                ext: LedgerEntryExt::V0,
            };

            Ok(Some((ledger_entry, entry.live_until_ledger_seq)))
        } else {
            Ok(None)
        }
    }
}

impl SnapshotSource for RpcSnapshotSource {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
        match self.fetch_ledger_entry(key) {
            Ok(Some((entry, ttl))) => Ok(Some((Rc::new(entry), ttl))),
            Ok(None) => Ok(None),
            Err(err) => {
                eprintln!("Error in RPC Snapshot Source: {err:?}");
                Err(HostError::from(soroban_sdk::Error::from((
                    ScErrorType::Storage,
                    ScErrorCode::InternalError,
                ))))
            }
        }
    }
}

impl From<RpcSnapshotSource> for SnapshotSourceInput {
    fn from(source: RpcSnapshotSource) -> Self {
        Self {
            source: Rc::new(source),
            ledger_info: None,
            snapshot: None,
        }
    }
}

#[derive(serde::Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u32,
    method: String,
    params: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
}

#[derive(serde::Deserialize)]
struct RpcError {
    message: String,
}

/// Response from getLedgerEntries
#[derive(serde::Deserialize)]
struct GetLedgerEntriesResponse {
    entries: Vec<GetLedgerEntriesResponseEntry>,
}

#[derive(serde::Deserialize)]
struct GetLedgerEntriesResponseEntry {
    xdr: String,
    #[serde(rename = "lastModifiedLedgerSeq")]
    last_modified_ledger_seq: u32,
    #[serde(rename = "liveUntilLedgerSeq")]
    live_until_ledger_seq: Option<u32>,
}
