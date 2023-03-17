use soroban_sdk::{
    contractimpl, contracttype, testutils::Address as _, Address, BytesN, Env, IntoVal, Symbol,
};

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
    e.storage().get_unchecked(&DataKey::Token).unwrap()
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn init(e: Env, contract: BytesN<32>) {
        e.storage().set(&DataKey::Token, &contract);
    }

    pub fn get_token(e: Env) -> BytesN<32> {
        get_token(&e)
    }

    pub fn incr_allow(e: Env, from: Address, spender: Address, amount: i128) {
        TokenClient::new(&e, &get_token(&e)).incr_allow(&from, &spender, &amount);
    }

    pub fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        TokenClient::new(&e, &get_token(&e)).allowance(&from, &spender)
    }
}

#[test]
fn test() {
    extern crate std;

    let env = Env::default();
    let admin = Address::random(&env);
    let token_contract_id = env.register_stellar_asset_contract(admin);

    let contract_id = env.register_contract(None, TestContract);
    let client = TestContractClient::new(&env, &contract_id);
    client.init(&token_contract_id);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.decimals(), 7);
    let from = Address::random(&env);
    let spender = Address::random(&env);
    client.incr_allow(&from, &spender, &20);

    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            from.clone(),
            token_client.contract_id,
            Symbol::new(&env, "incr_allow"),
            (&from, &spender, 20_i128).into_val(&env)
        )]
    );

    assert_eq!(client.allowance(&from, &spender), 20);
}
