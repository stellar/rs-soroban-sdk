#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Error, Vec};

// This test contract verifies that associated types in function
// signatures (both input and return) are resolved correctly.

pub trait AssociatedType {
    type Val;
    type ValVal;

    fn set_val(env: Env, input: Self::Val);
    fn get_val(env: Env) -> Self::Val;
    fn both(input: Self::Val) -> Self::Val;
    fn wrapped(input: Vec<Self::Val>) -> Result<Self::Val, Error>;
    fn double_wrapped(input: Option<Vec<Self::Val>>) -> Result<Vec<Self::Val>, Error>;
    fn valval(env: Env, input: Self::ValVal) -> Option<Self::ValVal>;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl AssociatedType for Contract {
    type Val = u64;
    type ValVal = Self::Val;

    fn set_val(env: Env, input: Self::Val) {
        env.storage().instance().set(&symbol_short!("val"), &input);
    }

    fn get_val(env: Env) -> Self::Val {
        env.storage().instance().get(&symbol_short!("val")).unwrap()
    }

    fn both(input: Self::Val) -> Self::Val {
        input + 1
    }

    fn wrapped(input: Vec<Self::Val>) -> Result<Self::Val, Error> {
        if input.is_empty() {
            Err(Error::from_contract_error(0))
        } else {
            let mut sum = 0;
            for val in input {
                sum += val;
            }
            Ok(sum)
        }
    }

    fn double_wrapped(input: Option<Vec<Self::Val>>) -> Result<Vec<Self::Val>, Error> {
        match input {
            Some(v) => Ok(v),
            None => Err(Error::from_contract_error(1)),
        }
    }

    fn valval(env: Env, input: Self::ValVal) -> Option<Self::ValVal> {
        Some(input)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{vec, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_associated_type_retval() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.set_val(&42u64);
        assert_eq!(client.get_val(), 42u64);
        assert_eq!(client.both(&42u64), 43u64);
        assert_eq!(client.wrapped(&vec![&e, 1u64, 2u64, 3u64]), 6u64);
        assert_eq!(
            client.try_wrapped(&vec![&e]).err(),
            Some(Ok(soroban_sdk::Error::from_contract_error(0)))
        );
        assert_eq!(
            client.double_wrapped(&Some(vec![&e, 4u64, 5u64])),
            vec![&e, 4u64, 5u64]
        );
        assert_eq!(
            client.try_double_wrapped(&None).err(),
            Some(Ok(soroban_sdk::Error::from_contract_error(1)))
        );
        assert_eq!(client.valval(&42u64), Some(42u64));
    }
}
