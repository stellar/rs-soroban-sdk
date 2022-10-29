use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "e7a0f67985ed069295c0b9cda84948ce037bf9663b6353f72b3180067dc300d0",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
