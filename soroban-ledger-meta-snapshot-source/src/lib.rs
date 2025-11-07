use soroban_env_host::storage::SnapshotSource;
use soroban_env_host::{xdr::LedgerKey, xdr::LedgerEntry, HostError};
use soroban_ledger_meta_downloader::{download_ledger_close_meta, S3Config};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{LedgerEntryChange as StellarLedgerEntryChange};
use std::rc::Rc;
use thiserror::Error;

/// Error type for mega snapshot source operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Ledger meta downloader error: {0}")]
    LedgerDownloader(#[from] soroban_ledger_meta_downloader::Error),
    #[error("Transaction hash not found in ledger {0}")]
    TransactionNotFound(String, u32),
    #[error("Ledger entry not found for key")]
    LedgerEntryNotFound,
    #[error("Invalid transaction hash format")]
    InvalidTransactionHash,
}

/// Meta snapshot source that downloads ledger meta and searches for ledger entries
pub struct MetaSnapshotSource {
    s3_config: S3Config,
    ledger_sequence: u32,
    transaction_hash: Option<Vec<u8>>,
}

impl MetaSnapshotSource {
    /// Create a new MetaSnapshotSource
    ///
    /// # Arguments
    /// * `config` - S3 configuration
    /// * `ledger_sequence` - Ledger sequence number
    /// * `transaction_hash` - Optional transaction hash as hex string
    pub fn new(
        config: S3Config,
        ledger_sequence: u32,
        transaction_hash: Option<String>,
    ) -> Result<Self, Error> {
        let transaction_hash_bytes = if let Some(hash_str) = transaction_hash {
            hex::decode(hash_str).map_err(|_| Error::InvalidTransactionHash)?
        } else {
            vec![]
        };

        let transaction_hash = if transaction_hash_bytes.is_empty() {
            None
        } else {
            Some(transaction_hash_bytes)
        };

        Ok(Self {
            s3_config: config,
            ledger_sequence,
            transaction_hash,
        })
    }

    /// Search for a ledger entry in the given transaction processing data
    fn search_ledger(
        &self,
        key: &LedgerKey,
        tx_processing: &[stellar_xdr::TransactionResultMeta],
    ) -> Option<(Rc<LedgerEntry>, Option<u32>)> {
        if let Some(tx_hash) = &self.transaction_hash {
            // Search for specific transaction
            if let Some(tx_proc) = tx_processing.iter().find(|tx| {
                tx.result.transaction_hash.as_slice() == tx_hash.as_slice()
            }) {
                // Found the transaction, search in its meta
                return search_in_tx_changes(&tx_proc.tx_apply_processing, key);
            }
            // Transaction not found
            None
        } else {
            // No specific transaction hash, search all transactions in reverse order
            for tx_proc in tx_processing.iter().rev() {
                if let Some(entry) = search_in_tx_changes(&tx_proc.tx_apply_processing, key) {
                    return Some(entry);
                }
            }
            None
        }
    }
}

impl SnapshotSource for MetaSnapshotSource {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
        let key = key.as_ref();
        let mut current_ledger = self.ledger_sequence;

        loop {
            // Download ledger meta
            let meta = match download_ledger_close_meta(current_ledger, &self.s3_config) {
                Ok(meta) => meta,
                Err(_) => return Ok(None), // Ledger not found or network error
            };

            // Get the transaction processing data
            let tx_processing = match &meta {
                stellar_xdr::LedgerCloseMeta::V0(v0) => &v0.tx_processing,
                stellar_xdr::LedgerCloseMeta::V1(v1) => &v1.tx_processing,
                _ => return Ok(None), // Unexpected version
            };

            // Search for the ledger entry in this ledger's transactions
            if let Some(entry) = self.search_ledger(key, tx_processing) {
                return Ok(Some(entry));
            }

            // Not found in this ledger, try previous ledger
            if current_ledger == 0 {
                return Ok(None);
            }
            current_ledger -= 1;
        }
    }
}

/// Search for a ledger entry in the transaction's meta
fn search_in_tx_changes(
    tx_meta: &stellar_xdr::TransactionMeta,
    key: &LedgerKey,
) -> Option<(Rc<LedgerEntry>, Option<u32>)> {
    // Get the changes based on transaction meta version
    let changes = match tx_meta {
        stellar_xdr::TransactionMeta::V0(v0) => {
            // V0 contains operations, each operation may have changes
            // For V0, we need to collect all changes from all operations
            let mut all_changes = Vec::new();
            for op_meta in v0.iter() {
                if let Some(changes) = &op_meta.changes {
                    all_changes.extend(changes.iter());
                }
            }
            all_changes
        }
        stellar_xdr::TransactionMeta::V1(v1) => v1.tx_changes.iter().collect::<Vec<_>>(),
        stellar_xdr::TransactionMeta::V2(v2) => v2.tx_changes.iter().collect::<Vec<_>>(),
        stellar_xdr::TransactionMeta::V3(v3) => v3.tx_changes.iter().collect::<Vec<_>>(),
        stellar_xdr::TransactionMeta::V4(v4) => v4.tx_changes.iter().collect::<Vec<_>>(),
    };

    // Search through changes for matching entries
    for change in changes {
        if let Some(entry) = extract_ledger_entry_from_change(change, key) {
            return Some(entry);
        }
    }
    None
}

/// Convert a stellar_xdr::LedgerEntry to soroban_env_host::xdr::LedgerEntry
/// Since both should have identical XDR structure, we can safely transmute
fn convert_ledger_entry(stellar_entry: &stellar_xdr::LedgerEntry) -> LedgerEntry {
    // Safe conversion assuming identical structure
    unsafe {
        std::ptr::read(stellar_entry as *const stellar_xdr::LedgerEntry as *const LedgerEntry)
    }
}

/// Convert a stellar_xdr::LedgerKey to soroban_env_host::xdr::LedgerKey
/// Since both should have identical XDR structure, we can safely transmute
fn convert_ledger_key(stellar_key: &stellar_xdr::LedgerKey) -> LedgerKey {
    // Safe conversion assuming identical structure
    unsafe {
        std::ptr::read(stellar_key as *const stellar_xdr::LedgerKey as *const LedgerKey)
    }
}

/// Extract a ledger entry from a ledger entry change if it matches the key
fn extract_ledger_entry_from_change(
    change: &StellarLedgerEntryChange,
    key: &LedgerKey,
) -> Option<(LedgerEntry, Option<u32>)> {
    match change {
        StellarLedgerEntryChange::Created(ledger_entry) |
        StellarLedgerEntryChange::Updated(ledger_entry) |
        StellarLedgerEntryChange::State(ledger_entry) |
        StellarLedgerEntryChange::Restored(ledger_entry) => {
            let ledger_entry_key = ledger_entry.to_key();
            if &ledger_entry_key == key {
                Some((ledger_entry.clone(), None))
            } else {
                None
            }
        }
        StellarLedgerEntryChange::Removed(ledger_key) => {
            if ledger_key == key {
                // TODO: Must distinguish between deleted vs not yet found.
                None // Entry was removed
            } else {
                None
            }
        }
    }
}
