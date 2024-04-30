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
fn live_until_ledger_getters() {
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

    // Initial live until ledgers are defined by min persistent/temp entry TTLs for the
    // persistent/temp entries respectively.
    // The current ledger is included into min TTL.
    let test_initial_storage_ttls = || {
        assert_eq!(e.storage().instance().get_live_until_ledger(), 1099);
        assert_eq!(e.storage().persistent().get_live_until_ledger(&1), 1099);
        assert_eq!(e.storage().temporary().get_live_until_ledger(&2), 1009);
    };
    e.as_contract(&contract_a, test_initial_storage_ttls);
    e.as_contract(&contract_b, test_initial_storage_ttls);

    // Instance and code have the same initial TTL as any other persistent entry.
    for from_contract in [&contract_a, &contract_b] {
        e.as_contract(from_contract, || {
            assert_eq!(
                e.deployer()
                    .get_contract_instance_live_until_ledger(&contract_a),
                1099
            );
            assert_eq!(
                e.deployer()
                    .get_contract_code_live_until_ledger(&contract_a),
                1099
            );
            assert_eq!(
                e.deployer()
                    .get_contract_instance_live_until_ledger(&contract_b),
                1099
            );
            assert_eq!(
                e.deployer()
                    .get_contract_code_live_until_ledger(&contract_b),
                1099
            );
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

    // Contract A has live until ledger increased corresponding to TTL extensions.
    // When TTL is extended, the current ledger is not included when computing the final
    // live_until_ledger.
    e.as_contract(&contract_a, || {
        assert_eq!(e.storage().instance().get_live_until_ledger(), 2000);
        assert_eq!(e.storage().persistent().get_live_until_ledger(&1), 1500);
        assert_eq!(e.storage().temporary().get_live_until_ledger(&2), 1300);
    });
    // Contract B has no TTLs extended for its own storage.
    e.as_contract(&contract_b, test_initial_storage_ttls);

    for from_contract in [&contract_a, &contract_b] {
        e.as_contract(from_contract, || {
            assert_eq!(
                e.deployer()
                    .get_contract_instance_live_until_ledger(&contract_a),
                2000
            );
            assert_eq!(
                e.deployer()
                    .get_contract_code_live_until_ledger(&contract_a),
                3000
            );
            // Instance hasn't been extended for B.
            assert_eq!(
                e.deployer()
                    .get_contract_instance_live_until_ledger(&contract_b),
                1099
            );
            // Code has been extended for B.
            assert_eq!(
                e.deployer()
                    .get_contract_code_live_until_ledger(&contract_b),
                3000
            );
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
        // The entry is written and the new `live_until_ledger` is set based on
        // min temp entry TTL.
        assert_eq!(e.storage().temporary().get(&1), Some(3));
        assert_eq!(e.storage().temporary().get_live_until_ledger(&1), 2099);
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
