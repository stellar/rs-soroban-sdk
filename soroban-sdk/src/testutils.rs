#![cfg(any(test, feature = "testutils"))]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

mod sign;
use rand::RngCore;
pub use sign::ed25519;

pub use crate::env::testutils::*;

use crate::{AccountId, BytesN, Env, RawVal, Symbol, Vec};

use crate::env::xdr;

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
    /// Prints all debug events to stdout.
    fn print(&self);
}

/// Test utilities for [`Accounts`][crate::accounts::Accounts].
pub trait Accounts {
    /// Generate an account ID.
    fn generate(&self) -> AccountId;

    /// Generate and account ID and creates an account.
    fn generate_and_create(&self) -> AccountId;

    /// Create an account.
    fn create(&self, id: &AccountId);

    /// Set the thresholds of an account.
    fn set_thresholds(&self, id: &AccountId, low: u8, med: u8, high: u8);

    /// Set the weight of a signer of an account.
    ///
    /// Setting a weight of zero removes the signer from the account.
    fn set_signer_weight(&self, id: &AccountId, signer: &BytesN<32>, weight: u8);

    /// Remove an account.
    fn remove(&self, id: &AccountId);
}

pub fn random_id() -> [u8; 32] {
    let mut id = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut id);
    id
}

pub fn random_account_id() -> xdr::AccountId {
    xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(
        random_id(),
    )))
}

impl Env {
    pub fn random_id_bytes(&self) -> BytesN<32> {
        BytesN::from_array(self, &random_id())
    }
}
