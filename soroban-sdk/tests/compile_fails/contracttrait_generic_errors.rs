use soroban_sdk::{contract, contractimpl, contracttrait, Env};

// `#[contractimpl(contracttrait)]` invokes the trait's macro by the trait's
// path, which cannot carry generic arguments.

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

fn main() {}
