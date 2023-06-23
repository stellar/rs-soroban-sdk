use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct State {
    pub a: i32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn state() -> State {
        State { a: 1 }
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let client = ContractClient::new(&env, &contract_id);
    let s = client.state();
    assert_eq!(s, State { a: 1 });
}
