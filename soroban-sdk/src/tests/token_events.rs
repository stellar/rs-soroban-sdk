use crate::{self as soroban_sdk, IntoVal};

use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _},
    token::{Approve, Transfer, TransferMuxed, Burn, Mint, Clawback},
    vec, Address, Env, Event, Map, Symbol, Val,
};

#[contract]
struct Contract;

#[test]
fn test_approve() {
    let env = Env::default();
    let id = env.register(Contract, ());
    let event = Approve {
        from: Address::generate(&env),
        spender: Address::generate(&env),
        amount: 123,
        expiration_ledger: 45,
    };
    env.as_contract(&id, || event.publish(&env));
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id.clone(),
                (
                    symbol_short!("approve"),
                    event.from.clone(),
                    event.spender.clone(),
                )
                    .into_val(&env),
                (123i128, 45u32,).into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let id = env.register(Contract, ());
    let event = Transfer {
        from: Address::generate(&env),
        to: Address::generate(&env),
        amount: 123,
    };
    env.as_contract(&id, || event.publish(&env));
    assert_eq!(
        env.events().all(),
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
                123i128.into_val(&env),
            ),
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
    assert_eq!(
        env.events().all(),
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
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id.clone(),
                (
                    symbol_short!("burn"),
                    event.from.clone(),
                )
                .into_val(&env),
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
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id.clone(),
                (
                    symbol_short!("mint"),
                    event.to.clone(),
                )
                .into_val(&env),
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
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id.clone(),
                (
                    symbol_short!("clawback"),
                    event.from.clone(),
                )
                .into_val(&env),
                123i128.into_val(&env),
            ),
        ]
    );
}
