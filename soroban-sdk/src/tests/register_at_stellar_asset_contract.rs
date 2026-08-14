use crate::{self as soroban_sdk, testutils::Address as _, token, Address, Env};
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello() -> u32 {
        1
    }

    pub fn decimals() -> u32 {
        99
    }
}

/// Registering a native contract at the address of a Stellar Asset Contract
/// replaces the asset contract, and calls to the address dispatch to the native
/// contract for both functions the asset contract has and functions it does
/// not.
#[test]
fn register_at_replaces_the_stellar_asset_contract() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let address = sac.address();

    // Before the native contract is registered, the asset contract answers.
    assert_eq!(token::TokenClient::new(&env, &address).decimals(), 7);

    env.register_at(&address, Contract, ());

    let client = ContractClient::new(&env, &address);
    assert_eq!(client.hello(), 1);
    assert_eq!(client.decimals(), 99);
}
