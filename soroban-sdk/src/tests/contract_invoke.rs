use crate as soroban_sdk;
use soroban_sdk::{contractimpl, xdr::ScStatusType, BytesN, Env, Status};
use stellar_xdr::ScVmErrorCode;

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn panic() -> i32 {
        panic!("I panicked")
    }
}

#[test]
#[should_panic(expected = "I panicked")]
fn test_invoke() {
    let e = Env::default();
    let contract_id = BytesN::from_array(&e, &[0; 32]);
    e.register_contract(&contract_id, Contract);

    ContractClient::new(&e, &contract_id).panic();
}

#[test]
// TODO: Remove the should_panic when this issue is fixed:
// https://github.com/stellar/rs-soroban-env/issues/430.
#[should_panic(expected = "I panicked")]
fn test_try_invoke() {
    let e = Env::default();
    let contract_id = BytesN::from_array(&e, &[0; 32]);
    e.register_contract(&contract_id, Contract);

    let res = ContractClient::new(&e, &contract_id).try_panic();
    assert_eq!(
        res,
        Err(Ok(Status::from_type_and_code(
            ScStatusType::VmError,
            ScVmErrorCode::Function as u32,
        )))
    );
}
