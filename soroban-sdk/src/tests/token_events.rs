use crate::{self as soroban_sdk, token::StellarAssetClient, IntoVal};

use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _},
    token::{Approve, Burn, Clawback, Mint, Transfer, TransferMuxed},
    vec, Address, Env, Event, Map, Symbol, Val,
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
    let id = env.register(Contract, ());
    let event = TransferMuxed {
        from: Address::generate(&env),
        to: Address::generate(&env),
        to_muxed_id: 45,
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
                (
                    symbol_short!("transfer"),
                    event.from.clone(),
                    event.to.clone(),
                )
                    .into_val(&env),
                Map::<Symbol, Val>::from_array(
                    &env,
                    [
                        (Symbol::new(&env, "to_muxed_id"), 45u32.into_val(&env)),
                        (Symbol::new(&env, "amount"), 123i128.into_val(&env)),
                    ],
                )
                .into(),
            ),
        ]
    );
}

#[test]
fn test_burn() {
    let env = Env::default();
    let id = env.register(Contract, ());
    let event = Burn {
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
                (symbol_short!("burn"), event.from.clone(),).into_val(&env),
                123i128.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    let id = env.register(Contract, ());
    let event = Mint {
        to: Address::generate(&env),
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
                (symbol_short!("mint"), event.to.clone(),).into_val(&env),
                123i128.into_val(&env),
            ),
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
