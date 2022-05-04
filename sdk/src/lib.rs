#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

// TODO: Make most of these not pub and shift their uses to appropriate places.
pub use stellar_contract_env::BitSet;
pub use stellar_contract_env::EnvVal;
pub use stellar_contract_env::EnvValType;
pub use stellar_contract_env::HasEnv;
pub use stellar_contract_env::Object;
pub use stellar_contract_env::Status;
pub use stellar_contract_env::Symbol;
pub use stellar_contract_env::Val;
pub use stellar_contract_env::ValType;

// mod env;
mod object_type;
// pub use env::Ctx;

#[cfg(target_family = "wasm")]
pub use stellar_contract_env::Guest as Env;
#[cfg(not(target_family = "wasm"))]
pub use stellar_contract_env::WeakHost as Env;

mod or_abort;
mod rt;
pub use or_abort::OrAbort;

mod bignum;
mod map;
mod vec;
pub use bignum::BigNum;
pub use map::Map;
pub use vec::Vec;

#[inline(always)]
pub const fn require(b: bool) {
    if !b {
        panic!();
    }
}
