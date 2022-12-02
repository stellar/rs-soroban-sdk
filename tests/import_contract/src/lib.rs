#![no_std]
use soroban_sdk::{contractimpl, BytesN, Env};

mod addcontract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/test_add_u64.wasm"
    );
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, contract_id: BytesN<32>, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{addcontract, Contract, ContractClient};

    #[test]
    fn test_add() {
        let e = Env::default();
        let add_contract_id = e.register_contract_wasm(None, addcontract::WASM);

        let contract_id = BytesN::from_array(&e, &[1; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let x = 10u64;
        let y = 12u64;
        let z = client.add_with(&add_contract_id, &x, &y);
        assert!(z == 22);
    }
}
