use soroban_auth::{testutils::ed25519::generate, Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BytesN, Env, IntoVal};

mod token_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_token_spec.wasm"
    );
    pub type TokenClient = Client;
}

use token_contract::TokenClient;

#[contracttype]
pub enum DataKey {
    Token,
}

fn get_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::Token).unwrap()
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn init(e: Env, contract: BytesN<32>) {
        e.storage().set(DataKey::Token, contract);
    }

    pub fn get_token(e: Env) -> BytesN<32> {
        get_token(&e)
    }

    pub fn approve(e: Env, spender: Identifier, amount: i128) {
        TokenClient::new(&e, get_token(&e)).incr_allow(&Signature::Invoker, &0, &spender, &amount);
    }

    pub fn allowance(e: Env, from: Identifier, spender: Identifier) -> i128 {
        TokenClient::new(&e, get_token(&e)).allowance(&from, &spender)
    }
}

#[test]
fn test() {
    use soroban_sdk::xdr::Asset;

    let env = Env::default();

    let token_contract_id = env.register_stellar_asset_contract(Asset::Native);

    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, TestContract);
    let client = TestContractClient::new(&env, &contract_id);
    client.init(&token_contract_id);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.name(), "native".into_val(&env));

    let (id, _signer) = generate(&env);

    let amount = 10;
    client.approve(&id, &amount);
    assert_eq!(
        client.allowance(&Identifier::Contract(contract_id), &id),
        amount
    );
}
