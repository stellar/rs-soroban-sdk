#![cfg_attr(target_family = "wasm", no_std)]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

mod env;
pub use env::BitSet;
pub use env::Env;
pub use env::IntoVal;
pub use env::OrAbort;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TryFromVal;
use env::*;

mod bigint;
mod map;
mod vec;
pub use bigint::BigInt;
pub use map::Map;
pub use vec::Vec;
