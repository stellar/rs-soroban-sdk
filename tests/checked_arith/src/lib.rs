#![no_std]
use soroban_sdk::{contract, contractimpl, Env, I256, U256};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn u256_checked_add(_env: Env, a: U256, b: U256) -> Option<U256> {
        a.checked_add(&b)
    }

    pub fn u256_checked_sub(_env: Env, a: U256, b: U256) -> Option<U256> {
        a.checked_sub(&b)
    }

    pub fn u256_checked_mul(_env: Env, a: U256, b: U256) -> Option<U256> {
        a.checked_mul(&b)
    }

    pub fn u256_checked_pow(_env: Env, base: U256, exp: u32) -> Option<U256> {
        base.checked_pow(exp)
    }

    pub fn i256_checked_add(_env: Env, a: I256, b: I256) -> Option<I256> {
        a.checked_add(&b)
    }

    pub fn i256_checked_sub(_env: Env, a: I256, b: I256) -> Option<I256> {
        a.checked_sub(&b)
    }

    pub fn i256_checked_mul(_env: Env, a: I256, b: I256) -> Option<I256> {
        a.checked_mul(&b)
    }

    pub fn i256_checked_pow(_env: Env, base: I256, exp: u32) -> Option<I256> {
        base.checked_pow(exp)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_u256_checked_add_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let a = U256::from_u32(&env, 10);
        let b = U256::from_u32(&env, 20);
        assert_eq!(
            client.u256_checked_add(&a, &b),
            Some(U256::from_u32(&env, 30))
        );
    }

    #[test]
    fn test_u256_checked_add_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let one = U256::from_u32(&env, 1);
        assert_eq!(client.u256_checked_add(&max, &one), None);
    }

    #[test]
    fn test_u256_checked_sub_underflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let a = U256::from_u32(&env, 5);
        let b = U256::from_u32(&env, 10);
        assert_eq!(client.u256_checked_sub(&a, &b), None);
    }

    #[test]
    fn test_u256_checked_mul_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let a = U256::from_u32(&env, 7);
        let b = U256::from_u32(&env, 6);
        assert_eq!(
            client.u256_checked_mul(&a, &b),
            Some(U256::from_u32(&env, 42))
        );
    }

    #[test]
    fn test_u256_checked_pow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let base = U256::from_u32(&env, 2);
        assert_eq!(
            client.u256_checked_pow(&base, &10),
            Some(U256::from_u32(&env, 1024))
        );
    }

    #[test]
    fn test_u256_checked_mul_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let two = U256::from_u32(&env, 2);
        assert_eq!(client.u256_checked_mul(&max, &two), None);
    }

    #[test]
    fn test_u256_checked_pow_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
        assert_eq!(client.u256_checked_pow(&max, &2), None);
    }

    #[test]
    fn test_i256_checked_add_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let a = I256::from_i32(&env, -10);
        let b = I256::from_i32(&env, 3);
        assert_eq!(
            client.i256_checked_add(&a, &b),
            Some(I256::from_i32(&env, -7))
        );
    }

    #[test]
    fn test_i256_checked_add_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let one = I256::from_i32(&env, 1);
        assert_eq!(client.i256_checked_add(&max, &one), None);
    }

    #[test]
    fn test_i256_checked_sub_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
        let one = I256::from_i32(&env, 1);
        assert_eq!(client.i256_checked_sub(&min, &one), None);
    }

    #[test]
    fn test_i256_checked_mul_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let two = I256::from_i32(&env, 2);
        assert_eq!(client.i256_checked_mul(&max, &two), None);
    }

    #[test]
    fn test_i256_checked_pow_overflow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
        assert_eq!(client.i256_checked_pow(&max, &2), None);
    }

    #[test]
    fn test_i256_checked_pow_via_contract() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let base = I256::from_i32(&env, -3);
        assert_eq!(
            client.i256_checked_pow(&base, &3),
            Some(I256::from_i32(&env, -27))
        );
    }
}
