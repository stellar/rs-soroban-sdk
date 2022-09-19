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
    fn test_hello() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.hello(&0);
        assert_eq!(res, symbol!("hello"));
    }

    #[test]
    #[should_panic]
    fn test_hello_uncaught_error() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let _ = client.hello(&1);
        // TODO:
        // e.assert_panic_with_status(Status::from_contract_error(Error::AnError as u32), |e| {
        //     let _ = client.hello(&1);
        // })
    }

    #[test]
    fn test_hello_error_handling_ok() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&0);
        assert_eq!(res, Ok(Ok(symbol!("hello"))));
    }

    #[test]
    fn test_hello_error_handling_err() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&0);
        assert_eq!(res, Err(Ok(Error::AnError)));
    }

    // let res = client.hello(&0);
    // assert_eq!(res, Ok(symbol!("hello")));

    // let res = client.hello(&1);
    // assert_eq!(res, Err(Error::AnError.into()));
}
