use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "de2bdc56dafd6b3b25b7224f7f9f33b4e32a62a1f3cc63b856244de236b690b8",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
