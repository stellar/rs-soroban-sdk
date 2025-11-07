use std::str::FromStr;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limits, ReadXdr, LedgerCloseMeta};
use thiserror::Error;
use rusoto_core::{Region, credential::StaticProvider, request::HttpClient};
use rusoto_s3::{S3Client, S3, GetObjectRequest, ListObjectsV2Request};
use futures::stream::TryStreamExt;

/// Configuration for connecting to S3-compatible storage
#[derive(Clone, Debug)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub root_path: String,
}

/// Error type for ledger meta downloader operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("S3 operation failed: {0}")]
    S3(#[from] rusoto_core::RusotoError<rusoto_s3::GetObjectError>),
    #[error("S3 list operation failed: {0}")]
    S3List(#[from] rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>),
    #[error("S3 credentials error: {0}")]
    S3Credentials(#[from] rusoto_core::RusotoError<rusoto_credential::CredentialsError>),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XDR parsing error: {0}")]
    Xdr(#[from] stellar_xdr::Error),
    #[error("Ledger sequence {0} not found")]
    LedgerNotFound(u32),
    #[error("Invalid ledger sequence: {0}")]
    InvalidSequence(u32),
}

/// Calculate the S3 path for a given ledger sequence
///
/// This implements the SEP-54 path structure for ledger metadata
fn get_path_for_ledger(ledger_sequence: u32) -> String {
    const PARTITION_SIZE: u32 = 64000;
    const BATCH_SIZE: u32 = 1;
    const MAX_UINT32: u32 = 0xFFFFFFFF;

    // Calculate partition boundaries
    let partition_start = (ledger_sequence / PARTITION_SIZE) * PARTITION_SIZE;
    let partition_end = partition_start + PARTITION_SIZE - 1;

    // Calculate batch boundaries
    let ledgers_into_partition = ledger_sequence % PARTITION_SIZE;
    let batch_start = partition_start + (ledgers_into_partition / BATCH_SIZE) * BATCH_SIZE;

    // Calculate hex prefixes using inverted sequence numbers
    let partition_prefix_hex = format!("{:08X}", MAX_UINT32 - partition_start);
    let batch_prefix_hex = format!("{:08X}", MAX_UINT32 - batch_start);

    // Assemble path components
    let partition_dir = format!("{}--{}-{}", partition_prefix_hex, partition_start, partition_end);
    let batch_file = format!("{}--{}.xdr.zst", batch_prefix_hex, batch_start);

    format!("v1.1/stellar/ledgers/pubnet/{}/{}", partition_dir, batch_file)
}

/// Download LedgerCloseMeta for a specific ledger sequence from S3-compatible storage
///
/// # Arguments
/// * `sequence` - The ledger sequence number to download
/// * `config` - S3 configuration (bucket, region, root path)
///
/// # Returns
/// The uncompressed LedgerCloseMeta for the specified ledger
pub async fn download_ledger_close_meta(
    sequence: u32,
    config: &S3Config,
) -> Result<LedgerCloseMeta, Error> {
    if sequence == 0 {
        return Err(Error::InvalidSequence(sequence));
    }

    let key = get_path_for_ledger(sequence);
    let region = Region::from_str(&config.region).map_err(|_| Error::InvalidSequence(sequence))?;

    // Create S3 client with anonymous credentials for public bucket access
    let credentials = StaticProvider::new_minimal("".to_string(), "".to_string());
    let s3_client = S3Client::new_with(HttpClient::new().unwrap(), credentials, region);

    // Get the compressed ledger data from S3
    let get_req = GetObjectRequest {
        bucket: config.bucket.clone(),
        key: key.clone(),
        ..Default::default()
    };

    let result = s3_client.get_object(get_req).await?;
    let body = result.body.ok_or_else(|| Error::LedgerNotFound(sequence))?;

    // Collect all chunks from the stream
    let chunks: Vec<bytes::Bytes> = body.try_collect().await?;
    let compressed_data: Vec<u8> = chunks.into_iter()
        .flat_map(|chunk| chunk.to_vec())
        .collect();

    // Decompress using zstd
    let decompressed_data = zstd::decode_all(compressed_data.as_slice())?;

    // Parse as LedgerCloseMetaBatch first, then extract the single ledger
    // According to the JavaScript code, batches only contain one ledger
    let mut limited_reader = stellar_xdr::Limited::new(
        std::io::Cursor::new(decompressed_data),
        Limits::none()
    );
    let mut batch: stellar_xdr::LedgerCloseMetaBatch = ReadXdr::read_xdr(&mut limited_reader)?;

    // Return the first (and assumed only) ledger from the batch
    batch.ledger_close_metas.into_iter().next().cloned()
        .ok_or_else(|| Error::LedgerNotFound(sequence))
}

/// Discover the latest available ledger sequence in the storage
///
/// # Arguments
/// * `config` - S3 configuration (bucket, region, root path)
///
/// # Returns
/// The highest ledger sequence number available
pub async fn discover_latest_ledger_sequence(config: &S3Config) -> Result<u32, Error> {
    let region = Region::from_str(&config.region).map_err(|_| Error::InvalidSequence(0))?;
    // Create S3 client with anonymous credentials for public bucket access
    let credentials = StaticProvider::new_minimal("".to_string(), "".to_string());
    let s3_client = S3Client::new_with(HttpClient::new().unwrap(), credentials, region);

    let list_req = ListObjectsV2Request {
        bucket: config.bucket.clone(),
        prefix: Some("v1.1/stellar/ledgers/pubnet/".to_string()),
        max_keys: Some(1000),
        ..Default::default()
    };

    let result = s3_client.list_objects_v2(list_req).await?;
    let contents = result.contents.ok_or_else(|| Error::LedgerNotFound(0))?;

    let mut sequence_numbers: Vec<u32> = contents
        .iter()
        .filter_map(|obj| obj.key.as_ref())
        .filter_map(|key| {
            // Parse filename from path like: v1.1/stellar/ledgers/pubnet/partition/batch--sequence.xdr.zst
            let parts: Vec<&str> = key.split('/').collect();
            if parts.len() >= 3 {
                let filename = parts.last()?;
                // Match pattern: batch--sequence.xdr.zst
                if let Some(captures) = regex::Regex::new(r"--(\d+)\.xdr\.zst$")
                    .ok()?
                    .captures(filename) {
                    captures.get(1)?.as_str().parse::<u32>().ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    sequence_numbers.sort();
    sequence_numbers.into_iter().last().ok_or_else(|| Error::LedgerNotFound(0))
}

