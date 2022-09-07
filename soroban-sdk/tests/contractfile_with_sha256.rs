pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "3593fc3d411499d3a755d0ce08b32084047e9a61861aed923aa72252c2cef770",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
