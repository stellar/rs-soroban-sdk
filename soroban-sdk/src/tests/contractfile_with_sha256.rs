use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "a7c46bfab561dd831365ccb4e16cc21d9063b1874bb6d70999e52459cebacd68",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
