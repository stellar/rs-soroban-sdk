use soroban_sdk::{contractimpl, contracttype, Address, BytesN, Env, IntoVal};

mod token_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_token_spec.wasm"
    );
    pub type TokenClient = Client;
}

use token_contract::TokenClient;

use soroban_sdk::xdr::{
    AccountId, AlphaNum12, AlphaNum4, Asset, AssetCode12, AssetCode4, PublicKey, Uint256,
};

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

fn run_test(asset: Asset, expected_name: &str) {
    let env = Env::default();

    let token_contract_id = env.register_stellar_asset_contract(asset);

    let contract_id = env.register_contract(None, TestContract);
    let client = TestContractClient::new(&env, &contract_id);
    client.init(&token_contract_id);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.name(), expected_name.into_val(&env));

    let from = Address::random(&env);
    let spender = Address::random(&env);
    client.incr_allow(&from, &spender, &20);

    // Smoke test check that authorization with wrong args didn't happen.
    assert!(!env.verify_top_authorization(
        &from,
        &token_client.contract_id,
        "incr_allow",
        (&from, &spender, 19_i128).into_val(&env),
    ));
    assert!(env.verify_top_authorization(
        &from,
        &token_client.contract_id,
        "incr_allow",
        (&from, &spender, 20_i128).into_val(&env),
    ));
    // Smoke test check that double authorization didn't happen.
    assert!(!env.verify_top_authorization(
        &from,
        &token_client.contract_id,
        "incr_allow",
        (&from, &spender, 20_i128).into_val(&env),
    ));

    assert_eq!(client.allowance(&from, &spender), 20);
}

#[test]
fn test_asset_4() {
    let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([65u8; 32])));
    let asset4 = Asset::CreditAlphanum4(AlphaNum4 {
        asset_code: AssetCode4([66u8; 4]),
        issuer: issuer.clone(),
    });
    run_test(asset4, "BBBB:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
}

#[test]
fn test_asset_12() {
    let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([65u8; 32])));
    let asset12 = Asset::CreditAlphanum12(AlphaNum12 {
        asset_code: AssetCode12([67u8; 12]),
        issuer,
    });
    run_test(asset12, "CCCCCCCCCCCC:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
}

#[test]
fn test_native_asset() {
    run_test(Asset::Native, "native");
}
