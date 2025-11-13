use async_compression::tokio::bufread::GzipDecoder;
use futures::StreamExt;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    io,
    path::PathBuf,
};
use stellar_xdr::curr::{
    self as xdr, BucketEntry, Frame, Limited, Limits, ReadXdr,
};
use tokio::fs::OpenOptions;
use tokio::io::BufReader;
use tokio_util::io::StreamReader;
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json decoding failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("xdr decoding failed: {0}")]
    Xdr(#[from] xdr::Error),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("url parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("bucket hash invalid: {0}")]
    BucketHashInvalid(String),

    #[error("corrupted bucket file: expected hash {expected}, got {actual}")]
    CorruptedBucket { expected: String, actual: String },

    #[error("ledger not found in archive")]
    LedgerNotFound,

    #[error("invalid archive url")]
    InvalidArchiveUrl,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Client for accessing Stellar History Archives
///
/// This client provides methods to retrieve ledger snapshots and bucket data
/// from Stellar History Archives. It handles caching of downloaded data to
/// improve performance for subsequent requests.
///
/// # Example
///
/// ```no_run
/// use soroban_ledger_snapshot_history_archive::HistoryArchiveClient;
/// use url::Url;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let archive_url = Url::parse("https://history.stellar.org/prd/core-live/core_live_001")?;
/// let mut client = HistoryArchiveClient::new(archive_url);
///
/// // Get the latest ledger sequence
/// let latest_ledger = client.get_latest_ledger_seq().await?;
/// println!("Latest ledger: {}", latest_ledger);
///
/// // Get buckets for a specific ledger
/// let buckets = client.get_buckets_for_ledger_seq(latest_ledger).await?;
/// println!("Found {} buckets", buckets.len());
///
/// // Get data for the first bucket
/// if let Some(bucket_hash) = buckets.first() {
///     let bucket_data = client.get_bucket(bucket_hash).await?;
///     println!("Bucket contains {} entries", bucket_data.len());
/// }
/// # Ok(())
/// # }
/// ```
pub struct HistoryArchiveClient {
    archive_url: Url,
    http_client: reqwest::Client,
    cache_dir: PathBuf,
    // Cache for history JSON files keyed by ledger sequence
    history_cache: HashMap<u32, History>,
    // Cache for bucket data keyed by bucket hash
    bucket_cache: HashMap<String, Vec<Frame<BucketEntry>>>,
}

impl HistoryArchiveClient {
    /// Create a new HistoryArchiveClient with the given archive URL
    pub fn new(archive_url: Url) -> Self {
        let cache_dir = std::env::temp_dir().join("soroban-history-cache");
        std::fs::create_dir_all(&cache_dir).ok(); // Ignore errors if dir already exists

        Self {
            archive_url,
            http_client: reqwest::Client::new(),
            cache_dir,
            history_cache: HashMap::new(),
            bucket_cache: HashMap::new(),
        }
    }

    /// Get the latest ledger sequence from the root history JSON file
    pub async fn get_latest_ledger_seq(&mut self) -> Result<u32> {
        let history = self.get_history(None).await?;
        Ok(history.current_ledger)
    }

    /// Get the full list of buckets for a specific ledger sequence, in order
    /// as they appear in the history JSON file. Results are cached.
    pub async fn get_buckets_for_ledger_seq(&mut self, ledger_seq: u32) -> Result<Vec<String>> {
        let history = self.get_history(Some(ledger_seq)).await?;
        let buckets = history
            .current_buckets
            .iter()
            .flat_map(|h| [h.curr.clone(), h.snap.clone()])
            .filter(|b| b != "0000000000000000000000000000000000000000000000000000000000000000")
            .collect::<Vec<_>>();
        Ok(buckets)
    }

    /// Get a bucket by its hash. Downloads, decompresses, and decodes the bucket
    /// as `Frame<BucketEntry>`. Results are cached.
    pub async fn get_bucket(&mut self, bucket_hash: &str) -> Result<Vec<Frame<BucketEntry>>> {
        if let Some(cached) = self.bucket_cache.get(bucket_hash) {
            return Ok(cached.clone());
        }

        let bucket_path = self.download_bucket(bucket_hash).await?;
        let entries = self.read_bucket_file(&bucket_path).await?;
        
        // Validate the bucket hash
        self.validate_bucket_hash(&bucket_path, bucket_hash)?;

        self.bucket_cache.insert(bucket_hash.to_string(), entries.clone());
        Ok(entries)
    }

    async fn get_history(&mut self, ledger: Option<u32>) -> Result<History> {
        if let Some(ledger) = ledger {
            if let Some(cached) = self.history_cache.get(&ledger) {
                return Ok(cached.clone());
            }
        }

        let history_url = self.build_history_url(ledger)?;
        let response = self.http_client.get(history_url).send().await?;
        
        if !response.status().is_success() {
            return Err(Error::Http(response.error_for_status().unwrap_err()));
        }

        let body = response.bytes().await?;
        let history: History = serde_json::from_slice(&body)?;

        if let Some(ledger) = ledger {
            if history.current_ledger != ledger {
                return Err(Error::LedgerNotFound);
            }
            self.history_cache.insert(ledger, history.clone());
        } else {
            // For latest, cache by the actual ledger number
            self.history_cache.insert(history.current_ledger, history.clone());
        }

        Ok(history)
    }

    fn build_history_url(&self, ledger: Option<u32>) -> Result<Url> {
        let mut archive_url = self.archive_url.to_string();
        if !archive_url.ends_with('/') {
            archive_url.push('/');
        }

        let history_path = if let Some(ledger) = ledger {
            let ledger_hex = format!("{:08x}", ledger);
            let ledger_hex_0 = &ledger_hex[0..2];
            let ledger_hex_1 = &ledger_hex[2..4];
            let ledger_hex_2 = &ledger_hex[4..6];
            format!("history/{}/{}/{}/history-{}.json", ledger_hex_0, ledger_hex_1, ledger_hex_2, ledger_hex)
        } else {
            ".well-known/stellar-history.json".to_string()
        };

        Url::parse(&(archive_url + &history_path)).map_err(Error::UrlParse)
    }

    async fn download_bucket(&self, bucket_hash: &str) -> Result<PathBuf> {
        let cache_path = self.cache_dir.join(format!("bucket-{}.xdr", bucket_hash));

        // Check if already cached
        if cache_path.exists() {
            if self.validate_bucket_hash(&cache_path, bucket_hash).is_ok() {
                return Ok(cache_path);
            }
            // Remove corrupted file
            std::fs::remove_file(&cache_path).ok();
        }

        let bucket_0 = &bucket_hash[0..2];
        let bucket_1 = &bucket_hash[2..4];
        let bucket_2 = &bucket_hash[4..6];
        let bucket_url = format!(
            "{}/bucket/{}/{}/{}/bucket-{}.xdr.gz",
            self.archive_url, bucket_0, bucket_1, bucket_2, bucket_hash
        );

        let bucket_url = Url::parse(&bucket_url)?;
        let response = self.http_client.get(bucket_url).send().await?;
        
        if !response.status().is_success() {
            return Err(Error::Http(response.error_for_status().unwrap_err()));
        }

        let stream = response.bytes_stream()
            .map(|result| result.map_err(std::io::Error::other));
        let stream_reader = StreamReader::new(stream);
        let buf_reader = BufReader::new(stream_reader);
        let mut decoder = GzipDecoder::new(buf_reader);
        
        let dl_path = cache_path.with_extension("dl");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&dl_path)
            .await?;

        tokio::io::copy(&mut decoder, &mut file).await?;
        std::fs::rename(&dl_path, &cache_path)?;

        Ok(cache_path)
    }

    async fn read_bucket_file(&self, path: &PathBuf) -> Result<Vec<Frame<BucketEntry>>> {
        let file = std::fs::File::open(path)?;
        let limited = &mut Limited::new(file, Limits::none());
        let entries = Frame::<BucketEntry>::read_xdr_iter(limited)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(entries)
    }

    fn validate_bucket_hash(&self, cache_path: &PathBuf, expected_hash: &str) -> Result<()> {
        let file = std::fs::File::open(cache_path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut std::io::BufReader::new(file), &mut hasher)?;
        let actual_hash = hex::encode(hasher.finalize());

        if actual_hash != expected_hash {
            return Err(Error::CorruptedBucket {
                expected: expected_hash.to_string(),
                actual: actual_hash,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct History {
    current_ledger: u32,
    current_buckets: Vec<HistoryBucket>,
    network_passphrase: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryBucket {
    curr: String,
    snap: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_client_creation() {
        let archive_url = Url::parse("https://history.stellar.org/prd/core-live/core_live_001").unwrap();
        let _client = HistoryArchiveClient::new(archive_url);
        // Just test that creation works
    }
}