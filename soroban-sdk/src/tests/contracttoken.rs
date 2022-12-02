use crate as soroban_sdk;
use soroban_sdk::{
    contractclient, contracttype, testutils::AccountId as _, AccountId, Bytes, BytesN, Env,
};

#[contractclient(name = "TokenClient")]
pub trait Token {
    fn init(env: Env, admin: Identifier, metadata: TokenMetadata);
    fn name(env: Env) -> Bytes;
}

#[derive(Clone)]
#[contracttype]
pub enum Identifier {
    Account(AccountId),
}

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: Bytes,
    pub symbol: Bytes,
    pub decimals: u32,
}

#[test]
fn test_register_token() {
    let e = Env::default();

    let contract_id = e.register_contract_token(None);
    let client = TokenClient::new(&e, &contract_id);

    client.init(
        &Identifier::Account(AccountId::random(&e)),
        &TokenMetadata {
            name: Bytes::from_slice(&e, b"testme"),
            decimals: 7,
            symbol: Bytes::from_slice(&e, &[]),
        },
    );

    assert_eq!(client.name(), Bytes::from_slice(&e, b"testme"))
}

#[test]
fn test_register_token_at_id() {
    let e = Env::default();

    let contract_id = BytesN::from_array(&e, &[1; 32]);
    e.register_contract_token(&contract_id);
    let client = TokenClient::new(&e, &contract_id);

    client.init(
        &Identifier::Account(AccountId::random(&e)),
        &TokenMetadata {
            name: Bytes::from_slice(&e, b"testme"),
            decimals: 7,
            symbol: Bytes::from_slice(&e, &[]),
        },
    );

    assert_eq!(client.name(), Bytes::from_slice(&e, b"testme"))
}

#[test]
fn test_reregister_over_wasm_with_token() {
    let e = Env::default();

    // Register a contract with bogus wasm.
    let contract_id = e.register_contract_wasm(None, &[]);
    // Reregister the contract with a token instead.
    e.register_contract_token(&contract_id);
    let client = TokenClient::new(&e, &contract_id);

    client.init(
        &Identifier::Account(AccountId::random(&e)),
        &TokenMetadata {
            name: Bytes::from_slice(&e, b"testme"),
            decimals: 7,
            symbol: Bytes::from_slice(&e, &[]),
        },
    );

    assert_eq!(client.name(), Bytes::from_slice(&e, b"testme"))
}
