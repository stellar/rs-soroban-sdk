use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Env, IntoVal, Val};

#[contract]
pub struct Contract;

#[contracttype]
pub enum Flag {
    A = 0,
    B = 1,
}

#[contracterror]
pub enum Error {
    AnError = 1,
}

#[contractimpl]
impl Contract {
    pub fn f(flag: Flag) -> Result<(), Error> {
        match flag {
            Flag::A => Ok(()),
            Flag::B => Err(Error::AnError),
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
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // See comment above about why this assertion is a match.
    let Ok(Ok(())) = client.try_f(&Flag::A) else {
        panic!("unexpected value returned");
    };
}

#[test]
fn test_error() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // See comment above about why this assertion is a match.
    let Err(Ok(Error::AnError)) = client.try_f(&Flag::B) else {
        panic!("unexpected value returned");
    };
}

#[test]
fn test_owned_to_val() {
    let env = Env::default();

    let e = Error::AnError;
    let val: Val = e.into_val(&env);
    let _: Error = val.into_val(&env);
}

#[test]
fn test_ref_to_val() {
    let env = Env::default();

    let e = Error::AnError;
    let val: Val = (&e).into_val(&env);
    let _: Error = val.into_val(&env);
}

#[test]
fn test_double_ref_to_val() {
    let env = Env::default();

    let e = Error::AnError;
    let val: Val = (&&e).into_val(&env);
    let _: Error = val.into_val(&env);
}
