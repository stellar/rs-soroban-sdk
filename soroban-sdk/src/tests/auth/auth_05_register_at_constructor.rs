//! Demonstrates that register_at switches to recording auth for Wasm constructors.
//!
//! Regression test for https://github.com/stellar/rs-soroban-sdk/issues/1738: a
//! Wasm contract whose constructor calls `require_auth` must succeed when
//! registered with `register_at`, the same as when registered with `register`.
//! The contract has to be a Wasm contract because native contracts registered
//! for testing do not go through the same constructor invocation path.

use crate as soroban_sdk;

use soroban_sdk::{testutils::Address as _, Address, Env};

mod contract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32v1-none/release/test_constructor_with_auth.wasm"
    );
}

#[test]
fn test() {
    let e = Env::default();

    // Note: auth is intentionally not mocked. Like `register`, `register_at`
    // must switch to recording auth for the constructor call so that a
    // constructor calling `require_auth` is auto-authorized during
    // registration. Without that switch the constructor runs under the
    // default enforcing auth and `require_auth` fails.
    let admin = Address::generate(&e);
    let contract_id = Address::generate(&e);
    e.register_at(&contract_id, contract::WASM, (&admin,));
}
