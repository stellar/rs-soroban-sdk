use crate::{self as soroban_sdk, auth::ContractExecutable};
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl,
    crypto::Hash,
    symbol_short,
    xdr::{
        ContractExecutable as XdrContractExecutable, ContractExecutableExternalRef,
        ContractIdPreimage, ContractIdPreimageFromAddress, CreateContractArgsV2, ScString, ScVal,
        SorobanAddressCredentials, SorobanAuthorizationEntry, SorobanAuthorizedFunction,
        SorobanAuthorizedInvocation, SorobanCredentials, Uint256,
    },
    Address, BytesN, Env, Executable, String, Vec,
};

mod add_u64_contract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(file = "doctest_fixtures/contract.wasm");
}

mod constructor_contract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(file = "doctest_fixtures/contract_with_constructor.wasm");
}

/// Owns executable reference entries that other contracts' executables
/// reference.
#[contract]
pub struct OwnerContract;

#[contractimpl]
impl OwnerContract {
    pub fn set_executable(env: Env, name: String, wasm_hash: BytesN<32>) {
        env.executable_refs().set(&name, &wasm_hash);
    }

    /// Switch the contract's own executable to an executable reference entry
    /// the contract itself owns.
    pub fn upgrade_to_own_ref(env: Env, name: String) {
        env.deployer()
            .update_current_contract_executable_ref(&env.current_contract_address(), &name);
    }
}

/// Deploys contracts with executable references, and can switch its own
/// executable to a reference.
#[contract]
pub struct DeployerContract;

#[contractimpl]
impl DeployerContract {
    pub fn deploy(env: Env, owner: Address, name: String, salt: BytesN<32>) -> Address {
        env.deployer()
            .with_current_contract(salt)
            .deploy_executable_ref(&owner, &name, ())
    }

    pub fn upgrade_to_ref(env: Env, owner: Address, name: String) {
        env.deployer()
            .update_current_contract_executable_ref(&owner, &name);
    }

    pub fn deploy_with_args(
        env: Env,
        owner: Address,
        name: String,
        salt: BytesN<32>,
        a: u32,
        b: i64,
    ) -> Address {
        env.deployer()
            .with_current_contract(salt)
            .deploy_executable_ref(&owner, &name, (a, b))
    }

    pub fn deploy_for(
        env: Env,
        deployer: Address,
        owner: Address,
        name: String,
        salt: BytesN<32>,
    ) -> Address {
        env.deployer()
            .with_address(deployer, salt)
            .deploy_executable_ref(&owner, &name, ())
    }
}

/// A custom account that records the executable from any create-contract auth
/// context it is asked to authorize, so tests can verify that the context the
/// host passes to `__check_auth` decodes through the SDK auth types.
#[contract]
pub struct RecordingAccount;

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecordingAccountError {
    Fail = 1,
}

#[contractimpl]
impl CustomAccountInterface for RecordingAccount {
    type Signature = ();
    type Error = RecordingAccountError;

    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        _signatures: (),
        auth_contexts: Vec<Context>,
    ) -> Result<(), RecordingAccountError> {
        for ctx in auth_contexts.iter() {
            if let Context::CreateContractHostFn(create) = ctx {
                env.storage()
                    .instance()
                    .set(&symbol_short!("exec"), &create.executable);
            }
        }
        Ok(())
    }
}

fn setup_owner_with_entry(env: &Env, wasm: &[u8]) -> (Address, BytesN<32>) {
    let wasm_hash = env.deployer().upload_contract_wasm(wasm);
    let owner = env.register(OwnerContract, ());
    OwnerContractClient::new(env, &owner)
        .set_executable(&String::from_str(env, "fleet"), &wasm_hash);
    (owner, wasm_hash)
}

#[test]
fn test_executable_ref_entry_storage() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    env.as_contract(&owner, || {
        let name = String::from_str(&env, "fleet");
        assert_eq!(env.executable_refs().has(&name), true);
        assert_eq!(env.executable_refs().get(&name), Some(wasm_hash.clone()));

        let missing = String::from_str(&env, "missing");
        assert_eq!(env.executable_refs().has(&missing), false);
        assert_eq!(env.executable_refs().get(&missing), None);
    });
}

#[test]
fn test_executable_ref_entry_does_not_collide_with_string_key() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    env.as_contract(&owner, || {
        // A regular persistent entry keyed by a String with the same value as
        // the tag is a separate entry.
        let name = String::from_str(&env, "fleet");
        env.storage().persistent().set(&name, &1u32);
        assert_eq!(env.storage().persistent().get(&name), Some(1u32));
        assert_eq!(env.executable_refs().get(&name), Some(wasm_hash));
    });
}

#[test]
fn test_executable_ref_entry_ttl() {
    let env = Env::default();
    let (owner, _) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    env.as_contract(&owner, || {
        let name = String::from_str(&env, "fleet");
        let ttl = env.executable_refs().get_ttl(&name);
        env.executable_refs()
            .extend_ttl(&name, ttl + 100, ttl + 100);
        assert_eq!(env.executable_refs().get_ttl(&name), ttl + 100);

        // The extension to ttl + 1000 is capped by max_extension at 300
        // ledgers over the current TTL of ttl + 100.
        env.executable_refs()
            .extend_ttl_with_limits(&name, ttl + 1000, 0, 300);
        assert_eq!(env.executable_refs().get_ttl(&name), ttl + 400);

        // The extension of 50 ledgers is below min_extension, so this is a
        // no-op.
        env.executable_refs()
            .extend_ttl_with_limits(&name, ttl + 450, 100, 1000);
        assert_eq!(env.executable_refs().get_ttl(&name), ttl + 400);
    });
}

#[test]
fn test_deploy_executable_ref() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    let deployer = env.register(DeployerContract, ());
    let deployed = DeployerContractClient::new(&env, &deployer).deploy(
        &owner,
        &String::from_str(&env, "fleet"),
        &BytesN::from_array(&env, &[0; 32]),
    );

    assert_eq!(deployed.executable(), Some(Executable::Wasm(wasm_hash)));
    assert_eq!(
        add_u64_contract::Client::new(&env, &deployed).add(&1, &2),
        3
    );
}

#[test]
fn test_executable_ref_entry_update_repoints_deployed_contract() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    let deployer = env.register(DeployerContract, ());
    let deployed_1 = DeployerContractClient::new(&env, &deployer).deploy(
        &owner,
        &String::from_str(&env, "fleet"),
        &BytesN::from_array(&env, &[0; 32]),
    );
    let deployed_2 = DeployerContractClient::new(&env, &deployer).deploy(
        &owner,
        &String::from_str(&env, "fleet"),
        &BytesN::from_array(&env, &[1; 32]),
    );
    assert_eq!(
        deployed_1.executable(),
        Some(Executable::Wasm(wasm_hash.clone()))
    );
    assert_eq!(deployed_2.executable(), Some(Executable::Wasm(wasm_hash)));

    // Re-point the ref entry
    let new_wasm_hash = env
        .deployer()
        .upload_contract_wasm(constructor_contract::WASM);
    OwnerContractClient::new(&env, &owner)
        .set_executable(&String::from_str(&env, "fleet"), &new_wasm_hash);

    assert_eq!(
        deployed_1.executable(),
        Some(Executable::Wasm(new_wasm_hash.clone()))
    );
    assert_eq!(
        deployed_2.executable(),
        Some(Executable::Wasm(new_wasm_hash))
    );
}

#[test]
fn test_update_current_contract_executable_ref() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    let contract = env.register(DeployerContract, ());
    DeployerContractClient::new(&env, &contract)
        .upgrade_to_ref(&owner, &String::from_str(&env, "fleet"));

    assert_eq!(
        contract.executable(),
        Some(Executable::Wasm(wasm_hash.clone()))
    );
    assert_eq!(
        add_u64_contract::Client::new(&env, &contract).add(&1, &2),
        3
    );
}

#[test]
fn test_update_current_contract_executable_ref_self_owned() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    // The owner switches its own executable to the entry it owns itself.
    OwnerContractClient::new(&env, &owner).upgrade_to_own_ref(&String::from_str(&env, "fleet"));

    assert_eq!(owner.executable(), Some(Executable::Wasm(wasm_hash)));
    assert_eq!(add_u64_contract::Client::new(&env, &owner).add(&1, &2), 3);
}

#[test]
fn test_deploy_executable_ref_with_constructor_args() {
    let env = Env::default();
    let wasm_hash = env
        .deployer()
        .upload_contract_wasm(constructor_contract::WASM);
    let owner = env.register(OwnerContract, ());
    OwnerContractClient::new(&env, &owner)
        .set_executable(&String::from_str(&env, "ctor"), &wasm_hash);

    let deployer = env.register(DeployerContract, ());
    let deployed = DeployerContractClient::new(&env, &deployer).deploy_with_args(
        &owner,
        &String::from_str(&env, "ctor"),
        &BytesN::from_array(&env, &[0; 32]),
        &1u32,
        &2i64,
    );

    // The constructor ran with the forwarded args; wrong or missing args
    // would have failed the deploy.
    assert_eq!(deployed.executable(), Some(Executable::Wasm(wasm_hash)));
}

#[test]
fn test_check_auth_receives_external_ref_context() {
    let env = Env::default();
    let (owner, _) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    let account = env.register(RecordingAccount, ());
    let deployer_contract = env.register(DeployerContract, ());

    // Authorize the deployment with real address credentials, so that the
    // host authenticates the account by invoking its `__check_auth` with the
    // authorization context the host builds.
    env.set_auths(&[SorobanAuthorizationEntry {
        credentials: SorobanCredentials::Address(SorobanAddressCredentials {
            address: (&account).try_into().unwrap(),
            nonce: 123,
            signature_expiration_ledger: 100,
            signature: ScVal::Void,
        }),
        root_invocation: SorobanAuthorizedInvocation {
            function: SorobanAuthorizedFunction::CreateContractV2HostFn(CreateContractArgsV2 {
                contract_id_preimage: ContractIdPreimage::Address(ContractIdPreimageFromAddress {
                    address: (&account).try_into().unwrap(),
                    salt: Uint256([7; 32]),
                }),
                executable: XdrContractExecutable::ExternalRef(ContractExecutableExternalRef {
                    executable_owner: (&owner).try_into().unwrap(),
                    tag: ScString("fleet".try_into().unwrap()),
                }),
                constructor_args: Default::default(),
            }),
            sub_invocations: Default::default(),
        },
    }]);

    let deployed = DeployerContractClient::new(&env, &deployer_contract).deploy_for(
        &account,
        &owner,
        &String::from_str(&env, "fleet"),
        &BytesN::from_array(&env, &[7; 32]),
    );
    assert!(deployed.executable().is_some());

    let recorded: ContractExecutable = env.as_contract(&account, || {
        env.storage()
            .instance()
            .get(&symbol_short!("exec"))
            .unwrap()
    });
    match recorded {
        ContractExecutable::ExternalRef(exec_ref) => {
            assert_eq!(exec_ref.owner, owner);
            assert_eq!(exec_ref.tag, String::from_str(&env, "fleet"));
        }
        ContractExecutable::Wasm(_) => panic!("expected external ref executable"),
    }
}

#[test]
#[should_panic(expected = "Wasm does not exist")]
fn test_executable_ref_entry_rejects_missing_wasm() {
    let env = Env::default();
    let owner = env.register(OwnerContract, ());
    env.as_contract(&owner, || {
        env.executable_refs().set(
            &String::from_str(&env, "fleet"),
            &BytesN::from_array(&env, &[1; 32]),
        );
    });
}

#[test]
#[should_panic(expected = "executable reference entry does not exist")]
fn test_deploy_executable_ref_missing_entry() {
    let env = Env::default();
    let owner = env.register(OwnerContract, ());
    let deployer = env.register(DeployerContract, ());
    DeployerContractClient::new(&env, &deployer).deploy(
        &owner,
        &String::from_str(&env, "missing"),
        &BytesN::from_array(&env, &[0; 32]),
    );
}
