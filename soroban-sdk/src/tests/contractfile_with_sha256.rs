use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "09fb4f148df9aadbaa58892a13649a62b8b2143a7cbfd5634ee829d4daeacb40",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
