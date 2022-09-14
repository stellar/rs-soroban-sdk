#![no_std]
use soroban_sdk::{contractimpl, contracttype, symbol, Symbol};

pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AnError,
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
    use soroban_sdk::{symbol, BytesN, Env};

    use crate::{Contract, ContractClient, Error};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let res = client.hello(&0);
        assert_eq!(res, Ok(symbol!("hello")));

        let res = client.hello(&1);
        assert_eq!(res, Err(Error::AnError));
    }
}
