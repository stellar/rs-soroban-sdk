use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "d86c76c8dd3d312881af263cd0190b6d6c40344f0788d05d178d275a1caefe66",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
