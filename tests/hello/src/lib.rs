#![no_std]
use soroban_sdk::{contractimpl, symbol, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello() -> Symbol {
        symbol!("hello")
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{symbol, BytesN, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let h = client.hello();
        assert!(h == symbol!("hello"));
    }
}
