// An `extend_to` large enough that `sequence_number + extend_to` overflows a
// u32 is clamped to the max TTL, rather than failing with an InternalError.
// https://github.com/stellar/rs-soroban-env/pull/1729
use crate::{
    self as soroban_sdk,
    testutils::{storage::Instance as _, storage::Persistent as _, Ledger as _},
};
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set_persistent(e: Env, key: Symbol) {
        e.storage().persistent().set(&key, &());
    }

    pub fn extend_persistent(e: Env, key: Symbol, threshold: u32, extend_to: u32) {
        e.storage()
            .persistent()
            .extend_ttl(&key, threshold, extend_to)
    }

    pub fn extend_instance(e: Env, threshold: u32, extend_to: u32) {
        e.storage().instance().extend_ttl(threshold, extend_to)
    }
}

#[test]
fn test_extend_persistent_overflow_clamps_to_max_ttl() {
    let e = Env::default();
    // Non-zero, so that `sequence_number + u32::MAX` overflows.
    e.ledger().set_sequence_number(1000);
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);
    let key = symbol_short!("k");

    client.set_persistent(&key);
    e.as_contract(&contract_id, || {
        assert_ne!(
            e.storage().persistent().get_ttl(&key),
            e.storage().max_ttl()
        );
    });

    client.extend_persistent(&key, &u32::MAX, &u32::MAX);

    e.as_contract(&contract_id, || {
        assert_eq!(
            e.storage().persistent().get_ttl(&key),
            e.storage().max_ttl()
        );
    });
}

#[test]
fn test_extend_instance_overflow_clamps_to_max_ttl() {
    let e = Env::default();
    // Non-zero, so that `sequence_number + u32::MAX` overflows.
    e.ledger().set_sequence_number(1000);
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    e.as_contract(&contract_id, || {
        assert_ne!(e.storage().instance().get_ttl(), e.storage().max_ttl());
    });

    client.extend_instance(&u32::MAX, &u32::MAX);

    e.as_contract(&contract_id, || {
        assert_eq!(e.storage().instance().get_ttl(), e.storage().max_ttl());
    });
}

#[test]
fn test_deployer_extend_ttl_overflow_clamps_to_max_ttl() {
    let e = Env::default();
    // Non-zero, so that `sequence_number + u32::MAX` overflows.
    e.ledger().set_sequence_number(1000);
    let contract_id = e.register(Contract, ());

    e.as_contract(&contract_id, || {
        assert_ne!(e.storage().instance().get_ttl(), e.storage().max_ttl());
    });

    e.deployer()
        .extend_ttl(contract_id.clone(), u32::MAX, u32::MAX);

    e.as_contract(&contract_id, || {
        assert_eq!(e.storage().instance().get_ttl(), e.storage().max_ttl());
    });
}
