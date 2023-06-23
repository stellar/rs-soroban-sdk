use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "7f9584c08becb64dd7c5cc3c9dc1803d32366d6fa634bb0ebfda2851125edb13",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
