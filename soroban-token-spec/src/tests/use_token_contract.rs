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
    use soroban_sdk::xdr::{
        AccountId, AlphaNum12, AlphaNum4, Asset, AssetCode12, AssetCode4, PublicKey, Uint256,
    };

    let env = Env::default();

    let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([65u8; 32])));
    let asset4 = Asset::CreditAlphanum4(AlphaNum4 {
        asset_code: AssetCode4([66u8; 4]),
        issuer: issuer.clone(),
    });
    let asset12 = Asset::CreditAlphanum12(AlphaNum12 {
        asset_code: AssetCode12([1u8; 12]),
        issuer,
    });
    let token_contract_id = env.register_stellar_asset_contract(asset4);
    // We aren't using the asset12 contract. We just want to make sure the registration works.
    env.register_stellar_asset_contract(asset12);

    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, TestContract);
    let client = TestContractClient::new(&env, &contract_id);
    client.init(&token_contract_id);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(
        token_client.name(),
        "BBBB:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".into_val(&env)
    );

    let (id, _signer) = generate(&env);

    let amount = 10;
    client.approve(&id, &amount);
    assert_eq!(
        client.allowance(&Identifier::Contract(contract_id), &id),
        amount
    );
}

#[test]
fn test_xlm_xfer() {
    use soroban_sdk::xdr::Asset;

    use soroban_sdk::testutils::Accounts;

    let env = Env::default();
    let user1 = env.accounts().generate_and_create();
    env.accounts().update_balance(&user1, 1000i64);

    let token_contract = env.register_stellar_asset_contract(Asset::Native);

    let contract = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract, TestContract);
    let client = TestContractClient::new(&env, &contract);
    client.init(&token_contract);

    let contract_id = Identifier::Contract(contract);

    let token_client = TokenClient::new(&env, &client.get_token());
    token_client
        .with_source_account(&user1)
        .xfer(&Signature::Invoker, &0, &contract_id, &1);
    assert_eq!(token_client.balance(&contract_id), 1);

    let user1_id = Identifier::Account(user1);
    assert_eq!(token_client.balance(&user1_id), 999);
}
