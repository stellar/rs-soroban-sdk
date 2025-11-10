use soroban_env_host::{storage::SnapshotSource, HostError};
use soroban_ledger_meta_downloader::{download_ledger_close_meta, S3Config};
use stellar_xdr::curr::{LedgerCloseMeta, LedgerKey, LedgerEntry, LedgerEntryChange};
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
}

/// Meta snapshot source that downloads ledger meta and searches for ledger entries
pub struct MetaSnapshotSource {
    s3_config: S3Config,
    ledger_sequence: u32,
    transaction_hash: Option<[u8; 32]>,
}

impl MetaSnapshotSource {
    /// Create a new MetaSnapshotSource
    ///
    /// # Arguments
    /// * `config` - S3 configuration
    /// * `ledger_sequence` - Ledger sequence number
    /// * `transaction_hash` - Optional transaction hash as byte array
    pub fn new(
        config: S3Config,
        ledger_sequence: u32,
        transaction_hash: Option<[u8; 32]>,
    ) -> Result<Self, Error> {
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
        let key = key.as_ref();
        let mut current_ledger = self.ledger_sequence;

        loop {
            // Download ledger meta
            let meta = match download_ledger_close_meta(current_ledger, &self.s3_config) {
                Ok(meta) => meta,
                Err(_) => return Ok(None), // Ledger not found or network error
            };

            // Search for the ledger entry in this ledger's transactions
            let changes = extract_ledger_entry_changes(meta);

            if let Some(entry) = entry {
                return Ok(Some(entry));
            }

            // Not found in this ledger, try previous ledger
            // TODO: Fallback to history archivs. If RPC ever supports historical ledger entry
            // lookup, fallback to that instead.
            if current_ledger == 0 {
                return Ok(None);
            }
            current_ledger -= 1;
        }
    }
}

// TODO: Write an iterator that is constructed with one input, a
// stellar_xdr::curr::LedgerCloseMeta. The iterator will iterate over all stellar_xdr::curr::LedgerEntryChange values
// that exist inside the LedgerCloseMeta in reverse. LedgerCloseMeta is a large nested value.
// LedgerEntryChange can be found in a variety of places inside the value. LedgerEntryChange does
// not internally have any id or value that indicates what order it should be in. Instead, the
// position and place for here it is stored in the LedgerCloseMeta will inform the order.
//
// When iterating in reverse the LedgerEntryChange values should be looked for in order of these
// places:
//
// 1. for each tx in the result meta the post_tx_apply_fee_processing in reverse
// 2. for each tx in the result meta the tx_apply_processing.*.tx_changes_after in reverse
// 3. for each tx in the result meta the tx_apply_processing.*.operations.changes in reverse
// 4. for each tx in the result meta the tx_apply_processing.*.tx_changes_before in reverse
// 1. for each tx in the result meta the fee_processing in reverse
//
// The item value the iterator should return is the (stellar_xdr::LedgerKey,
// Option<stellar_xdr::LedgerEntry>) at each change.
//
// The iterator must keep track of what LedgerKey's it has seen before. If it has seen a LedgerKey
// before when iterating backwards, it should ignore any other changes it comes across for the same
// ledger entry.
//
// Note that LedgerEntryChange values are always two values side-by-side, a
// LedgerEntryChange::State that captures the state just before the change, and a
// LedgerEntryChange::* that captures the state after the change. Always include both, but include
// the State after the other when iterating backwards.

/// Extract a ledger entry from a ledger entry change if it matches the key
fn extract_ledger_entry_from_change(
    change: &LedgerEntryChange,
    key: &LedgerKey,
) -> Option<(LedgerEntry, Option<u32>)> {
    match change {
        LedgerEntryChange::Created(ledger_entry) |
        LedgerEntryChange::Updated(ledger_entry) |
        LedgerEntryChange::State(ledger_entry) |
        LedgerEntryChange::Restored(ledger_entry) => {
            let ledger_entry_key = ledger_entry.to_key();
            if &ledger_entry_key == key {
                Some((ledger_entry.clone(), None))
            } else {
                None
            }
        }
        LedgerEntryChange::Removed(ledger_key) => {
            if ledger_key == key {
                // TODO: Must distinguish between deleted vs not yet found.
                None // Entry was removed
            } else {
                None
            }
        }
    }
}
