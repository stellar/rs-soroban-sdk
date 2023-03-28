use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "18788979ec03430c36f36986267f4535c7293258211fa25bc49b48c2a95f7fd8",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
