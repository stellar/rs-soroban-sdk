use crate::testutils::storage::{Instance, Persistent, Temporary};
use crate::testutils::{Deployer as _, Ledger};
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

    // Shorter names for the with_limits variants (max 32 chars)
    pub fn ext_ttl_persistent_lim(env: Env, k: i32, extend_to: u32, min_ext: u32, max_ext: u32) {
        env.storage().persistent().extend_ttl_with_limits(
            &DataKey::Key(k),
            extend_to,
            min_ext,
            max_ext,
        );
    }

    pub fn ext_ttl_instance_lim(env: Env, extend_to: u32, min_ext: u32, max_ext: u32) {
        env.storage()
            .instance()
            .extend_ttl_with_limits(extend_to, min_ext, max_ext);
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
#[should_panic(expected = "trying to extend temporary entry past max TTL allowed by network")]
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

fn run_extend_ttl_with_limits_scenarios(
    e: &Env,
    extend_fn: &dyn Fn(u32, u32, u32),
    get_ttl_fn: &dyn Fn() -> u32,
) {
    let max_ttl = e.storage().max_ttl();
    // Initial TTL is 99 (min_persistent_entry_ttl - 1).
    assert_eq!(get_ttl_fn(), 99);

    // Extension clamped by max_extension.
    // extend_to=500, min=50, max=200 => extension = min(500-99, 200) = 200
    extend_fn(500, 50, 200);
    assert_eq!(get_ttl_fn(), 299); // 99 + 200

    // Repeated extension accumulates.
    // TTL=299, extend_to=500 => extension = min(500-299, 200) = 200
    extend_fn(500, 50, 200);
    assert_eq!(get_ttl_fn(), 499); // 299 + 200

    // Extension below min_extension is no-op.
    // TTL=499, extend_to=500 => extension = 1 < min(50) => no-op
    extend_fn(500, 50, 200);
    assert_eq!(get_ttl_fn(), 499); // unchanged

    // extend_to <= current TTL is no-op.
    extend_fn(400, 1, 500);
    assert_eq!(get_ttl_fn(), 499); // unchanged

    // max_extension caps a large extension.
    // TTL=499, extend_to=max_ttl(20000), max_extension=5000 => extension = 5000
    extend_fn(max_ttl, 1, 5000);
    assert_eq!(get_ttl_fn(), 5499); // 499 + 5000

    // Extension past max TTL is clamped.
    extend_fn(max_ttl + 1000, 1, 20000);
    assert_eq!(get_ttl_fn(), max_ttl);
}

#[test]
fn test_persistent_extend_ttl_with_limits() {
    let e = Env::default();
    e.ledger().set_min_persistent_entry_ttl(100);
    e.ledger().set_max_entry_ttl(20_000);

    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);
    e.as_contract(&contract_id, || {
        e.storage().persistent().set(&DataKey::Key(11), &1111_i32);
    });

    run_extend_ttl_with_limits_scenarios(
        &e,
        &|extend_to, min_ext, max_ext| {
            client.ext_ttl_persistent_lim(&11, &extend_to, &min_ext, &max_ext);
        },
        &|| {
            e.as_contract(&contract_id, || {
                e.storage().persistent().get_ttl(&DataKey::Key(11))
            })
        },
    );
}

#[test]
fn test_instance_extend_ttl_with_limits() {
    let e = Env::default();
    e.ledger().set_min_persistent_entry_ttl(100);
    e.ledger().set_max_entry_ttl(20_000);

    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    run_extend_ttl_with_limits_scenarios(
        &e,
        &|extend_to, min_ext, max_ext| {
            client.ext_ttl_instance_lim(&extend_to, &min_ext, &max_ext);
        },
        &|| e.as_contract(&contract_id, || e.storage().instance().get_ttl()),
    );
}

#[test]
#[should_panic(expected = "max_extension must be >= min_extension")]
fn test_extend_ttl_with_limits_invalid_max_less_than_min() {
    // This is a smoke test for how the error handling works when
    // max_extension < min_extension; this is part of the host function
    // implementation, so no need to cover all the scenarios.
    let e = Env::default();
    e.ledger().set_min_persistent_entry_ttl(100);
    e.ledger().set_max_entry_ttl(20_000);
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);
    e.as_contract(&contract_id, || {
        e.storage().persistent().set(&DataKey::Key(11), &1111_i32);
    });
    client.ext_ttl_persistent_lim(&11, &500, &200, &100);
}

#[test]
fn test_deployer_extensions_with_limits() {
    let e = Env::default();
    e.ledger().set_sequence_number(1000);
    e.ledger().set_min_persistent_entry_ttl(100);
    e.ledger().set_min_temp_entry_ttl(10);
    e.ledger().set_max_entry_ttl(20_000);

    let contract_a = e.register(Contract, ());
    let contract_b = e.register(Contract, ());
    let setup = || {
        e.storage().persistent().set(&1, &3);
        e.storage().temporary().set(&2, &4);
    };
    e.as_contract(&contract_a, setup);
    e.as_contract(&contract_b, setup);

    // Test extend_ttl_with_limits for deployer
    // Initial TTL is 99 for instance (min_persistent_entry_ttl - 1)

    // Test extend_ttl_with_limits (both instance and code)
    e.as_contract(&contract_a, || {
        // extend_to=500, min_extension=50, max_extension=200
        // extension = min(500-99, 200) = 200 (since 401 > 200)
        // new TTL = 99 + 200 = 299
        e.deployer()
            .extend_ttl_with_limits(contract_a.clone(), 500, 50, 200);
    });
    assert_eq!(e.deployer().get_contract_instance_ttl(&contract_a), 299);
    assert_eq!(e.deployer().get_contract_code_ttl(&contract_a), 299);
    // Contract B should be unaffected (different instance, but same code)
    // Code is shared so it's extended too
    assert_eq!(e.deployer().get_contract_instance_ttl(&contract_b), 99);
    assert_eq!(e.deployer().get_contract_code_ttl(&contract_b), 299);

    // Test extend_ttl_for_contract_instance_with_limits (instance only)
    e.as_contract(&contract_a, || {
        // extend_to=1000, min_extension=100, max_extension=300
        // instance TTL is 299, extension = min(1000-299, 300) = 300
        // new instance TTL = 299 + 300 = 599
        e.deployer().extend_ttl_for_contract_instance_with_limits(
            contract_a.clone(),
            1000,
            100,
            300,
        );
    });
    assert_eq!(e.deployer().get_contract_instance_ttl(&contract_a), 599);
    // Code should not change
    assert_eq!(e.deployer().get_contract_code_ttl(&contract_a), 299);

    // Test extend_ttl_for_code_with_limits (code only)
    e.as_contract(&contract_a, || {
        // extend_to=2000, min_extension=100, max_extension=500
        // code TTL is 299, extension = min(2000-299, 500) = 500
        // new code TTL = 299 + 500 = 799
        e.deployer()
            .extend_ttl_for_code_with_limits(contract_a.clone(), 2000, 100, 500);
    });
    assert_eq!(e.deployer().get_contract_instance_ttl(&contract_a), 599);
    assert_eq!(e.deployer().get_contract_code_ttl(&contract_a), 799);
    // Contract B code is shared, so it's also extended
    assert_eq!(e.deployer().get_contract_code_ttl(&contract_b), 799);

    // Test that min_extension prevents extension when below threshold
    e.as_contract(&contract_a, || {
        // extend_to=610, min_extension=100, max_extension=500
        // instance TTL is 599, extension would be 610-599=11 < min_extension(100)
        // Should be a no-op
        e.deployer().extend_ttl_for_contract_instance_with_limits(
            contract_a.clone(),
            610,
            100,
            500,
        );
    });
    assert_eq!(e.deployer().get_contract_instance_ttl(&contract_a), 599); // unchanged
}
