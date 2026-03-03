#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use test_spec_lib::{EnumA, EnumIntA, ErrorA, EventA, StructA, StructTupleA};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn fn_struct_a(f1: u32, f2: bool) -> StructA {
        StructA { f1, f2 }
    }

    pub fn fn_struct_tuple_a(f1: i64, f2: i64) -> StructTupleA {
        StructTupleA(f1, f2)
    }

    pub fn fn_enum_a() -> EnumA {
        EnumA::V2
    }

    pub fn fn_enum_int_a() -> EnumIntA {
        EnumIntA::V3
    }

    pub fn fn_error_a(input: u32) -> Result<u32, ErrorA> {
        if input == 0 {
            Err(ErrorA::E2)
        } else {
            Ok(input)
        }
    }

    pub fn fn_event_a(env: Env, f1: Address, f2: String) {
        EventA { f1, f2 }.publish(&env);
    }
}
