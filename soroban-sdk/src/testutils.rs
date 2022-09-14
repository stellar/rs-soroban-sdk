#![cfg(feature = "testutils")]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing.

mod test_sign;
pub use test_sign::ed25519;

pub use crate::env::testutils::*;

use crate::{Env, RawVal, Symbol, Vec};

#[doc(hidden)]
pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: Env, args: &[RawVal]) -> Option<RawVal>;
}

/// Test utilities for [`Events`][crate::Events].
pub trait Events {
    /// Returns all events that have been published by contracts.
    ///
    /// Returns a [`Vec`] of three element tuples containing:
    /// - Contract ID
    /// - Event Topics as a [`Vec<RawVal>`]
    /// - Event Data as a [`RawVal`]
    fn all(&self) -> Vec<(crate::BytesN<32>, Vec<RawVal>, RawVal)>;
}
