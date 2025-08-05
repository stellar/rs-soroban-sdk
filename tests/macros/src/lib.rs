#![no_std]
use soroban_sdk::{contract, contractimpl};
use proc_macros::parse_item_fn;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Test that attribute macros that expect to be used on fns are composable with contractimpl.
    #[parse_item_fn]
    pub fn empty() {}
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_empty() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.empty();
    }
}
