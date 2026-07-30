#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn void_fn(_void_arg: ()) -> () {}

    pub fn tuple1(arg: (u32,)) -> (u32,) {
        arg
    }

    pub fn tuple2(arg: (u32, i64)) -> (u32, i64) {
        arg
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    mod wasm {
        soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/test_tuples.wasm");
    }

    #[test]
    fn test_native_void() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        client.void_fn(&());
    }

    #[test]
    fn test_native_tuple1() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let result = client.tuple1(&(42u32,));
        assert_eq!(result, (42u32,));
    }

    #[test]
    fn test_native_tuple2() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let result = client.tuple2(&(42u32, -100i64));
        assert_eq!(result, (42u32, -100i64));
    }

    #[test]
    fn test_wasm_void() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);
        client.void_fn(&());
    }

    #[test]
    fn test_wasm_tuple1() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);
        let result = client.tuple1(&(42u32,));
        assert_eq!(result, (42u32,));
    }

    #[test]
    fn test_wasm_tuple2() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);
        let result = client.tuple2(&(42u32, -100i64));
        assert_eq!(result, (42u32, -100i64));
    }
}
