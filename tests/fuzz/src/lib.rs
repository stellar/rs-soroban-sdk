#![no_std]
use soroban_sdk::{contractimpl, U256};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: U256, b: U256) -> bool {
        a < b
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let x = 10u64;
        let y = 12u64;
        let z = client.add(&x, &y);
        assert!(z == 22);
    }
}
