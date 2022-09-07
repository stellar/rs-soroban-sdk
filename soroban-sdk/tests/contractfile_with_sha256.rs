pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "1fe14981cdfc7949303bf3617b3b14c0db76828e42771510c0a391601ef4325b",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
