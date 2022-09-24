pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "7d4839b87fc1d005e4ef213b1e5c53e6ffddf577b0ca69818420c7a8865c385b",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
