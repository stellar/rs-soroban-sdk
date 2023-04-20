use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "2d04e781c8d1f00f257a03d143ac91187f72630c1a67b12c07d8d789e91ee258",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
