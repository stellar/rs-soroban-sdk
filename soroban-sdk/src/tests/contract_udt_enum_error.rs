use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contracterror, contractimpl, Env};

#[contract]
pub struct Contract;

#[contracterror]
pub enum Error {
    AnError = 1,
}

#[contractimpl]
impl Contract {
    pub fn f(flag: u32) -> Result<(), Error> {
        if flag == 0 {
            Ok(())
        } else {
            Err(Error::AnError)
        }
    }
}

// The assertions in the following tests intentionally don't use assert_eq, and
// unwraps the object using Rust's pattern matching, so that no derives like
// Debug, Eq, PartialEq, etc are required on the error enum type because this
// test serves to ensure that none of the generated code for the error type or
// the client that uses it is dependent on derives that may or may not be
// available.

#[test]
fn test_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    // See comment above about why this assertion is a match.
    let Ok(Ok(())) = client.try_f(&0) else {
        panic!("unexpected value returned");
    };
}

#[test]
fn test_error() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    // See comment above about why this assertion is a match.
    let Err(Ok(Error::AnError)) = client.try_f(&1) else {
        panic!("unexpected value returned");
    };
}
