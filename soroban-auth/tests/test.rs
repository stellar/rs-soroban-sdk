#![cfg(feature = "testutils")]

use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_auth::{
    check_auth, Ed25519Signature, Identifier, NonceAuth, Signature, SignaturePayload,
    SignaturePayloadV0,
};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, BytesN, Env, IntoVal};

#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
}

fn read_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id);
    e.contract_data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

struct NonceForSignature(Signature);

impl NonceAuth for NonceForSignature {
    fn read_nonce(e: &Env, id: Identifier) -> BigInt {
        read_nonce(e, id)
    }

    fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt {
        let key = DataKey::Nonce(id.clone());
        let nonce = Self::read_nonce(e, id);
        e.contract_data().set(key, &nonce + 1);
        nonce
    }

    fn signature(&self) -> &Signature {
        &self.0
    }
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn verify_sig(e: Env, sig: Signature, nonce: BigInt) {
        let auth_id = sig.get_identifier(&e);

        check_auth(
            &e,
            &NonceForSignature(sig),
            nonce.clone(),
            symbol!("verify_sig"),
            (&auth_id, nonce).into_val(&e),
        );
    }

    pub fn nonce(e: Env, id: Identifier) -> BigInt {
        read_nonce(&e, id)
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
