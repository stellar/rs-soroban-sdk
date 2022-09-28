pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "c98df4de5c71eccfcf9c0df3c86b27aeff9fa5142e8a37bb547e66db3995b033",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
