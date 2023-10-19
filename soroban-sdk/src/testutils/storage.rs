use crate::{Map, Val};

/// Test utilities for [`Persistent`][crate::storage::Persistent].
pub trait Persistent {
    /// Returns all data stored in persistent storage for the contract.
    fn all(&self) -> Map<Val, Val>;
}

/// Test utilities for [`Temporary`][crate::storage::Temporary].
pub trait Temporary {
    /// Returns all data stored in temporary storage for the contract.
    fn all(&self) -> Map<Val, Val>;
}

/// Test utilities for [`Instance`][crate::storage::Instance].
pub trait Instance {
    /// Returns all data stored in Instance storage for the contract.
    fn all(&self) -> Map<Val, Val>;
}
