#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn empty() {}
}

#[contractimpl]
impl Contract {
    pub fn empty2() {}
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        client.empty();
        client.empty2();
    }
}
