#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl};

#[contract]
pub struct Contract;

#[contracterror]
#[derive(Debug, PartialEq)]
pub enum Error {
    Overflow = 1,
}

#[contracterror]
#[derive(Debug, PartialEq)]
pub enum MyError {
    Overflow = 1,
}

#[contractimpl]
impl Contract {
    pub fn add(a: u64, b: u64) -> u64 {
        a + b
    }
    pub fn safe_add(a: u64, b: u64) -> Result<u64, Error> {
        a.checked_add(b).ok_or(Error::Overflow)
    }
    pub fn safe_add_two(a: u64, b: u64) -> Result<u64, MyError> {
        a.checked_add(b).ok_or(MyError::Overflow)
    }
}

#[cfg(test)]
mod test {
    use core::u64;

    use soroban_sdk::Env;

    use crate::{Contract, ContractClient, Error};

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let x = 10u64;
        let y = 12u64;
        let z = client.add(&x, &y);
        assert!(z == 22);
    }
    #[test]
    fn test_safe_add() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let x = u64::MAX;
        let y = 1;
        let z = client.try_safe_add(&x, &y);
        assert_eq!(z, Err(Ok(Error::Overflow)));
    }
}
