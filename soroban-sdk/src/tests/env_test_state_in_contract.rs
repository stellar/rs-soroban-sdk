//! Tests for the test state of the [Env] passed into a contract function.
//!
//! The host only knows `EnvImpl`, so when it dispatches into a natively
//! registered contract it hands the function set an `&EnvImpl`. There is no
//! channel by which the calling test's [Env] could travel through the host and
//! come back, so the SDK builds one on the spot. The `env_impl` half is cloned,
//! so everything a contract can observe on chain is real and shared. The
//! `test_state` half has no counterpart to clone, so testutils functionality
//! that depends on it cannot work inside a contract function, and panics when
//! used there rather than silently operating on empty state.

use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractimpl,
    testutils::{Address as _, EnvTestConfig},
    Address, Env,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Uses the generators.
    pub fn gen_address(env: Env) -> Address {
        Address::generate(&env)
    }

    // Uses the generators, to generate the contract id to register at.
    pub fn register(env: Env) -> Address {
        env.register(Contract, ())
    }

    // Uses the auth snapshot.
    pub fn auth_count(env: Env, addr: Address) -> u32 {
        addr.require_auth();
        env.auths().len() as u32
    }

    // Uses the config.
    pub fn set_config(env: Env) {
        let mut env = env;
        env.set_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
    }

    // Uses the ledger snapshot.
    pub fn to_ledger_snapshot(env: Env) {
        env.to_ledger_snapshot();
    }

    // Uses the generators and the auth snapshot.
    pub fn to_snapshot(env: Env) {
        env.to_snapshot();
    }
}

/// The generators live in the test state, so generating an address inside a
/// contract panics. Without the panic the generators would restart from zero on
/// every invocation, silently returning addresses that collide with addresses
/// the calling test generated, and with each other.
#[test]
#[should_panic(expected = "generating values is unavailable inside a contract function")]
fn test_generate_address_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.gen_address();
}

/// Registering a contract generates the contract id to register at, and so it
/// too depends on the generators.
#[test]
#[should_panic(expected = "generating values is unavailable inside a contract function")]
fn test_register_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.register();
}

/// The auth snapshot lives in the test state, so calling `auths` inside a
/// contract panics. Without the panic the contract would observe an empty
/// snapshot, and assertions made on it inside a contract would pass vacuously.
#[test]
#[should_panic(expected = "the record of authorizations is unavailable inside a contract function")]
fn test_auths_in_contract() {
    let env = Env::default();
    env.mock_all_auths();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    let addr = Address::generate(&env);
    client.auth_count(&addr);
}

/// The config lives in the test state, so changing it inside a contract panics.
/// Without the panic the change would apply to an Env that is dropped when the
/// invocation returns, and so would have no effect.
#[test]
#[should_panic(expected = "the test config is unavailable inside a contract function")]
fn test_set_config_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.set_config();
}

/// The ledger snapshot the Env was created from lives in the test state, so
/// creating a ledger snapshot inside a contract panics.
#[test]
#[should_panic(expected = "the ledger snapshot is unavailable inside a contract function")]
fn test_to_ledger_snapshot_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.to_ledger_snapshot();
}

/// A snapshot is made up of the generators, the auth snapshot, and the ledger
/// snapshot, all of which live in the test state, so creating a snapshot inside
/// a contract panics.
#[test]
#[should_panic(expected = "generating values is unavailable inside a contract function")]
fn test_to_snapshot_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.to_snapshot();
}

/// The test's own Env keeps its test state even while executing in a contract
/// frame, so testutils functionality remains available to the test. It is the
/// Env the host passes to a contract function that has no test state, not any
/// Env executing in a contract frame.
#[test]
fn test_generate_address_in_contract_frame() {
    let env = Env::default();

    let contract_id = env.register(Contract, ());

    let first = Address::generate(&env);
    let second = env.as_contract(&contract_id, || Address::generate(&env));
    let third = env.as_contract(&contract_id, || Address::generate(&env));

    assert_ne!(first, second);
    assert_ne!(second, third);
}
