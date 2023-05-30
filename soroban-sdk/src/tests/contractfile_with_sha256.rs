use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "c51f05bd99a9ca1bd77f57ddaba0ca42e6c652bb41be32c48b9fbbb490550e3c",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
