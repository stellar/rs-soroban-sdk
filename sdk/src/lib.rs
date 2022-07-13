#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

#[cfg(target_family = "wasm")]
use stellar_contract_env_panic_handler_wasm32_unreachable as _;

pub use stellar_contract_macros::{contract, contractimpl, contracttype, ContractType};

mod env;
pub use env::INTERFACE_VERSION;
pub use env::xdr;
pub use env::BitSet;
pub use env::ConversionError;
pub use env::Env;
pub use env::EnvVal;
pub use env::IntoEnvVal;
pub use env::IntoVal;
pub use env::Object;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TryFromVal;
use env::*;

mod bigint;
mod binary;
mod iter;
mod map;
mod vec;
pub use bigint::BigInt;
pub use binary::{ArrayBinary, Binary, FixedLengthBinary, VariableLengthBinary};
pub use iter::{UncheckedEnumerable, UncheckedIter};
pub use map::Map;
pub use vec::Vec;
