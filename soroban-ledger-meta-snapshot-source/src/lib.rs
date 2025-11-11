use soroban_env_host::{storage::SnapshotSource, HostError};
use soroban_ledger_meta_downloader::{download_ledger_close_meta, S3Config};
use stellar_xdr::curr::{LedgerCloseMeta, LedgerKey, LedgerEntry, LedgerEntryChange, LedgerEntryChanges, TransactionMeta, TransactionResultMeta, TransactionResultMetaV1};
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
            let changes = LedgerEntryChangesIterator::new(&meta);

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

/// Extracted transaction processing components from LedgerCloseMeta
pub enum TransactionProcessingComponents<'a> {
    V0(&'a [TransactionResultMeta]),
    V1(&'a [TransactionResultMeta]),
    V2(&'a [TransactionResultMetaV1]),
}

/// Iterator over ledger entry changes in reverse order from a LedgerCloseMeta
pub struct LedgerEntryChangesIterator<'a> {
    components: TransactionProcessingComponents<'a>,
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

impl<'a> LedgerEntryChangesIterator<'a> {
    /// Create a new iterator over ledger entry changes
    pub fn new(meta: &'a LedgerCloseMeta) -> Self {
        Self {
            components: TransactionProcessingComponents::from(meta),
            seen_keys: std::collections::HashSet::new(),
            state: IteratorState::PostTxApplyFeeProcessing { tx_idx: 0, change_idx: 0 },
        }
    }
}

impl<'a> Iterator for LedgerEntryChangesIterator<'a> {
    type Item = (LedgerKey, Option<LedgerEntry>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.state {
                IteratorState::PostTxApplyFeeProcessing { tx_idx, change_idx } => {
                    let tx_count = self.components.len();

                    // If we've processed all transactions, move to next state
                    if *tx_idx >= tx_count {
                        self.state = IteratorState::TxChangesAfter { tx_idx: 0, change_idx: 0 };
                        continue;
                    }

                    // Skip if this version doesn't have post_tx_apply_fee_processing
                    if let Some(changes) = self.components.post_tx_apply_fee_processing(*tx_idx) {
                        // If we've processed all changes in this transaction's post_tx_apply_fee_processing
                        if *change_idx >= changes.len() {
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }

                        // Get the change in reverse order
                        let change = &changes[changes.len() - 1 - *change_idx];
                        let (key, entry) = extract_key_entry(change);

                        // Skip if we've already seen this key
                        if self.seen_keys.contains(&key) {
                            *change_idx += 1;
                            continue;
                        }

                        self.seen_keys.insert(key.clone());
                        *change_idx += 1;
                        return Some((key, entry));
                    } else {
                        // Version doesn't have post_tx_apply_fee_processing, skip to next state
                        *tx_idx += 1;
                        *change_idx = 0;
                        continue;
                    }
                }
                IteratorState::TxChangesAfter { tx_idx, change_idx } => {
                    let tx_count = self.components.len();

                    if *tx_idx >= tx_count {
                        self.state = IteratorState::OperationsChanges { tx_idx: 0, op_idx: 0, change_idx: 0 };
                        continue;
                    }

                    if let Some(changes) = self.components.tx_changes_after(*tx_idx) {
                        if *change_idx >= changes.len() {
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }

                        let change = &changes[changes.len() - 1 - *change_idx];
                        let (key, entry) = extract_key_entry(change);

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
                }
                IteratorState::OperationsChanges { tx_idx, op_idx, change_idx } => {
                    let tx_count = self.components.len();

                    if *tx_idx >= tx_count {
                        self.state = IteratorState::TxChangesBefore { tx_idx: 0, change_idx: 0 };
                        continue;
                    }

                    // Handle operations for all TransactionMeta versions
                    let op_count = self.components.operation_count(*tx_idx);

                    if *op_idx >= op_count {
                        *tx_idx += 1;
                        *op_idx = 0;
                        *change_idx = 0;
                        continue;
                    }

                    let changes = self.components.operation_changes(*tx_idx, *op_idx);

                    if *change_idx >= changes.len() {
                        *op_idx += 1;
                        *change_idx = 0;
                        continue;
                    }

                    let change = &changes[changes.len() - 1 - *change_idx];
                    let (key, entry) = extract_key_entry(change);

                    if self.seen_keys.contains(&key) {
                        *change_idx += 1;
                        continue;
                    }

                    self.seen_keys.insert(key.clone());
                    *change_idx += 1;
                    return Some((key, entry));
                }
                IteratorState::TxChangesBefore { tx_idx, change_idx } => {
                    let tx_count = self.components.len();

                    if *tx_idx >= tx_count {
                        self.state = IteratorState::FeeProcessing { tx_idx: 0, change_idx: 0 };
                        continue;
                    }

                    if let Some(changes) = self.components.tx_changes_before(*tx_idx) {
                        if *change_idx >= changes.len() {
                            *tx_idx += 1;
                            *change_idx = 0;
                            continue;
                        }

                        let change = &changes[changes.len() - 1 - *change_idx];
                        let (key, entry) = extract_key_entry(change);

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
                }
                IteratorState::FeeProcessing { tx_idx, change_idx } => {
                    let tx_count = self.components.len();

                    if *tx_idx >= tx_count {
                        self.state = IteratorState::Done;
                        continue;
                    }

                    let changes = self.components.fee_processing(*tx_idx);

                    if *change_idx >= changes.len() {
                        *tx_idx += 1;
                        *change_idx = 0;
                        continue;
                    }

                    let change = &changes[changes.len() - 1 - *change_idx];
                    let (key, entry) = extract_key_entry(change);

                    if self.seen_keys.contains(&key) {
                        *change_idx += 1;
                        continue;
                    }

                    self.seen_keys.insert(key.clone());
                    *change_idx += 1;
                    return Some((key, entry));
                }
                IteratorState::Done => return None,
            }
        }
    }
}

impl<'a> TransactionProcessingComponents<'a> {
    pub fn len(&self) -> usize {
        match self {
            TransactionProcessingComponents::V0(slice) => slice.len(),
            TransactionProcessingComponents::V1(slice) => slice.len(),
            TransactionProcessingComponents::V2(slice) => slice.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice(&self, start: usize) -> Self {
        match self {
            Self::V0(slice) => Self::V0(&slice[start..]),
            Self::V1(slice) => Self::V1(&slice[start..]),
            Self::V2(slice) => Self::V2(&slice[start..]),
        }
    }

    pub fn fee_processing(&self, index: usize) -> &'a LedgerEntryChanges {
        match self {
            Self::V0(slice) => &slice[index].fee_processing,
            Self::V1(slice) => &slice[index].fee_processing,
            Self::V2(slice) => &slice[index].fee_processing,
        }
    }

    fn tx_apply_processing(&self, index: usize) -> &'a TransactionMeta {
        match self {
            Self::V0(slice) => &slice[index].tx_apply_processing,
            Self::V1(slice) => &slice[index].tx_apply_processing,
            Self::V2(slice) => &slice[index].tx_apply_processing,
        }
    }

    /// Extract tx_changes_before from any TransactionMeta version
    pub fn tx_changes_before(&self, index: usize) -> Option<&'a LedgerEntryChanges> {
        match self.tx_apply_processing(index) {
            TransactionMeta::V0(_) => None,
            TransactionMeta::V1(tx_meta) => Some(&tx_meta.tx_changes),
            TransactionMeta::V2(tx_meta) => Some(&tx_meta.tx_changes_before),
            TransactionMeta::V3(tx_meta) => Some(&tx_meta.tx_changes_before),
            TransactionMeta::V4(tx_meta) => Some(&tx_meta.tx_changes_before),
        }
    }

    /// Get the number of operations for a transaction from any TransactionMeta version
    pub fn operation_count(&self, tx_index: usize) -> usize {
        match self.tx_apply_processing(tx_index) {
            TransactionMeta::V0(operations) => operations.len(),
            TransactionMeta::V1(tx_meta) => tx_meta.operations.len(),
            TransactionMeta::V2(tx_meta) => tx_meta.operations.len(),
            TransactionMeta::V3(tx_meta) => tx_meta.operations.len(),
            TransactionMeta::V4(tx_meta) => tx_meta.operations.len(),
        }
    }

    /// Extract changes for a specific operation from any TransactionMeta version
    pub fn operation_changes(&self, tx_index: usize, op_index: usize) -> &'a LedgerEntryChanges {
        match self.tx_apply_processing(tx_index) {
            TransactionMeta::V0(operations) => &operations[op_index].changes,
            TransactionMeta::V1(tx_meta) => &tx_meta.operations[op_index].changes,
            TransactionMeta::V2(tx_meta) => &tx_meta.operations[op_index].changes,
            TransactionMeta::V3(tx_meta) => &tx_meta.operations[op_index].changes,
            TransactionMeta::V4(tx_meta) => &tx_meta.operations[op_index].changes,
        }
    }

    /// Extract tx_changes_after from any TransactionMeta version
    pub fn tx_changes_after(&self, index: usize) -> Option<&'a LedgerEntryChanges> {
        match self.tx_apply_processing(index) {
            TransactionMeta::V0(_) => None,
            TransactionMeta::V1(tx_meta) => Some(&tx_meta.tx_changes),
            TransactionMeta::V2(tx_meta) => Some(&tx_meta.tx_changes_after),
            TransactionMeta::V3(tx_meta) => Some(&tx_meta.tx_changes_after),
            TransactionMeta::V4(tx_meta) => Some(&tx_meta.tx_changes_after),
        }
    }

    pub fn post_tx_apply_fee_processing(&self, index: usize) -> Option<&'a LedgerEntryChanges> {
        match self {
            Self::V0(_) => None,
            Self::V1(_) => None,
            Self::V2(slice) => Some(&slice[index].post_tx_apply_fee_processing),
        }
    }
}

impl<'a> From<&'a LedgerCloseMeta> for TransactionProcessingComponents<'a> {
    fn from(meta: &'a LedgerCloseMeta) -> Self {
        match meta {
            LedgerCloseMeta::V0(meta_v0) => Self::V0(&meta_v0.tx_processing),
            LedgerCloseMeta::V1(meta_v1) => Self::V1(&meta_v1.tx_processing),
            LedgerCloseMeta::V2(meta_v2) => Self::V2(&meta_v2.tx_processing),
        }
    }
}

