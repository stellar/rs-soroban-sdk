use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "3fbbb2d963abf7a0c5cb5676bed7faa7492266ac9c9f84df0b8af8cb71b0d3f6",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
