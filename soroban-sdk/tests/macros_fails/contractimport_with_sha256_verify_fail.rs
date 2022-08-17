#![cfg(feature = "testutils")]

soroban_sdk::contractimport!(
    file = "../../../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "0000000000000000000000000000000000000000000000000000000000000000",
);

pub fn main() {}
