#![no_std]
use soroban_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: u128, b: u128) -> u128 {
        a + b
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

        let x = 2u128.pow(70);
        let y = 4u128.pow(20);
        let z = client.add(&x, &y);
        assert!(z == x + y);
    }
}
