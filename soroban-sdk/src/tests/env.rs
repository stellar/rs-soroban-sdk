use crate::{
    self as soroban_sdk, contract, contractimpl,
    env::EnvTestConfig,
    testutils::{Address as _, Logs as _},
    xdr::{ScErrorCode, ScErrorType},
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

    let c1addr = env1.register(Contract, ());
    let c2addr = env2.register(Contract, ());

    let c1client = ContractClient::new(&env1, &c1addr);
    let c2client = ContractClient::new(&env2, &c2addr);

    c1client.test();
    c2client.test();

    let c1addr2 = Address::generate(&env1);
    let c2addr2 = Address::generate(&env2);
    let r1 = c1client.try_need_auth(&c1addr2);
    let r2 = c2client.try_need_auth(&c2addr2);
    assert_eq!(
        r1,
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );
    assert_eq!(
        r2,
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );

    let logs1 = env1.logs().all();
    let logs2 = env2.logs().all();
    assert_eq!(logs1, &["[Diagnostic Event] contract:CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM, topics:[log], data:\"test\""]);
    assert_eq!(logs2, &["[Diagnostic Event] contract:CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM, topics:[log], data:\"test\""]);
}

#[test]
fn register_contract_deploys_predictable_contract_ids() {
    let env1 = Env::default();
    let env2 = Env::from_snapshot(env1.to_snapshot());

    let env1addr1 = env1.register(Contract, ());
    println!("env1 addr1 {:?}", env1addr1.contract_id());
    let env1addr2 = env1.register(Contract, ());
    println!("env1 addr2 {:?}", env1addr2.contract_id());
    let env2addr1 = env2.register(Contract, ());
    println!("env2 addr1 {:?}", env2addr1.contract_id());
    let env2addr2 = env2.register(Contract, ());
    println!("env2 addr2 {:?}", env2addr2.contract_id());

    let env3 = Env::from_snapshot(env1.to_snapshot());
    let env1addr3 = env1.register(Contract, ());
    println!("env1 addr3 {:?}", env1addr3.contract_id());
    let env2addr3 = env2.register(Contract, ());
    println!("env2 addr3 {:?}", env2addr3.contract_id());
    let env3addr3 = env3.register(Contract, ());
    println!("env3 addr3 {:?}", env3addr3.contract_id());

    // Check that contracts deployed in the envs are consistent and predictable.
    assert_eq!(env2addr1.contract_id(), env1addr1.contract_id());
    assert_eq!(env2addr2.contract_id(), env1addr2.contract_id());
    assert_eq!(env2addr3.contract_id(), env1addr3.contract_id());
    assert_eq!(env3addr3.contract_id(), env1addr3.contract_id());
}

/// Test that the test snapshot file is written.
#[test]
fn test_snapshot_file() {
    let p = std::path::Path::new("test_snapshots")
        .join("tests")
        .join("env")
        .join("test_snapshot_file");
    let p1 = p.with_extension("1.json");
    let p2 = p.with_extension("2.json");
    let _ = std::fs::remove_file(&p1);
    let _ = std::fs::remove_file(&p2);
    {
        let e1 = Env::default();
        assert!(!p1.exists());
        assert!(!p2.exists());
        let e2 = e1.clone();
        assert!(!p1.exists());
        assert!(!p2.exists());
        {
            let _ = Env::default(); // When dropped won't be written because empty.
        } // Env dropped, nothing written.
        assert!(!p1.exists());
        assert!(!p2.exists());
        {
            let e3 = Env::default(); // When dropped will be written to p1.
            let _ = e3.register(Contract, ());
        } // Env dropped, written to p1.
        let c = e1.register(Contract, ());
        assert!(p1.exists());
        assert!(!p2.exists());
        e1.as_contract(&c, || {});
        assert!(p1.exists());
        assert!(!p2.exists());
        e2.as_contract(&c, || {});
        assert!(p1.exists());
        assert!(!p2.exists());
    } // Env dropped, written to p2.
    assert!(p1.exists());
    assert!(p2.exists());
    let _ = std::fs::remove_file(&p1);
    let _ = std::fs::remove_file(&p2);
}

/// Test that the test snapshot file is not written when disabled.
#[test]
fn test_snapshot_file_disabled() {
    let p = std::path::Path::new("test_snapshots")
        .join("tests")
        .join("env")
        .join("test_snapshot_file_disabled");
    let p1 = p.with_extension("1.json");
    let p2 = p.with_extension("2.json");
    let _ = std::fs::remove_file(&p1);
    let _ = std::fs::remove_file(&p2);
    {
        let e1 = Env::default();
        let _ = e1.register(Contract, ());
        let e2 = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let _ = e2.register(Contract, ());
        assert!(!p1.exists());
        assert!(!p2.exists());
    }
    assert!(p1.exists());
    assert!(!p2.exists());
    let _ = std::fs::remove_file(&p1);
}

/// Test that the test snapshot file is not written when disabled after
/// creation.
#[test]
fn test_snapshot_file_disabled_after_creation() {
    let p = std::path::Path::new("test_snapshots")
        .join("tests")
        .join("env")
        .join("test_snapshot_file_disabled_after_creation");
    let p1 = p.with_extension("1.json");
    let p2 = p.with_extension("2.json");
    let _ = std::fs::remove_file(&p1);
    let _ = std::fs::remove_file(&p2);
    {
        let e1 = Env::default();
        let _ = e1.register(Contract, ());
        let mut e2 = Env::default();
        e2.set_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let _ = e2.register(Contract, ());
        assert!(!p1.exists());
        assert!(!p2.exists());
    }
    assert!(p1.exists());
    assert!(!p2.exists());
    let _ = std::fs::remove_file(&p1);
}
