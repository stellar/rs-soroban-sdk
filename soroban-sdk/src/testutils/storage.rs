use crate::{Env, IntoVal, Map, Val};

/// Test utilities for [`Persistent`][crate::storage::Persistent].
pub trait Persistent {
    /// Returns all data stored in persistent storage for the contract.
    fn all(&self) -> Map<Val, Val>;

    /// Gets the ledger until which the persistent entry with a given key lives.
    ///
    /// Panics if there is no entry corresponding to the key, or if the entry has expired.
    fn get_live_until_ledger<K: IntoVal<Env, Val>>(&self, key: &K) -> u32;
}

/// Test utilities for [`Temporary`][crate::storage::Temporary].
pub trait Temporary {
    /// Returns all data stored in temporary storage for the contract.
    fn all(&self) -> Map<Val, Val>;

    /// Gets the ledger until which the temp entry with a given key lives.
    ///
    /// Panics if there is no entry corresponding to the key.
    fn get_live_until_ledger<K: IntoVal<Env, Val>>(&self, key: &K) -> u32;
}

/// Test utilities for [`Instance`][crate::storage::Instance].
pub trait Instance {
    /// Returns all data stored in Instance storage for the contract.
    fn all(&self) -> Map<Val, Val>;

    /// Gets the ledger until which the instance entry for the current contract lives.
    fn get_live_until_ledger(&self) -> u32;
}
