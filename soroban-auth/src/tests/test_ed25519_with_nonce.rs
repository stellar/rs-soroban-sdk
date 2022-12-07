use crate::testutils::ed25519::{generate, sign};
use crate::{verify, Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, symbol, BytesN, Env};

#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
}

fn read_nonce(e: &Env, id: &Identifier) -> i128 {
    let key = DataKey::Nonce(id.clone());
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

fn verify_and_consume_nonce(e: &Env, id: &Identifier, expected_nonce: i128) {
    // replay protection is not required for invoker authorization because
    // there's no cryptographic signature involved. All that's checked is the
    // invoker, so this contract just expects 0.
    if matches!(id, Identifier::Contract(_)) {
        if expected_nonce != 0 {
            panic!("nonce should be zero for contract")
        }
        return;
    }

    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, id);

    if nonce != expected_nonce {
        panic!("incorrect nonce")
    }
    e.storage().set(key, &nonce + 1);
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn verify_sig(e: Env, sig: Signature, nonce: i128) {
        let auth_id = sig.identifier(&e);

        verify_and_consume_nonce(&e, &auth_id, nonce);

        verify(&e, &sig, symbol!("verify_sig"), (&auth_id, nonce));
    }

    pub fn nonce(e: Env, id: Identifier) -> i128 {
        read_nonce(&e, &id)
    }
}

pub struct OuterTestContract;

#[contractimpl]
impl OuterTestContract {
    pub fn authorize(e: Env, contract_id: BytesN<32>) {
        let client = TestContractClient::new(&e, contract_id);
        client.verify_sig(&Signature::Invoker, &0);
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
