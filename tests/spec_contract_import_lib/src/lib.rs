#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use test_spec_lib::{
    EnumA, EnumB, EnumIntA, EnumIntB, ErrorA, ErrorB, ErrorC, EventA, EventB, StructA, StructB,
    StructTupleA, StructTupleB,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Struct functions (A, B)

    pub fn create_struct_a(f1: u32, f2: bool) -> StructA {
        StructA { f1, f2 }
    }

    pub fn create_struct_b(f1: i64, f2: String) -> StructB {
        StructB { f1, f2 }
    }

    // Struct tuple functions (A, B)

    pub fn create_struct_tuple_a(f1: i64, f2: i64) -> StructTupleA {
        StructTupleA(f1, f2)
    }

    pub fn create_struct_tuple_b(f1: u128, f2: u128) -> StructTupleB {
        StructTupleB(f1, f2)
    }

    // Enum/Union functions (A, B)

    pub fn get_enum_a() -> EnumA {
        EnumA::V2
    }

    pub fn get_enum_b(value: i64) -> EnumB {
        EnumB::V2(value)
    }

    // Enum Int functions (A, B)

    pub fn get_enum_int_a() -> EnumIntA {
        EnumIntA::V3
    }

    pub fn get_enum_int_b() -> EnumIntB {
        EnumIntB::V2
    }

    // Error functions (A, B)

    pub fn check_a(input: u32) -> Result<u32, ErrorA> {
        if input == 0 {
            Err(ErrorA::E2)
        } else {
            Ok(input)
        }
    }

    pub fn check_b(input: u32) -> Result<u32, ErrorB> {
        if input > 1000 {
            Err(ErrorB::E3)
        } else {
            Ok(input)
        }
    }

    pub fn check_c(input: u32) -> Result<u32, ErrorC> {
        if input < 10 {
            Err(ErrorC::E1)
        } else {
            Ok(input)
        }
    }

    // Event functions (A, B)

    pub fn emit_event_a(env: Env, f1: Address, f2: String) {
        EventA { f1, f2 }.publish(&env);
    }

    pub fn emit_event_b(env: Env, f1: Address, f2: Address, f3: i128) {
        EventB { f1, f2, f3 }.publish(&env);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_struct_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let s = client.create_struct_a(&10, &true);
        assert_eq!(s.f1, 10);
        assert_eq!(s.f2, true);
    }

    #[test]
    fn test_struct_tuple_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let s = client.create_struct_tuple_a(&5, &10);
        assert_eq!(s.0, 5);
        assert_eq!(s.1, 10);
    }

    #[test]
    fn test_enum_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let v = client.get_enum_a();
        assert_eq!(v, EnumA::V2);
    }

    #[test]
    fn test_enum_int_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let v = client.get_enum_int_a();
        assert_eq!(v, EnumIntA::V3);
    }

    #[test]
    fn test_check_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let result = client.try_check_a(&10);
        assert_eq!(result, Ok(Ok(10)));

        let result = client.try_check_a(&0);
        assert_eq!(result, Err(Ok(ErrorA::E2)));
    }

    #[test]
    fn test_emit_event_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let f1 = Address::generate(&e);
        let f2 = String::from_str(&e, "test");
        client.emit_event_a(&f1, &f2);
        // Event was emitted successfully if no panic
    }
}
