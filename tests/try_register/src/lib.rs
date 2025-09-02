#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct ContractWithConstructor;

#[contractimpl]
impl ContractWithConstructor {
    pub fn __constructor(env: Env, value: u32) {
        if value > 100 {
            env.storage().persistent().set(&1, &value);
        } else {
            panic!("Value must be greater than 100");
        }
    }

    pub fn get_value(env: Env) -> u32 {
        env.storage().persistent().get(&1).unwrap()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_try_register_success() {
        let env = Env::default();
        let result = env.try_register(ContractWithConstructor, (150_u32,));
        assert!(result.is_ok());

        let contract_id = result.unwrap();
        let client = ContractWithConstructorClient::new(&env, &contract_id);
        assert_eq!(client.get_value(), 150);
    }

    #[test]
    fn test_try_register_constructor_failure() {
        let env = Env::default();
        let result = env.try_register(ContractWithConstructor, (50_u32,));
        assert!(result.is_err());
    }

    #[test]
    fn test_try_register_at_success() {
        let env = Env::default();
        let contract_id = soroban_sdk::Address::generate(&env);

        let result = env.try_register_at(&contract_id, ContractWithConstructor, (200_u32,));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), contract_id);

        let client = ContractWithConstructorClient::new(&env, &contract_id);
        assert_eq!(client.get_value(), 200);
    }

    #[test]
    fn test_try_register_at_failure() {
        let env = Env::default();
        let contract_id = soroban_sdk::Address::generate(&env);
        let result = env.try_register_at(&contract_id, ContractWithConstructor, (25_u32,));
        assert!(result.is_err());
    }
}
