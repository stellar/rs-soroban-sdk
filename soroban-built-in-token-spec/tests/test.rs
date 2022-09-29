#![cfg(feature = "testutils")]

use soroban_auth::{
    testutils::ed25519::{generate, sign},
    Identifier, Signature,
};
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, BytesN, Env, IntoVal};

mod token_contract;
use token_contract::{TokenClient, TokenMetadata};

#[contracttype]
pub enum DataKey {
    Token,
}

fn get_contract_id(e: &Env) -> Identifier {
    Identifier::Contract(e.get_current_contract().into())
}

fn get_token(e: &Env) -> BytesN<32> {
    e.contract_data().get_unchecked(DataKey::Token).unwrap()
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn init(e: Env, salt: BytesN<32>) {
        let id = e.deployer().from_current_contract(salt).deploy_token();
        let metadata = TokenMetadata {
            name: "name".into_val(&e),
            symbol: "symbol".into_val(&e),
            decimals: 7u32,
        };
        TokenClient::new(&e, &id).init_token(&get_contract_id(&e), &metadata);

        e.contract_data().set(DataKey::Token, id);
    }

    pub fn get_token(e: Env) -> BytesN<32> {
        get_token(&e)
    }

    pub fn mint(e: Env, to: Identifier, amount: BigInt) {
        TokenClient::new(&e, get_token(&e)).mint(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &to,
            &amount,
        );
    }

    pub fn set_admin(e: Env, new_admin: Identifier) {
        TokenClient::new(&e, get_token(&e)).set_admin(
            &Signature::Invoker,
            &BigInt::zero(&e),
            &new_admin,
        );
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, TestContract);
    let client = TestContractClient::new(&env, &contract_id);

    let salt = BytesN::from_array(&env, &[1; 32]);
    client.init(&salt);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.name(), "name".into_val(&env));

    let (id, signer) = generate(&env);
    let (id2, _signer2) = generate(&env);

    let ten = BigInt::from_u32(&env, 10);
    client.mint(&id, &ten);
    assert_eq!(token_client.balance(&id), &ten);

    // transger admin so we can test ed25519 auth
    client.set_admin(&id);

    let nonce = &token_client.nonce(&id);
    let sig = sign(
        &env,
        &signer,
        &client.get_token(),
        symbol!("mint"),
        (&id, nonce, &id2, &ten),
    );
    token_client.mint(&sig, nonce, &id2, &ten);
    assert_eq!(token_client.balance(&id2), &ten);
}
