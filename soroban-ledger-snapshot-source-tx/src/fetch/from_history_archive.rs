use flate2::read::GzDecoder;
use std::io::{self, copy, Cursor, Write};
use stellar_xdr::{self as xdr, Frame, Limited, ReadXdr};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("downloading history: {0}")]
    DownloadingHistory(reqwest::Error),

    #[error("downloading history: got status code {0}")]
    DownloadingHistoryGotStatusCode(reqwest::StatusCode),

    #[error("json decoding history: {0}")]
    JsonDecodingHistory(serde_json::Error),

    #[error("getting bucket: {0}")]
    GettingBucket(reqwest::Error),

    #[error("getting bucket: got status code {0}")]
    GettingBucketGotStatusCode(reqwest::StatusCode),

    #[error("streaming bucket: {0}")]
    StreamingBucket(io::Error),

    #[error("streaming history: {0}")]
    StreamingHistory(io::Error),

    #[error("xdr parsing error: {0}")]
    Xdr(#[from] xdr::Error),

    #[error("invalid ledger hex {0:?}: expected at least 6 hex characters")]
    InvalidLedgerHex(String),

    #[error("invalid bucket hash {0:?}: expected at least 6 hex characters")]
    InvalidBucketHash(String),
}

/// Split the first six characters of a hex string into the three two-character
/// directory prefixes used by the history archive's nested layout. Returns
/// `None` if the string is shorter than six bytes or a split would fall on a
/// non-char boundary, so callers can surface a typed error instead of panicking
/// on a malformed/truncated value.
fn hash_prefixes(hex: &str) -> Option<(&str, &str, &str)> {
    Some((hex.get(0..2)?, hex.get(2..4)?, hex.get(4..6)?))
}

#[allow(dead_code)]
pub(crate) fn history(archive_url: &str, ledger: u32) -> Result<History, Error> {
    let mut bytes = Vec::new();
    get_history(archive_url, ledger, &mut bytes)?;
    parse_history(Cursor::new(bytes))
}

pub fn get_history<W: Write + ?Sized>(
    archive_url: &str,
    ledger: u32,
    writer: &mut W,
) -> Result<(), Error> {
    let history_url = {
        let ledger_hex = format!("{ledger:08x}");
        let (ledger_hex_0, ledger_hex_1, ledger_hex_2) = hash_prefixes(&ledger_hex)
            .ok_or_else(|| Error::InvalidLedgerHex(ledger_hex.clone()))?;
        format!("{archive_url}/history/{ledger_hex_0}/{ledger_hex_1}/{ledger_hex_2}/history-{ledger_hex}.json")
    };

    let mut response = reqwest::blocking::Client::new()
        .get(&history_url)
        .send()
        .map_err(Error::DownloadingHistory)?;

    if !response.status().is_success() {
        return Err(Error::DownloadingHistoryGotStatusCode(response.status()));
    }

    copy(&mut response, writer).map_err(Error::StreamingHistory)?;
    Ok(())
}

pub fn parse_history<R: std::io::Read>(reader: R) -> Result<History, Error> {
    let history: History = serde_json::from_reader(reader).map_err(Error::JsonDecodingHistory)?;
    Ok(history)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub current_ledger: u32,
    pub current_buckets: Vec<HistoryBucket>,
    pub hot_archive_buckets: Option<Vec<HistoryBucket>>,
    pub network_passphrase: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryBucket {
    pub curr: String,
    pub snap: String,
}

pub fn get_bucket<W: Write + ?Sized>(
    archive_url: &str,
    bucket: &str,
    writer: &mut W,
) -> Result<Option<u64>, Error> {
    let (bucket_0, bucket_1, bucket_2) =
        hash_prefixes(bucket).ok_or_else(|| Error::InvalidBucketHash(bucket.to_string()))?;
    let bucket_url =
        format!("{archive_url}/bucket/{bucket_0}/{bucket_1}/{bucket_2}/bucket-{bucket}.xdr.gz");

    let response = reqwest::blocking::Client::new()
        .get(&bucket_url)
        .send()
        .map_err(Error::GettingBucket)?;

    if !response.status().is_success() {
        return Err(Error::GettingBucketGotStatusCode(response.status()));
    }

    let content_length = response.content_length();

    let mut decoder = GzDecoder::new(response);
    copy(&mut decoder, writer).map_err(Error::StreamingBucket)?;
    Ok(content_length)
}

pub fn parse_bucket<'a, R: std::io::Read + 'a>(
    reader: &'a mut Limited<R>,
) -> impl Iterator<Item = Result<Frame<xdr::BucketEntry>, xdr::Error>> + 'a {
    Frame::<xdr::BucketEntry>::read_xdr_iter(reader)
}
