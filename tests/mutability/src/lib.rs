// A test vector for testing the interactions of the soroban-sdk macros with generics. The contract
// macros support generics sparingly and almost not at all. So there are very few places that
// generics are permitted. The places that they are permitted are almost meaningless to test
// because they have no utilitiy, but in the interest of capturing all of the places generics are
// supported this test vector exists.

#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn calc(a: u32, mut b: u32) -> u32 {
        b *= 2;
        a + b
    }
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

        let res = client.calc(&1, &2);
        assert_eq!(res, 5);
    }
}
