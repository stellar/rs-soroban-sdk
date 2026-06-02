//! Implements SEP-54

use std::io::{copy, Cursor, Write};

use stellar_xdr::curr as stellar_xdr;
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
    #[error("Ledger sequence {0} not found")]
    LedgerNotFound(u32),
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
    let mut ledger = Vec::new();
    get_ledger(meta_url, sequence, &mut ledger)?;
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
    write: &mut W,
) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();

    // Construct S3 REST API URL for GET object
    let key = path_for_ledger(sequence);
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
    let mut batch: LedgerCloseMetaBatch = ReadXdr::read_xdr(&mut limited_reader)?;

    // Return the first (and assumed only) ledger from the batch
    batch
        .ledger_close_metas
        .into_iter()
        .next()
        .cloned()
        .ok_or(Error::LedgerNotFound(sequence))
}

/// Calculate the S3 path for a given ledger sequence
///
/// This implements the SEP-54 path structure for ledger metadata
fn path_for_ledger(ledger_sequence: u32) -> String {
    // Matches the current deployment's configuration
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

    // SEP-54 object paths use inverted (descending) hex prefixes so that the
    // newest ledgers sort first. These are pinned against values computed from
    // the documented partition (64000) and batch (1) sizes, and the 61292152
    // case anchors the path actually fetched by the mainnet fork tests.
    #[test]
    fn path_for_ledger_known_values() {
        assert_eq!(path_for_ledger(0), "FFFFFFFF--0-63999/FFFFFFFF--0.xdr.zst",);
        assert_eq!(
            path_for_ledger(128),
            "FFFFFFFF--0-63999/FFFFFF7F--128.xdr.zst",
        );
        assert_eq!(
            path_for_ledger(63999),
            "FFFFFFFF--0-63999/FFFF0600--63999.xdr.zst",
        );
        // Crossing the partition boundary moves into the next partition dir.
        assert_eq!(
            path_for_ledger(64000),
            "FFFF05FF--64000-127999/FFFF05FF--64000.xdr.zst",
        );
        assert_eq!(
            path_for_ledger(61292152),
            "FC596DFF--61248000-61311999/FC58C187--61292152.xdr.zst",
        );
    }

    #[test]
    fn path_for_ledger_partition_alignment() {
        // First ledger of a partition: the batch prefix equals the partition
        // prefix and the batch number equals the partition start.
        for partition in 0u32..3 {
            let start = partition * 64000;
            let path = path_for_ledger(start);
            let (dir, file) = path.split_once('/').unwrap();
            assert!(dir.ends_with(&format!("--{}-{}", start, start + 63999)));
            assert!(file.ends_with(&format!("--{}.xdr.zst", start)));
        }
    }
}
