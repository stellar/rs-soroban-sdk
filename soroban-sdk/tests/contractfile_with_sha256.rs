pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "082f8d7a70e88996451d2fa53844a95f8c6da4e9179f9ce133bd40489194b3ae"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
