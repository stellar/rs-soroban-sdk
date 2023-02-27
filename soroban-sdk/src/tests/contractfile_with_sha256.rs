use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "50ef498f6a4ed5f793f4b0cacc3283eec3d134f88f4e597936efa558c5ba5093",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
