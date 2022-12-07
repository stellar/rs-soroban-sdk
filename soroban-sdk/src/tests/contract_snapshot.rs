use crate::{self as soroban_sdk, BytesN};
use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store(env: Env, k: i32, v: i32) {
        env.storage().set(k, v)
    }
    pub fn get(env: Env, k: i32) -> i32 {
        env.storage().get(k).unwrap().unwrap()
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_id = [0u8; 32];
    e.register_contract(&BytesN::from_array(&e, &contract_id), Contract);
    let client = ContractClient::new(&e, &contract_id);

    client.store(&2, &4);
    assert_eq!(client.get(&2), 4);

    let snapshot = e.to_snapshot();

    let e = Env::from_snapshot(snapshot);
    e.register_contract(&BytesN::from_array(&e, &contract_id), Contract);
    let client = ContractClient::new(&e, &contract_id);

    assert_eq!(client.get(&2), 4);
}
