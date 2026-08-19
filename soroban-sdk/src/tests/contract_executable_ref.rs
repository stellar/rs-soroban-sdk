use crate as soroban_sdk;
use soroban_sdk::auth::{ContractExecutable, ContractExecutableRef};
use soroban_sdk::{
    contract, contractimpl,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr, Address, BytesN, Env, Executable, String,
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
            .update_current_contract(ContractExecutable::ExternalRef(ContractExecutableRef {
                owner: env.current_contract_address(),
                tag: name,
            }));
    }
}

/// Deploys contracts with executable references, and can switch its own
/// executable to a reference.
#[contract]
pub struct DeployerContract;

#[contractimpl]
impl DeployerContract {
    pub fn deploy(env: Env, owner: Address, name: String, salt: BytesN<32>) -> Address {
        env.deployer().with_current_contract(salt).deploy_v2(
            ContractExecutable::ExternalRef(ContractExecutableRef { owner, tag: name }),
            (),
        )
    }

    pub fn upgrade_to_ref(env: Env, owner: Address, name: String) {
        env.deployer()
            .update_current_contract(ContractExecutable::ExternalRef(ContractExecutableRef {
                owner,
                tag: name,
            }));
    }

    /// Deploy using whatever executable the caller provides, demonstrating that
    /// an executable can be passed around as a single type.
    pub fn deploy_any(env: Env, executable: ContractExecutable, salt: BytesN<32>) -> Address {
        env.deployer()
            .with_current_contract(salt)
            .deploy_v2(executable, ())
    }

    /// Upgrade to whatever executable the caller provides.
    pub fn upgrade_any(env: Env, executable: ContractExecutable) {
        env.deployer().update_current_contract(executable);
    }

    pub fn deploy_with_args(
        env: Env,
        owner: Address,
        name: String,
        salt: BytesN<32>,
        a: u32,
        b: i64,
    ) -> Address {
        env.deployer().with_current_contract(salt).deploy_v2(
            ContractExecutable::ExternalRef(ContractExecutableRef { owner, tag: name }),
            (a, b),
        )
    }

    pub fn deploy_for(
        env: Env,
        deployer: Address,
        owner: Address,
        name: String,
        salt: BytesN<32>,
    ) -> Address {
        env.deployer().with_address(deployer, salt).deploy_v2(
            ContractExecutable::ExternalRef(ContractExecutableRef { owner, tag: name }),
            (),
        )
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
fn test_deploy_executable_ref_auth() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();
    let (owner, _) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    let deployer_address = Address::generate(&env);
    let deployer_contract = env.register(DeployerContract, ());
    let deployed = DeployerContractClient::new(&env, &deployer_contract).deploy_for(
        &deployer_address,
        &owner,
        &String::from_str(&env, "fleet"),
        &BytesN::from_array(&env, &[7; 32]),
    );

    assert_eq!(
        env.auths(),
        std::vec![(
            deployer_address.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::CreateContractV2HostFn(xdr::CreateContractArgsV2 {
                    contract_id_preimage: xdr::ContractIdPreimage::Address(
                        xdr::ContractIdPreimageFromAddress {
                            address: (&deployer_address).try_into().unwrap(),
                            salt: xdr::Uint256([7; 32]),
                        }
                    ),
                    executable: xdr::ContractExecutable::ExternalRef(
                        xdr::ContractExecutableExternalRef {
                            executable_owner: (&owner).try_into().unwrap(),
                            tag: xdr::ScString("fleet".try_into().unwrap()),
                        }
                    ),
                    constructor_args: Default::default(),
                }),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert!(deployed.executable().is_some());
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

#[test]
fn test_deploy_any_with_wasm_executable() {
    let env = Env::default();
    let wasm_hash = env.deployer().upload_contract_wasm(add_u64_contract::WASM);

    let deployer = env.register(DeployerContract, ());
    let deployed = DeployerContractClient::new(&env, &deployer).deploy_any(
        &ContractExecutable::Wasm(wasm_hash.clone()),
        &BytesN::from_array(&env, &[3; 32]),
    );

    assert_eq!(deployed.executable(), Some(Executable::Wasm(wasm_hash)));
    assert_eq!(
        add_u64_contract::Client::new(&env, &deployed).add(&1, &2),
        3
    );
}

#[test]
fn test_update_current_contract_with_wasm_executable() {
    let env = Env::default();
    let wasm_hash = env.deployer().upload_contract_wasm(add_u64_contract::WASM);

    let contract = env.register(DeployerContract, ());
    DeployerContractClient::new(&env, &contract)
        .upgrade_any(&ContractExecutable::Wasm(wasm_hash.clone()));

    assert_eq!(contract.executable(), Some(Executable::Wasm(wasm_hash)));
    assert_eq!(
        add_u64_contract::Client::new(&env, &contract).add(&1, &2),
        3
    );
}

#[test]
fn test_update_current_contract_with_ref_executable() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    let contract = env.register(DeployerContract, ());
    DeployerContractClient::new(&env, &contract).upgrade_any(&ContractExecutable::ExternalRef(
        ContractExecutableRef {
            owner,
            tag: String::from_str(&env, "fleet"),
        },
    ));

    assert_eq!(contract.executable(), Some(Executable::Wasm(wasm_hash)));
    assert_eq!(
        add_u64_contract::Client::new(&env, &contract).add(&1, &2),
        3
    );
}

/// The deprecated update_current_contract_wasm still accepts a Wasm hash and
/// has the same effect as update_current_contract.
#[test]
fn test_deprecated_update_current_contract_wasm() {
    let env = Env::default();
    let wasm_hash = env.deployer().upload_contract_wasm(add_u64_contract::WASM);

    let contract = env.register(OwnerContract, ());
    env.as_contract(&contract, || {
        #[allow(deprecated)]
        env.deployer()
            .update_current_contract_wasm(wasm_hash.clone());
    });

    assert_eq!(
        contract.executable(),
        Some(Executable::Wasm(wasm_hash.clone()))
    );
    assert_eq!(
        add_u64_contract::Client::new(&env, &contract).add(&1, &2),
        3
    );
}

/// deploy_v2 and update_current_contract accept a Wasm hash, and anything that
/// converts into one, as well as a ContractExecutable.
#[test]
fn test_deploy_v2_and_update_current_contract_accept_wasm_hash() {
    let env = Env::default();
    let wasm_hash = env.deployer().upload_contract_wasm(add_u64_contract::WASM);

    let contract = env.register(OwnerContract, ());
    env.as_contract(&contract, || {
        let deployed = env
            .deployer()
            .with_current_contract(BytesN::from_array(&env, &[8; 32]))
            .deploy_v2(wasm_hash.clone(), ());
        assert_eq!(
            deployed.executable(),
            Some(Executable::Wasm(wasm_hash.clone()))
        );

        let deployed = env
            .deployer()
            .with_current_contract(BytesN::from_array(&env, &[9; 32]))
            .deploy_v2(wasm_hash.to_array(), ());
        assert_eq!(
            deployed.executable(),
            Some(Executable::Wasm(wasm_hash.clone()))
        );

        env.deployer().update_current_contract(wasm_hash.clone());
    });
    assert_eq!(
        contract.executable(),
        Some(Executable::Wasm(wasm_hash.clone()))
    );

    let contract = env.register(OwnerContract, ());
    env.as_contract(&contract, || {
        env.deployer().update_current_contract(wasm_hash.to_array());
    });
    assert_eq!(contract.executable(), Some(Executable::Wasm(wasm_hash)));
}
