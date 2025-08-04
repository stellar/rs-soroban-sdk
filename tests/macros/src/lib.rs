#![no_std]
use soroban_sdk::{contract, contractimpl};
use must_be_empty::must_be_empty;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // The must_be_empty attribute macro is used to test functions that have third party attributes
    // in use on functions inside contractimpl blocks, to ensure the contractimpl macro and other
    // SDKs macros interact well with third party macros.
    #[must_be_empty]
    pub fn empty() {}
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.empty();
    }
}
