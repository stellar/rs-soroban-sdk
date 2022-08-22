pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "4443da92909ba1b01564d65c74052cd5d39dceec7cc52e41bda38a13fdb6e160"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
