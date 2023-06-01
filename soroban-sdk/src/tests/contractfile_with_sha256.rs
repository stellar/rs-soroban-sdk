use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "b5dbebd9685208acff793bf22ac64730dad9a68f7cfb244da1522274ceb9be08",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
