use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttrait, Env};

// The generic arguments each impl gives a contracttrait select what the trait's
// default fns do.

pub trait Provider {
    fn value() -> u32;
}

pub struct One;

impl Provider for One {
    fn value() -> u32 {
        1
    }
}

pub struct Two;

impl Provider for Two {
    fn value() -> u32 {
        2
    }
}

#[contracttrait]
pub trait GenericValue<P: Provider> {
    fn value(_env: Env) -> u32 {
        P::value()
    }

    fn doubled(_env: Env) -> u32 {
        P::value() * 2
    }
}

#[contract]
pub struct ContractOne;

#[contractimpl(contracttrait)]
impl GenericValue<One> for ContractOne {}

#[contract]
pub struct ContractTwo;

#[contractimpl(contracttrait)]
impl GenericValue<Two> for ContractTwo {
    fn doubled(_env: Env) -> u32 {
        20
    }
}

#[contracttrait]
pub trait GenericScale<const N: u32> {
    fn scale(_env: Env, x: u32) -> u32 {
        x * N
    }
}

#[contract]
pub struct ContractScale;

#[contractimpl(contracttrait)]
impl GenericScale<3> for ContractScale {}

#[test]
fn test_type_argument() {
    let e = Env::default();

    let contract_id = e.register(ContractOne, ());
    let client = ContractOneClient::new(&e, &contract_id);
    assert_eq!(client.value(), 1);
    assert_eq!(client.doubled(), 2);

    let contract_id = e.register(ContractTwo, ());
    let client = ContractTwoClient::new(&e, &contract_id);
    assert_eq!(client.value(), 2);
    assert_eq!(client.doubled(), 20);
}

#[test]
fn test_const_argument() {
    let e = Env::default();
    let contract_id = e.register(ContractScale, ());
    let client = ContractScaleClient::new(&e, &contract_id);

    assert_eq!(client.scale(&2), 6);
}
