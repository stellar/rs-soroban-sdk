//! Check that the proptest strategies generate prototypes that can be
//! converted to their contract types.
#![cfg(feature = "testutils-proptest")]

use crate::{self as soroban_sdk};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use soroban_sdk::{
    contracttype, testutils::arbitrary::SorobanArbitrary, Address, Bytes, BytesN, Env, IntoVal,
    Map, MuxedAddress, String, Symbol, Val, Vec,
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
    /// Prototypes implement proptest's `Arbitrary`, so they can be named
    /// directly in the parameter list.
    #[test]
    fn test_prototype_in_parameter_list(
        address: <Address as SorobanArbitrary>::Prototype,
        addresses: <Vec<Address> as SorobanArbitrary>::Prototype,
        bytes: <Bytes as SorobanArbitrary>::Prototype,
        val: <Val as SorobanArbitrary>::Prototype,
        deposit: <Deposit as SorobanArbitrary>::Prototype,
        action: <Action as SorobanArbitrary>::Prototype,
    ) {
        let env = Env::default();
        let _address: Address = address.into_val(&env);
        let _addresses: Vec<Address> = addresses.into_val(&env);
        let _bytes: Bytes = bytes.into_val(&env);
        let _val: Val = val.into_val(&env);
        let _deposit: Deposit = deposit.into_val(&env);
        let _action: Action = action.into_val(&env);
    }

    #[test]
    fn test_address(address: <Address as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _address: Address = address.into_val(&env);
    }

    #[test]
    fn test_addresses(addresses: <Vec<Address> as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _addresses: Vec<Address> = addresses.into_val(&env);
    }

    #[test]
    fn test_muxed_address(address: <MuxedAddress as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _address: MuxedAddress = address.into_val(&env);
    }

    #[test]
    fn test_val(val: <Val as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _val: Val = val.into_val(&env);
    }

    #[test]
    fn test_map(map: <Map<Symbol, Val> as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _map: Map<Symbol, Val> = map.into_val(&env);
    }

    #[test]
    fn test_bytes(bytes: <Bytes as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _bytes: Bytes = bytes.into_val(&env);
    }

    #[test]
    fn test_string(string: <String as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _string: String = string.into_val(&env);
    }

    #[test]
    fn test_deposit(deposit: <Deposit as SorobanArbitrary>::Prototype) {
        let env = Env::default();
        let _deposit: Deposit = deposit.into_val(&env);
    }

    #[test]
    fn test_action(action: <Action as SorobanArbitrary>::Prototype) {
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

    for proto in samples(any::<<Address as SorobanArbitrary>::Prototype>(), 100) {
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
    for proto in samples(any::<<MuxedAddress as SorobanArbitrary>::Prototype>(), 100) {
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

#[contracttype]
pub struct Keys {
    pub a: BytesN<32>,
    pub b: BytesN<32>,
    pub c: BytesN<32>,
    pub d: BytesN<32>,
    pub e: BytesN<32>,
    pub f: BytesN<32>,
    pub g: BytesN<32>,
    pub h: BytesN<32>,
    pub i: BytesN<32>,
    pub j: BytesN<32>,
    pub k: BytesN<32>,
    pub l: BytesN<32>,
}

/// The entropy budget comes from the prototype's `arbitrary` size hint, so a
/// prototype larger than the budget given to the unbounded types is generated
/// in full rather than with a zeroed tail. `Keys` needs 384 bytes for its
/// twelve 32-byte fields.
#[test]
fn test_entropy_budget_covers_large_prototype() {
    let env = Env::default();

    let last_field_all_zero = samples(any::<<Keys as SorobanArbitrary>::Prototype>(), 20)
        .into_iter()
        .all(|proto| {
            let keys: Keys = proto.into_val(&env);
            keys.l == BytesN::from_array(&env, &[0u8; 32])
        });

    assert!(
        !last_field_all_zero,
        "the last field was zero in every sample, so the entropy budget ran out"
    );
}

/// A budget too small for the value truncates it: `arbitrary` zero-fills once
/// the bytes run out, so a 64 byte budget cuts a vec of 32 byte addresses short
/// where the default budget does not.
#[test]
fn test_entropy_budget_bounds_collection_length() {
    let env = Env::default();

    fn longest<S>(env: &Env, strategy: S) -> u32
    where
        S: Strategy<Value = <Vec<Address> as SorobanArbitrary>::Prototype>,
    {
        samples(strategy, 50)
            .into_iter()
            .map(|proto| {
                let v: Vec<Address> = proto.into_val(env);
                v.len()
            })
            .max()
            .unwrap()
    }

    let truncated = longest(
        &env,
        proptest_arbitrary_interop::arb_sized::<<Vec<Address> as SorobanArbitrary>::Prototype>(64),
    );
    let default = longest(&env, any::<<Vec<Address> as SorobanArbitrary>::Prototype>());

    assert!(
        default > truncated,
        "expected the default budget to outrun a 64 byte one, got {default} and {truncated}"
    );
}

proptest! {
    /// The prototype of a tuple is an `ArbitraryTupleN` struct rather than a
    /// tuple of prototypes, so it needs its own `Arbitrary` implementation and
    /// is not covered by proptest's tuple implementations.
    #[test]
    fn test_tuple_prototype_in_parameter_list(
        pair: <(u32, Address) as SorobanArbitrary>::Prototype,
        nested: <(Address, Vec<Address>, i128) as SorobanArbitrary>::Prototype,
    ) {
        let env = Env::default();
        let _pair: (u32, Address) = pair.into_val(&env);
        let _nested: (Address, Vec<Address>, i128) = nested.into_val(&env);
    }
}

/// A collection is given entropy sized for its elements, so a collection of a
/// large prototype is not cut short. `Keys` needs 384 bytes, and with a budget
/// sized for a small element the second element onwards runs out of bytes part
/// way through, which `arbitrary` fills with zeros.
#[test]
fn test_collection_budget_covers_large_elements() {
    let env = Env::default();
    let zero = BytesN::from_array(&env, &[0u8; 32]);

    let mut sampled_long_enough = 0;
    let mut truncated = 0;
    for proto in samples(any::<<Vec<Keys> as SorobanArbitrary>::Prototype>(), 100) {
        let v: Vec<Keys> = proto.into_val(&env);
        if v.len() < 2 {
            continue;
        }
        sampled_long_enough += 1;
        let last = v.get(v.len() - 1).unwrap();
        if last.l == zero {
            truncated += 1;
        }
    }

    assert!(
        sampled_long_enough > 0,
        "no sample had 2 or more elements, so the assertion below proves nothing"
    );
    assert_eq!(
        truncated, 0,
        "the last field was zero in {truncated} of {sampled_long_enough} samples, \
         so the entropy budget ran out part way through the collection"
    );
}
