#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

// The associated type to hold a default impl for a trait pattern is a pattern that's seen in the
// OpenZeppelin contract library.

pub struct DefaultImpl;

impl Trait for DefaultImpl {
    type Impl = Self;
    fn exec(env: &Env) -> String {
        String::from_str(env, "default")
    }
}

pub trait Trait {
    type Impl: Trait;

    fn exec(env: &Env) -> String {
        Self::Impl::exec(env)
    }
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Trait for Contract {
    type Impl = DefaultImpl;
    fn exec(env: &Env) -> String {
        Self::Impl::exec(env)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{Env, String};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_exec() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.exec();
        assert_eq!(res, String::from_str(&e, "default"));
    }
}
