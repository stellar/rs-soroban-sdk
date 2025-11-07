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
}

impl SnapshotSource for MetaSnapshotSource {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
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

            // If we have a transaction hash, find that specific transaction
            // TODO: Reorganise this if else so that most of the inner work is shared by both
            // cases.
            if let Some(tx_hash) = &self.transaction_hash {
                if let Some(tx_proc) = tx_processing.iter().find(|tx| {
                    tx.result.transaction_hash.as_slice() == tx_hash.as_slice()
                }) {
                    // Found the transaction, search in its before state
                    // TODO: Implement search
                    // TODO: look in here: tx_proc.tx_apply_processing which is this type:
                    // pub enum TransactionMeta {
                    //     V0(VecM<OperationMeta>),
                    //     V1(TransactionMetaV1),
                    //     V2(TransactionMetaV2),
                    //     V3(TransactionMetaV3),
                    //     V4(TransactionMetaV4),
                    // }
                    // Each of those inner types contains a LedgerEntryChanges which contains:
                    // pub enum LedgerEntryChange {
                    //     Created(LedgerEntry),
                    //     Updated(LedgerEntry),
                    //     Removed(LedgerKey),
                    //     State(LedgerEntry),
                    //     Restored(LedgerEntry),
                    // }
                    // Which can be used to get the LedgerEntry that matches the ledger key.
                    // Or if it is removed then stop searching and return None.
                    return Ok(None);
                }
                // Transaction not found in this ledger, try previous ledger
                if current_ledger == 0 {
                    return Ok(None);
                }
                current_ledger -= 1;
                continue;
            } else {
                // No specific transaction hash, search all transactions in reverse order
                // Start from the last transaction and work backwards
                for _tx_proc in tx_processing.iter().rev() {
                    // TODO: Implement search same as todos above
                }
                // Not found in this ledger, try previous ledger
                if current_ledger == 0 {
                    return Ok(None);
                }
                current_ledger -= 1;
            }
        }
    }
}

// TODO: Implement search_in_tx_changes and extract_ledger_entry_from_change
// Need to figure out the correct stellar_xdr types
