#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

pub use stellar_contract_host::BitSet;
pub use stellar_contract_host::HostConvertable;
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

#[inline(always)]
pub const fn require(b: bool) {
    if !b {
        panic!();
    }
}

#[inline(always)]
pub fn log_value(v: Val) {
    unsafe {
        host::context::log_value(v);
    }
}

#[inline(always)]
pub fn call0(contract: Val, func: Symbol) -> Val {
    unsafe { host::call::call0(contract, func.into()) }
}

#[inline(always)]
pub fn call1(contract: Val, func: Symbol, a: Val) -> Val {
    unsafe { host::call::call1(contract, func.into(), a) }
}

#[inline(always)]
pub fn call2(contract: Val, func: Symbol, a: Val, b: Val) -> Val {
    unsafe { host::call::call2(contract, func.into(), a, b) }
}

#[inline(always)]
pub fn call3(contract: Val, func: Symbol, a: Val, b: Val, c: Val) -> Val {
    unsafe { host::call::call3(contract, func.into(), a, b, c) }
}

#[inline(always)]
pub fn call4(contract: Val, func: Symbol, a: Val, b: Val, c: Val, d: Val) -> Val {
    unsafe { host::call::call4(contract, func.into(), a, b, c, d) }
}
