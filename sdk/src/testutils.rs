#![cfg(feature = "testutils")]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

mod test_sign;
pub use test_sign::ed25519;

use crate::{Env, RawVal, Symbol};

#[doc(hidden)]
pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: Env, args: &[RawVal]) -> Option<RawVal>;
}
