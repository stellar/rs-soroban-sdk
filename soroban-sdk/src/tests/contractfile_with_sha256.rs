use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "a3c977dded2fb2e117e62e8aacdd340b9f0b68ae71d85518d3ff56824fbb6e15",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
