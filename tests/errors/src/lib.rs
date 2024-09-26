#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, symbol_short, Env, Symbol,
};

#[contract]
pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AnError = 1,
}

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, flag: u32) -> Result<Symbol, Error> {
        env.storage()
            .persistent()
            .set(&symbol_short!("persisted"), &true);
        if flag == 0 {
            Ok(symbol_short!("hello"))
        } else if flag == 1 {
            Err(Error::AnError)
        } else if flag == 2 {
            panic_with_error!(&env, Error::AnError)
        } else if flag == 3 {
            panic!("an error")
        } else if flag == 4 {
            panic_with_error!(&env, soroban_sdk::Error::from_contract_error(9))
        } else {
            unimplemented!()
        }
    }

    #[cfg(test)]
    pub fn persisted(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&symbol_short!("persisted"))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{symbol_short, xdr, Env, InvokeError};

    use crate::{Contract, ContractClient, Error};

    #[test]
    fn hello_ok() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.hello(&0);
        assert_eq!(res, symbol_short!("hello"));
        assert!(client.persisted());
    }

    #[test]
    fn try_hello_ok() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&0);
        assert_eq!(res, Ok(Ok(symbol_short!("hello"))));
        assert!(client.persisted());
    }

    #[test]
    fn try_hello_error() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&1);
        assert_eq!(res, Err(Ok(Error::AnError)));
        assert!(!client.persisted());
    }

    #[test]
    fn try_hello_error_panic() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&2);
        assert_eq!(res, Err(Ok(Error::AnError)));
        assert!(!client.persisted());
    }

    #[test]
    fn try_hello_error_panic_string() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&3);
        assert_eq!(res, Err(Err(InvokeError::Abort)));
        assert!(!client.persisted());
    }

    #[test]
    fn try_hello_error_unexpected_contract_error() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let res = client.try_hello(&4);
        assert_eq!(res, Err(Err(InvokeError::Contract(9))));
        assert!(!client.persisted());
    }

    #[test]
    fn type_conversion() {
        // Error can be converted into InvokeError.
        assert_eq!(
            <_ as Into<InvokeError>>::into(Error::AnError),
            InvokeError::Contract(1),
        );

        // InvokeError can be converted into Error or itself.
        assert_eq!(
            <_ as TryInto<Error>>::try_into(InvokeError::Contract(1)),
            Ok(Error::AnError),
        );
        assert_eq!(
            <_ as TryInto<Error>>::try_into(InvokeError::Contract(2)),
            Err(InvokeError::Contract(2)),
        );
        assert_eq!(
            <_ as TryInto<Error>>::try_into(InvokeError::Abort),
            Err(InvokeError::Abort),
        );

        // Error can be converted into Env Error.
        assert_eq!(
            <_ as Into<soroban_sdk::Error>>::into(Error::AnError),
            soroban_sdk::Error::from_contract_error(1),
        );

        // Env Error can be converted into Error.
        assert_eq!(
            <_ as TryInto<Error>>::try_into(soroban_sdk::Error::from_contract_error(1)),
            Ok(Error::AnError),
        );
        assert_eq!(
            <_ as TryInto<Error>>::try_into(soroban_sdk::Error::from_contract_error(2)),
            Err(soroban_sdk::Error::from_contract_error(2)),
        );
        assert_eq!(
            <_ as TryInto<Error>>::try_into(soroban_sdk::Error::from_type_and_code(
                xdr::ScErrorType::Context,
                xdr::ScErrorCode::InvalidAction
            )),
            Err(soroban_sdk::Error::from_type_and_code(
                xdr::ScErrorType::Context,
                xdr::ScErrorCode::InvalidAction
            )),
        );

        // Env Error can be converted into InvokeError.
        assert_eq!(
            <_ as Into<InvokeError>>::into(soroban_sdk::Error::from_contract_error(1)),
            InvokeError::Contract(1),
        );
        assert_eq!(
            <_ as Into<InvokeError>>::into(soroban_sdk::Error::from_type_and_code(
                xdr::ScErrorType::Context,
                xdr::ScErrorCode::InvalidAction
            )),
            InvokeError::Abort,
        );
    }
}
