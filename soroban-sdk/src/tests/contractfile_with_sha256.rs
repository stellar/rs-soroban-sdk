use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "b7f7064dcffbd203fac4cb2b8503495a4c0d0d57324609affc753f5594523a69",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
