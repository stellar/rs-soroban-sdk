use crate as soroban_sdk;
use soroban_sdk::{contractimpl, Env};
use stellar_xdr::{ScErrorCode, ScErrorType};

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
#[should_panic(expected = "Error(Context, InternalError)")]
fn test_invoke_expect_error() {
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
        Err(Ok(soroban_sdk::Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InternalError
        )))
    );
}
