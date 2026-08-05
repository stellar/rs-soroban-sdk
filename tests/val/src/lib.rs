#![no_std]
use soroban_sdk::{contract, contractimpl, Val};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn val(v: Val) -> Val {
        v
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{Env, IntoVal, String, TryFromVal, Val};

    use crate::{Contract, ContractClient};

    mod wasm {
        soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/test_val.wasm");
    }

    #[test]
    fn test_native_scalar() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let v: Val = 42u32.into_val(&e);
        let r = client.val(&v);
        assert_eq!(u32::try_from_val(&e, &r), Ok(42u32));
    }

    #[test]
    fn test_native_object() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let s = String::from_str(&e, "hello");
        let r = client.val(&s.to_val());
        assert_eq!(String::try_from_val(&e, &r), Ok(s));
    }

    #[test]
    fn test_wasm_scalar() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);

        let v: Val = 42u32.into_val(&e);
        let r = client.val(&v);
        assert_eq!(u32::try_from_val(&e, &r), Ok(42u32));
    }

    #[test]
    fn test_wasm_object() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);

        let s = String::from_str(&e, "hello");
        let r = client.val(&s.to_val());
        assert_eq!(String::try_from_val(&e, &r), Ok(s));
    }
}
