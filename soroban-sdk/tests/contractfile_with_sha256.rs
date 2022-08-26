pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "58e28b943aeb95f3d0f9f8a87a2049d6f52a41f5dbaaa0b44e00a0e41d40cb68"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
