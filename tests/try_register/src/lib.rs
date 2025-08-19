#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct GoodContract;

#[contractimpl]
impl GoodContract {
    pub fn __constructor(env: Env, value: u32) {
        // Constructor that succeeds
        if value > 100 {
            env.storage().persistent().set(&"value", &value);
        } else {
            panic!("Value must be greater than 100");
        }
    }

    pub fn get_value(env: Env) -> u32 {
        env.storage().persistent().get(&"value").unwrap_or(0)
    }
}

#[test]
fn test_try_register_success() {
    let env = Env::default();
    
    // This should succeed 
    let result = env.try_register(GoodContract, (150_u32,));
    assert!(result.is_ok());
    
    let contract_id = result.unwrap();
    let client = GoodContractClient::new(&env, &contract_id);
    assert_eq!(client.get_value(), 150);
}

#[test]
fn test_try_register_constructor_failure() {
    let env = Env::default();
    
    // This should fail due to constructor validation
    let result = env.try_register(GoodContract, (50_u32,));
    assert!(result.is_err());
}

#[test]
fn test_try_register_at_success() {
    let env = Env::default();
    let contract_id = soroban_sdk::Address::generate(&env);
    
    // This should succeed with specific contract ID
    let result = env.try_register_at(&contract_id, GoodContract, (200_u32,));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), contract_id);
    
    let client = GoodContractClient::new(&env, &contract_id);
    assert_eq!(client.get_value(), 200);
}

#[test]
fn test_try_register_at_failure() {
    let env = Env::default();
    let contract_id = soroban_sdk::Address::generate(&env);
    
    // This should fail due to constructor validation
    let result = env.try_register_at(&contract_id, GoodContract, (25_u32,));
    assert!(result.is_err());
}