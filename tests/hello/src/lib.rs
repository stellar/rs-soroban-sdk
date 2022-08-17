#![no_std]
use soroban_sdk::{contractimpl, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello() -> Symbol {
        Symbol::from_str("hello")
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env, Symbol};

    use crate::{hello, Contract};

    #[test]
    fn test_hello() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, [0; 32]);
        e.register_contract(&contract_id, Contract);

        let h = hello::invoke(&e, &contract_id);
        assert!(h == Symbol::from_str("hello"));
    }
}
