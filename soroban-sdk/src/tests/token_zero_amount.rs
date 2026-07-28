//! Tests documenting what a Stellar Asset Contract does with a zero amount.
//!
//! A zero-amount transfer is accepted — zero is the inclusive lower bound of
//! the allowed range — but it is not a free no-op: every check and side effect
//! other than the balance arithmetic still happens.

use crate as soroban_sdk;

use soroban_sdk::{
    symbol_short,
    testutils::{storage::Persistent as _, Address as _, Events as _, IssuerFlags},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, IntoVal,
};

/// A funded sender, an unfunded recipient, and clients for both interfaces.
fn setup(env: &Env) -> (TokenClient<'_>, StellarAssetClient<'_>, Address, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let token = TokenClient::new(env, &sac.address());
    let admin_client = StellarAssetClient::new(env, &sac.address());
    let from = Address::generate(env);
    let to = Address::generate(env);
    admin_client.mint(&from, &100);
    (token, admin_client, from, to)
}

#[test]
fn zero_transfer_succeeds_and_leaves_balances_unchanged() {
    let env = Env::default();
    let (token, _, from, to) = setup(&env);

    token.transfer(&from, &to, &0);

    assert_eq!(token.balance(&from), 100);
    assert_eq!(token.balance(&to), 0);
}

#[test]
fn zero_transfer_from_an_account_with_no_balance_succeeds() {
    let env = Env::default();
    let (token, _, _, _) = setup(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    // Neither address has ever held the asset, so there is no balance entry to
    // debit. Sending zero out of nothing is still accepted.
    token.transfer(&from, &to, &0);

    assert_eq!(token.balance(&from), 0);
    assert_eq!(token.balance(&to), 0);
}

#[test]
fn zero_transfer_still_requires_auth() {
    let env = Env::default();
    let (token, _, from, to) = setup(&env);

    // Drop the mocked auths: a zero transfer is not exempt from `from`'s
    // authorization, so it fails to authenticate like any other transfer.
    env.set_auths(&[]);
    assert!(token.try_transfer(&from, &to, &0).is_err());
}

#[test]
fn zero_transfer_records_an_authorized_invocation() {
    extern crate std;

    let env = Env::default();
    let (token, _, from, to) = setup(&env);

    token.transfer(&from, &to, &0);

    assert_eq!(
        env.auths(),
        std::vec![(
            from.clone(),
            crate::testutils::AuthorizedInvocation {
                function: crate::testutils::AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&from, &to, 0_i128).into_val(&env),
                )),
                sub_invocations: std::vec![],
            },
        )],
    );
}

#[test]
fn zero_transfer_emits_a_transfer_event() {
    let env = Env::default();
    let (token, _, from, to) = setup(&env);

    // Read the asset name up front; it is a contract call, and the recorded
    // events only cover the most recent invocation.
    let name = token.name();

    token.transfer(&from, &to, &0);

    // The event is shaped exactly like a real transfer's, carrying 0 as the
    // amount, so a zero transfer is indistinguishable from a funds movement to
    // anything watching the event stream for the topics alone.
    let topics = (symbol_short!("transfer"), from.clone(), to.clone(), name);
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                token.address.clone(),
                topics.into_val(&env),
                0_i128.into_val(&env),
            ),
        ],
    );
}

#[test]
fn zero_transfer_creates_a_balance_entry_for_a_new_recipient() {
    let env = Env::default();
    let (token, _, from, to) = setup(&env);

    let entries = || env.as_contract(&token.address, || env.storage().persistent().all().len());
    let before = entries();

    token.transfer(&from, &to, &0);

    // Receiving zero still writes a balance entry for a recipient that did not
    // have one, so a zero transfer permanently grows the contract's state.
    assert_eq!(entries(), before + 1);
    assert_eq!(token.balance(&to), 0);
}

#[test]
fn zero_transfer_rejected_when_a_balance_is_deauthorized() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    sac.issuer().set_flag(IssuerFlags::RevocableFlag);
    let token = TokenClient::new(&env, &sac.address());
    let admin_client = StellarAssetClient::new(&env, &sac.address());

    let frozen = Address::generate(&env);
    let other = Address::generate(&env);
    admin_client.mint(&frozen, &100);
    admin_client.mint(&other, &100);
    admin_client.set_authorized(&frozen, &false);

    // The deauthorization check runs before the amount is looked at, on both
    // sides of the transfer.
    assert!(token.try_transfer(&frozen, &other, &0).is_err());
    assert!(token.try_transfer(&other, &frozen, &0).is_err());
}

#[test]
fn zero_transfer_to_a_classic_account_still_requires_a_trustline() {
    let env = Env::default();
    let (token, _, from, _) = setup(&env);

    // A classic account address that holds no trustline for this asset. Zero
    // amount or not, the transfer has to load the trustline to apply it, so it
    // fails — a zero transfer is not a safe way to probe an arbitrary account.
    let no_trustline = Address::from_str(
        &env,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
    );

    assert!(token.try_transfer(&from, &no_trustline, &0).is_err());
    assert!(token.try_transfer(&no_trustline, &from, &0).is_err());
}

#[test]
fn zero_transfer_from_needs_no_allowance() {
    let env = Env::default();
    let (token, _, from, to) = setup(&env);
    let spender = Address::generate(&env);

    // `transfer_from` requires `allowance >= amount`, which zero satisfies with
    // no allowance ever having been granted, and nothing is deducted.
    assert_eq!(token.allowance(&from, &spender), 0);
    token.transfer_from(&spender, &from, &to, &0);
    assert_eq!(token.allowance(&from, &spender), 0);
    assert_eq!(token.balance(&from), 100);
}

#[test]
fn negative_transfer_is_rejected() {
    let env = Env::default();
    let (token, _, from, to) = setup(&env);

    // Zero is allowed, so the boundary of the rejected range is -1.
    assert!(token.try_transfer(&from, &to, &-1).is_err());
    assert!(token.try_transfer(&from, &to, &0).is_ok());
}
