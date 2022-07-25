#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

#[cfg(target_family = "wasm")]
use stellar_contract_env_panic_handler_wasm32_unreachable as _;

#[cfg_attr(target_family = "wasm", link_section = "contractenvmetav0")]
static ENV_META_XDR: [u8; env::meta::XDR.len()] = env::meta::XDR;

pub use stellar_contract_macros::{contractimpl, contracttype, ContractType};

mod env;
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
pub use binary::{ArrayBinary, Binary};
pub use iter::{UncheckedEnumerable, UncheckedIter};
pub use map::Map;
pub use vec::Vec;

pub mod testutils;
