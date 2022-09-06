pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "d4096b608d95d17aca620e294a3840b683b19badb6e9c0ce110ce8f1ce421b78",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
