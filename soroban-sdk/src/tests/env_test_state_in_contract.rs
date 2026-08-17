//! Demonstrates that the [Env] passed into a natively executing contract
//! function carries a zeroed test state.
//!
//! The host only knows `EnvImpl`, so when it dispatches into a native contract
//! it hands the function set an `&EnvImpl`. There is no channel by which the
//! calling test's [Env] could travel through the host and come back, so the SDK
//! builds one on the spot, cloning `env_impl` but defaulting `test_state`. The
//! `env_impl` half being shared means everything a contract can observe on
//! chain is real. The `test_state` half being zeroed means any testutils API
//! reached from inside the contract silently reads from empty scaffolding.
//!
//! Both tests below are `#[ignore]`d because they assert the behaviour that is
//! wanted, not the behaviour that exists, and so they fail today. Run them with
//! `cargo test -- --ignored`. When the underlying problem is addressed they
//! should pass and the `#[ignore]` attributes should be removed.

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

/// The generators live in the test state, so they restart from zero on every
/// invocation. Addresses generated inside a contract collide with addresses the
/// calling test generated, and with each other.
#[test]
#[ignore = "demonstrates the zeroed test state problem, fails today"]
fn test_generated_addresses_do_not_collide() {
    let env = Env::default();

    let outer = Address::generate(&env);
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let first = client.gen_address();
    let second = client.gen_address();

    assert_ne!(outer, first);
    assert_ne!(first, second);
}

/// The auth snapshot lives in the test state, so a contract always observes an
/// empty one and assertions made on it inside a contract pass vacuously.
#[test]
#[ignore = "demonstrates the zeroed test state problem, fails today"]
fn test_auths_agree_inside_and_outside_contract() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let addr = Address::generate(&env);

    let seen_inside = client.auth_count(&addr) as usize;
    let seen_outside = env.auths().len();

    assert_eq!(seen_inside, seen_outside);
}
