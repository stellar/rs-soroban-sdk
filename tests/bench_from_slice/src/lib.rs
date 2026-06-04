#![no_std]
//! Contract used by the WASM compute-budget benches in
//! `soroban-sdk/src/tests/bench_from_slice_wasm.rs`.
//!
//! Each benched operation has:
//! - a `baseline_*` twin with the same signature that does the minimal amount
//!   of work, so its budget can be subtracted to remove dispatch + setup
//!   overhead, and
//! - a `vec_from_array_*` twin that builds the same Vec using the already-bulk
//!   `Vec::from_array`, as a reference for how cheap the operation can be.
//!
//! Every function returns a value derived from the constructed Vec (read back
//! from the host) so that the construction cannot be optimized out of the
//! guest WASM.

use soroban_sdk::{contract, contractimpl, Env, Vec};

#[contract]
pub struct Contract;

fn fingerprint(v: &Vec<u32>) -> u32 {
    let len = v.len();
    if len == 0 {
        0
    } else {
        v.get_unchecked(0).wrapping_add(v.get_unchecked(len - 1))
    }
}

#[contractimpl]
impl Contract {
    pub fn baseline_32(_env: Env) -> u32 {
        let data = [7u32; 32];
        data[0].wrapping_add(data[31])
    }
    pub fn baseline_96(_env: Env) -> u32 {
        let data = [7u32; 96];
        data[0].wrapping_add(data[95])
    }
    pub fn baseline_192(_env: Env) -> u32 {
        let data = [7u32; 192];
        data[0].wrapping_add(data[191])
    }

    // Vec::from_slice (the operation under test).
    pub fn vec_from_slice_32(env: Env) -> u32 {
        fingerprint(&Vec::from_slice(&env, &[7u32; 32]))
    }
    pub fn vec_from_slice_96(env: Env) -> u32 {
        fingerprint(&Vec::from_slice(&env, &[7u32; 96]))
    }
    pub fn vec_from_slice_192(env: Env) -> u32 {
        fingerprint(&Vec::from_slice(&env, &[7u32; 192]))
    }

    // Vec::from_array (already bulk), reference.
    pub fn vec_from_array_32(env: Env) -> u32 {
        fingerprint(&Vec::from_array(&env, [7u32; 32]))
    }
    pub fn vec_from_array_96(env: Env) -> u32 {
        fingerprint(&Vec::from_array(&env, [7u32; 96]))
    }
    pub fn vec_from_array_192(env: Env) -> u32 {
        fingerprint(&Vec::from_array(&env, [7u32; 192]))
    }
}
