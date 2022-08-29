pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "ac021f2706482e0e9c98c9271a036752d1b8e212bb6ad1269d555c987eb4c652"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
