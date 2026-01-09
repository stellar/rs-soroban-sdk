use flate2::read::GzDecoder;
use std::io::{self, copy, Cursor, Write};
use stellar_xdr::curr::{self as xdr, Frame, Limited, ReadXdr};

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
}

pub fn history(archive_url: &str, ledger: u32) -> Result<History, Error> {
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
        let ledger_hex_0 = ledger_hex[0..=1].to_string();
        let ledger_hex_1 = ledger_hex[2..=3].to_string();
        let ledger_hex_2 = ledger_hex[4..=5].to_string();
        format!("{archive_url}/history/{ledger_hex_0}/{ledger_hex_1}/{ledger_hex_2}/history-{ledger_hex}.json")
    };
    //eprintln!("url: {history_url}");

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
    let bucket_0 = &bucket[0..=1];
    let bucket_1 = &bucket[2..=3];
    let bucket_2 = &bucket[4..=5];
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
