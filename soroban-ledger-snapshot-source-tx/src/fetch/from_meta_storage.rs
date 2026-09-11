//! Implements SEP-54

use std::io::{copy, Cursor, Write};

use stellar_xdr::{LedgerCloseMeta, LedgerCloseMetaBatch, Limits, ReadXdr};
use thiserror::Error;

/// Error type for ledger meta downloader operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XDR parsing error: {0}")]
    Xdr(#[from] stellar_xdr::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Ledger sequence {0} not found")]
    LedgerNotFound(u32),
    #[error("SEP-54 storage config (.config.json) not found")]
    ConfigNotFound,
}

/// SEP-54 storage configuration, read from `<meta_url>/.config.json`.
///
/// Describes how ledgers are grouped into batch files and partitions for this
/// deployment, so object paths can be derived from the actual layout rather
/// than hardcoded constants. Only the fields needed to derive paths are read;
/// other documented fields (`version`, `compression`, `networkPassphrase`) are
/// ignored.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Number of ledgers bundled into each batch (file).
    pub ledgers_per_batch: u32,
    /// Number of batches in a partition (directory).
    pub batches_per_partition: u32,
}

/// Download LedgerCloseMeta for a specific ledger sequence from S3-compatible storage
///
/// # Arguments
/// * `meta_url` - URL to the SEP-54 compatible ledger meta storage
/// * `sequence` - The ledger sequence number to download
///
/// # Returns
/// The uncompressed LedgerCloseMeta for the specified ledger
#[allow(dead_code)]
pub(crate) fn ledger(meta_url: &str, sequence: u32) -> Result<LedgerCloseMeta, Error> {
    let mut config_bytes = Vec::new();
    get_config(meta_url, &mut config_bytes)?;
    let config = parse_config(Cursor::new(config_bytes))?;
    let mut ledger = Vec::new();
    get_ledger(
        meta_url,
        sequence,
        config.ledgers_per_batch,
        config.batches_per_partition,
        &mut ledger,
    )?;
    parse_ledger(sequence, Cursor::new(ledger))
}

/// Download compressed LedgerCloseMeta bytes for a specific ledger sequence from S3-compatible storage
///
/// # Arguments
/// * `meta_url` - URL to the SEP-54 compatible ledger meta storage
/// * `sequence` - The ledger sequence number to download
/// * `write` - Writer to write the decompressed data to
///
/// # Returns
/// The compressed bytes for the specified ledger
pub fn get_ledger<W: Write + ?Sized>(
    meta_url: &str,
    sequence: u32,
    ledgers_per_batch: u32,
    batches_per_partition: u32,
    write: &mut W,
) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();

    // Construct S3 REST API URL for GET object
    let key = path_for_ledger(sequence, ledgers_per_batch, batches_per_partition);
    let url = format!("{meta_url}/{key}");

    // Make HTTP GET request
    let response = client.get(&url).send()?;
    if !response.status().is_success() {
        return Err(Error::LedgerNotFound(sequence));
    }

    // Get the response body and decompress directly to writer
    let mut decoder = zstd::stream::Decoder::new(response)?;
    copy(&mut decoder, write)?;
    Ok(())
}

/// Download the SEP-54 storage configuration (`<meta_url>/.config.json`).
///
/// Unlike the ledger objects this is plain (uncompressed) JSON, so the response
/// body is written through verbatim.
pub fn get_config<W: Write + ?Sized>(meta_url: &str, write: &mut W) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{meta_url}/.config.json");
    let mut response = client.get(&url).send()?;
    if !response.status().is_success() {
        return Err(Error::ConfigNotFound);
    }
    copy(&mut response, write)?;
    Ok(())
}

/// Parse the SEP-54 [`Config`] from a JSON reader.
pub fn parse_config<R: std::io::Read>(reader: R) -> Result<Config, Error> {
    Ok(serde_json::from_reader(reader)?)
}

/// Parse LedgerCloseMeta from compressed bytes reader
///
/// # Arguments
/// * `sequence` - The ledger sequence being parsed, used only to produce a
///   meaningful `LedgerNotFound` error when the batch is empty
/// * `reader` - Reader for the compressed bytes
///
/// # Returns
/// The uncompressed LedgerCloseMeta
pub fn parse_ledger<R: std::io::Read>(sequence: u32, reader: R) -> Result<LedgerCloseMeta, Error> {
    // Parse as LedgerCloseMetaBatch first, then extract the single ledger
    let mut limited_reader = stellar_xdr::Limited::new(reader, Limits::none());
    let batch: LedgerCloseMetaBatch = ReadXdr::read_xdr(&mut limited_reader)?;

    // Return the first (and assumed only) ledger from the batch
    batch
        .ledger_close_metas
        .into_iter()
        .next()
        .ok_or(Error::LedgerNotFound(sequence))
}

/// Calculate the S3 path for a given ledger sequence
///
/// This implements the SEP-54 path structure for ledger metadata. The partition
/// and batch sizes come from the storage's `.config.json` (see [`Config`]):
/// `partition_size = ledgers_per_batch * batches_per_partition` and
/// `batch_size = ledgers_per_batch`.
fn path_for_ledger(
    ledger_sequence: u32,
    ledgers_per_batch: u32,
    batches_per_partition: u32,
) -> String {
    let partition_size = ledgers_per_batch * batches_per_partition;
    let batch_size = ledgers_per_batch;

    // Calculate partition boundaries
    let partition_start = (ledger_sequence / partition_size) * partition_size;
    let partition_end = partition_start + partition_size - 1;

    // Calculate batch boundaries
    let ledgers_into_partition = ledger_sequence % partition_size;
    let batch_start = partition_start + (ledgers_into_partition / batch_size) * batch_size;

    // Calculate hex prefixes using inverted sequence numbers
    let partition_prefix_hex = format!("{:08X}", u32::MAX - partition_start);
    let batch_prefix_hex = format!("{:08X}", u32::MAX - batch_start);

    // Assemble path components
    let partition_dir = format!(
        "{}--{}-{}",
        partition_prefix_hex, partition_start, partition_end
    );
    let batch_file = format!("{}--{}.xdr.zst", batch_prefix_hex, batch_start);

    format!("{}/{}", partition_dir, batch_file)
}

#[cfg(test)]
mod test {
    use super::path_for_ledger;

    // The mainnet deployment's SEP-54 config: one ledger per batch file, 64000
    // batches per partition (partition_size = 1 * 64000 = 64000).
    const LEDGERS_PER_BATCH: u32 = 1;
    const BATCHES_PER_PARTITION: u32 = 64000;

    // SEP-54 object paths use inverted (descending) hex prefixes so that the
    // newest ledgers sort first. These are pinned against values computed from
    // the documented partition (64000) and batch (1) sizes, and the 61292152
    // case anchors the path actually fetched by the mainnet fork tests.
    #[test]
    fn path_for_ledger_known_values() {
        assert_eq!(
            path_for_ledger(0, LEDGERS_PER_BATCH, BATCHES_PER_PARTITION),
            "FFFFFFFF--0-63999/FFFFFFFF--0.xdr.zst",
        );
        assert_eq!(
            path_for_ledger(128, LEDGERS_PER_BATCH, BATCHES_PER_PARTITION),
            "FFFFFFFF--0-63999/FFFFFF7F--128.xdr.zst",
        );
        assert_eq!(
            path_for_ledger(63999, LEDGERS_PER_BATCH, BATCHES_PER_PARTITION),
            "FFFFFFFF--0-63999/FFFF0600--63999.xdr.zst",
        );
        // Crossing the partition boundary moves into the next partition dir.
        assert_eq!(
            path_for_ledger(64000, LEDGERS_PER_BATCH, BATCHES_PER_PARTITION),
            "FFFF05FF--64000-127999/FFFF05FF--64000.xdr.zst",
        );
        assert_eq!(
            path_for_ledger(61292152, LEDGERS_PER_BATCH, BATCHES_PER_PARTITION),
            "FC596DFF--61248000-61311999/FC58C187--61292152.xdr.zst",
        );
    }

    #[test]
    fn path_for_ledger_partition_alignment() {
        // First ledger of a partition: the batch prefix equals the partition
        // prefix and the batch number equals the partition start.
        for partition in 0u32..3 {
            let start = partition * 64000;
            let path = path_for_ledger(start, LEDGERS_PER_BATCH, BATCHES_PER_PARTITION);
            let (dir, file) = path.split_once('/').unwrap();
            assert!(dir.ends_with(&format!("--{}-{}", start, start + 63999)));
            assert!(file.ends_with(&format!("--{}.xdr.zst", start)));
        }
    }

    // A non-trivial batch size (e.g. ledgers_per_batch = 2) groups multiple
    // ledgers into one file: the batch start is rounded down to the batch
    // boundary and the filename reflects that batch start, not the ledger.
    #[test]
    fn path_for_ledger_multi_ledger_batches() {
        // ledgers_per_batch = 2, batches_per_partition = 8 -> partition_size 16.
        assert_eq!(
            path_for_ledger(5, 2, 8),
            "FFFFFFFF--0-15/FFFFFFFB--4.xdr.zst",
        );
    }
}
