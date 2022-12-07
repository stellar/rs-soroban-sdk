use crate as soroban_sdk;
use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store(env: Env, k: i32, v: i32) {
        env.storage().set(k, v)
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    client.store(&2, &4);

    assert_eq!(
        e.as_contract(&contract_id, || e.storage().get::<_, i32>(2)),
        Some(Ok(4))
    );
}
