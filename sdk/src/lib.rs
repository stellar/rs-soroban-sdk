#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

#[cfg(target_family = "wasm")]
mod guest;
#[cfg(target_family = "wasm")]
pub use guest::*;

#[cfg(not(target_family = "wasm"))]
mod host;
#[cfg(not(target_family = "wasm"))]
pub use host::*;

mod object_type;

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
