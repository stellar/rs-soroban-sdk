#![cfg(feature = "testutils")]

use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_auth::{
    check_auth, Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0,
};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{contractimpl, contracttype, symbol, vec, BigInt, BytesN, Env, IntoVal};

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
    match id {
        Identifier::Contract(_) => {
            if BigInt::zero(&e) != expected_nonce {
                panic!("nonce should be zero for Contract")
            }
        }
        _ => {}
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

        check_auth(
            &e,
            &sig,
            symbol!("verify_sig"),
            (&auth_id, nonce).into_val(&e),
        );
    }

    pub fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, &id)
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
    let client = TestContractClient::new(&env, contract_id);

    let kp = generate_keypair();
    let id = make_identifier(&env, &kp);
    let nonce = client.nonce(&id);

    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        function: symbol!("verify_sig"),
        call_cntxt: vec![&env],
        contract: BytesN::from_array(&env, &[0; 32]),
        network: env.ledger().network_passphrase(),
        args: (&id, &nonce).into_val(&env),
    });
    let sig = Signature::Ed25519(Ed25519Signature {
        public_key: BytesN::from_array(&env, &kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(&env),
    });

    client.verify_sig(&sig, &nonce);
}

// Test a contract that forwards a signature to another contract
pub struct ForwardingContract;

#[contractimpl]
impl ForwardingContract {
    pub fn fwd_sig(e: Env, contract_id: BytesN<32>, sig: Signature, nonce: BigInt) {
        let client = TestContractClient::new(&e, contract_id);
        client.verify_sig(&sig, &nonce)
    }
}

#[test]
fn test_context() {
    let env = Env::default();

    let recv_contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&recv_contract_id, TestContract);

    let fwd_contract_id = BytesN::from_array(&env, &[1; 32]);
    env.register_contract(&fwd_contract_id, ForwardingContract);

    let recv_client = TestContractClient::new(&env, &recv_contract_id);
    let fwd_client = ForwardingContractClient::new(&env, &fwd_contract_id);

    let kp = generate_keypair();
    let id = make_identifier(&env, &kp);
    let nonce = recv_client.nonce(&id);

    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        function: symbol!("verify_sig"),
        call_cntxt: vec![&env, (fwd_contract_id, symbol!("fwd_sig"))],
        contract: recv_contract_id.clone(),
        network: env.ledger().network_passphrase(),
        args: (&id, &nonce).into_val(&env),
    });
    let sig = Signature::Ed25519(Ed25519Signature {
        public_key: BytesN::from_array(&env, &kp.public.to_bytes()),
        signature: kp.sign(msg).unwrap().into_val(&env),
    });

    fwd_client.fwd_sig(&recv_contract_id, &sig, &nonce);
}
