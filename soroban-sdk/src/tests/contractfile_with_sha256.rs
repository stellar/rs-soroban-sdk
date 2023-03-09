use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "4a691cf1fb566451424a769113d19336686624562ceb436f7a652ae8967343aa",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
