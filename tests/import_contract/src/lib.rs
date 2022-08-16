#![no_std]
use soroban_sdk::{contractimpl, BytesN, Env};

mod addcontract {
    use soroban_sdk::contractimport;
    contractimport!(wasm = "target/wasm32-unknown-unknown/release/example_add_i32.wasm");
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: i32, y: i32, contract_id: BytesN<32>) -> i32 {
        addcontract::Client::add(&env, &contract_id, x, y)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{add_with, addcontract, Contract};

    #[test]
    fn test_add() {
        let e = Env::default();

        let add_contract_id = BytesN::from_array(&e, [0; 32]);
        e.register_contract_wasm(&add_contract_id, addcontract::WASM);

        let contract_id = BytesN::from_array(&e, [1; 32]);
        e.register_contract(&contract_id, Contract);

        let x = 10i32;
        let y = 12i32;
        let z = add_with::invoke(&e, &contract_id, &x, &y, &add_contract_id);
        assert!(z == 22);
    }
}
