#![no_std]
use soroban_sdk::{contracterror, contractimpl, panic_error, symbol, Env, Status, Symbol};

pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AnError = 1,
}

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, flag: u32) -> Result<Symbol, Error> {
        env.data().set(symbol!("persisted"), true);
        if flag == 0 {
            Ok(symbol!("hello"))
        } else if flag == 1 {
            Err(Error::AnError)
        } else if flag == 2 {
            panic_error!(&env, Error::AnError)
        } else if flag == 3 {
            panic!("an error")
        } else {
            unimplemented!()
        }
    }

    #[cfg(test)]
    pub fn persisted(env: Env) -> bool {
        env.data()
            .get(symbol!("persisted"))
            .unwrap_or_else(|| Ok(false))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{
        symbol,
        xdr::{ScStatus, ScVmErrorCode},
        BytesN, Env, Status,
    };

    use crate::{Contract, ContractClient, Error};

    #[test]
    fn hello_ok() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.hello(&0);
        assert_eq!(res, symbol!("hello"));
        assert!(client.persisted());
    }

    #[test]
    // TODO: Remove the should_panic when this issue is fixed:
    // https://github.com/stellar/rs-soroban-sdk/issues/642
    #[should_panic(expected = "Status(ContractError(1))")]
    fn hello_error() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let expected_status = Status::from_contract_error(Error::AnError as u32);
        e.assert_panic_with_status(expected_status, |_| {
            let _ = client.hello(&1);
        });
        assert!(!client.persisted());
    }

    #[test]
    // TODO: Remove the should_panic when this issue is fixed:
    // https://github.com/stellar/rs-soroban-sdk/issues/642
    #[should_panic(expected = "Status(ContractError(1))")]
    fn hello_error_panic() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let expected_status = Status::from_contract_error(Error::AnError as u32);
        e.assert_panic_with_status(expected_status, |_| {
            let _ = client.hello(&2);
        });
        assert!(!client.persisted());
    }

    #[test]
    fn hello_error_panic_string() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let expected_string = "an error";
        e.assert_panic_with_string(expected_string, |_| {
            let _ = client.hello(&3);
        });
        // TODO: Remove this when
        // https://github.com/stellar/rs-soroban-env/issues/462 is resolved.
        // assert!(!client.persisted());
    }

    #[test]
    fn try_hello_ok() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&0);
        assert_eq!(res, Ok(Ok(symbol!("hello"))));
        assert!(client.persisted());
    }

    #[test]
    fn try_hello_error() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&1);
        assert_eq!(res, Err(Ok(Error::AnError)));
        assert!(!client.persisted());
    }

    #[test]
    // TODO: Remove the should_panic when this issue is fixed:
    // https://github.com/stellar/rs-soroban-sdk/issues/642
    #[should_panic(expected = "Status(ContractError(1))")]
    fn try_hello_error_panic() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&2);
        assert_eq!(res, Err(Ok(Error::AnError)));
        assert!(!client.persisted());
    }

    #[test]
    // TODO: Remove the should_panic when this issue is fixed:
    // https://github.com/stellar/rs-soroban-env/issues/430
    #[should_panic(expected = "an error")]
    fn try_hello_error_panic_string() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&3);
        assert_eq!(
            res,
            Err(Err(Status::from_status(ScStatus::VmError(
                ScVmErrorCode::TrapUnreachable
            ),)))
        );
        assert!(!client.persisted());
    }
}
