use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "d85caca68cae120de2ebf069e3653774f2e86e1e153dff9efea42b8885abd970",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
