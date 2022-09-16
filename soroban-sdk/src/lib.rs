//! Soroban SDK supports writing programs for the Soroban smart contract
//! platform.
//!
//! ### Docs
//!
//! See [soroban.stellar.org](https://soroban.stellar.org) for documentation.
//!
//! ### Examples
//!
//! ```
//! use soroban_sdk::{contractimpl, symbol, vec, BytesN, Env, Symbol, Vec};
//!
//! pub struct HelloContract;
//!
//! #[contractimpl]
//! impl HelloContract {
//!     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
//!         vec![&env, symbol!("Hello"), to]
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_id = BytesN::from_array(&env, &[0; 32]);
//!     env.register_contract(&contract_id, HelloContract);
//!     let client = HelloContractClient::new(&env, &contract_id);
//!
//!     let words = client.hello(&symbol!("Dev"));
//!
//!     assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! More examples are available at <https://soroban.stellar.org/docs/category/examples>.

#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

#[cfg(not(target_family = "wasm"))]
extern crate std;

#[cfg(all(target_family = "wasm", feature = "testutils"))]
compile_error!("'testutils' feature is not supported on 'wasm' target");

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
#[cfg(target_family = "wasm")]
#[export_name = "_"]
fn __link_sections() {
    #[link_section = "contractenvmetav0"]
    static __ENV_META_XDR: [u8; env::meta::XDR.len()] = env::meta::XDR;
}

#[doc(hidden)]
pub use bytes_lit::bytes as __bytes_lit_bytes;

pub use soroban_sdk_macros::{
    contractclient, contractfile, contractimpl, contractimport, contracttype,
};

mod env;

pub mod xdr;

pub use env::ConversionError;

pub use env::Env;
/// Raw value of the Soroban smart contract platform that types can be converted
/// to and from for storing, or passing between contracts.
///
pub use env::RawVal;

/// Used to do conversions between values in the Soroban environment.
pub use env::FromVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::IntoVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::TryFromVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::TryIntoVal;

pub use env::Symbol;

mod envhidden {
    pub use super::env::EnvVal;
    pub use super::env::Object;
    pub use super::env::Status;
}
#[doc(hidden)]
pub use envhidden::*;

mod operators;

mod account;
mod bigint;
mod bytes;
pub mod contract_data;
pub mod deploy;
pub mod events;
pub mod iter;
pub mod ledger;
pub mod logging;
mod map;
mod set;
mod symbol;
mod vec;
pub use account::Account;
pub use bigint::{BigInt, Sign};
pub use bytes::{Bytes, BytesN};
pub use map::Map;
pub use set::Set;
pub use vec::Vec;

pub mod serde;

pub mod testutils;
