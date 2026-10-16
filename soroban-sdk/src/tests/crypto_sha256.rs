use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, crypto::Hash, Bytes, BytesN, Env};

#[test]
fn test_sha256() {
    let env = Env::default();

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a
    );
    let hash: BytesN<32> = env.crypto().sha256(&input).into();
    assert_eq!(hash, expect);
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn sha256(env: Env, data: Bytes) -> Hash<32> {
        env.crypto().sha256(&data)
    }
}

#[test]
fn test_sha256_returned_from_contract_fn() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a
    );
    // The client returns a Hash as a BytesN, because it can't guarantee the
    // bytes it receives came from a secure cryptographic hash function.
    let hash: BytesN<32> = client.sha256(&input);
    assert_eq!(hash, expect);
    let hash: BytesN<32> = client.try_sha256(&input).unwrap().unwrap();
    assert_eq!(hash, expect);
}
