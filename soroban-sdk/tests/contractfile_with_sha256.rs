pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "c0c5db07dd4db1ff9f38d69b6ef9b7a73acaa8af94eba17515874be9b22f2a48",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
