use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "79c66725beb0f3112bf5625d6ebb411ce4da1ddcd17db5024a6844831c53ba70",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
