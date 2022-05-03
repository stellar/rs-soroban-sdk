#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

pub use stellar_contract_host::BitSet;
pub use stellar_contract_host::ObjType;
pub use stellar_contract_host::Object;
pub use stellar_contract_host::Status;
pub use stellar_contract_host::Symbol;
pub use stellar_contract_host::Val;

#[cfg(target_family = "wasm")]
mod host;
#[cfg(target_family = "wasm")]
mod host_fns;
#[cfg(target_family = "wasm")]
pub use host::Host;
#[cfg(not(target_family = "wasm"))]
pub use stellar_contract_host::Host;

mod bignum;
mod map;
mod or_abort;
mod rt;
mod vec;

pub use bignum::BigNum;
pub use map::Map;
pub use or_abort::OrAbort;
pub use vec::Vec;
