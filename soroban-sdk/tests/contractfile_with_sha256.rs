pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "bd54d7acd68edca80653a4cebdf8204fdb678de773911eba73fdd96962efd9b5",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
