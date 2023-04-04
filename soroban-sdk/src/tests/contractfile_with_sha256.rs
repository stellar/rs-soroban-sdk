use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "41797b764142f3cb973ac855ac74647e9e53cc881d47d5806bfae55f6ff76d76",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
