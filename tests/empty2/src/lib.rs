#![no_std]
use soroban_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let _client = ContractClient::new(&e, &contract_id);
    }
}
