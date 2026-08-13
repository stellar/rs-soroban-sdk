//! Check that the `arb` proptest strategy generates values that can be
//! converted to their contract types.
#![cfg(feature = "testutils-proptest")]

use crate::{self as soroban_sdk};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use soroban_sdk::{
    contracttype, testutils::proptest::arb, Address, Bytes, Env, IntoVal, Map, MuxedAddress,
    String, Symbol, Val, Vec,
};

#[contracttype]
pub struct Deposit {
    pub address: Address,
    pub amount: i128,
}

#[contracttype]
pub enum Action {
    Noop,
    Deposit(Deposit),
    Transfer(Address, i128),
}

proptest! {
    #[test]
    fn test_address(address in arb::<Address>()) {
        let env = Env::default();
        let _address: Address = address.into_val(&env);
    }

    #[test]
    fn test_addresses(addresses in arb::<Vec<Address>>()) {
        let env = Env::default();
        let _addresses: Vec<Address> = addresses.into_val(&env);
    }

    #[test]
    fn test_muxed_address(address in arb::<MuxedAddress>()) {
        let env = Env::default();
        let _address: MuxedAddress = address.into_val(&env);
    }

    #[test]
    fn test_val(val in arb::<Val>()) {
        let env = Env::default();
        let _val: Val = val.into_val(&env);
    }

    #[test]
    fn test_map(map in arb::<Map<Symbol, Val>>()) {
        let env = Env::default();
        let _map: Map<Symbol, Val> = map.into_val(&env);
    }

    #[test]
    fn test_bytes(bytes in arb::<Bytes>()) {
        let env = Env::default();
        let _bytes: Bytes = bytes.into_val(&env);
    }

    #[test]
    fn test_string(string in arb::<String>()) {
        let env = Env::default();
        let _string: String = string.into_val(&env);
    }

    #[test]
    fn test_deposit(deposit in arb::<Deposit>()) {
        let env = Env::default();
        let _deposit: Deposit = deposit.into_val(&env);
    }

    #[test]
    fn test_action(action in arb::<Action>()) {
        let env = Env::default();
        let _action: Action = action.into_val(&env);
    }
}

/// Generate values from a strategy, deterministically, for tests that assert
/// on the kinds of values generated.
fn samples<T: Strategy>(strategy: T, count: usize) -> std::vec::Vec<T::Value> {
    let mut runner = proptest::test_runner::TestRunner::deterministic();
    (0..count)
        .map(|_| strategy.new_tree(&mut runner).unwrap().current())
        .collect()
}

/// Addresses are generated as contract addresses, which have a strkey
/// beginning with "C", and never as account addresses.
#[test]
fn test_addresses_are_contracts() {
    let env = Env::default();

    for proto in samples(arb::<Address>(), 100) {
        let address: Address = proto.into_val(&env);
        let strkey = address.to_string().to_string();
        assert!(
            strkey.starts_with('C'),
            "expected a contract address, got {strkey}"
        );
    }
}

/// Muxed addresses are generated as contract addresses ("C") and muxed account
/// addresses ("M").
#[test]
fn test_muxed_address_variants() {
    let env = Env::default();

    let mut contracts = 0;
    let mut muxed = 0;
    for proto in samples(arb::<MuxedAddress>(), 100) {
        let address: MuxedAddress = proto.into_val(&env);
        match address.to_strkey().to_string().chars().next().unwrap() {
            'C' => contracts += 1,
            'M' => muxed += 1,
            c => panic!("unexpected address strkey prefix: {c}"),
        }
    }

    assert!(contracts > 0, "no contract addresses generated");
    assert!(muxed > 0, "no muxed addresses generated");
}
