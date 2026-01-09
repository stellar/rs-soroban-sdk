use serde_json::json;
use std::io::{copy, Write};
use stellar_xdr::curr::{
    Error as XdrError, LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, Limits, ReadXdr,
    WriteXdr,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http")]
    Http(#[from] reqwest::Error),
    #[error("serde")]
    Serde(#[from] serde_json::Error),
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("xdr")]
    Xdr(#[from] XdrError),
    #[error("rpc error: {0}")]
    Rpc(String),
}

pub fn get_ledger_entry<W: Write + ?Sized>(
    rpc_url: &str,
    key: &LedgerKey,
    write: &mut W,
) -> Result<(), Error> {
    let key_xdr = key.to_xdr_base64(Limits::none()).map_err(Error::Xdr)?;

    let request = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "getLedgerEntries".to_string(),
        params: json!({
            "keys": [key_xdr]
        }),
    };

    let client = reqwest::blocking::Client::new();
    let response = client.post(rpc_url).json(&request).send()?;

    copy(&mut response.bytes()?.as_ref(), write)?;

    Ok(())
}

pub fn parse_ledger_entry<R: std::io::Read>(
    reader: R,
) -> Result<Option<(LedgerEntry, Option<u32>)>, Error> {
    let response: RpcResponse<GetLedgerEntriesResponse> = serde_json::from_reader(reader)?;

    let result = match response {
        RpcResponse::Result { result } => result,
        RpcResponse::Error { error } => return Err(Error::Rpc(error.message)),
    };

    let Some(entry) = result.entries.first() else {
        return Ok(None);
    };

    let data = LedgerEntryData::from_xdr_base64(&entry.xdr, Limits::none()).map_err(Error::Xdr)?;

    let ledger_entry = LedgerEntry {
        data,
        last_modified_ledger_seq: entry.last_modified_ledger_seq,
        // The RPC does not expose the extension information, so setting this to v0 and for
        // contract tests this should have no material impact.
        ext: LedgerEntryExt::V0,
    };

    Ok(Some((ledger_entry, entry.live_until_ledger_seq)))
}

#[derive(serde::Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u32,
    method: String,
    params: serde_json::Value,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum RpcResponse<T> {
    Result { result: T },
    Error { error: RpcError },
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

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetLedgerEntriesResponseEntry {
    xdr: String,
    last_modified_ledger_seq: u32,
    live_until_ledger_seq: Option<u32>,
}
