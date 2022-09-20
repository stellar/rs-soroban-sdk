pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "797acfc51769afd405d7176aa913b754301474e3b4a2c8317860ce02efc54b60",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
