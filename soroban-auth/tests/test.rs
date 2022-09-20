#![cfg(feature = "testutils")]

use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_auth::{
    verify, Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0,
};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, BytesN, Env, IntoVal};

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
        let auth_id = sig.get_identifier(&e);

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

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_identifier(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, TestContract);
    let client = TestContractClient::new(&env, &contract_id);

    let kp = generate_keypair();
    let id = make_identifier(&env, &kp);
    let nonce = client.nonce(&id);

    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        function: symbol!("verify_sig"),
        contract: BytesN::from_array(&env, &[0; 32]),
        network: env.ledger().network_passphrase(),
        args: (&id, &nonce).into_val(&env),
    });
    let sig = Signature::Ed25519(Ed25519Signature {
        public_key: BytesN::from_array(&env, &kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(&env),
    });

    client.verify_sig(&sig, &nonce);

    //Make sure the Nonce doesn't increment for Signature::Contract
    let outer_contract_id = BytesN::from_array(&env, &[1; 32]);
    let outer_client = OuterTestContractClient::new(&env, &outer_contract_id);
    env.register_contract(&outer_contract_id, OuterTestContract);

    outer_client.authorize(&contract_id);
    outer_client.authorize(&contract_id);
}
