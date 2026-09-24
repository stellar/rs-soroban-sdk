//! Demonstrates that register and register_at switch to recording auth for
//! native contract constructors.
//!
//! A native contract whose constructor calls `require_auth` must succeed when
//! registered with `register` or `register_at`, the same as the Wasm paths.
//! Without switching to recording auth the constructor runs under the default
//! enforcing auth and `require_auth` fails.

use crate as soroban_sdk;

use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&"admin", &admin);
    }
}

#[contract]
pub struct Probe;

#[contractimpl]
impl Probe {
    pub fn need_auth(_env: Env, address: Address) {
        address.require_auth();
    }
}

// Auth is intentionally not mocked in these tests.

#[test]
fn register() {
    let e = Env::default();

    let admin = Address::generate(&e);
    e.register(Contract, (&admin,));
}

#[test]
fn register_at() {
    let e = Env::default();

    let admin = Address::generate(&e);
    let contract_id = Address::generate(&e);
    e.register_at(&contract_id, Contract, (&admin,));
}

// If registration fails while the constructor is running, the previous auth
// manager must be restored rather than left in recording mode. Otherwise later
// require_auth calls would be silently auto-authorized.
#[test]
fn register_restores_auth_before_panics() {
    let e = Env::default();

    // A native contract with a function that requires auth, used to probe
    // whether the environment is enforcing authorization.
    let probe = e.register(Probe, ());
    let probe_client = ProbeClient::new(&e, &probe);
    let user = Address::generate(&e);

    // Before the failed registration an unmocked require_auth is enforced.
    let pre = probe_client.try_need_auth(&user);
    assert!(pre.is_err());

    // Registering the contract with missing constructor arguments fails while
    // constructing. Catch the failure inside a contract frame so it does not
    // abort the test.
    let another = e.register(Probe, ());
    let register_result = e.try_as_contract::<_, soroban_sdk::Error>(&another, || {
        e.register(Contract, ());
    });
    assert!(register_result.is_err());

    // The previous auth manager must have been restored: require_auth is still
    // enforced after the failed registration.
    let post = probe_client.try_need_auth(&user);
    assert!(post.is_err());
    assert_eq!(pre, post);
}
