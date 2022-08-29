pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "29c2ddba158eb180082be9137d7818271f99c366d623e248363b7b518159f523",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
