#[soroban_sdk::contractfile(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "58f98f171987ffff001c4c7f15357bb2c2a4870ec9294c28a401d6c0143ed0e5"
)]
pub struct X;

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
