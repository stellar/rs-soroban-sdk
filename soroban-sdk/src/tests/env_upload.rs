use crate::{self as soroban_sdk, contract, contractimpl, BytesN, ContractExecutable, Env};

#[contract]
struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello() -> u32 {
        1
    }
}

#[contract]
struct OtherContract;

#[contractimpl]
impl OtherContract {
    pub fn hello() -> u32 {
        2
    }
}

#[test]
fn upload_generates_a_new_wasm_hash_for_each_call() {
    let env = Env::default();

    let wasm_hash1 = env.upload(Contract);
    let wasm_hash2 = env.upload(Contract);

    assert_ne!(wasm_hash1, wasm_hash2);
}

#[test]
fn uploaded_contract_is_deployable() {
    let env = Env::default();

    let wasm_hash = env.upload(Contract);

    let deployer = env.register(Contract, ());
    let contract_id = env.as_contract(&deployer, || {
        env.deployer()
            .with_address(deployer.clone(), BytesN::from_array(&env, &[0u8; 32]))
            .deploy_contract(ContractExecutable::Wasm(wasm_hash), ())
    });

    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.hello(), 1);
}

#[test]
fn upload_at_uploads_to_the_wasm_hash_given() {
    let env = Env::default();

    let wasm_hash = env.upload_at([9u8; 32], Contract);
    assert_eq!(wasm_hash, BytesN::from_array(&env, &[9u8; 32]));

    let deployer = env.register(Contract, ());
    let contract_id = env.as_contract(&deployer, || {
        env.deployer()
            .with_address(deployer.clone(), BytesN::from_array(&env, &[0u8; 32]))
            .deploy_contract(ContractExecutable::Wasm(wasm_hash), ())
    });

    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.hello(), 1);
}

#[test]
fn upload_at_replaces_the_contract_uploaded_to_the_wasm_hash() {
    let env = Env::default();

    let wasm_hash = env.upload(Contract);

    let deployer = env.register(Contract, ());
    let contract_id = env.as_contract(&deployer, || {
        env.deployer()
            .with_address(deployer.clone(), BytesN::from_array(&env, &[0u8; 32]))
            .deploy_contract(ContractExecutable::Wasm(wasm_hash.clone()), ())
    });

    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.hello(), 1);

    // Re-uploading changes what already deployed instances dispatch to,
    // without touching the instances themselves.
    env.upload_at(wasm_hash, OtherContract);
    assert_eq!(client.hello(), 2);
}
