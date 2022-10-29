use crate as soroban_sdk;
use soroban_sdk::{contractimpl, xdr::ScStatusType, Env, Status};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn panic() -> i32 {
        panic!("I panicked")
    }
}

#[test]
#[should_panic(expected = "I panicked")]
fn test_invoke_expect_string() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);

    ContractClient::new(&e, &contract_id).panic();
}

#[test]
#[should_panic(expected = "Status(UnknownError(0)")]
fn test_invoke_expect_status() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);

    ContractClient::new(&e, &contract_id).panic();
}

#[test]
fn test_try_invoke() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);

    let res = ContractClient::new(&e, &contract_id).try_panic();
    assert_eq!(
        res,
        Err(Ok(Status::from_type_and_code(
            ScStatusType::UnknownError,
            0,
        )))
    );
}
