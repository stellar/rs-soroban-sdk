use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "35e51986009a661965c8d7ad5b4ea1644278f69cdd3b2242f7e0eb4aee06ff76",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
