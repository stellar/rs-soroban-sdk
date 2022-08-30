pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "e353760a571abf2c69493af1f433593438b540a509cc19f4f56bd0d4b1e9c5db",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
