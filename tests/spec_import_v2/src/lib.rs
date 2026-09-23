#![cfg_attr(target_family = "wasm", no_std)]

// The contract of test_spec_import, built with soroban-sdk 28.0.0 from
// crates.io for test_spec_shaking_v2 to import, since that SDK cannot read the
// spec of a contract built with the current one.
#[cfg(target_family = "wasm")]
#[allow(unused_attributes)]
#[path = "../../spec_import/src/lib.rs"]
mod contract;
