#![no_main]

//! Differential fuzz of the const SHA-256 against the `sha2` crate.
//!
//! The marker that spec shaking keys on is derived from this hash, so a
//! divergence from a real SHA-256 would silently strip spec entries that are in
//! use. The unit tests cover the padding boundaries by hand; this covers
//! arbitrary lengths and contents.
//!
//! The implementation is included by path rather than imported from
//! `soroban-spec` so that `sha256` stays private to the crate.

use libfuzzer_sys::fuzz_target;
use sha2::{Digest, Sha256};

#[path = "../../src/shaking/sha256.rs"]
mod sha256;

fuzz_target!(|data: &[u8]| {
    let expected: [u8; 32] = Sha256::digest(data).into();
    assert_eq!(
        sha256::sha256(data),
        expected,
        "const sha256 diverged from sha2 for {} bytes",
        data.len()
    );
});
