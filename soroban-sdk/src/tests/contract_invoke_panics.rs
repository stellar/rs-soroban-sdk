use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl,
    xdr::{ScErrorCode, ScErrorType},
    Address, Env, Error,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn panic() -> i32 {
        panic!("I panicked")
    }

    pub fn need_auth(user: Address) {
        user.require_auth();
    }
}

#[test]
fn test_client_restores_auth_after_panic() {
    use soroban_sdk::testutils::Address as _;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    assert_eq!(
        client.try_need_auth(&user),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );

    let failure = catch_unwind(AssertUnwindSafe(|| {
        client.mock_all_auths().panic();
    }));
    assert!(failure.is_err());

    assert_eq!(
        client.try_need_auth(&user),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        ))),
        "unmocked call succeeded after the mocked client panicked"
    );
}

#[test]
fn test_client_restores_previous_auth_after_panic() {
    use soroban_sdk::testutils::Address as _;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    let failure = catch_unwind(AssertUnwindSafe(|| {
        client.set_auths(&[]).panic();
    }));
    assert!(failure.is_err());

    assert!(
        client.try_need_auth(&user).is_ok(),
        "env mocked auth was not restored after the client panicked"
    );
}

#[test]
fn test_client_restores_auth_after_success_and_try_error() {
    use soroban_sdk::testutils::Address as _;

    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    assert_eq!(
        client.try_need_auth(&user),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );
    client.mock_all_auths().need_auth(&user);
    assert_eq!(
        client.try_need_auth(&user),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );
    assert!(client.mock_all_auths().try_need_auth(&user).is_ok());
    assert_eq!(
        client.try_need_auth(&user),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );
    assert_eq!(
        client.mock_all_auths().try_panic(),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );
    assert_eq!(
        client.try_need_auth(&user),
        Err(Ok(Error::from_type_and_code(
            ScErrorType::Context,
            ScErrorCode::InvalidAction
        )))
    );
}
