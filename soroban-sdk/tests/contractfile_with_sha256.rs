pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "a99f9c800317a653dd6fe106e6c9789f0033b69b6bf9ff4c7fcdd63530557cb3",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
