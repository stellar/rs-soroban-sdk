use crate as soroban_sdk;
use soroban_sdk::{contractimpl, xdr::ScStatusType, Env, Status};
use soroban_sdk_macros::contracterror;

pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Zero = 1,
}

#[contractimpl]
impl Contract {
    pub fn assert(env: Env, value: u32) -> u32 {
        assert_with_error!(&env, value > 0, Error::Zero);
        value
    }
}

#[test]
#[should_panic(expected = "Status(ContractError(1)")]
fn test_invoke_expect_status() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);

    ContractClient::new(&e, &contract_id).assert(&0);
}

#[test]
fn test_try_invoke() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);

    let res = ContractClient::new(&e, &contract_id).try_assert(&0);
    assert_eq!(
        res,
        Err(Ok(Status::from_type_and_code(
            ScStatusType::ContractError,
            1,
        )))
    );
}
