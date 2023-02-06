use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "8defec8d424eb76db7e1d66a7d19e31ff34afae5dafdf5fca9ec59ed53ab9a63",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
