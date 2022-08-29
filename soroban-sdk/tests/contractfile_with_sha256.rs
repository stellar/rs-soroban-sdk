pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "2e3b1bdaaf988ebfb44cfb811275b1605a08b7189e5f6b095cb5df8b20ecbaf4"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
