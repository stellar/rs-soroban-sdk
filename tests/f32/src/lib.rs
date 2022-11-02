#![no_std]
use soroban_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn f32(a: u32) -> u64 {
        let b: f64 = 1.234;
        let c = a as f64 * b;
        c as u64
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_native() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        assert_eq!(client.f32(&8), 9);
    }

    mod imported {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32-unknown-unknown/release/test_f32.wasm"
        );
    }

    #[test]
    fn test_wasm() {
        let e = Env::default();
        let contract_id = e.register_contract_wasm(None, imported::WASM);
        let client = ContractClient::new(&e, &contract_id);

        assert_eq!(client.f32(&8), 9);
    }
}
