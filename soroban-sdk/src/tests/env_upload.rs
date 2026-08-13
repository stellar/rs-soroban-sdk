use crate::{
    self as soroban_sdk, contract, contractimpl, testutils::Address as _, Address, BytesN, Env,
};

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

fn deploy(env: &Env, wasm_hash: &BytesN<32>) -> Address {
    env.mock_all_auths();
    let deployer = Address::generate(env);
    let salt = BytesN::from_array(env, &[0u8; 32]);
    env.deployer()
        .with_address(deployer, salt)
        .deploy_v2(wasm_hash.clone(), ())
}

#[test]
fn upload_generates_a_new_wasm_hash_for_each_call() {
    let env = Env::default();

    let wasm_hash1 = env.upload(Contract);
    let wasm_hash2 = env.upload(Contract);

    assert_ne!(wasm_hash1, wasm_hash2);
}

#[test]
fn upload_generates_predictable_wasm_hashes() {
    let env1 = Env::default();
    let env2 = Env::from_snapshot(env1.to_snapshot());

    let env1hash1 = env1.upload(Contract);
    let env1hash2 = env1.upload(Contract);
    let env2hash1 = env2.upload(Contract);
    let env2hash2 = env2.upload(Contract);

    assert_eq!(env2hash1, env1hash1);
    assert_eq!(env2hash2, env1hash2);
}

#[test]
fn upload_is_callable_via_a_deployed_instance() {
    let env = Env::default();

    let wasm_hash = env.upload(Contract);
    let contract_id = deploy(&env, &wasm_hash);

    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.hello(), 1);
}

#[test]
fn upload_at_uploads_to_the_wasm_hash_given() {
    let env = Env::default();

    let wasm_hash = BytesN::from_array(&env, &[9u8; 32]);
    assert_eq!(env.upload_at(&wasm_hash, Contract), wasm_hash);

    let contract_id = deploy(&env, &wasm_hash);
    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.hello(), 1);
}

#[test]
fn upload_at_replaces_the_contract_uploaded_to_the_wasm_hash() {
    let env = Env::default();

    let wasm_hash = env.upload(Contract);
    let contract_id = deploy(&env, &wasm_hash);
    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.hello(), 1);

    // Re-uploading changes what already deployed instances dispatch to,
    // without touching the instances themselves.
    env.upload_at(&wasm_hash, OtherContract);
    assert_eq!(client.hello(), 2);
}
