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

pub mod xdr;

pub use env::BitSet;
pub use env::ConversionError;
pub use env::Env;
pub use env::EnvVal;
pub use env::IntoVal;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TryFromVal;
pub use env::TryIntoVal;

#[doc(hidden)]
mod envhidden {
    pub use super::env::EnvType;
    pub use super::env::Object;
}
pub use envhidden::*;

mod account;
mod bigint;
mod binary;
mod contract_data;
pub mod iter;
mod map;
mod vec;
pub use account::Account;
pub use bigint::BigInt;
pub use binary::{Binary, FixedBinary};
pub use contract_data::ContractData;
pub use map::Map;
pub use vec::Vec;

pub mod serde;

pub mod testutils;
