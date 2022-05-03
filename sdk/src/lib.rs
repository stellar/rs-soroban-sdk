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
pub use stellar_contract_host::ValType;

#[cfg(target_family = "wasm")]
mod host;
#[cfg(not(target_family = "wasm"))]
mod testing;
#[cfg(not(target_family = "wasm"))]
use testing::host;

mod or_abort;
mod rt;
pub use or_abort::OrAbort;

mod bignum;
mod map;
mod vec;
pub use bignum::BigNum;
pub use map::Map;
pub use vec::Vec;
