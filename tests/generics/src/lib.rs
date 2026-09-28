// A test vector for testing the interactions of the soroban-sdk macros with generics. The contract
// macros support generics sparingly and almost not at all. So there are very few places that
// generics are permitted. The places that they are permitted are almost meaningless to test
// because they have no utilitiy, but in the interest of capturing all of the places generics are
// supported this test vector exists. Only lifetimes are supported, type and const parameters are
// rejected, see soroban-sdk/tests/compile_fails/generics_errors.rs.

#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttrait, Address, Env};

#[contract]
pub struct Contract;

#[contractevent]
pub struct Exec<'a, 'b>
where
    'a: 'b,
{
    #[topic]
    from: &'a Address,
    result: &'b u32,
}

#[contractimpl]
impl<'a, 'b> Contract
where
    'a: 'b,
{
    pub fn exec(i1: u32, i2: &u32, i3: &'b u32) -> u32 {
        i1 + i2 + *i3
    }

    pub fn exec_fn_lifetime<'c>(env: Env, from: Address, i1: &'c u32) -> u32 {
        Exec {
            from: &from,
            result: i1,
        }
        .publish(&env);
        *i1
    }
}

#[contracttrait]
pub trait Trait {
    fn exec_trait_lifetime<'c>(_env: Env, i1: &'c u32) -> u32 {
        *i1
    }
}

#[contractimpl(contracttrait)]
impl Trait for Contract {}

#[cfg(test)]
mod test {
    use soroban_sdk::{testutils::Address as _, Address, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.exec(&1, &2, &3);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_fn_lifetime() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let from = Address::generate(&e);
        let res = client.exec_fn_lifetime(&from, &4);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_trait_lifetime() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.exec_trait_lifetime(&5);
        assert_eq!(res, 5);
    }
}
