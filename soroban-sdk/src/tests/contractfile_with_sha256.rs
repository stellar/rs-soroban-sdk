use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "ccdd95bcb18a2c0dcf4d277cb12bd905ce2442b38d9320c294184e6a5135b6e0",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
