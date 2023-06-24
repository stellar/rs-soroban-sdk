#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

extern crate alloc;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn num_list(env: Env, count: u32) -> soroban_sdk::Vec<u32> {
        let mut v1 = alloc::vec![];
        (0..count).for_each(|i| v1.push(i));

        let mut v2 = soroban_sdk::vec![&env];
        for i in v1 {
            v2.push_back(i);
        }

        v2
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{vec, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let list = client.num_list(&5);
        assert_eq!(list, vec![&e, 0, 1, 2, 3, 4])
    }
}
