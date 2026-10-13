use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttrait, Env};

// An empty impl of a trait without default fns leaves the trait's macro with no
// default fns to generate.
#[contracttrait]
pub trait Empty {}

#[contract]
pub struct Contract;

#[contractimpl(contracttrait)]
impl Empty for Contract {}

#[test]
fn test_empty_impl_of_trait_without_default_fns() {
    let e = Env::default();
    e.register(Contract, ());
}
