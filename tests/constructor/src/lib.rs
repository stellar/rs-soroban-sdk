#![no_std]
#[cfg(test)]
use soroban_sdk::IntoVal;
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contract]
pub struct Contract;

#[contracttype]
pub enum DataKey {
    Persistent(u32),
    Temp(u32),
    Instance(u32),
}

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, init_key: u32, init_value: i64) {
        env.storage()
            .persistent()
            .set(&DataKey::Persistent(init_key), &init_value);
        env.storage()
            .temporary()
            .set(&DataKey::Temp(init_key * 2), &(init_value * 2));
        env.storage()
            .instance()
            .set(&DataKey::Instance(init_key * 3), &(init_value * 3));
    }

    pub fn get_data(env: Env, key: DataKey) -> Option<i64> {
        match key {
            DataKey::Persistent(_) => env.storage().persistent().get(&key),
            DataKey::Temp(_) => env.storage().temporary().get(&key),
            DataKey::Instance(_) => env.storage().instance().get(&key),
        }
    }
}

#[test]
fn test_constructor() {
    let env = Env::default();
    let contract_id = env.register_contract_with_constructor(None, Contract, (100_u32, 1000_i64));
    let client = ContractClient::new(&env, &contract_id);
    assert_eq!(client.get_data(&DataKey::Persistent(100)), Some(1000));
    assert_eq!(client.get_data(&DataKey::Temp(200)), Some(2000));
    assert_eq!(client.get_data(&DataKey::Instance(300)), Some(3000));

    assert_eq!(client.get_data(&DataKey::Persistent(10)), None);
    assert_eq!(client.get_data(&DataKey::Temp(20)), None);
    assert_eq!(client.get_data(&DataKey::Instance(30)), None)
}

#[test]
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_passing_no_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register_contract(None, Contract);
}

#[test]
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_missing_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register_contract_with_constructor(None, Contract, (100_u32,).into_val(&env));
}

#[test]
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_passing_extra_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register_contract_with_constructor(
        None,
        Contract,
        (100_u32, 1000_i64, 123_u32).into_val(&env),
    );
}

#[test]
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_passing_incorrectly_typed_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ =
        env.register_contract_with_constructor(None, Contract, (100_u32, 1000_u32).into_val(&env));
}
