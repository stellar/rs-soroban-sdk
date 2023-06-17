use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "49513de30458f5add851d5c0070e853053226e09e4563241bd5106eff0b3e337",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
