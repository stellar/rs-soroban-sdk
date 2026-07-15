//! Demonstrates that register_at switches to recording auth for WASM constructors.

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

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let contract_id = Address::generate(&e);
    e.register_at(&contract_id, Contract, (&admin,));
}
