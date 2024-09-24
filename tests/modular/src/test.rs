#![cfg(test)]

use crate::{Contract, ContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();

    let id = env.register(Contract, ());
    let client = ContractClient::new(&env, &id);

    assert_eq!(client.zero(), 0);
    assert_eq!(client.one(), 1);
    assert_eq!(client.two(), 2);
}
