#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

#[cfg(target_family = "wasm")]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// __link_sections returns and does nothing, but it contains link sections that
/// should be ensured end up in the final build of any contract using the SDK.
///
/// In Rust's build system sections only get included into the final build if
/// the object file containing those sections are processed by the linker, but
/// as an optimization step if no code is called in an object file it is
/// discarded.  This has the unfortunate effect of causing anything else in
/// those object files, such as link sections, to be discarded. Placing anything
/// that must be included in the build inside an exported function ensures the
/// object files won't be discarded. wasm-bindgen does a similar thing to this,
/// and so this seems to be a reasonably accepted way to work around this
/// limitation in the build system.
///
/// This has an unfortunate side-effect that all contracts will have a function
/// in the resulting WASM named `_`, however this function won't be rendered in
/// the contract specification. The overhead of this is very minimal on file
/// size.
///
/// See https://github.com/stellar/rs-soroban-sdk/issues/383 for more details.
#[export_name = "_"]
fn __link_sections() {
    #[cfg_attr(target_family = "wasm", link_section = "contractenvmetav0")]
    static __ENV_META_XDR: [u8; env::meta::XDR.len()] = env::meta::XDR;
}

pub use soroban_sdk_macros::{
    contractclient, contractfile, contractimpl, contractimport, contracttype, ContractType,
};

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
mod bytes;
mod contract_data;
pub mod iter;
mod ledger;
mod map;
mod vec;
pub use account::Account;
pub use bigint::BigInt;
#[allow(deprecated)]
pub use bytes::{Binary, FixedBinary};
pub use bytes::{Bytes, BytesN};
pub use contract_data::ContractData;
pub use ledger::Ledger;
pub use map::Map;
pub use vec::Vec;

pub mod serde;

pub mod testutils;
