use crate as soroban_sdk;
use soroban_sdk::{contractimpl, testutils::budget::CostType, Bytes, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env) -> Bytes {
        let mut b = Bytes::from_array(&e, b"abcdefghijklmnopqrstuvwyxz");
        b.append(&Bytes::from_array(&e, b"0123456789"));
        b
    }
}

#[test]
fn test_budget() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    e.budget().reset();
    let b = client.add();
    e.budget().print();

    assert_eq!(e.budget().input(CostType::BytesAppend), 36);
    assert_eq!(
        b,
        Bytes::from_array(&e, b"abcdefghijklmnopqrstuvwyxz0123456789")
    );
}
