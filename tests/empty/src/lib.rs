#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};
use soroban_token_sdk::TokenUtils as _;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn empty(env: &Env, a: Address) {
        soroban_token_sdk::events::TransferWithAmountOnly {
            to: a.clone(),
            from: a,
            amount: 1,
        }
        .publish(env);
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{testutils::Address as _, Address, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let a = Address::generate(&e);
        client.empty(&a);
    }
}
