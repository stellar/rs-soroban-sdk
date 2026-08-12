//! Check that the `arb` proptest strategy generates values that can be
//! converted to their contract types.
#![cfg(feature = "testutils-proptest")]

use crate::testutils::arbitrary::arb;
use crate::{self as soroban_sdk};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use soroban_sdk::{contracttype, Address, Env, IntoVal, MuxedAddress, Vec};

#[contracttype]
pub struct Deposit {
    pub address: Address,
    pub amount: i128,
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
    fn test_deposit(deposit in arb::<Deposit>()) {
        let env = Env::default();
        let _deposit: Deposit = deposit.into_val(&env);
    }
}

/// Addresses are generated as both account addresses, which have a strkey
/// beginning with "G", and contract addresses, which have a strkey beginning
/// with "C".
#[test]
fn test_address_variants() {
    let env = Env::default();
    let mut runner = proptest::test_runner::TestRunner::deterministic();
    let strategy = arb::<Address>();

    let mut accounts = 0;
    let mut contracts = 0;
    for _ in 0..100 {
        let proto = strategy.new_tree(&mut runner).unwrap().current();
        let address: Address = proto.into_val(&env);
        match address.to_string().to_string().chars().next().unwrap() {
            'G' => accounts += 1,
            'C' => contracts += 1,
            c => panic!("unexpected address strkey prefix: {c}"),
        }
    }

    assert!(accounts > 0, "no account addresses generated");
    assert!(contracts > 0, "no contract addresses generated");
}
