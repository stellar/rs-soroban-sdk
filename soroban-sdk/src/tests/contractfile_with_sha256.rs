use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "c88c56ae1ba2b4d81b7bd8a0ab17c0ccb717db46eac707536059a626108bf5ae",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
