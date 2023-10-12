use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
enum DataKey {
    Key(i32),
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn get_persistent(env: Env, k: i32) -> Option<i32> {
        env.storage().persistent().get(&DataKey::Key(k))
    }

    pub fn set_persistent(env: Env, k: i32, v: i32) {
        env.storage().persistent().set(&DataKey::Key(k), &v);
    }

    pub fn get_temporary(env: Env, k: i32) -> Option<i32> {
        env.storage().temporary().get(&DataKey::Key(k))
    }

    pub fn set_temporary(env: Env, k: i32, v: i32) {
        env.storage().temporary().set(&DataKey::Key(k), &v);
    }

    pub fn get_instance(env: Env, k: i32) -> Option<i32> {
        env.storage().instance().get(&DataKey::Key(k))
    }

    pub fn set_instance(env: Env, k: i32, v: i32) {
        env.storage().instance().set(&DataKey::Key(k), &v);
    }

    pub fn extend_ttl_persistent(env: Env, k: i32) {
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Key(k), 100, 100);
    }

    pub fn extend_ttl_temporary(env: Env, k: i32) {
        env.storage()
            .temporary()
            .extend_ttl(&DataKey::Key(k), 100, 100);
    }

    pub fn extend_ttl_instance(env: Env) {
        env.storage().instance().extend_ttl(100, 100);
    }
}

#[test]
fn test_storage() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    // Smoke test instance bump before putting any data into it.
    client.extend_ttl_instance();

    assert!(client.get_persistent(&11).is_none());
    assert!(client.get_temporary(&11).is_none());
    assert!(client.get_instance(&11).is_none());

    // Setup the the storage with some values.
    e.as_contract(&contract_id, || {
        e.storage().persistent().set(&DataKey::Key(11), &1111_i32);
        e.storage().temporary().set(&DataKey::Key(11), &2222_i32);
        e.storage().instance().set(&DataKey::Key(11), &3333_i32);
    });
    assert_eq!(client.get_persistent(&11), Some(1111));
    assert_eq!(client.get_temporary(&11), Some(2222));
    assert_eq!(client.get_instance(&11), Some(3333));

    client.set_persistent(&22, &111);
    assert_eq!(client.get_persistent(&22), Some(111));
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .persistent()
            .get(&DataKey::Key(22))),
        Some(111_i32)
    );

    client.set_temporary(&22, &222);
    assert_eq!(client.get_temporary(&22), Some(222));
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .temporary()
            .get(&DataKey::Key(22))),
        Some(222_i32)
    );

    client.set_instance(&22, &333);
    assert_eq!(client.get_instance(&22), Some(333));
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .instance()
            .get(&DataKey::Key(22))),
        Some(333_i32)
    );

    // Smoke test temp/persistent bumps. This can be enhanced when we provided
    // expiration ledger getter for tests.
    client.extend_ttl_persistent(&11);
    client.extend_ttl_temporary(&11);
}
