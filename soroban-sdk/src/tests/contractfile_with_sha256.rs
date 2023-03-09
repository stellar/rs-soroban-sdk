use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "4f19178d3bbcc05e54ab93fdf402b5e57cd5f1d995227fc6e9ffb29e714aee9c",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
