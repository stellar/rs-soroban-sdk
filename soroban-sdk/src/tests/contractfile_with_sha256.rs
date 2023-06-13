use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "e1999f2f615c801df85960ecda15790a4c4499a5a446978b51cee61c106a4fab",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
