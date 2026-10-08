use crate::{
    self as soroban_sdk,
    testutils::{AuthorizedFunction, AuthorizedInvocation},
};

use soroban_sdk::{
    contract, contractimpl, contracttype,
    testutils::{Address as _, IssuerFlags, MockAuth, MockAuthInvoke},
    token::Client as TokenClient,
    Address, Env, IntoVal, Symbol,
};

#[contracttype]
pub enum DataKey {
    Token,
}

fn get_token(e: &Env) -> Address {
    e.storage().persistent().get(&DataKey::Token).unwrap()
}

#[contract]
pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn init(e: Env, contract: Address) {
        e.storage().persistent().set(&DataKey::Token, &contract);
    }

    pub fn get_token(e: Env) -> Address {
        get_token(&e)
    }

    pub fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        TokenClient::new(&e, &get_token(&e)).approve(&from, &spender, &amount, &expiration_ledger);
    }

    pub fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        TokenClient::new(&e, &get_token(&e)).allowance(&from, &spender)
    }
}

#[test]
fn test_issuer_flags() {
    extern crate std;

    let env = Env::default();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);

    assert_eq!(sac.issuer().flags(), 0);

    let required_and_revocable =
        (IssuerFlags::RequiredFlag as u32) | (IssuerFlags::RevocableFlag as u32);
    sac.issuer().set_flag(IssuerFlags::RequiredFlag);
    sac.issuer().set_flag(IssuerFlags::RevocableFlag);

    assert_eq!(sac.issuer().flags(), required_and_revocable);

    sac.issuer().clear_flag(IssuerFlags::RequiredFlag);
    assert_eq!(sac.issuer().flags(), IssuerFlags::RevocableFlag as u32);
}

#[test]
fn test_mock_all_auth() {
    extern crate std;

    let env = Env::default();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let token_contract_id = sac.address();

    let contract_id = env.register(TestContract, ());
    let client = TestContractClient::new(&env, &contract_id);
    client.init(&token_contract_id);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.decimals(), 7);
    let from = Address::generate(&env);
    let spender = Address::generate(&env);

    // `TestContract` doesn't call `require_auth` before calling into token,
    // thus we need to allow non-root auth and regular `mock_all_auths` will
    // fail.
    assert!(client
        .mock_all_auths()
        .try_approve(&from, &spender, &20, &200)
        .is_err());
    client
        .mock_all_auths_allowing_non_root_auth()
        .approve(&from, &spender, &20, &200);

    let auths = env.auths();
    assert_eq!(
        auths,
        std::vec![(
            from.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token_contract_id.clone(),
                    Symbol::new(&env, "approve"),
                    (&from, &spender, 20_i128, 200_u32).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(client.allowance(&from, &spender), 20);

    // Also test `try_` with `mock_all_auths_allowing_non_root_auth` succeeds.
    let from2 = Address::generate(&env);
    assert!(client
        .mock_all_auths_allowing_non_root_auth()
        .try_approve(&from2, &spender, &30, &200)
        .is_ok());

    let auths = env.auths();
    assert_eq!(
        auths,
        std::vec![(
            from2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token_contract_id.clone(),
                    Symbol::new(&env, "approve"),
                    (&from2, &spender, 30_i128, 200_u32).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(client.allowance(&from2, &spender), 30);
}

#[test]
fn test_mock_auth() {
    extern crate std;

    let env = Env::default();

    let admin = Address::generate(&env);
    let token_contract_id = env.register_stellar_asset_contract_v2(admin).address();

    let contract_id = env.register(TestContract, ());
    let client = TestContractClient::new(&env, &contract_id);
    client.init(&token_contract_id);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.decimals(), 7);
    let from = Address::generate(&env);
    let spender = Address::generate(&env);

    client
        .mock_auths(&[MockAuth {
            address: &from,
            invoke: &MockAuthInvoke {
                contract: &token_contract_id,
                fn_name: "approve",
                args: (&from, &spender, 20_i128, 200_u32).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .approve(&from, &spender, &20, &200);

    assert_eq!(client.allowance(&from, &spender), 20);
}

#[test]
fn test_transfer_to_muxed_contract() {
    use crate::{
        env::xdr::{ContractEventBody, ScMap, ScMapEntry, ScSymbol, ScVal},
        testutils::{Events, MuxedAddress as _},
        token::StellarAssetClient,
        MuxedAddress,
    };

    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let token = TokenClient::new(&env, &sac.address());
    let from = Address::generate(&env);
    StellarAssetClient::new(&env, &sac.address()).mint(&from, &100);

    let to_contract = env.register(TestContract, ());
    let to = MuxedAddress::new(&to_contract, 123456);
    token.transfer(&from, &to, &40);

    let events = env.events().all().filter_by_contract(&sac.address());
    let ContractEventBody::V0(body) = &events.events().last().unwrap().body;
    assert_eq!(
        body.data,
        ScVal::Map(Some(ScMap(
            [
                ScMapEntry {
                    key: ScVal::Symbol(ScSymbol("amount".try_into().unwrap())),
                    val: 40i128.into(),
                },
                ScMapEntry {
                    key: ScVal::Symbol(ScSymbol("to_muxed_id".try_into().unwrap())),
                    val: 123456u64.into(),
                },
            ]
            .try_into()
            .unwrap()
        )))
    );

    assert_eq!(token.balance(&from), 60);
    assert_eq!(token.balance(&to_contract), 40);
}
