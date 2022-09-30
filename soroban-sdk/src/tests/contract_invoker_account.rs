use crate as soroban_sdk;
use soroban_sdk::{contractimpl, Account, BytesN, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn exists(env: Env) -> bool {
        let a = match env.invoker() {
            soroban_sdk::Invoker::Account(a) => a,
            _ => panic!("must be invoked by account"),
        };
        Account::exists(&a)
    }
}

#[test]
fn test_hello() {
    let e = Env::default();
    let contract_id = BytesN::from_array(&e, &[0; 32]);
    e.register_contract(&contract_id, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let exists = client.exists();

    // TODO: This test is somewhat odd in that the account invoking the
    // contract would always exist in the real world, but doesn't exist in
    // tests. We should find a way to align the test behavior with real
    // world behavior.
    assert!(!exists);
}
