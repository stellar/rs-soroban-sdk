use crate::testutils::{Deployer, Ledger};
use crate::{
    self as soroban_sdk,
    testutils::storage::{Instance as _, Persistent as _, Temporary as _},
    Map, Val,
};
use soroban_sdk::{contract, Env};

#[contract]
pub struct Contract;

#[test]
fn all() {
    let e = Env::default();
    let id = e.register_contract(None, Contract);

    e.as_contract(&id, || {
        e.storage().instance().set(&1, &2);
        e.storage().instance().set(&1, &true);
        e.storage().instance().set(&2, &3);
        e.storage().persistent().set(&10, &20);
        e.storage().persistent().set(&10, &false);
        e.storage().persistent().set(&20, &30);
        e.storage().temporary().set(&100, &200);
        e.storage().temporary().set(&100, &());
        e.storage().temporary().set(&200, &300);
    });

    e.as_contract(&id, || {
        assert_eq!(
            e.storage().instance().all(),
            Map::<Val, Val>::from_array(&e, [(1.into(), true.into()), (2.into(), 3.into())])
        );
        assert_eq!(
            e.storage().persistent().all(),
            Map::<Val, Val>::from_array(&e, [(10.into(), false.into()), (20.into(), 30.into())])
        );
        assert_eq!(
            e.storage().temporary().all(),
            Map::<Val, Val>::from_array(&e, [(100.into(), ().into()), (200.into(), 300.into())])
        );
    });
}

#[test]
fn ttl_getters() {
    let e = Env::default();
    e.ledger().set_sequence_number(1000);
    e.ledger().set_min_persistent_entry_ttl(100);
    e.ledger().set_min_temp_entry_ttl(10);

    let contract_a = e.register_contract(None, Contract);
    let contract_b = e.register_contract(None, Contract);
    let setup = || {
        e.storage().persistent().set(&1, &3);
        e.storage().temporary().set(&2, &4);
    };
    e.as_contract(&contract_a, setup);
    e.as_contract(&contract_b, setup);

    // Initial TTLs are defined by min persistent/temp entry TTL settings for the
    // persistent/temp entries respectively.
    let test_initial_storage_ttls = || {
        assert_eq!(e.storage().instance().get_ttl(), 100);
        assert_eq!(e.storage().persistent().get_ttl(&1), 100);
        assert_eq!(e.storage().temporary().get_ttl(&2), 10);
    };
    e.as_contract(&contract_a, test_initial_storage_ttls);
    e.as_contract(&contract_b, test_initial_storage_ttls);

    // Instance and code have the same initial TTL as any other persistent entry.
    for from_contract in [&contract_a, &contract_b] {
        e.as_contract(from_contract, || {
            assert_eq!(e.deployer().get_contract_instance_ttl(&contract_a), 100);
            assert_eq!(e.deployer().get_contract_code_ttl(&contract_a), 100);
            assert_eq!(e.deployer().get_contract_instance_ttl(&contract_b), 100);
            assert_eq!(e.deployer().get_contract_code_ttl(&contract_b), 100);
        });
    }

    // Extend instance, code and entry TTLs for contract A.
    // Contract A and B share the code, so this also extends code (but not instance) for B.
    e.as_contract(&contract_a, || {
        e.storage().instance().extend_ttl(100, 1000);
        e.deployer()
            .extend_ttl_for_code(contract_a.clone(), 1000, 2000);
        e.storage().persistent().extend_ttl(&1, 100, 500);
        e.storage().temporary().extend_ttl(&2, 10, 300);
    });

    // Contract A has TTL extended for its entries.
    // When TTL is extended, the current ledger is not included in `extend_to`
    // parameter, so e.g. extending an entry to live for 1000 ledgers from now
    // means that the TTL becomes 1001 (current ledger + 1000 ledgers of extension).
    e.as_contract(&contract_a, || {
        assert_eq!(e.storage().instance().get_ttl(), 1001);
        assert_eq!(e.storage().persistent().get_ttl(&1), 501);
        assert_eq!(e.storage().temporary().get_ttl(&2), 301);
    });
    // Contract B has no TTLs extended for its own storage.
    e.as_contract(&contract_b, test_initial_storage_ttls);

    for from_contract in [&contract_a, &contract_b] {
        e.as_contract(from_contract, || {
            assert_eq!(e.deployer().get_contract_instance_ttl(&contract_a), 1001);
            assert_eq!(e.deployer().get_contract_code_ttl(&contract_a), 2001);
            // Instance hasn't been extended for B.
            assert_eq!(e.deployer().get_contract_instance_ttl(&contract_b), 100);
            // Code has been extended for B.
            assert_eq!(e.deployer().get_contract_code_ttl(&contract_b), 2001);
        });
    }
}

#[test]
fn temp_entry_expiration() {
    let e = Env::default();
    e.ledger().set_sequence_number(1000);
    e.ledger().set_min_temp_entry_ttl(100);
    let contract = e.register_contract(None, Contract);
    e.as_contract(&contract, || {
        e.storage().temporary().set(&1, &2);

        // Temp entry acts as if it doesn't exist after expiration.
        e.ledger().set_sequence_number(1100);
        assert!(!e.storage().temporary().has(&1));

        // Bump the ledger sequence back - the entry would exist again - test environment
        // doesn't *actually* delete anything. Normally ledger sequence can never decrease
        // though.
        e.ledger().set_sequence_number(1099);
        assert!(e.storage().temporary().has(&1));

        // Bump the ledger sequence past expiration and set the new value for the entry.
        e.ledger().set_sequence_number(2000);
        assert!(!e.storage().temporary().has(&1));
        e.storage().temporary().set(&1, &3);
        // The entry is written and the new TTL is set based on min temp entry TTL
        // setting.
        assert_eq!(e.storage().temporary().get(&1), Some(3));
        assert_eq!(e.storage().temporary().get_ttl(&1), 100);
    });
}

#[test]
#[should_panic(expected = "[testing-only] Accessed contract data key key that has been archived")]
fn test_persistent_entry_expiration() {
    let e = Env::default();
    e.ledger().set_sequence_number(1000);
    e.ledger().set_min_persistent_entry_ttl(100);

    let contract = e.register_contract(None, Contract);
    e.as_contract(&contract, || {
        e.storage().persistent().set(&1, &2);

        e.ledger().set_sequence_number(1100);
        // Persistent entries are archived when they expire and they no longer can be accessed
        // by the contracts at all.
        // In actual networks the contract interaction won't happen at all if the footprint
        // has any archived entries, but in the tests the closest thing we can do to that is
        // to just panic (even if the value doesn't need to be accessed).
        let _ = e.storage().persistent().has(&1);
    });
}
