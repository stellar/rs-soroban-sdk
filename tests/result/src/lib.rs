#![no_std]
use soroban_sdk::{contracterror, contractimpl, symbol, Status, Symbol};

pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AnError = 1,
}

#[contractimpl]
impl Contract {
    pub fn hello(flag: u32) -> Result<Symbol, Error> {
        if flag == 0 {
            Ok(symbol!("hello"))
        } else {
            Err(Error::AnError)
        }
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{symbol, BytesN, Env, Status};

    use crate::{Contract, ContractClient, Error};

    #[test]
    fn hello_ok() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.hello(&0);
        assert_eq!(res, symbol!("hello"));
    }

    #[test]
    // TODO: Remove the should_panic when this issue is fixed:
    // https://github.com/stellar/rs-soroban-sdk/issues/642
    #[should_panic("Status(ContractError(1))")]
    fn hello_error() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let expected_status = Status::from_contract_error(Error::AnError as u32);
        e.assert_panic_with_status(expected_status, |_| {
            let _ = client.hello(&1);
        })
    }

    #[test]
    fn try_hello_ok() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&0);
        assert_eq!(res, Ok(Ok(symbol!("hello"))));
    }

    #[test]
    fn try_hello_error() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&1);
        assert_eq!(res, Err(Ok(Error::AnError)));
    }
}
