#![cfg(any(test, feature = "testutils"))]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

mod sign;
pub use sign::ed25519;

pub use crate::env::testutils::*;

use crate::{Env, RawVal, Symbol, Vec};

#[doc(hidden)]
pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: Env, args: &[RawVal]) -> Option<RawVal>;
}

/// Test utilities for [`Ledger`][crate::ledger::Ledger].
pub trait Ledger {
    /// Set ledger info.
    fn set(&self, l: LedgerInfo);

    /// Get ledger info.
    fn get(&self) -> LedgerInfo;

    /// Modify the ledger info.
    fn with_mut<F>(&self, f: F)
    where
        F: FnMut(&mut LedgerInfo);
}

/// Test utilities for [`Events`][crate::events::Events].
pub trait Events {
    /// Returns all events that have been published by contracts.
    ///
    /// Returns a [`Vec`] of three element tuples containing:
    /// - Contract ID
    /// - Event Topics as a [`Vec<RawVal>`]
    /// - Event Data as a [`RawVal`]
    fn all(&self) -> Vec<(crate::BytesN<32>, Vec<RawVal>, RawVal)>;
}

/// Test utilities for [`Logger`][crate::logging::Logger].
pub trait Logger {
    /// Returns all debug events that have been logged.
    fn all(&self) -> std::vec::Vec<String>;
    /// Prints all debug events to stdout.
    fn print(&self);
}

/// Test utilities for [`AccountId`][crate::accounts::AccountId].
pub trait AccountId {
    /// Generate a random account ID.
    //
    // The value filled is not cryptographically secure.
    fn random(env: &Env) -> crate::AccountId;
}

/// Test utilities for [`Accounts`][crate::accounts::Accounts].
pub trait Accounts {
    /// Generate an account ID.
    fn generate(&self) -> crate::AccountId;

    /// Generate and account ID and creates an account.
    fn generate_and_create(&self) -> crate::AccountId;

    /// Create an account.
    fn create(&self, id: &crate::AccountId);

    /// Set the thresholds of an account.
    fn set_thresholds(&self, id: &crate::AccountId, low: u8, med: u8, high: u8);

    /// Set the weight of a signer of an account.
    ///
    /// Setting a weight of zero removes the signer from the account.
    fn set_signer_weight(&self, id: &crate::AccountId, signer: &crate::BytesN<32>, weight: u8);

    /// Remove an account.
    fn remove(&self, id: &crate::AccountId);
}

/// Test utilities for [`BytesN`][crate::BytesN].
pub trait BytesN<const N: usize> {
    // Generate a BytesN filled with random bytes.
    //
    // The value filled is not cryptographically secure.
    fn random(env: &Env) -> crate::BytesN<N>;
}

/// Generates an array of N random bytes.
///
/// The value returned is not cryptographically secure.
pub(crate) fn random<const N: usize>() -> [u8; N] {
    use rand::RngCore;
    let mut arr = [0u8; N];
    rand::thread_rng().fill_bytes(&mut arr);
    arr
}
