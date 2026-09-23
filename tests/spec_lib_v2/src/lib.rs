#![cfg_attr(target_family = "wasm", no_std)]

// The types of test_spec_lib, built with soroban-sdk 28.0.0 from crates.io for
// test_spec_shaking_v2.
#[cfg(target_family = "wasm")]
#[allow(unused_attributes)]
#[path = "../../spec_lib/src/lib.rs"]
mod lib;
#[cfg(target_family = "wasm")]
pub use lib::*;
