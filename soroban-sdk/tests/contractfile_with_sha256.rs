pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "0ef130ce81f57cb03a9e20b0e8b5c9e47bf8c50cf0a811e4b25535ad95b92a7e"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
