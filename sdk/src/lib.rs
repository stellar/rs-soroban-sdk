#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

#[cfg(target_family = "wasm")]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// Env meta XDR returns the env meta XDR that describes the environment this
/// SDK is built with. This link section exists inside an exported function
/// which is imported by each contract function to ensure that the link section
/// is referenced by every object file that gets built, to ensure the link
/// section isn't only referenced in an object file that gets discarded.
/// See https://github.com/stellar/rs-soroban-sdk/issues/383 for more details.
#[doc(hidden)]
pub fn __env_meta_xdr() -> &'static [u8] {
    #[cfg_attr(target_family = "wasm", link_section = "contractenvmetav0")]
    static __ENV_META_XDR: [u8; env::meta::XDR.len()] = env::meta::XDR;
    &__ENV_META_XDR
}

pub use soroban_sdk_macros::{contractimpl, contracttype, ContractType};

mod env;

pub mod xdr;

pub use env::ConversionError;

pub use env::Env;
pub use env::EnvVal;
pub use env::RawVal;

pub use env::IntoVal;
pub use env::TryFromVal;
pub use env::TryIntoVal;

pub use env::Symbol;

mod envhidden {
    pub use super::env::EnvType;
    pub use super::env::Object;
    pub use super::env::Status;
}
#[doc(hidden)]
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
