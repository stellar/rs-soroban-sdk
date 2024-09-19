#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
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
        let contract_id = e.register(Contract, ());
        let _client = ContractClient::new(&e, &contract_id);
    }
}
