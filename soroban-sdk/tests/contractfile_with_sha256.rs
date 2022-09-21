pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "994ab7061a7b085e666eb36583636f1990f8d92fa1bf0b9b1e343472d209026b",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
