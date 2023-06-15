use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "2a83b4b0370b8806b08c56f5c1cf8430c7eb029accb6cdbc403d5b82479b1316",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
