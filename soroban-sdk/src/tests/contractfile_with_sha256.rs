use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "88d88c54624c85d4d6e4654150cd76aaf34614dfaef67c73aa4a793614687798",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
