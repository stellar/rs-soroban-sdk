#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

// This test contract verifies that single-level associated types in function
// signatures (both input and return) are resolved correctly.

pub trait AssociatedType {
    type Val;

    fn set_val(env: Env, input: Self::Val);
    fn get_val(env: Env) -> Self::Val;
    fn both(input: Self::Val) -> Self::Val;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl AssociatedType for Contract {
    type Val = u64;

    fn set_val(env: Env, input: Self::Val) {
        env.storage().instance().set(&symbol_short!("val"), &input);
    }

    fn get_val(env: Env) -> Self::Val {
        env.storage().instance().get(&symbol_short!("val")).unwrap()
    }

    fn both(input: Self::Val) -> Self::Val {
        input + 1
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_associated_type_retval() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.set_val(&42u64);
        assert_eq!(client.get_val(), 42u64);
        assert_eq!(client.both(&42u64), 43u64);
    }
}
