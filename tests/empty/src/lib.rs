#![no_std]
use soroban_sdk::{contractclient, contractimpl};

pub struct Contract;

#[contractimpl]
#[contractclient(name = "Client")]
impl Contract {
    pub fn empty() {}
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{Client, Contract};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);

        Client::empty(&e, &contract_id);
    }
}
