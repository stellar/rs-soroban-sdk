use crate::{self as soroban_sdk};
use crate::{bytes, vec, Bytes, Env, Val, Vec};
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct TestPrngContract;

#[contractimpl]
impl TestPrngContract {
    pub fn prng_reseed(env: Env, bytes: Bytes) {
        env.prng().prng_reseed(&bytes);
    }

    pub fn prng_u64_in_inclusive_range(env: Env, min: u64, max: u64) -> u64 {
        env.prng().prng_u64_in_inclusive_range(min, max)
    }

    pub fn prng_vec_shuffle(env: Env, vec: Vec<u32>) -> Vec<Val> {
        env.prng().prng_vec_shuffle::<Vec<u32>>(vec.into())
    }
}

#[test]
fn test_prng_reseed() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TestPrngContract);
    env.host().set_base_prng_seed([0; 32]).unwrap();
    let client = TestPrngContractClient::new(&env, &contract_id);

    let seed = bytes!(
        &env,
        0x0000000000000000000000000000000000000000000000000000000000000001
    );
    assert_eq!(client.prng_u64_in_inclusive_range(&0, &9), 6);

    client.prng_reseed(&seed);

    assert_eq!(client.prng_u64_in_inclusive_range(&0, &9), 8);
}

#[test]
fn test_prng_vec_shuffle() {
    let env = Env::default();
    env.host().set_base_prng_seed([0; 32]).unwrap();
    let contract_id = env.register_contract(None, TestPrngContract);
    let client = TestPrngContractClient::new(&env, &contract_id);

    let vec = vec![&env, 1, 2, 3];

    assert_eq!(
        client.prng_vec_shuffle(&vec),
        vec![&env, Val::from(2u32), Val::from(3u32), Val::from(1u32)]
    );
}

#[test]
fn test_prng_u64_in_inclusive_range() {
    let env = Env::default();
    env.host().set_base_prng_seed([0; 32]).unwrap();
    let contract_id = env.register_contract(None, TestPrngContract);
    let client = TestPrngContractClient::new(&env, &contract_id);

    assert_eq!(client.prng_u64_in_inclusive_range(&0, &9), 6);
}
