use crate::{self as soroban_sdk, testutils::Accounts};
use soroban_sdk::IntoVal;
use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn deposit(env: Env) {}
}

#[test]
fn test_hello() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    // Create account, that has 10,000.0 native asset.
    let a = e.accounts().generate_and_create();
    e.deployer().with_current_contract([0u8; 32].into_val(&e)).deploy_token();
    // e.invoke_contract(contract_id, func, args)

    client.with_source_account(&a).deposit();
}
