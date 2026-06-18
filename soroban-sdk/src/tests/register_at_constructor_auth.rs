//! Regression test for https://github.com/stellar/rs-soroban-sdk/issues/1738.
//!
//! `register_at` must run a Wasm contract's constructor under recording auth,
//! the same way `register` does, so that constructors calling `require_auth`
//! succeed instead of failing with an unexpected auth error.

use crate as soroban_sdk;

use soroban_sdk::{testutils::Address as _, Address, Env};

// A Wasm contract whose `__constructor` calls `require_auth` on the admin
// address it is passed and then stores it in instance storage.
const WASM: &[u8] =
    include_bytes!("../../../target/wasm32v1-none/release/test_constructor_with_auth.wasm");

#[test]
fn test_register_at_runs_constructor_under_recording_auth() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = Address::generate(&env);

    // Without recording auth around the constructor, the `require_auth` call in
    // the constructor fails and this panics. With it, the constructor succeeds.
    env.register_at(&contract_id, WASM, (admin.clone(),));

    // Confirm the constructor actually ran by reading back the stored admin.
    let stored: Address =
        env.as_contract(&contract_id, || env.storage().instance().get(&()).unwrap());
    assert_eq!(stored, admin);
}
