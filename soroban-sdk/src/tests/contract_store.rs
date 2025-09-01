use crate::testutils::storage::{Instance, Persistent, Temporary};
use crate::testutils::Ledger;
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

    pub fn extend_ttl_persistent(env: Env, k: i32, extend_to: u32) {
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::Key(k), extend_to, extend_to);
    }

    pub fn extend_ttl_temporary(env: Env, k: i32, extend_to: u32) {
        env.storage()
            .temporary()
            .extend_ttl(&DataKey::Key(k), extend_to, extend_to);
    }

    pub fn extend_ttl_instance(env: Env, extend_to: u32) {
        env.storage().instance().extend_ttl(extend_to, extend_to);
    }
}

#[test]
fn test_storage() {
    let e = Env::default();
    e.ledger().set_min_persistent_entry_ttl(100);
    e.ledger().set_min_temp_entry_ttl(50);
    e.ledger().set_max_entry_ttl(20_000);

    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    // Smoke test instance bump before putting any data into it.
    client.extend_ttl_instance(&1000);
    assert_eq!(
        e.as_contract(&contract_id, || e.storage().instance().get_ttl()),
        1000
    );

    assert!(client.get_persistent(&11).is_none());
    assert!(client.get_temporary(&11).is_none());
    assert!(client.get_instance(&11).is_none());

    // Setup the storage with some values.
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

    client.extend_ttl_persistent(&11, &10_000);
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .persistent()
            .get_ttl(&DataKey::Key(11))),
        10_000
    );

    client.extend_ttl_persistent(&11, &e.storage().max_ttl());
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .persistent()
            .get_ttl(&DataKey::Key(11))),
        e.storage().max_ttl()
    );
    // Persistent entry bumps past max TTL are clamped to be just max TTL.
    client.extend_ttl_persistent(&11, &(e.storage().max_ttl() + 1000));
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .persistent()
            .get_ttl(&DataKey::Key(11))),
        e.storage().max_ttl()
    );

    client.extend_ttl_temporary(&11, &5_000);
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .temporary()
            .get_ttl(&DataKey::Key(11))),
        5_000
    );
    client.extend_ttl_temporary(&11, &e.storage().max_ttl());
    assert_eq!(
        e.as_contract(&contract_id, || e
            .storage()
            .temporary()
            .get_ttl(&DataKey::Key(11))),
        e.storage().max_ttl()
    );
    // Extending temp entry TTL past max will panic, so that's not covered in this test.

    client.extend_ttl_instance(&2000);
    assert_eq!(
        e.as_contract(&contract_id, || e.storage().instance().get_ttl()),
        2000
    );

    client.extend_ttl_instance(&e.storage().max_ttl());
    assert_eq!(
        e.as_contract(&contract_id, || e.storage().instance().get_ttl()),
        e.storage().max_ttl()
    );
    // Persistent entry bumps past max TTL are clamped to be just max TTL, and
    // the instance storage is just a persistent entry.
    client.extend_ttl_instance(&(e.storage().max_ttl() + 1000));
    assert_eq!(
        e.as_contract(&contract_id, || e.storage().instance().get_ttl()),
        e.storage().max_ttl()
    );
}

#[test]
#[should_panic(expected = "trying to extend past max live_until ledger")]
fn test_temp_storage_extension_past_max_ttl_panics() {
    let e = Env::default();
    e.ledger().set_min_temp_entry_ttl(50);
    e.ledger().set_max_entry_ttl(20_000);
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);
    e.as_contract(&contract_id, || {
        e.storage().temporary().set(&DataKey::Key(11), &2222_i32);
    });
    client.extend_ttl_temporary(&11, &(e.storage().max_ttl() + 1));
}
