extern crate std;

use crate::{
    Approve, Burn, Clawback, Mint, MintWithAmountOnly, SetAdmin, SetAuthorized, Transfer,
    TransferWithAmountOnly,
};
use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _, MuxedAddress as _},
    token::StellarAssetClient,
    vec, xdr, Address, Env, IntoVal, Map, MuxedAddress, Symbol, Val,
};
use std::rc::Rc;

#[contract]
struct Contract;

#[test]
fn test_approve() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let spender = Address::generate(&env);
    let amount = 123;
    let live_until_ledger = 45;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = Approve {
        from: from.clone(),
        spender: spender.clone(),
        sep0011_asset: sep0011_asset.clone(),
        amount,
        expiration_ledger: live_until_ledger,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        symbol_short!("approve"),
        from.clone(),
        spender.clone(),
        sep0011_asset.clone(),
    );
    let data = (amount, live_until_ledger);

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
    client.approve(&from, &spender, &amount, &live_until_ledger);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("approve"), from, spender, sep0011_asset,).into_val(&env),
                data.into_val(&env),
            ),
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

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = TransferWithAmountOnly {
        from: from.clone(),
        to: to.clone(),
        sep0011_asset: sep0011_asset.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        symbol_short!("transfer"),
        from.clone(),
        to.clone(),
        sep0011_asset.clone(),
    );
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
    client.mint(&from, &amount);
    client.transfer(&from, &to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("transfer"), from, to, sep0011_asset).into_val(&env),
                data.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_transfer_with_id() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let to = MuxedAddress::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = Transfer {
        from: from.clone(),
        to: to.address(),
        sep0011_asset: sep0011_asset.clone(),
        to_muxed_id: to.id(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        symbol_short!("transfer"),
        from.clone(),
        to.address(),
        sep0011_asset.clone(),
    );
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
    let trust_line_asset = |asset: xdr::Asset| match asset {
        xdr::Asset::Native => xdr::TrustLineAsset::Native,
        xdr::Asset::CreditAlphanum4(a) => xdr::TrustLineAsset::CreditAlphanum4(a),
        xdr::Asset::CreditAlphanum12(a) => xdr::TrustLineAsset::CreditAlphanum12(a),
    };

    client.mint(&from, &amount);
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

    client.transfer(&from, &to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("transfer"), from, to.address(), sep0011_asset).into_val(&env),
                data.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_transfer_from() {
    let env = Env::default();
    env.mock_all_auths();

    let spender = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    // Verify the event published is consistent with the asset contract.
    // transfer_from emits a transfer event identical to transfer.
    client.mint(&from, &amount);
    client.approve(&from, &spender, &amount, &200);
    client.transfer_from(&spender, &from, &to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("transfer"), from, to, sep0011_asset).into_val(&env),
                amount.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = Burn {
        from: from.clone(),
        sep0011_asset: sep0011_asset.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("burn"), from.clone(), sep0011_asset.clone());
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
    client.mint(&from, &amount);
    client.burn(&from, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("burn"), from, sep0011_asset).into_val(&env),
                data.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_burn_from() {
    let env = Env::default();
    env.mock_all_auths();

    let spender = Address::generate(&env);
    let from = Address::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    // Verify the event published is consistent with the asset contract.
    // burn_from emits a burn event identical to burn.
    client.mint(&from, &amount);
    client.approve(&from, &spender, &amount, &200);
    client.burn_from(&spender, &from, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("burn"), from, sep0011_asset).into_val(&env),
                amount.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let to = Address::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = MintWithAmountOnly {
        to: to.clone(),
        sep0011_asset: sep0011_asset.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("mint"), to.clone(), sep0011_asset.clone());
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
    client.mint(&to, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("mint"), to, sep0011_asset).into_val(&env),
                data.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_mint_with_id() {
    let env = Env::default();
    env.mock_all_auths();

    let to = MuxedAddress::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = Mint {
        to: to.address(),
        sep0011_asset: sep0011_asset.clone(),
        to_muxed_id: to.id(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("mint"), to.address(), sep0011_asset);
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
}

#[test]
fn test_clawback() {
    let env = Env::default();
    env.mock_all_auths();

    let from = Address::generate(&env);
    let amount = 123;

    let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
    asset
        .issuer()
        .set_flag(xdr::AccountFlags::ClawbackEnabledFlag);
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = Clawback {
        from: from.clone(),
        sep0011_asset: sep0011_asset.clone(),
        amount,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        symbol_short!("clawback"),
        from.clone(),
        sep0011_asset.clone(),
    );
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
    client.mint(&from, &amount);
    client.clawback(&from, &amount);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("clawback"), from, sep0011_asset).into_val(&env),
                data.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_set_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);

    // Register the asset to get the SEP-0011 asset string it uses in events.
    let asset = env.register_stellar_asset_contract_v2(admin.clone());
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = SetAdmin {
        admin: admin.clone(),
        sep0011_asset: sep0011_asset.clone(),
        new_admin: new_admin.clone(),
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        symbol_short!("set_admin"),
        admin.clone(),
        sep0011_asset.clone(),
    );
    let data = new_admin.clone();

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
    client.set_admin(&new_admin);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("set_admin"), admin, sep0011_asset).into_val(&env),
                new_admin.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_set_authorized() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let authorizee = Address::generate(&env);
    let authorize = true;

    // Register the asset to get the SEP-0011 asset string it uses in events.
    let asset = env.register_stellar_asset_contract_v2(admin.clone());
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = SetAuthorized {
        id: authorizee.clone(),
        sep0011_asset: sep0011_asset.clone(),
        authorize,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        Symbol::new(&env, "set_authorized"),
        authorizee.clone(),
        sep0011_asset.clone(),
    );
    let data = authorize;

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
    client.set_authorized(&authorizee, &authorize);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (
                    Symbol::new(&env, "set_authorized"),
                    authorizee,
                    sep0011_asset
                )
                    .into_val(&env),
                authorize.into_val(&env),
            ),
        ]
    );
}
