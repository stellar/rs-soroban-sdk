use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "d0ede3651a0664f20ce8ec02c0686fa36074a1f849a6af0b5381c07f42d80f96",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
