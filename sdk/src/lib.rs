#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

#[cfg(target_family = "wasm")]
use stellar_contract_env_panic_handler_wasm32_unreachable as _;

pub use stellar_contract_macros::{contract, contractimpl, contracttype, ContractType};

mod env;
#[doc(hidden)]
pub use env::meta;
pub mod xdr {
    pub use super::env::xdr::*;
}
pub use env::BitSet;
pub use env::ConversionError;
pub use env::Env;
#[doc(hidden)]
pub use env::EnvType;
pub use env::EnvVal;
#[doc(hidden)]
pub use env::IntoEnvVal;
pub use env::IntoVal;
#[doc(hidden)]
pub use env::Object;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TryFromVal;
#[doc(hidden)]
pub use env::TryIntoEnvVal;
pub use env::TryIntoVal;
use env::*;

mod bigint;
mod binary;
pub mod iter;
mod map;
mod vec;
pub use bigint::BigInt;
pub use binary::{ArrayBinary, Binary, FixedLengthBinary, VariableLengthBinary};
pub use map::Map;
pub use vec::Vec;

mod test_contract;
mod test_sign;
#[cfg(feature = "testutils")]
pub use test_contract::ContractFunctionSet;
#[cfg(feature = "testutils")]
pub use test_sign::{ed25519, Sign};
