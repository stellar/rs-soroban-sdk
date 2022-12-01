use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "9cdd4b74295e226c9d6f26d40106a59fe0b69ba1d2b5f7a9cc3e5e94f5508ad5",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
