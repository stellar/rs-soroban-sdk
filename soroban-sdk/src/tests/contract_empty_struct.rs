use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct ContractBraces {}

#[contractimpl]
impl ContractBraces {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

#[test]
fn test_braces() {
    let e = Env::default();
    let id = e.register(ContractBraces, ());
    let client = ContractBracesClient::new(&e, &id);
    assert_eq!(client.add(&1, &2), 3);
}
