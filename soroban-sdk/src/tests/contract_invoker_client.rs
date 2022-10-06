use crate as soroban_sdk;
use soroban_sdk::{contractimpl, testutils::Accounts, Address, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn whoami(e: Env) -> Address {
        e.invoker()
    }
}

extern crate std;

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let default = e.source_account();
    std::println!("default: {:?}", default);

    let invoker_1 = e.accounts().generate();
    std::println!("invoker 1: {:?}", invoker_1);

    let invoker_2 = e.accounts().generate();
    std::println!("invoker 2: {:?}", invoker_2);

    assert_ne!(&invoker_1, &invoker_2);
    assert_ne!(&invoker_1, &e.source_account());
    assert_ne!(&invoker_2, &e.source_account());

    let result_1a = client.with_source_account(&invoker_1).whoami();
    std::println!("result 1a: {:?}", result_1a);

    let result_1b = client.with_source_account(&invoker_1).whoami();
    std::println!("result 1b: {:?}", result_1b);

    assert_eq!(result_1a, result_1b);
    assert_eq!(result_1a, Address::Account(invoker_1.clone()));
    assert_eq!(result_1b, Address::Account(invoker_1.clone()));

    let result_default = client.whoami();
    std::println!("result default: {:?}", result_default);
    assert_ne!(result_default, result_1a);
    assert_ne!(result_default, result_1b);
    assert_eq!(result_default, Address::Account(default.clone()));

    assert_eq!(result_1a, result_1b);
    assert_eq!(result_1a, Address::Account(invoker_1.clone()));
    assert_eq!(result_1b, Address::Account(invoker_1.clone()));

    let result_2 = client.with_source_account(&invoker_2).whoami();
    std::println!("result 2: {:?}", result_2);
    assert_ne!(result_1a, result_2);
    assert_ne!(result_1b, result_2);
    assert_eq!(result_2, Address::Account(invoker_2.clone()));
}
