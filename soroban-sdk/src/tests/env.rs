use crate::{
    self as soroban_sdk, contract, contractimpl,
    testutils::{Address as _, Logs as _},
    Address, Env, Error,
};

#[test]
// Env::default is expected to configure the underlying host with a source
// account in tests so that the Env is configured similarly to how it will be
// configured for real. Some functions in Env have in the past or may now make
// assumptions about a source account being set. This is something small we do
// to make sure we don't accidentally introduce Env functionality that will
// panick in SDK tests.
fn default_has_source_account_configured_in_host() {
    let env = Env::default();
    assert!(env.host().source_account_address().unwrap().is_some());
}

#[contract]
struct Contract;

#[contractimpl]
impl Contract {
    pub fn test(env: Env) {
        // This will panic if the prng is not seeded.
        env.prng().gen::<u64>();
        // Create a diagnostic.
        env.logs().add("test", &[]);
    }

    pub fn need_auth(env: Env, address: Address) {
        // This should fail because auths aren't mocked.
        env.require_auth(&address);
    }
}

#[test]
fn default_and_from_snapshot_same_settings() {
    let env1 = Env::default();
    let env2 = Env::from_snapshot(env1.to_snapshot());

    assert!(env1.host().source_account_address().unwrap().is_some());
    assert!(env2.host().source_account_address().unwrap().is_some());

    let c1addr = env1.register_contract(None, Contract);
    let c2addr = env2.register_contract(None, Contract);

    let c1client = ContractClient::new(&env1, &c1addr);
    let c2client = ContractClient::new(&env2, &c2addr);

    c1client.test();
    c2client.test();

    let c1addr2 = Address::random(&env1);
    let c2addr2 = Address::random(&env2);
    let r1 = c1client.try_need_auth(&c1addr2);
    let r2 = c2client.try_need_auth(&c2addr2);
    assert_eq!(
        r1,
        Err(Ok(Error::from_type_and_code(
            stellar_xdr::curr::ScErrorType::Context,
            stellar_xdr::curr::ScErrorCode::InvalidAction
        )))
    );
    assert_eq!(
        r2,
        Err(Ok(Error::from_type_and_code(
            stellar_xdr::curr::ScErrorType::Context,
            stellar_xdr::curr::ScErrorCode::InvalidAction
        )))
    );

    let logs1 = env1.logs().all();
    let logs2 = env2.logs().all();
    assert!(!logs1.is_empty());
    assert!(!logs2.is_empty());
}
