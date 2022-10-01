#![cfg(any(test, feature = "testutils"))]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

mod sign;
pub use sign::ed25519;

pub use crate::env::testutils::*;

use crate::{xdr, Env, RawVal, Symbol, Vec};

#[doc(hidden)]
pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: Env, args: &[RawVal]) -> Option<RawVal>;
}

/// Test utilities for [`Ledger`][crate::ledger::Ledger].
pub trait Ledger {
    /// Set ledger info.
    fn set(&self, l: LedgerInfo);

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
}

/// Test utilities for [`Accounts`][crate::accounts::Accounts].
pub trait Accounts {
    /// Create an account.
    fn create(&self, id: &xdr::AccountId);

    /// Set the details for an account.
    ///
    /// Creates the account if the account does not exist. Updates the details if it does.
    fn set(&self, l: xdr::AccountEntry);

    /// Modify an account.
    fn with_mut<F>(&self, id: &xdr::AccountId, f: F)
    where
        F: FnMut(&mut xdr::AccountEntry);

    /// Remove an account.
    fn remove(&self, id: &xdr::AccountId);
}
