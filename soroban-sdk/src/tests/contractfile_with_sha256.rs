use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "f0554f39a5ea3fe414d11b52831ea9a5a65e62a171b8c18f2bd51b2365cbb242",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
