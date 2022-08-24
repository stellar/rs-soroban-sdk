pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
    sha256 = "cce96634dca6b60232ee09d750c034804bb36aaadfb714fc0a6128ea4aea42d8"
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
