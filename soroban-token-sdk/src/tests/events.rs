extern crate std;

use core::i64;
use std::rc::Rc;

use crate::events::{Approve, Burn, Clawback, Mint, Transfer, TransferLegacy};
use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _, MuxedAddress as _},
    token::StellarAssetClient,
    vec, xdr, Address, Env, IntoVal, Map, MuxedAddress, Symbol, Val,
};

#[contract]
struct Contract;

#[test]
fn test_approve() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let spender = Address::generate(&env);
    let amount = 123;
    let expiration_ledger = 45;

    let event = Approve {
        from: from.clone(),
        spender: spender.clone(),
        amount,
        expiration_ledger,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("approve"), from.clone(), spender.clone());
    let data = (amount, expiration_ledger);

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1, t2) = topics;
    let topics = (t0, t1, t2, client.name());

    client.approve(&from, &spender, &amount, &expiration_ledger);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}

#[test]
fn test_transfer_legacy() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let amount = 123;

    let event = TransferLegacy {
        from: from.clone(),
        to: to.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("transfer"), from.clone(), to.clone());
    let data = amount;

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1, t2) = topics;
    let topics = (t0, t1, t2, client.name());

    client.mint(&from, &123);
    client.transfer(&from, &to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}

#[test]
fn test_transfer_without_id() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let to: MuxedAddress = MuxedAddress::generate(&env).address().into();
    let amount = 123;

    let event = Transfer {
        from: from.clone(),
        to: to.address(),
        to_muxed_id: to.id(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("transfer"), from.clone(), to.address());
    let data = Map::<Symbol, Val>::from_array(
        &env,
        [
            (Symbol::new(&env, "to_muxed_id"), Val::VOID.to_val()),
            (Symbol::new(&env, "amount"), amount.into_val(&env)),
        ],
    );

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env))
        ]
    );

    // No comparison is made with the Stellar Asset Contract for publishing Transfer with a
    // MuxedAddress that does not contain an ID, because the Stellar Asset Contract for legacy
    // reasons, to minimise the changes to its behavior over time, still publishes TransferLegacy
    // in this case. See [`test_transfer_with_id`] for a test that exercises Transfer in the case
    // that the Stellar Asset contract does publish that event.
}

#[test]
fn test_transfer_with_id() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let to = MuxedAddress::generate(&env);
    let amount = 123;

    let event = Transfer {
        from: from.clone(),
        to: to.address(),
        to_muxed_id: to.id(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("transfer"), from.clone(), to.address());
    let data = Map::<Symbol, Val>::from_array(
        &env,
        [
            (Symbol::new(&env, "to_muxed_id"), to.id().into_val(&env)),
            (Symbol::new(&env, "amount"), amount.into_val(&env)),
        ],
    );

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env))
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let trust_line_asset = |asset: xdr::Asset| {
        // TODO: Move this to rs-stellar-xdr.
        match asset {
            xdr::Asset::Native => xdr::TrustLineAsset::Native,
            xdr::Asset::CreditAlphanum4(a) => xdr::TrustLineAsset::CreditAlphanum4(a),
            xdr::Asset::CreditAlphanum12(a) => xdr::TrustLineAsset::CreditAlphanum12(a),
        }
    };

    client.mint(&from, &123);
    env.host()
        .add_ledger_entry(
            &Rc::new(xdr::LedgerKey::Trustline(xdr::LedgerKeyTrustLine {
                account_id: to.address().try_into().unwrap(),
                asset: trust_line_asset(asset.asset()),
            })),
            &Rc::new(xdr::LedgerEntry {
                data: xdr::LedgerEntryData::Trustline(xdr::TrustLineEntry {
                    account_id: to.address().try_into().unwrap(),
                    asset: trust_line_asset(asset.asset()),
                    balance: 0,
                    flags: xdr::TrustLineFlags::AuthorizedFlag as u32,
                    limit: i64::MAX,
                    ext: xdr::TrustLineEntryExt::V0,
                }),
                last_modified_ledger_seq: 0,
                ext: xdr::LedgerEntryExt::V0,
            }),
            None,
        )
        .unwrap();

    let (t0, t1, t2) = topics;
    let topics = (t0, t1, t2, client.name());

    client.transfer(&from, &to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let amount = 123;

    let event = Burn {
        from: from.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("burn"), from.clone());
    let data = amount;

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1) = topics;
    let topics = (t0, t1, client.name());

    client.mint(&from, &amount);
    client.burn(&from, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let to = Address::generate(&env);
    let amount = 123;

    let event = Mint {
        to: to.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("mint"), to.clone());
    let data = amount;

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1) = topics;
    let topics = (t0, t1, client.name());

    client.mint(&to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}

#[test]
fn test_clawback() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let amount = 123;

    let event = Clawback {
        from: from.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("clawback"), from.clone());
    let data = amount;

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    asset
        .issuer()
        .set_flag(xdr::AccountFlags::ClawbackEnabledFlag);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1) = topics;
    let topics = (t0, t1, client.name());

    client.mint(&from, &amount);
    client.clawback(&from, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}
