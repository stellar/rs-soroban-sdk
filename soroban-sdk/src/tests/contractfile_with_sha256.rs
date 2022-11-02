use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "134b7bb632955e2851c85015852fbf1e0d6e7625029b2e38a54cb9044d9501da",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
