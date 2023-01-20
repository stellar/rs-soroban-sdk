use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "6c94401d269f2cbcd141c112ad2dc08371f04d557f45386d89d3d822dae18ce1",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
