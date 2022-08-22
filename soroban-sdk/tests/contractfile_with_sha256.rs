pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "6c5a4fe67b30474ff71fe489ac7321d850b91cc2c7db3f36f377fad3d3087d6f"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
