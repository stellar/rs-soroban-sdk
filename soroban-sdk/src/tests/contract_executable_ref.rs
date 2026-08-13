use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr::{
        ContractExecutable, ContractExecutableExternalRef, ContractIdPreimage,
        ContractIdPreimageFromAddress, CreateContractArgsV2, ScString, ScVal, Uint256,
    },
    Address, BytesN, Env, Executable, ExecutableTag, IntoVal, String, TryFromVal, Val,
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
        let tag = ExecutableTag::new(&env, &name);
        env.storage().persistent().set(&tag, &wasm_hash);
    }
}

/// Deploys contracts with executable references, and can switch its own
/// executable to a reference.
#[contract]
pub struct DeployerContract;

#[contractimpl]
impl DeployerContract {
    pub fn deploy(env: Env, owner: Address, name: String, salt: BytesN<32>) -> Address {
        let tag = ExecutableTag::new(&env, &name);
        env.deployer()
            .with_current_contract(salt)
            .deploy_executable_ref(&owner, &tag, ())
    }

    pub fn upgrade_to_ref(env: Env, owner: Address, name: String) {
        let tag = ExecutableTag::new(&env, &name);
        env.deployer()
            .update_current_contract_executable_ref(&owner, &tag);
    }

    pub fn deploy_with_args(
        env: Env,
        owner: Address,
        name: String,
        salt: BytesN<32>,
        a: u32,
        b: i64,
    ) -> Address {
        let tag = ExecutableTag::new(&env, &name);
        env.deployer()
            .with_current_contract(salt)
            .deploy_executable_ref(&owner, &tag, (a, b))
    }

    pub fn deploy_for(
        env: Env,
        deployer: Address,
        owner: Address,
        name: String,
        salt: BytesN<32>,
    ) -> Address {
        let tag = ExecutableTag::new(&env, &name);
        env.deployer()
            .with_address(deployer, salt)
            .deploy_executable_ref(&owner, &tag, ())
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
fn test_executable_tag_conversions() {
    let env = Env::default();

    let tag = ExecutableTag::from_str(&env, "v1");
    assert_eq!(tag, ExecutableTag::new(&env, &String::from_str(&env, "v1")));
    assert_ne!(tag, ExecutableTag::from_str(&env, "v2"));
    assert!(tag < ExecutableTag::from_str(&env, "v2"));

    assert_eq!(format!("{:?}", tag), "ExecutableTag(v1)");

    let val: Val = tag.clone().into_val(&env);
    let rt: ExecutableTag = val.into_val(&env);
    assert_eq!(tag, rt);

    let sc_val = ScVal::from(&tag);
    assert_eq!(
        sc_val,
        ScVal::ExecutableTag(ScString("v1".try_into().unwrap()))
    );
    let rt = ExecutableTag::try_from_val(&env, &sc_val).unwrap();
    assert_eq!(tag, rt);
}

#[test]
fn test_executable_ref_entry_storage() {
    let env = Env::default();
    let (owner, wasm_hash) = setup_owner_with_entry(&env, add_u64_contract::WASM);

    env.as_contract(&owner, || {
        let tag = ExecutableTag::from_str(&env, "fleet");
        assert_eq!(env.storage().persistent().has(&tag), true);
        assert_eq!(
            env.storage().persistent().get::<_, BytesN<32>>(&tag),
            Some(wasm_hash.clone())
        );
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
                function: AuthorizedFunction::CreateContractV2HostFn(CreateContractArgsV2 {
                    contract_id_preimage: ContractIdPreimage::Address(
                        ContractIdPreimageFromAddress {
                            address: (&deployer_address).try_into().unwrap(),
                            salt: Uint256([7; 32]),
                        }
                    ),
                    executable: ContractExecutable::ExternalRef(ContractExecutableExternalRef {
                        executable_owner: (&owner).try_into().unwrap(),
                        tag: ScString("fleet".try_into().unwrap()),
                    }),
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

// these tests exist because we document the rules the host enforces on `ExecutableTag` entries.
// if one fails, the docs likely need to be updated to reflect the new behavior.

#[test]
#[should_panic(expected = "executable reference entries may only use persistent storage")]
fn test_executable_ref_entry_rejects_temporary_storage() {
    let env = Env::default();
    let wasm_hash = env.deployer().upload_contract_wasm(add_u64_contract::WASM);
    let owner = env.register(OwnerContract, ());
    env.as_contract(&owner, || {
        let tag = ExecutableTag::from_str(&env, "fleet");
        env.storage().temporary().set(&tag, &wasm_hash);
    });
}

#[test]
#[should_panic(expected = "executable reference value must be a 32-byte Wasm hash")]
fn test_executable_ref_entry_rejects_non_hash_value() {
    let env = Env::default();
    let owner = env.register(OwnerContract, ());
    env.as_contract(&owner, || {
        let tag = ExecutableTag::from_str(&env, "fleet");
        env.storage().persistent().set(&tag, &1u32);
    });
}

#[test]
#[should_panic(expected = "Wasm does not exist")]
fn test_executable_ref_entry_rejects_missing_wasm() {
    let env = Env::default();
    let owner = env.register(OwnerContract, ());
    env.as_contract(&owner, || {
        let tag = ExecutableTag::from_str(&env, "fleet");
        env.storage()
            .persistent()
            .set(&tag, &BytesN::from_array(&env, &[1; 32]));
    });
}

#[test]
#[should_panic(expected = "executable reference entries cannot be deleted")]
fn test_executable_ref_entry_rejects_removal() {
    let env = Env::default();
    let (owner, _) = setup_owner_with_entry(&env, add_u64_contract::WASM);
    env.as_contract(&owner, || {
        let tag = ExecutableTag::from_str(&env, "fleet");
        env.storage().persistent().remove(&tag);
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
