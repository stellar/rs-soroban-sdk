#![no_std]
use soroban_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: u64, b: u64) -> u64 {
        a + b
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let x = 10u64;
        let y = 12u64;
        let z = client.add(&x, &y);
        assert!(z == 22);
    }
}
