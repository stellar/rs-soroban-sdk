use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limits, ReadXdr, LedgerCloseMeta};
use thiserror::Error;

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
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
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
pub fn download_ledger_close_meta(
    sequence: u32,
    config: &S3Config,
) -> Result<LedgerCloseMeta, Error> {
    if sequence == 0 {
        return Err(Error::InvalidSequence(sequence));
    }

    let key = get_path_for_ledger(sequence);
    let client = reqwest::blocking::Client::new();

    // Construct S3 REST API URL for GET object
    let url = format!("https://{}.s3.{}.amazonaws.com/{}",
        config.bucket, config.region, key);

    // Make HTTP GET request
    let response = client.get(&url).send()?;
    if !response.status().is_success() {
        return Err(Error::LedgerNotFound(sequence));
    }

    // Get the response body
    let compressed_data = response.bytes()?.to_vec();

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
pub fn discover_latest_ledger_sequence(config: &S3Config) -> Result<u32, Error> {
    let client = reqwest::blocking::Client::new();

    // Construct S3 REST API URL for LIST objects v2
    let url = format!("https://{}.s3.{}.amazonaws.com/",
        config.bucket, config.region);

    // Make HTTP GET request with query parameters for listing objects
    let response = client
        .get(&url)
        .query(&[
            ("list-type", "2"),
            ("prefix", "v1.1/stellar/ledgers/pubnet/"),
            ("max-keys", "1000"),
        ])
        .send()?;

    if !response.status().is_success() {
        return Err(Error::LedgerNotFound(0));
    }

    // Parse the XML response to extract object keys
    let xml_content = response.text()?;
    let mut sequence_numbers: Vec<u32> = Vec::new();

    // Simple XML parsing - look for <Key> tags
    for line in xml_content.lines() {
        if line.contains("<Key>") && line.contains("</Key>") {
            if let Some(key_start) = line.find("<Key>") {
                if let Some(key_end) = line.find("</Key>") {
                    let key = &line[key_start + 5..key_end];
                    // Parse filename from path like: v1.1/stellar/ledgers/pubnet/partition/batch--sequence.xdr.zst
                    let parts: Vec<&str> = key.split('/').collect();
                    if parts.len() >= 3 {
                        if let Some(filename) = parts.last() {
                            // Match pattern: batch--sequence.xdr.zst
                            if let Some(captures) = regex::Regex::new(r"--(\d+)\.xdr\.zst$")
                                .ok()
                                .and_then(|re| re.captures(filename)) {
                                if let Some(seq_match) = captures.get(1) {
                                    if let Ok(seq) = seq_match.as_str().parse::<u32>() {
                                        sequence_numbers.push(seq);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sequence_numbers.sort();
    sequence_numbers.into_iter().last().ok_or_else(|| Error::LedgerNotFound(0))
}

