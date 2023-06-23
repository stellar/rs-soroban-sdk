use crate::{self as soroban_sdk};
use crate::{bytes, bytesn, vec, Bytes, BytesN, Env, IntoVal, Vec};
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct TestCryptoContract;

#[contractimpl]
impl TestCryptoContract {
    pub fn sha256(env: Env, bytes: Bytes) -> BytesN<32> {
        env.crypto().sha256(&bytes)
    }

    pub fn prng_reseed(env: Env, bytes: Bytes) {
        env.crypto().prng_reseed(&bytes);
    }

    pub fn u64_in_inclusive_range(env: Env, min: u64, max: u64) -> u64 {
        env.crypto().u64_in_inclusive_range(min, max)
    }

    pub fn vec_shuffle(env: Env, vec: Vec<u32>) -> Vec<u32> {
        env.crypto().vec_shuffle(vec.into()).into_val(&env)
    }

    pub fn verify_sig_ed25519(
        env: Env,
        public_key: BytesN<32>,
        message: Bytes,
        signature: BytesN<64>,
    ) {
        env.crypto()
            .ed25519_verify(&public_key, &message, &signature);
    }
}

#[test]
fn test_prng_reseed() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TestCryptoContract);
    env.host().set_base_prng_seed([0; 32]);
    let client = TestCryptoContractClient::new(&env, &contract_id);

    let seed = bytes!(
        &env,
        0x0000000000000000000000000000000000000000000000000000000000000001
    );
    assert_eq!(client.u64_in_inclusive_range(&0, &9), 6);

    client.prng_reseed(&seed);

    assert_eq!(client.u64_in_inclusive_range(&0, &9), 8);
}

#[test]
fn test_sha256() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);

    let bytes = bytes!(&env, 0x01);

    assert_eq!(client.sha256(&bytes), bytesn!(&env, 0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a));
}

#[test]
fn test_vec_shuffle() {
    let env = Env::default();
    env.host().set_base_prng_seed([0; 32]);
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);

    let vec = vec![&env, 1, 2, 3];

    assert_eq!(client.vec_shuffle(&vec), vec![&env, 2, 3, 1]);
}

#[test]
fn test_u64_in_inclusive_range() {
    let env = Env::default();
    env.host().set_base_prng_seed([0; 32]);
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);

    assert_eq!(client.u64_in_inclusive_range(&0, &9), 6);
}

#[test]
fn test_verify_sig_ed25519() {
    let env = Env::default();
    env.host().set_base_prng_seed([0; 32]);
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);
    // From https://datatracker.ietf.org/doc/html/rfc8032#section-7.1
    let public_key: BytesN<32> = bytes!(
        &env,
        0x3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c
    )
    .try_into()
    .unwrap();
    let signature = bytesn!(
        &env,
        0x92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00
    );
    let message = bytes!(&env, 0x72);

    assert_eq!(client.verify_sig_ed25519(&public_key, &message, &signature), ());
}

#[test]
#[should_panic]
fn test_verify_sig_ed25519_invalid_sig() {
    let env = Env::default();
    env.host().set_base_prng_seed([0; 32]);
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);
    // From https://datatracker.ietf.org/doc/html/rfc8032#section-7.1
    let public_key = bytesn!(
        &env,
        0x3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c
    )
    .try_into()
    .unwrap();
    let signature = bytesn!(
        &env,
        0x92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00
    );
    let message = bytes!(&env, 0x73);

    client.verify_sig_ed25519(&public_key, &message, &signature);
}
