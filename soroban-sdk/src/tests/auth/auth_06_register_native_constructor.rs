//! Demonstrates that register and register_at switch to recording auth for
//! native contract constructors.
//!
//! A native contract whose constructor calls `require_auth` must succeed when
//! registered with `register` or `register_at`, the same as the Wasm paths.
//! Without switching to recording auth the constructor runs under the default
//! enforcing auth and `require_auth` fails.

use crate as soroban_sdk;

use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&"admin", &admin);
    }
}

// Auth is intentionally not mocked in these tests.

#[test]
fn register() {
    let e = Env::default();

    let admin = Address::generate(&e);
    e.register(Contract, (&admin,));
}

#[test]
fn register_at() {
    let e = Env::default();

    let admin = Address::generate(&e);
    let contract_id = Address::generate(&e);
    e.register_at(&contract_id, Contract, (&admin,));
}
