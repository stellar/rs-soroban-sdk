use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "2a7398491f3999fa3c0b595cd84f15ad1d4449b6525b59ce67a4504595a72bfd",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
