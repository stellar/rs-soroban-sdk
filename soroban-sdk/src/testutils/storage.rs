use crate::{Env, IntoVal, Map, Val};

/// Test utilities for [`Persistent`][crate::storage::Persistent].
pub trait Persistent {
    /// Returns all data stored in persistent storage for the contract.
    fn all(&self) -> Map<Val, Val>;

    /// Gets the TTL for the persistent storage entry corresponding to the provided key.
    ///
    /// TTL is the number of ledgers left until the persistent entry is considered
    /// expired, excluding the current ledger.
    ///
    /// Panics if there is no entry corresponding to the key, or if the entry has expired.
    fn get_ttl<K: IntoVal<Env, Val>>(&self, key: &K) -> u32;
}

/// Test utilities for [`Temporary`][crate::storage::Temporary].
pub trait Temporary {
    /// Returns all data stored in temporary storage for the contract.
    fn all(&self) -> Map<Val, Val>;

    /// Gets the TTL for the temporary storage entry corresponding to the provided key.
    ///
    /// TTL is the number of ledgers left until the temporary entry is considered
    /// non-existent, excluding the current ledger.
    ///
    /// Panics if there is no entry corresponding to the key.
    fn get_ttl<K: IntoVal<Env, Val>>(&self, key: &K) -> u32;
}

/// Test utilities for [`Instance`][crate::storage::Instance].
pub trait Instance {
    /// Returns all data stored in Instance storage for the contract.
    fn all(&self) -> Map<Val, Val>;

    /// Gets the TTL for the current contract's instance entry.
    ///
    /// TTL is the number of ledgers left until the instance entry is considered
    /// expired, excluding the current ledger.
    fn get_ttl(&self) -> u32;
}
