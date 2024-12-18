#![no_std]
use soroban_sdk::{contract, contractimpl};

use test_workspace_lib::Value;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn value() -> Value {
        Value { value: 13 }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_add() {
        let e = Env::default();

        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let z = client.value();
        assert_eq!(z, Value { value: 13 });
    }
}
