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
    testutils::{
        self, Address as _, BytesN as _, Deployer as _, EnvTestConfig, Events as _, Ledger as _,
    },
    Address, BytesN, Env,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Uses the generators.
    pub fn gen_address(env: Env) -> Address {
        Address::generate(&env)
    }

    // Registering contracts inside a contract.
    pub fn register(env: Env) -> Address {
        env.register(Contract, ())
    }

    pub fn do_register_at(env: Env, addr: Address) -> Address {
        env.register_at(&addr, Contract, ())
    }

    pub fn do_upload_at(env: Env) -> BytesN<32> {
        env.upload_at([1u8; 32], Contract)
    }

    // Cost estimation and budget.
    pub fn do_cost_estimate(env: Env) {
        let _ = env.cost_estimate();
    }

    #[allow(deprecated)]
    pub fn do_budget(env: Env) {
        let _ = env.budget();
    }

    // Uses the auth snapshot.
    pub fn auth_count(env: Env, addr: Address) -> u32 {
        addr.require_auth();
        env.auths().len() as u32
    }

    // Auth mocking.
    pub fn do_mock_all_auths(env: Env) {
        env.mock_all_auths();
    }

    pub fn do_mock_non_root(env: Env) {
        env.mock_all_auths_allowing_non_root_auth();
    }

    pub fn do_set_auths(env: Env) {
        env.set_auths(&[]);
    }

    // Running as contract.
    pub fn do_as_contract(env: Env, addr: Address) {
        env.as_contract(&addr, || ());
    }

    pub fn do_try_as_contract(env: Env, addr: Address) {
        let _: Result<(), Result<soroban_sdk::Error, soroban_sdk::InvokeError>> =
            env.try_as_contract(&addr, || ());
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

    // Ledger testutils.
    pub fn do_ledger_set(env: Env) {
        env.ledger().set(testutils::default_ledger_info());
    }

    pub fn do_ledger_set_ts(env: Env) {
        env.ledger().set_timestamp(100);
    }

    pub fn do_ledger_get(env: Env) {
        let _ = env.ledger().get();
    }

    // Events and logs.
    pub fn do_events_all(env: Env) {
        let _ = env.events().all();
    }

    // BytesN random.
    pub fn do_bytes_random(env: Env) -> BytesN<32> {
        BytesN::random(&env)
    }

    // Deployer testutils.
    pub fn do_deployer_get_ttl(env: Env, addr: Address) -> u32 {
        env.deployer().get_contract_instance_ttl(&addr)
    }

    // Storage testutils.
    pub fn do_storage_persist(env: Env) {
        use testutils::storage::Persistent;
        let _ = env.storage().persistent().all();
    }

    pub fn do_storage_instance(env: Env) {
        use testutils::storage::Instance;
        let _ = env.storage().instance().all();
    }

    pub fn do_storage_temp(env: Env) {
        use testutils::storage::Temporary;
        let _ = env.storage().temporary().all();
    }
}

/// The generators live in the test state, so generating an address inside a
/// contract panics. Without the panic the generators would restart from zero on
/// every invocation, silently returning addresses that collide with addresses
/// the calling test generated, and with each other.
#[test]
#[should_panic(
    expected = "generating values like addresses is unavailable inside a contract function"
)]
fn test_generate_address_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.gen_address();
}

/// Registering a contract inside a contract panics.
#[test]
#[should_panic(expected = "registering contracts is unavailable inside a contract function")]
fn test_register_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.register();
}

/// Registering a contract with a pre-specified address inside a contract panics.
#[test]
#[should_panic(expected = "registering contracts is unavailable inside a contract function")]
fn test_register_at_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    let addr = Address::generate(&env);
    client.do_register_at(&addr);
}

/// Uploading contract code inside a contract panics.
#[test]
#[should_panic(expected = "uploading contract code is unavailable inside a contract function")]
fn test_upload_at_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_upload_at();
}

/// Cost estimation inside a contract panics.
#[test]
#[should_panic(expected = "cost estimation is unavailable inside a contract function")]
fn test_cost_estimate_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_cost_estimate();
}

/// Accessing the budget inside a contract panics.
#[test]
#[should_panic(expected = "the budget is unavailable inside a contract function")]
fn test_budget_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_budget();
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

/// Mocking authorizations inside a contract panics.
#[test]
#[should_panic(expected = "mocking authorizations is unavailable inside a contract function")]
fn test_mock_all_auths_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_mock_all_auths();
}

/// Mocking authorizations allowing non-root auth inside a contract panics.
#[test]
#[should_panic(expected = "mocking authorizations is unavailable inside a contract function")]
fn test_mock_all_auths_allowing_non_root_auth_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_mock_non_root();
}

/// Setting authorizations inside a contract panics.
#[test]
#[should_panic(expected = "setting authorizations is unavailable inside a contract function")]
fn test_set_auths_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_set_auths();
}

/// Running as another contract inside a contract panics.
#[test]
#[should_panic(
    expected = "running code as another contract is unavailable inside a contract function"
)]
fn test_as_contract_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    let addr = Address::generate(&env);
    client.do_as_contract(&addr);
}

/// Attempting to run as another contract inside a contract panics.
#[test]
#[should_panic(
    expected = "running code as another contract is unavailable inside a contract function"
)]
fn test_try_as_contract_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    let addr = Address::generate(&env);
    client.do_try_as_contract(&addr);
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
#[should_panic(
    expected = "generating values like addresses is unavailable inside a contract function"
)]
fn test_to_snapshot_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.to_snapshot();
}

/// Modifying ledger info inside a contract panics.
#[test]
#[should_panic(expected = "modifying ledger info is unavailable inside a contract function")]
fn test_ledger_set_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_ledger_set();
}

/// Modifying ledger fields inside a contract panics.
#[test]
#[should_panic(expected = "modifying ledger info is unavailable inside a contract function")]
fn test_ledger_set_timestamp_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_ledger_set_ts();
}

/// Getting ledger info struct inside a contract panics.
#[test]
#[should_panic(expected = "getting ledger info is unavailable inside a contract function")]
fn test_ledger_get_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_ledger_get();
}

/// Accessing recorded events inside a contract panics.
#[test]
#[should_panic(expected = "the record of events is unavailable inside a contract function")]
fn test_events_all_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_events_all();
}

/// Generating random bytes inside a contract panics.
#[test]
#[should_panic(expected = "generating random bytes is unavailable inside a contract function")]
fn test_bytes_random_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_bytes_random();
}

/// Getting contract ttl via deployer inside a contract panics.
#[test]
#[should_panic(expected = "getting contract ttl is unavailable inside a contract function")]
fn test_deployer_get_instance_ttl_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    let addr = Address::generate(&env);
    client.do_deployer_get_ttl(&addr);
}

/// Inspecting persistent storage inside a contract panics.
#[test]
#[should_panic(expected = "inspecting contract storage is unavailable inside a contract function")]
fn test_storage_persistent_all_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_storage_persist();
}

/// Inspecting instance storage inside a contract panics.
#[test]
#[should_panic(expected = "inspecting contract storage is unavailable inside a contract function")]
fn test_storage_instance_all_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_storage_instance();
}

/// Inspecting temporary storage inside a contract panics.
#[test]
#[should_panic(expected = "inspecting contract storage is unavailable inside a contract function")]
fn test_storage_temporary_all_in_contract() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.do_storage_temp();
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
