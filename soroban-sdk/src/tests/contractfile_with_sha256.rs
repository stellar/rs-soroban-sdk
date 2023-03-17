use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "eb756067d9341b7658c8943d69a41ff548da1917f023c6b63085ebece3fd69e2",
);

#[test]
fn test_spec() {
    assert!(!WASM.is_empty());
}
