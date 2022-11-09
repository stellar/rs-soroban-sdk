use crate as soroban_sdk;
use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store(env: Env, k: i32, v: i32) {
        env.data().set(k, v)
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    client.store(&2, &4);

    e.as_contract(contract_id, || e.data().get::<_, i32>(2));
}
