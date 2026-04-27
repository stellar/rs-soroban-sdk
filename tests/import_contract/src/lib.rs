#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

mod addcontract {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/test_add_u64.wasm");
}

#[contracterror]
#[derive(Debug, PartialEq)]
pub enum Error {
    Abort = 0,
    Overflow = 1,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, contract_id: Address, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
    pub fn safe_add_with(env: Env, contract_id: Address, x: u64, y: u64) -> Result<u64, Error> {
        match addcontract::Client::new(&env, &contract_id).try_safe_add(&x, &y) {
            Ok(Ok(i)) => Ok(i),
            Err(Ok(addcontract::Error::Overflow)) => Err(Error::Overflow),
            _ => Err(Error::Abort),
        }
    }
    pub fn safe_add_with_two(env: Env, contract_id: Address, x: u64, y: u64) -> Result<u64, Error> {
        match addcontract::Client::new(&env, &contract_id).try_safe_add_two(&x, &y) {
            Ok(Ok(i)) => Ok(i),
            Err(Ok(addcontract::MyError::Overflow)) => Err(Error::Overflow),
            _ => Err(Error::Abort),
        }
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{addcontract, Contract, ContractClient, Error};

    #[test]
    fn test_add() {
        let e = Env::default();
        let add_contract_id = e.register(addcontract::WASM, ());

        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let x = 10u64;
        let y = 12u64;
        let z = client.add_with(&add_contract_id, &x, &y);
        assert!(z == 22);
    }

    #[test]
    fn test_safe_add() {
        let e = Env::default();
        let add_contract_id = e.register(addcontract::WASM, ());

        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let x = u64::MAX;
        let y = 1;
        let z = client.try_safe_add_with(&add_contract_id, &x, &y);
        assert_eq!(z, Err(Ok(Error::Overflow)));
        let z = client.try_safe_add_with_two(&add_contract_id, &x, &y);
        assert_eq!(z, Err(Ok(Error::Overflow)));
    }
}
