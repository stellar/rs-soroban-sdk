use serde_json::json;
use std::io::{copy, Write};
use stellar_xdr::{
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

    // Reject non-success responses before writing, so a transient error body
    // (e.g. a 429/5xx error page) is never persisted into the cache and then
    // replayed as a permanent parse failure on every subsequent run.
    let response = response.error_for_status()?;

    copy(&mut response.bytes()?.as_ref(), write)?;

    Ok(())
}

/// Parse a `getLedgerEntries` response.
///
/// Returns the entry (when present), its optional live-until ledger, and the
/// `latestLedger` the RPC node reports having observed. The caller needs
/// `latest_ledger` to decide whether the node is current enough for the answer
/// to be trusted (see `LedgerEntryFetcher::fetch_from_rpc`).
pub fn parse_ledger_entry<R: std::io::Read>(
    reader: R,
) -> Result<Option<(LedgerEntry, Option<u32>, u32)>, Error> {
    let response: RpcResponse<GetLedgerEntriesResponse> = serde_json::from_reader(reader)?;

    let result = match response {
        RpcResponse::Result { result } => result,
        RpcResponse::Error { error } => return Err(Error::Rpc(error.message)),
    };

    let latest_ledger = result.latest_ledger;

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

    Ok(Some((
        ledger_entry,
        entry.live_until_ledger_seq,
        latest_ledger,
    )))
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
#[serde(rename_all = "camelCase")]
struct GetLedgerEntriesResponse {
    entries: Vec<GetLedgerEntriesResponseEntry>,
    // The latest ledger the node has observed. Defaults to 0 when absent (e.g.
    // an older cached response), which conservatively makes any entry look like
    // it came from a lagging node and is therefore treated as not usable.
    #[serde(default)]
    latest_ledger: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetLedgerEntriesResponseEntry {
    xdr: String,
    last_modified_ledger_seq: u32,
    live_until_ledger_seq: Option<u32>,
}

#[cfg(test)]
mod test {
    use super::{parse_ledger_entry, Error};
    use std::io::Cursor;
    use stellar_xdr::{Hash, LedgerEntryData, LedgerEntryExt, Limits, TtlEntry, WriteXdr};

    fn sample_entry_xdr() -> String {
        // A small, fully-deterministic LedgerEntryData to base64-encode into a
        // simulated RPC response.
        LedgerEntryData::Ttl(TtlEntry {
            key_hash: Hash([7u8; 32]),
            live_until_ledger_seq: 123,
        })
        .to_xdr_base64(Limits::none())
        .unwrap()
    }

    #[test]
    fn parses_entry_with_ttl() {
        let xdr = sample_entry_xdr();
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":1,"result":{{"latestLedger":1000,"entries":[{{"xdr":"{xdr}","lastModifiedLedgerSeq":42,"liveUntilLedgerSeq":99}}]}}}}"#,
        );
        let (entry, ttl, latest_ledger) = parse_ledger_entry(Cursor::new(body)).unwrap().unwrap();
        assert_eq!(entry.last_modified_ledger_seq, 42);
        assert_eq!(ttl, Some(99));
        assert_eq!(latest_ledger, 1000);
        // The RPC does not expose extension info, so it is always V0.
        assert!(matches!(entry.ext, LedgerEntryExt::V0));
        assert!(matches!(entry.data, LedgerEntryData::Ttl(_)));
    }

    #[test]
    fn missing_live_until_yields_none_ttl() {
        let xdr = sample_entry_xdr();
        let body = format!(
            r#"{{"result":{{"latestLedger":50,"entries":[{{"xdr":"{xdr}","lastModifiedLedgerSeq":7}}]}}}}"#,
        );
        let (_, ttl, latest_ledger) = parse_ledger_entry(Cursor::new(body)).unwrap().unwrap();
        assert_eq!(ttl, None);
        assert_eq!(latest_ledger, 50);
    }

    #[test]
    fn missing_latest_ledger_defaults_to_zero() {
        // An older cached response may omit latestLedger; it must default to 0
        // so the usability gate treats it as a lagging node rather than failing
        // to parse.
        let xdr = sample_entry_xdr();
        let body =
            format!(r#"{{"result":{{"entries":[{{"xdr":"{xdr}","lastModifiedLedgerSeq":7}}]}}}}"#,);
        let (_, _, latest_ledger) = parse_ledger_entry(Cursor::new(body)).unwrap().unwrap();
        assert_eq!(latest_ledger, 0);
    }

    #[test]
    fn empty_entries_yields_none() {
        let body = r#"{"result":{"entries":[]}}"#;
        assert!(parse_ledger_entry(Cursor::new(body)).unwrap().is_none());
    }

    #[test]
    fn rpc_error_is_surfaced() {
        let body = r#"{"error":{"message":"boom"}}"#;
        let err = parse_ledger_entry(Cursor::new(body)).unwrap_err();
        match err {
            Error::Rpc(msg) => assert_eq!(msg, "boom"),
            other => panic!("expected Rpc error, got {other:?}"),
        }
    }
}
