use crate as soroban_sdk;

use soroban_sdk::{
    contractimpl, contracttype,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token::Client as TokenClient,
    Address, Env, IntoVal, Symbol,
};

#[contracttype]
pub enum DataKey {
    Token,
}

fn get_token(e: &Env) -> Address {
    e.storage().get(&DataKey::Token).unwrap()
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn init(e: Env, contract: Address) {
        e.storage().set(&DataKey::Token, &contract);
    }

    pub fn get_token(e: Env) -> Address {
        get_token(&e)
    }

    pub fn increase_allowance(e: Env, from: Address, spender: Address, amount: i128) {
        TokenClient::new(&e, &get_token(&e)).increase_allowance(&from, &spender, &amount);
    }

    pub fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        TokenClient::new(&e, &get_token(&e)).allowance(&from, &spender)
    }
}

#[test]
fn test_mock_all_auth() {
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

    client
        .mock_all_auths()
        .increase_allowance(&from, &spender, &20);

    assert_eq!(
        env.auths(),
        std::vec![(
            from.clone(),
            token_contract_id.clone(),
            Symbol::new(&env, "increase_allowance"),
            (&from, &spender, 20_i128).into_val(&env)
        )]
    );

    assert_eq!(client.allowance(&from, &spender), 20);
}

#[test]
fn test_mock_auth() {
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

    client
        .mock_auths(&[MockAuth {
            address: &from,
            nonce: 0,
            invoke: &MockAuthInvoke {
                contract: &token_contract_id,
                fn_name: "increase_allowance",
                args: (&from, &spender, 20_i128).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .increase_allowance(&from, &spender, &20);

    assert_eq!(client.allowance(&from, &spender), 20);
}
