//! Check what happens when a test contract is registered at the address of a
//! Stellar Asset Contract.
//!
//! Which of the two the host dispatches to depends on the order it checks the
//! test contract registry against the other executable types. That order is an
//! implementation detail of the host, not something the SDK specifies, so this
//! test exists to notice if it changes rather than to require the behavior.
//!
//! See https://github.com/stellar/rs-soroban-env/pull/1720#discussion_r3780106275.

use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl, testutils::Address as _, token::StellarAssetClient, Address, Env,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello() -> u32 {
        1234
    }
}

/// A test contract registered at the address of a Stellar Asset Contract
/// replaces it. Calls to the address dispatch to the test contract, and the
/// asset contract's own functions are no longer reachable there.
#[test]
fn test_register_over_stellar_asset_contract() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let address = sac.address();

    // The asset contract answers at its address, before anything is registered
    // over it.
    let asset_client = StellarAssetClient::new(&env, &address);
    assert_eq!(asset_client.decimals(), 7);

    env.register_at(&address, Contract, ());

    // The test contract answers at the address now.
    let client = ContractClient::new(&env, &address);
    assert_eq!(client.hello(), 1234);

    // And the asset contract does not. The error is a Context InvalidAction,
    // because the test contract that the call dispatches to has no function of
    // that name, not because the address has nothing at it.
    assert!(asset_client.try_decimals().is_err());
}
