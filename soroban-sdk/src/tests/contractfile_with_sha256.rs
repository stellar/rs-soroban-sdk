use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "58b7eeba82f4ff7cd34000422057d2dead13f22806fb01f2f1c93088e9b3319d",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
