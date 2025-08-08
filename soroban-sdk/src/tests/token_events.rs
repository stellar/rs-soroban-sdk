use core::i64;
use std::rc::Rc;

use crate::{self as soroban_sdk};

use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _, MuxedAddress as _},
    token::StellarAssetClient,
    token::{Approve, Burn, Clawback, Mint, Transfer, TransferMuxed},
    vec, xdr, Address, Env, Event, IntoVal, Map, MuxedAddress, Symbol, Val,
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
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let amount = 123;

    let event = Transfer {
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
fn test_transfer_muxed() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let to = MuxedAddress::generate(&env);
    let amount = 123;

    let event = TransferMuxed {
        from: from.clone(),
        to: to.address(),
        to_muxed_id: to.id().unwrap(),
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

    client.mint(&from, &123);
    env.host()
        .add_ledger_entry(
            &Rc::new(xdr::LedgerKey::Trustline(xdr::LedgerKeyTrustLine {
                account_id: to.address().try_into().unwrap(),
                asset: asset.trust_line_asset(),
            })),
            &Rc::new(xdr::LedgerEntry {
                data: xdr::LedgerEntryData::Trustline(xdr::TrustLineEntry {
                    account_id: to.address().try_into().unwrap(),
                    asset: asset.trust_line_asset(),
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
    let id = env.register(Contract, ());
    let event = Clawback {
        from: Address::generate(&env),
        amount: 123,
    };
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (
                id.clone(),
                (symbol_short!("clawback"), event.from.clone(),).into_val(&env),
                123i128.into_val(&env),
            ),
        ]
    );
}
