#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

mod imported {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/test_spec_contract_import_lib.wasm"
    );
}

pub use imported::{EnumA, EnumIntA, ErrorA, EventA, StructA, StructTupleA};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Struct function (A)

    pub fn wrap_struct_a(env: Env, contract_id: Address, f1: u32, f2: bool) -> StructA {
        imported::Client::new(&env, &contract_id).create_struct_a(&f1, &f2)
    }

    // Struct tuple function (A)

    pub fn wrap_struct_tuple_a(env: Env, contract_id: Address, f1: i64, f2: i64) -> StructTupleA {
        imported::Client::new(&env, &contract_id).create_struct_tuple_a(&f1, &f2)
    }

    // Enum/Union function (A)

    pub fn wrap_enum_a(env: Env, contract_id: Address) -> EnumA {
        imported::Client::new(&env, &contract_id).get_enum_a()
    }

    // Enum Int function (A)

    pub fn wrap_enum_int_a(env: Env, contract_id: Address) -> EnumIntA {
        imported::Client::new(&env, &contract_id).get_enum_int_a()
    }

    // Error function (A)

    pub fn wrap_check_a(env: Env, contract_id: Address, input: u32) -> u32 {
        imported::Client::new(&env, &contract_id).check_a(&input)
    }

    // Event function (A)

    pub fn wrap_emit_event_a(env: Env, contract_id: Address, f1: Address, f2: String) {
        imported::Client::new(&env, &contract_id).emit_event_a(&f1, &f2);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_wrap_struct_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let s = client.wrap_struct_a(&imported_contract_id, &10, &true);
        assert_eq!(s.f1, 10);
        assert_eq!(s.f2, true);
    }

    #[test]
    fn test_wrap_struct_tuple_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let s = client.wrap_struct_tuple_a(&imported_contract_id, &5, &10);
        assert_eq!(s.0, 5);
        assert_eq!(s.1, 10);
    }

    #[test]
    fn test_wrap_enum_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let v = client.wrap_enum_a(&imported_contract_id);
        assert_eq!(v, EnumA::V2);
    }

    #[test]
    fn test_wrap_enum_int_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let v = client.wrap_enum_int_a(&imported_contract_id);
        assert_eq!(v, EnumIntA::V3);
    }

    #[test]
    fn test_wrap_check_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let result = client.wrap_check_a(&imported_contract_id, &10);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_wrap_emit_event_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let f1 = Address::generate(&e);
        let f2 = String::from_str(&e, "test");
        client.wrap_emit_event_a(&imported_contract_id, &f1, &f2);
        // Event was emitted successfully if no panic
    }
}
