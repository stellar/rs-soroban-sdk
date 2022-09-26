pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "dd5db60c98c2dc971c9754e306e36122a1d3982d4e0928b36fb6491a95f58420",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
