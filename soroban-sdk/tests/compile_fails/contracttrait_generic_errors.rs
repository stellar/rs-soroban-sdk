use soroban_sdk::{contract, contractimpl, contracttrait, Env};

// `#[contractimpl(contracttrait)]` invokes the trait's macro by the trait's
// path, which cannot carry generic arguments.

#[contracttrait]
pub trait WithType<T> {
    fn plain(_env: Env) -> u32 {
        1
    }
}

#[contracttrait]
pub trait WithLifetime<'a> {
    fn plain(_env: Env) -> u32 {
        2
    }
}

#[contract]
pub struct C;

#[contractimpl(contracttrait)]
impl WithType<u32> for C {}

#[contract]
pub struct D;

#[contractimpl(contracttrait)]
impl<'a> WithLifetime<'a> for D {}

fn main() {}
