#![no_std]
use soroban_sdk::{contract, contractimpl, contracttrait};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn empty() {}
}

#[contractimpl]
impl Contract {
    pub fn empty2() {}
}

#[contracttrait]
trait Trait {
    fn empty3() {}
}

#[contractimpl]
impl Trait for Contract {
    fn empty3() {}
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.empty();
        client.empty2();
        client.empty3();
    }
}
