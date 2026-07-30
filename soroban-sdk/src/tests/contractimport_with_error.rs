use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

mod errcontract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/test_errors.wasm");
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello_with(env: Env, contract_id: Address, flag: errcontract::Flag) -> Symbol {
        errcontract::Client::new(&env, &contract_id).hello(&flag)
    }
}

#[test]
fn test_functional() {
    let e = Env::default();

    let err_contract_id = e.register(errcontract::WASM, ());

    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    let z = client.hello_with(&err_contract_id, &errcontract::Flag::A);
    assert!(z == symbol_short!("hello"));
}
