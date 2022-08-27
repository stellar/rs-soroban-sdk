pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "21503b60e37e5488d383c666c42b310191b15082e57c94d81dd0686c092f4750"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
