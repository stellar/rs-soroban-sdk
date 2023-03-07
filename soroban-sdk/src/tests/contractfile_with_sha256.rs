use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "4448c5334008794ffdccbc55bfd1807dad98396b26b4e5f47d1426d8914d77d8",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
