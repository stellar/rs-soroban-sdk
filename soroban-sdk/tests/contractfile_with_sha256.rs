pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "26f5c00d05439950c7edbfcc1f2bd47f383094b2105f8473168407636b050258",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
