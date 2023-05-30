#![no_std]
use soroban_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let _client = ContractClient::new(&e, &contract_id);
    }
}
