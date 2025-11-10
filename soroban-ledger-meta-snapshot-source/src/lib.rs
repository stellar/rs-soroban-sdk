use soroban_env_host::{storage::SnapshotSource, HostError};
use soroban_ledger_meta_downloader::{download_ledger_close_meta, S3Config};
use stellar_xdr::curr::{LedgerCloseMeta, LedgerKey, LedgerEntry, LedgerEntryChange};
use std::{collections::HashSet, rc::Rc};
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
            let changes = extract_ledger_entry_changes(&meta);

            for (change_key, entry) in changes {
                if change_key == *key {
                    if let Some(entry) = entry {
                        return Ok(Some((Rc::new(entry), None)));
                    } else {
                        // Entry was removed
                        return Ok(None);
                    }
                }
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

/// Iterator over ledger entry changes in reverse order from a LedgerCloseMeta
pub struct LedgerEntryChangesIterator<'a> {
    meta: &'a LedgerCloseMeta,
    seen_keys: std::collections::HashSet<LedgerKey>,
    state: IteratorState,
}

enum IteratorState {
    PostTxApplyFeeProcessing { tx_idx: usize, change_idx: usize },
    TxChangesAfter { tx_idx: usize, change_idx: usize },
    OperationsChanges { tx_idx: usize, op_idx: usize, change_idx: usize },
    TxChangesBefore { tx_idx: usize, change_idx: usize },
    FeeProcessing { tx_idx: usize, change_idx: usize },
    Done,
}

impl<'a> LedgerEntryChangesIterator<'a> {
    /// Create a new iterator over ledger entry changes
    pub fn new(meta: &'a LedgerCloseMeta) -> Self {
        Self {
            meta,
            seen_keys: std::collections::HashSet::new(),
            state: IteratorState::PostTxApplyFeeProcessing { tx_idx: 0, change_idx: 0 },
        }
    }

    /// Extract the key and entry from a ledger entry change
    fn extract_key_entry(change: &LedgerEntryChange) -> (LedgerKey, Option<LedgerEntry>) {
        match change {
            LedgerEntryChange::Created(ledger_entry) |
            LedgerEntryChange::Updated(ledger_entry) |
            LedgerEntryChange::State(ledger_entry) |
            LedgerEntryChange::Restored(ledger_entry) => {
                (ledger_entry.to_key(), Some(ledger_entry.clone()))
            }
            LedgerEntryChange::Removed(ledger_key) => {
                (ledger_key.clone(), None)
            }
        }
    }
}

impl<'a> Iterator for LedgerEntryChangesIterator<'a> {
    type Item = (LedgerKey, Option<LedgerEntry>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.state {
                IteratorState::PostTxApplyFeeProcessing { tx_idx, change_idx } => {
                    if let LedgerCloseMeta::V2(meta) = self.meta {
                        let tx_count = meta.tx_processing.len();

                        // If we've processed all transactions, move to next state
                        if *tx_idx >= tx_count {
                            self.state = IteratorState::TxChangesAfter { tx_idx: 0, change_idx: 0 };
                            continue;
                        }

                        let tx = &meta.tx_processing[*tx_idx];
                        let changes = &tx.post_tx_apply_fee_processing;

                        // If we've processed all changes in this transaction's post_tx_apply_fee_processing
                        if *change_idx >= changes.len() {
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }

                        // Get the change in reverse order
                        let change = &changes[changes.len() - 1 - *change_idx];
                        let (key, entry) = Self::extract_key_entry(change);

                        // Skip if we've already seen this key
                        if self.seen_keys.contains(&key) {
                            *change_idx += 1;
                            continue;
                        }

                        self.seen_keys.insert(key.clone());
                        *change_idx += 1;
                        return Some((key, entry));
                    } else {
                        // For other versions, skip to done for now
                        self.state = IteratorState::Done;
                        continue;
                    }
                }
                IteratorState::TxChangesAfter { tx_idx, change_idx } => {
                    if let LedgerCloseMeta::V2(meta) = self.meta {
                        let tx_count = meta.tx_processing.len();

                        if *tx_idx >= tx_count {
                            self.state = IteratorState::OperationsChanges { tx_idx: 0, op_idx: 0, change_idx: 0 };
                            continue;
                        }

                        let tx = &meta.tx_processing[*tx_idx];

                        if let stellar_xdr::curr::TransactionMeta::V3(tx_meta) = &tx.tx_apply_processing {
                            let changes = &tx_meta.tx_changes_after;

                            if *change_idx >= changes.len() {
                                *tx_idx += 1;
                                *change_idx = 0;
                                continue;
                            }

                            let change = &changes[changes.len() - 1 - *change_idx];
                            let (key, entry) = Self::extract_key_entry(change);

                            if self.seen_keys.contains(&key) {
                                *change_idx += 1;
                                continue;
                            }

                            self.seen_keys.insert(key.clone());
                            *change_idx += 1;
                            return Some((key, entry));
                        } else {
                            // Skip unsupported tx meta versions
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }
                    } else {
                        self.state = IteratorState::Done;
                        continue;
                    }
                }
                IteratorState::OperationsChanges { tx_idx, op_idx, change_idx } => {
                    if let LedgerCloseMeta::V2(meta) = self.meta {
                        let tx_count = meta.tx_processing.len();

                        if *tx_idx >= tx_count {
                            self.state = IteratorState::TxChangesBefore { tx_idx: 0, change_idx: 0 };
                            continue;
                        }

                        let tx = &meta.tx_processing[*tx_idx];

                        if let stellar_xdr::curr::TransactionMeta::V3(tx_meta) = &tx.tx_apply_processing {
                            let op_count = tx_meta.operations.len();

                            if *op_idx >= op_count {
                                *tx_idx += 1;
                                *op_idx = 0;
                                *change_idx = 0;
                                continue;
                            }

                            let op = &tx_meta.operations[*op_idx];
                            let changes = &op.changes;

                            if *change_idx >= changes.len() {
                                *op_idx += 1;
                                *change_idx = 0;
                                continue;
                            }

                            let change = &changes[changes.len() - 1 - *change_idx];
                            let (key, entry) = Self::extract_key_entry(change);

                            if self.seen_keys.contains(&key) {
                                *change_idx += 1;
                                continue;
                            }

                            self.seen_keys.insert(key.clone());
                            *change_idx += 1;
                            return Some((key, entry));
                        } else {
                            *tx_idx += 1;
                            *op_idx = 0;
                            *change_idx = 0;
                            continue;
                        }
                    } else {
                        self.state = IteratorState::Done;
                        continue;
                    }
                }
                IteratorState::TxChangesBefore { tx_idx, change_idx } => {
                    if let LedgerCloseMeta::V2(meta) = self.meta {
                        let tx_count = meta.tx_processing.len();

                        if *tx_idx >= tx_count {
                            self.state = IteratorState::FeeProcessing { tx_idx: 0, change_idx: 0 };
                            continue;
                        }

                        let tx = &meta.tx_processing[*tx_idx];

                        if let stellar_xdr::curr::TransactionMeta::V3(tx_meta) = &tx.tx_apply_processing {
                            let changes = &tx_meta.tx_changes_before;

                            if *change_idx >= changes.len() {
                                *tx_idx += 1;
                                *change_idx = 0;
                                continue;
                            }

                            let change = &changes[changes.len() - 1 - *change_idx];
                            let (key, entry) = Self::extract_key_entry(change);

                            if self.seen_keys.contains(&key) {
                                *change_idx += 1;
                                continue;
                            }

                            self.seen_keys.insert(key.clone());
                            *change_idx += 1;
                            return Some((key, entry));
                        } else {
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }
                    } else {
                        self.state = IteratorState::Done;
                        continue;
                    }
                }
                IteratorState::FeeProcessing { tx_idx, change_idx } => {
                    if let LedgerCloseMeta::V2(meta) = self.meta {
                        let tx_count = meta.tx_processing.len();

                        if *tx_idx >= tx_count {
                            self.state = IteratorState::Done;
                            continue;
                        }

                        let tx = &meta.tx_processing[*tx_idx];
                        let changes = &tx.fee_processing;

                        if *change_idx >= changes.len() {
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }

                        let change = &changes[changes.len() - 1 - *change_idx];
                        let (key, entry) = Self::extract_key_entry(change);

                        if self.seen_keys.contains(&key) {
                            *change_idx += 1;
                            continue;
                        }

                        self.seen_keys.insert(key.clone());
                        *change_idx += 1;
                        return Some((key, entry));
                    } else {
                        self.state = IteratorState::Done;
                        continue;
                    }
                }
                IteratorState::Done => return None,
            }
        }
    }
}

/// Extract ledger entry changes from a ledger close meta
pub fn extract_ledger_entry_changes(meta: &LedgerCloseMeta) -> impl Iterator<Item = (LedgerKey, Option<LedgerEntry>)> + '_ {
    LedgerEntryChangesIterator::new(meta)
}

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
