use soroban_sdk::xdr::{
    LedgerCloseMeta, LedgerEntry, LedgerEntryChange, LedgerEntryChanges, LedgerKey,
    TransactionMeta, TransactionResultMeta, TransactionResultMetaV1,
};

/// Iterator over ledger entry changes in reverse order from a LedgerCloseMeta.
/// Within each phase, non-State changes are yielded first, then State changes.
pub struct LedgerEntryChangesIterator<'a> {
    tx_result_meta: TransactionResultMetasNormalized<'a>,
    /// Current iteration position, or None if done
    position: Option<IteratorPosition>,
}

/// Current position within the iteration
#[derive(Clone, Copy, Debug)]
struct IteratorPosition {
    phase: ProcessingPhase,
    change_idx: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum ProcessingPhase {
    /// Special phase for boundary tx: TxChangesBefore with State group only, in forward order
    BoundaryTxChangesBefore {
        tx_idx: usize,
    },
    /// Special phase for boundary tx: OperationsChanges with State group only, in forward order
    BoundaryOperationsChanges {
        tx_idx: usize,
        op_idx: usize,
    },
    // Normal phases work backwards and only yield Mutation changes.
    // Since we iterate in reverse, we see the final state first, making State entries redundant.
    PostTxApplyFeeProcessing {
        tx_idx: usize,
    },
    TxChangesAfter {
        tx_idx: usize,
    },
    OperationsChanges {
        tx_idx: usize,
        op_idx: usize,
    },
    TxChangesBefore {
        tx_idx: usize,
    },
    FeeProcessing {
        tx_idx: usize,
    },
    /// Terminal phase indicating iteration is complete
    Done,
}

impl ProcessingPhase {
    /// Start iterating from the end of the ledger (after all txs have been applied).
    fn starting_from_end(tx_count: usize) -> Self {
        if tx_count == 0 {
            Self::Done
        } else {
            Self::PostTxApplyFeeProcessing {
                tx_idx: tx_count - 1,
            }
        }
    }

    /// Start iterating from just before a specific transaction was applied.
    /// This starts at the tx's TxChangesBefore with only the Before group,
    /// then proceeds to OperationsChanges Before group, both in forward order.
    /// This captures the state of entries just before the tx modified them.
    fn starting_from_tx(boundary_tx_idx: usize) -> Self {
        Self::BoundaryTxChangesBefore {
            tx_idx: boundary_tx_idx,
        }
    }

    fn tx_idx(&self) -> Option<usize> {
        match self {
            Self::BoundaryTxChangesBefore { tx_idx }
            | Self::BoundaryOperationsChanges { tx_idx, .. }
            | Self::PostTxApplyFeeProcessing { tx_idx }
            | Self::TxChangesAfter { tx_idx }
            | Self::OperationsChanges { tx_idx, .. }
            | Self::TxChangesBefore { tx_idx }
            | Self::FeeProcessing { tx_idx } => Some(*tx_idx),
            Self::Done => None,
        }
    }

    /// Get the iteration direction for this phase
    fn direction(&self) -> IterDirection {
        match self {
            Self::BoundaryTxChangesBefore { .. } | Self::BoundaryOperationsChanges { .. } => {
                IterDirection::Forward
            }
            _ => IterDirection::Reverse,
        }
    }

    /// Check if a change should be yielded by the iterator in this phase.
    /// Boundary phases only yield State entries, normal phases only yield mutations.
    fn should_yield(&self, change: &LedgerEntryChange) -> bool {
        match self {
            // Boundary phases only yield State entries
            Self::BoundaryTxChangesBefore { .. } | Self::BoundaryOperationsChanges { .. } => {
                matches!(change, LedgerEntryChange::State(_))
            }
            // Normal phases yield on mutations (non-State)
            Self::PostTxApplyFeeProcessing { .. }
            | Self::TxChangesAfter { .. }
            | Self::OperationsChanges { .. }
            | Self::TxChangesBefore { .. }
            | Self::FeeProcessing { .. } => !matches!(change, LedgerEntryChange::State(_)),
            // Done phase yields nothing
            Self::Done => false,
        }
    }

    fn get_changes<'a>(
        &self,
        components: &'a TransactionResultMetasNormalized<'a>,
    ) -> Option<&'a LedgerEntryChanges> {
        match self {
            Self::BoundaryTxChangesBefore { tx_idx } => components.tx_changes_before(*tx_idx),
            Self::BoundaryOperationsChanges { tx_idx, op_idx } => {
                Some(components.operation_changes(*tx_idx, *op_idx))
            }
            Self::PostTxApplyFeeProcessing { tx_idx } => {
                components.post_tx_apply_fee_processing(*tx_idx)
            }
            Self::TxChangesAfter { tx_idx } => components.tx_changes_after(*tx_idx),
            Self::OperationsChanges { tx_idx, op_idx } => {
                Some(components.operation_changes(*tx_idx, *op_idx))
            }
            Self::TxChangesBefore { tx_idx } => components.tx_changes_before(*tx_idx),
            Self::FeeProcessing { tx_idx } => Some(components.fee_processing(*tx_idx)),
            Self::Done => None,
        }
    }

    fn advance(&self, components: &TransactionResultMetasNormalized) -> ProcessingPhase {
        let tx_count = components.len();
        match self {
            // Boundary phases: iterate forward through Before snapshots, then switch to normal flow.
            Self::BoundaryTxChangesBefore { tx_idx } => {
                let op_count = components.operation_count(*tx_idx);
                if op_count > 0 {
                    Self::BoundaryOperationsChanges {
                        tx_idx: *tx_idx,
                        op_idx: 0,
                    }
                } else {
                    // No ops, go to previous tx with normal flow
                    tx_idx.checked_sub(1).map_or(
                        Self::FeeProcessing {
                            tx_idx: tx_count - 1,
                        },
                        |i| Self::TxChangesAfter { tx_idx: i },
                    )
                }
            }
            Self::BoundaryOperationsChanges { tx_idx, op_idx } => {
                let op_count = components.operation_count(*tx_idx);
                if *op_idx + 1 < op_count {
                    // More ops to process in forward order
                    Self::BoundaryOperationsChanges {
                        tx_idx: *tx_idx,
                        op_idx: op_idx + 1,
                    }
                } else {
                    // Done with boundary tx, go to previous tx with normal flow
                    tx_idx.checked_sub(1).map_or(
                        Self::FeeProcessing {
                            tx_idx: tx_count - 1,
                        },
                        |i| Self::TxChangesAfter { tx_idx: i },
                    )
                }
            }
            // Normal phases: iterate backwards, only yielding After (mutation) changes.
            Self::PostTxApplyFeeProcessing { tx_idx } => tx_idx.checked_sub(1).map_or(
                Self::TxChangesAfter {
                    tx_idx: tx_count - 1,
                },
                |i| Self::PostTxApplyFeeProcessing { tx_idx: i },
            ),
            Self::TxChangesAfter { tx_idx } => {
                let op_count = components.operation_count(*tx_idx);
                if op_count > 0 {
                    Self::OperationsChanges {
                        tx_idx: *tx_idx,
                        op_idx: op_count - 1,
                    }
                } else {
                    Self::TxChangesBefore { tx_idx: *tx_idx }
                }
            }
            Self::OperationsChanges { tx_idx, op_idx } => {
                op_idx
                    .checked_sub(1)
                    .map_or(Self::TxChangesBefore { tx_idx: *tx_idx }, |i| {
                        Self::OperationsChanges {
                            tx_idx: *tx_idx,
                            op_idx: i,
                        }
                    })
            }
            Self::TxChangesBefore { tx_idx } => tx_idx.checked_sub(1).map_or(
                Self::FeeProcessing {
                    tx_idx: tx_count - 1,
                },
                |i| Self::TxChangesAfter { tx_idx: i },
            ),
            Self::FeeProcessing { tx_idx } => tx_idx
                .checked_sub(1)
                .map_or(Self::Done, |i| Self::FeeProcessing { tx_idx: i }),
            Self::Done => Self::Done,
        }
    }
}

/// Extract the key and entry from a ledger entry change
fn extract_key_entry(change: &LedgerEntryChange) -> (LedgerKey, Option<LedgerEntry>) {
    match change {
        LedgerEntryChange::Created(ledger_entry)
        | LedgerEntryChange::Updated(ledger_entry)
        | LedgerEntryChange::State(ledger_entry)
        | LedgerEntryChange::Restored(ledger_entry) => {
            (ledger_entry.to_key(), Some(ledger_entry.clone()))
        }
        LedgerEntryChange::Removed(ledger_key) => (ledger_key.clone(), None),
    }
}

/// Direction of iteration through changes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IterDirection {
    Forward,
    Reverse,
}

impl IterDirection {
    /// Get the element at the given logical index from changes
    fn get(self, changes: &LedgerEntryChanges, idx: usize) -> &LedgerEntryChange {
        match self {
            Self::Forward => &changes[idx],
            Self::Reverse => &changes[changes.len() - 1 - idx],
        }
    }
}

impl<'a> LedgerEntryChangesIterator<'a> {
    /// Create a new iterator over ledger entry changes
    ///
    /// # Arguments
    /// * `meta` - The ledger close meta to iterate over
    /// * `tx_hash` - Optional transaction hash to start from. When set, the iterator
    ///   starts from just before that transaction was applied (at the tx's "Before"
    ///   changes), skipping the post-fee processing and all changes that occur after
    ///   the transaction executes including the changes produced from that tx.
    pub fn new(meta: &'a LedgerCloseMeta, tx_hash: Option<[u8; 32]>) -> Self {
        let tx_result_meta = TransactionResultMetasNormalized::from(meta);

        let position = if let Some(ref hash) = tx_hash {
            // Find the transaction and start from its State group
            tx_result_meta
                .find_tx_by_hash(hash)
                .map(|tx_idx| IteratorPosition {
                    phase: ProcessingPhase::starting_from_tx(tx_idx),
                    change_idx: 0,
                })
        } else {
            Some(IteratorPosition {
                phase: ProcessingPhase::starting_from_end(tx_result_meta.len()),
                change_idx: 0,
            })
        };

        Self {
            tx_result_meta,
            position,
        }
    }
}

impl<'a> Iterator for LedgerEntryChangesIterator<'a> {
    type Item = (ProcessingPhase, [u8; 32], LedgerKey, Option<LedgerEntry>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.position.as_mut()?;

            // Check if iteration is complete
            if matches!(pos.phase, ProcessingPhase::Done) {
                return None;
            }

            // Get tx_idx and changes, advancing if either is not available
            let (Some(tx_idx), Some(changes)) = (
                pos.phase.tx_idx(),
                pos.phase.get_changes(&self.tx_result_meta),
            ) else {
                pos.phase = pos.phase.advance(&self.tx_result_meta);
                pos.change_idx = 0;
                continue;
            };

            // If we've processed all changes in this phase for current group
            if pos.change_idx >= changes.len() {
                // Advance to next phase (which handles After -> Before transition)
                pos.phase = pos.phase.advance(&self.tx_result_meta);
                pos.change_idx = 0;
                continue;
            }

            let change = pos.phase.direction().get(changes, pos.change_idx);

            if !pos.phase.should_yield(change) {
                // Skip this change, it belongs to the other group
                pos.change_idx += 1;
                continue;
            }

            let phase = pos.phase;
            pos.change_idx += 1;
            let hash = *self.tx_result_meta.tx_hash(tx_idx);
            let (key, entry) = extract_key_entry(change);
            return Some((phase, hash, key, entry));
        }
    }
}

/// Extracted transaction processing components from LedgerCloseMeta
enum TransactionResultMetasNormalized<'a> {
    V0(&'a [TransactionResultMeta]),
    V1(&'a [TransactionResultMetaV1]),
}

impl<'a> From<&'a LedgerCloseMeta> for TransactionResultMetasNormalized<'a> {
    fn from(meta: &'a LedgerCloseMeta) -> Self {
        match meta {
            LedgerCloseMeta::V0(meta_v0) => Self::V0(&meta_v0.tx_processing),
            LedgerCloseMeta::V1(meta_v1) => Self::V0(&meta_v1.tx_processing),
            LedgerCloseMeta::V2(meta_v2) => Self::V1(&meta_v2.tx_processing),
        }
    }
}

impl<'a> TransactionResultMetasNormalized<'a> {
    pub fn len(&self) -> usize {
        match self {
            TransactionResultMetasNormalized::V0(slice) => slice.len(),
            TransactionResultMetasNormalized::V1(slice) => slice.len(),
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn fee_processing(&self, index: usize) -> &'a LedgerEntryChanges {
        match self {
            Self::V0(slice) => &slice[index].fee_processing,
            Self::V1(slice) => &slice[index].fee_processing,
        }
    }

    pub fn tx_changes_before(&self, index: usize) -> Option<&'a LedgerEntryChanges> {
        self.tx_apply_processing(index).tx_changes_before()
    }

    pub fn operation_count(&self, tx_index: usize) -> usize {
        self.tx_apply_processing(tx_index).operation_count()
    }

    pub fn operation_changes(&self, tx_index: usize, op_index: usize) -> &'a LedgerEntryChanges {
        self.tx_apply_processing(tx_index)
            .operation_changes(op_index)
    }

    pub fn tx_changes_after(&self, index: usize) -> Option<&'a LedgerEntryChanges> {
        self.tx_apply_processing(index).tx_changes_after()
    }

    fn tx_apply_processing(&self, index: usize) -> TransactionMetaNormalized<'a> {
        let meta = match self {
            Self::V0(s) => &s[index].tx_apply_processing,
            Self::V1(s) => &s[index].tx_apply_processing,
        };
        TransactionMetaNormalized(meta)
    }

    pub fn post_tx_apply_fee_processing(&self, index: usize) -> Option<&'a LedgerEntryChanges> {
        match self {
            Self::V0(_) => None,
            Self::V1(slice) => Some(&slice[index].post_tx_apply_fee_processing),
        }
    }

    pub fn tx_hash(&self, index: usize) -> &[u8; 32] {
        match self {
            Self::V0(slice) => &slice[index].result.transaction_hash.0,
            Self::V1(slice) => &slice[index].result.transaction_hash.0,
        }
    }

    /// Find the index of a transaction by its hash
    pub fn find_tx_by_hash(&self, hash: &[u8; 32]) -> Option<usize> {
        for i in 0..self.len() {
            if self.tx_hash(i) == hash {
                return Some(i);
            }
        }
        None
    }
}

/// Normalized view of TransactionMeta across all versions
struct TransactionMetaNormalized<'a>(&'a TransactionMeta);

impl<'a> TransactionMetaNormalized<'a> {
    fn tx_changes_before(&self) -> Option<&'a LedgerEntryChanges> {
        match self.0 {
            TransactionMeta::V0(_) => None,
            TransactionMeta::V1(m) => Some(&m.tx_changes), // TODO: Should this be before or after, or just ignored?
            TransactionMeta::V2(m) => Some(&m.tx_changes_before),
            TransactionMeta::V3(m) => Some(&m.tx_changes_before),
            TransactionMeta::V4(m) => Some(&m.tx_changes_before),
        }
    }

    fn tx_changes_after(&self) -> Option<&'a LedgerEntryChanges> {
        match self.0 {
            TransactionMeta::V0(_) | TransactionMeta::V1(_) => None,
            TransactionMeta::V2(m) => Some(&m.tx_changes_after),
            TransactionMeta::V3(m) => Some(&m.tx_changes_after),
            TransactionMeta::V4(m) => Some(&m.tx_changes_after),
        }
    }

    fn operation_count(&self) -> usize {
        match self.0 {
            TransactionMeta::V0(ops) => ops.len(),
            TransactionMeta::V1(m) => m.operations.len(),
            TransactionMeta::V2(m) => m.operations.len(),
            TransactionMeta::V3(m) => m.operations.len(),
            TransactionMeta::V4(m) => m.operations.len(),
        }
    }

    fn operation_changes(&self, index: usize) -> &'a LedgerEntryChanges {
        match self.0 {
            TransactionMeta::V0(ops) => &ops[index].changes,
            TransactionMeta::V1(m) => &m.operations[index].changes,
            TransactionMeta::V2(m) => &m.operations[index].changes,
            TransactionMeta::V3(m) => &m.operations[index].changes,
            TransactionMeta::V4(m) => &m.operations[index].changes,
        }
    }
}
