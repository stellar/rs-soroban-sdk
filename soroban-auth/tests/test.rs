#![cfg(feature = "testutils")]

use soroban_auth::testutils::ed25519::{generate, sign};
use soroban_auth::{verify, Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, BytesN, Env};

#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
}

fn read_nonce(e: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    e.contract_data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

fn verify_and_consume_nonce(e: &Env, id: &Identifier, expected_nonce: &BigInt) {
    // replay protection is not required for Contract authorization because
    // there's no cryptographic signature involved. All that's checked is the
    // invoking contract, so this contract just expects 0.
    if matches!(id, Identifier::Contract(_)) {
        if BigInt::zero(&e) != expected_nonce {
            panic!("nonce should be zero for Contract")
        }
        return;
    }

    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, id);

    if nonce != expected_nonce {
        panic!("incorrect nonce")
    }
    e.contract_data().set(key, &nonce + 1);
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn verify_sig(e: Env, sig: Signature, nonce: BigInt) {
        let auth_id = sig.identifier(&e);

        verify_and_consume_nonce(&e, &auth_id, &nonce);

        verify(&e, &sig, symbol!("verify_sig"), (&auth_id, nonce));
    }

    pub fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
    }
}

pub struct OuterTestContract;

#[contractimpl]
impl OuterTestContract {
    pub fn authorize(e: Env, contract_id: BytesN<32>) {
        let client = TestContractClient::new(&e, contract_id);
        client.verify_sig(&Signature::Contract, &BigInt::zero(&e));
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, TestContract);
    let client = TestContractClient::new(&env, &contract_id);

    let (id, signer) = generate(&env);
    let nonce = client.nonce(&id);

    let sig = sign(
        &env,
        &signer,
        &contract_id,
        symbol!("verify_sig"),
        (&id, &nonce),
    );

    client.verify_sig(&sig, &nonce);

    //Make sure the Nonce doesn't increment for Signature::Contract
    let outer_contract_id = BytesN::from_array(&env, &[1; 32]);
    let outer_client = OuterTestContractClient::new(&env, &outer_contract_id);
    env.register_contract(&outer_contract_id, OuterTestContract);

    outer_client.authorize(&contract_id);
    outer_client.authorize(&contract_id);
}
