use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contract_args, contract_client, contract_error, contract_event, contract_impl,
    contract_meta, contract_trait, contract_type, symbol_short, testutils::Address as _, Address,
    BytesN, Env, Symbol,
};

contract_meta!(key = "module", val = "snake_case_macros");

#[contract_error]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidParam = 1,
    Unauthorized = 2,
}

#[contract_type]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomData {
    pub id: u32,
    pub name: Symbol,
}

#[contract_type]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Pending = 0,
    Approved = 1,
}

#[contract_event]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationEvent {
    #[topic]
    pub caller: Address,
    pub code: u32,
}

#[contract_trait]
pub trait TraitContract {
    fn trait_fn(_env: Env) -> u32 {
        42
    }
}

#[contract_client(name = "ExternalClient")]
pub trait ExternalInterface {
    fn external_call(env: Env, value: u32) -> u32;
}

#[contract_args(name = "ExternalArgs")]
pub trait ExternalArgsInterface {
    fn args_call(env: Env, value: u32);
}

#[contract]
pub struct SnakeCaseContract;

#[contract_impl]
impl SnakeCaseContract {
    pub fn do_something(
        env: Env,
        caller: Address,
        data: CustomData,
        status: Status,
    ) -> Result<u32, Error> {
        if status == Status::Pending {
            return Err(Error::InvalidParam);
        }

        OperationEvent {
            caller,
            code: data.id,
        }
        .publish(&env);

        Ok(data.id + 10)
    }

    pub fn make_bytes(env: Env) -> BytesN<4> {
        bytes_n!(&env, [1, 2, 3, 4])
    }

    pub fn external_call(_env: Env, value: u32) -> u32 {
        value * 2
    }
}

#[contract_impl(contract_trait)]
impl TraitContract for SnakeCaseContract {
    fn trait_fn(_env: Env) -> u32 {
        100
    }
}

#[test]
fn test_snake_case_macros() {
    let env = Env::default();
    let contract_id = env.register(SnakeCaseContract, ());
    let client = SnakeCaseContractClient::new(&env, &contract_id);

    let caller = Address::generate(&env);
    let data = CustomData {
        id: 5,
        name: symbol_short!("test"),
    };

    // Error case
    let err = client.try_do_something(&caller, &data, &Status::Pending);
    assert_eq!(err, Err(Ok(Error::InvalidParam)));

    // Success case
    let res = client.do_something(&caller, &data, &Status::Approved);
    assert_eq!(res, 15);

    // Check trait fn implemented with contract_trait and contract_impl(contract_trait)
    assert_eq!(client.trait_fn(), 100);

    // Check bytes_n!
    let bytes = client.make_bytes();
    assert_eq!(bytes, bytes_n!(&env, [1, 2, 3, 4]));

    // Check external client generated via contract_client
    let ext_client = ExternalClient::new(&env, &contract_id);
    assert_eq!(ext_client.external_call(&21), 42);
}
