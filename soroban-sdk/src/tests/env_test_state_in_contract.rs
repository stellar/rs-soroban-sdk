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
use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn gen_address(env: Env) -> Address {
        Address::generate(&env)
    }

    pub fn auth_count(env: Env, addr: Address) -> u32 {
        addr.require_auth();
        env.auths().len() as u32
    }
}

/// The generators live in the test state, so generating an address inside a
/// contract panics. Without the panic the generators would restart from zero on
/// every invocation, silently returning addresses that collide with addresses
/// the calling test generated, and with each other.
#[test]
#[should_panic(expected = "Error(WasmVm, InvalidAction)")]
fn test_generate_address_in_contract() {
    let env = Env::default();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.gen_address();
}

/// The auth snapshot lives in the test state, so calling `auths` inside a
/// contract panics. Without the panic the contract would observe an empty
/// snapshot, and assertions made on it inside a contract would pass vacuously.
#[test]
#[should_panic(expected = "Error(WasmVm, InvalidAction)")]
fn test_auths_in_contract() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let addr = Address::generate(&env);

    client.auth_count(&addr);
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
