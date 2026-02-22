#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use test_spec_lib::{
    EnumA, EnumB, EnumIntA, EnumIntB, ErrorA, ErrorB, ErrorC, EventA, EventB, StructA, StructB,
    StructTupleA, StructTupleB,
};
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl Contract {
    pub fn create_struct_a(f1: u32, f2: bool) -> StructA {
        StructA { f1, f2 }
    }
    pub fn create_struct_b(f1: i64, f2: String) -> StructB {
        StructB { f1, f2 }
    }
    pub fn create_struct_tuple_a(f1: i64, f2: i64) -> StructTupleA {
        StructTupleA(f1, f2)
    }
    pub fn create_struct_tuple_b(f1: u128, f2: u128) -> StructTupleB {
        StructTupleB(f1, f2)
    }
    pub fn get_enum_a() -> EnumA {
        EnumA::V2
    }
    pub fn get_enum_b(value: i64) -> EnumB {
        EnumB::V2(value)
    }
    pub fn get_enum_int_a() -> EnumIntA {
        EnumIntA::V3
    }
    pub fn get_enum_int_b() -> EnumIntB {
        EnumIntB::V2
    }
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
    pub fn emit_event_a(env: Env, f1: Address, f2: String) {
        EventA { f1, f2 }.publish(&env);
    }
    pub fn emit_event_b(env: Env, f1: Address, f2: Address, f3: i128) {
        EventB { f1, f2, f3 }.publish(&env);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_A: [u8; 84usize] =
        super::Contract::spec_xdr_create_struct_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_a() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0fcreate_struct_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_B: [u8; 84usize] =
        super::Contract::spec_xdr_create_struct_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_b() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0fcreate_struct_b\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructB\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_tuple_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_TUPLE_A: [u8; 96usize] =
        super::Contract::spec_xdr_create_struct_tuple_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_tuple_a() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x15create_struct_tuple_a\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_tuple_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_TUPLE_B: [u8; 96usize] =
        super::Contract::spec_xdr_create_struct_tuple_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_tuple_b() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x15create_struct_tuple_b\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\n\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleB"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_ENUM_A: [u8; 48usize] = super::Contract::spec_xdr_get_enum_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_a() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nget_enum_a\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumA\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_ENUM_B: [u8; 68usize] = super::Contract::spec_xdr_get_enum_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_b() -> [u8; 68usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nget_enum_b\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumB\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_int_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_ENUM_INT_A: [u8; 52usize] =
        super::Contract::spec_xdr_get_enum_int_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_int_a() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0eget_enum_int_a\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_int_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_ENUM_INT_B: [u8; 52usize] =
        super::Contract::spec_xdr_get_enum_int_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_int_b() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0eget_enum_int_b\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntB"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__check_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CHECK_A: [u8; 72usize] = super::Contract::spec_xdr_check_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_check_a() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07check_a\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorA\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__check_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CHECK_B: [u8; 72usize] = super::Contract::spec_xdr_check_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_check_b() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07check_b\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorB\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__check_c__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CHECK_C: [u8; 72usize] = super::Contract::spec_xdr_check_c();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_check_c() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07check_c\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorC\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__emit_event_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMIT_EVENT_A: [u8; 64usize] = super::Contract::spec_xdr_emit_event_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_emit_event_a() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cemit_event_a\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__emit_event_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMIT_EVENT_B: [u8; 80usize] = super::Contract::spec_xdr_emit_event_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_emit_event_b() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cemit_event_b\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn create_struct_a(&self, f1: &u32, f2: &bool) -> StructA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_create_struct_a(
        &self,
        f1: &u32,
        f2: &bool,
    ) -> Result<
        Result<
            StructA,
            <StructA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn create_struct_b(&self, f1: &i64, f2: &String) -> StructB {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_create_struct_b(
        &self,
        f1: &i64,
        f2: &String,
    ) -> Result<
        Result<
            StructB,
            <StructB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn create_struct_tuple_a(&self, f1: &i64, f2: &i64) -> StructTupleA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_create_struct_tuple_a(
        &self,
        f1: &i64,
        f2: &i64,
    ) -> Result<
        Result<
            StructTupleA,
            <StructTupleA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn create_struct_tuple_b(&self, f1: &u128, f2: &u128) -> StructTupleB {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_create_struct_tuple_b(
        &self,
        f1: &u128,
        f2: &u128,
    ) -> Result<
        Result<
            StructTupleB,
            <StructTupleB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn get_enum_a(&self) -> EnumA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_get_enum_a(
        &self,
    ) -> Result<
        Result<
            EnumA,
            <EnumA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn get_enum_b(&self, value: &i64) -> EnumB {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        res
    }
    pub fn try_get_enum_b(
        &self,
        value: &i64,
    ) -> Result<
        Result<
            EnumB,
            <EnumB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        res
    }
    pub fn get_enum_int_a(&self) -> EnumIntA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_get_enum_int_a(
        &self,
    ) -> Result<
        Result<
            EnumIntA,
            <EnumIntA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn get_enum_int_b(&self) -> EnumIntB {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_get_enum_int_b(
        &self,
    ) -> Result<
        Result<
            EnumIntB,
            <EnumIntB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn check_a(&self, input: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_check_a(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorA, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn check_b(&self, input: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_check_b(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorB, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn check_c(&self, input: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_check_c(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorC, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn emit_event_a(&self, f1: &Address, f2: &String) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_emit_event_a(
        &self,
        f1: &Address,
        f2: &String,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn emit_event_b(&self, f1: &Address, f2: &Address, f3: &i128) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                    f3.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_emit_event_b(
        &self,
        f1: &Address,
        f2: &Address,
        f3: &i128,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                    f3.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_a<'i>(f1: &'i u32, f2: &'i bool) -> (&'i u32, &'i bool) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_b<'i>(f1: &'i i64, f2: &'i String) -> (&'i i64, &'i String) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_tuple_a<'i>(f1: &'i i64, f2: &'i i64) -> (&'i i64, &'i i64) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_tuple_b<'i>(f1: &'i u128, f2: &'i u128) -> (&'i u128, &'i u128) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_b<'i>(value: &'i i64) -> (&'i i64,) {
        (value,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_int_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_int_b<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn check_a<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn check_b<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn check_c<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn emit_event_a<'i>(f1: &'i Address, f2: &'i String) -> (&'i Address, &'i String) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn emit_event_b<'i>(
        f1: &'i Address,
        f2: &'i Address,
        f3: &'i i128,
    ) -> (&'i Address, &'i Address, &'i i128) {
        (f1, f2, f3)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_a` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_a(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_a` instead")]
#[export_name = "create_struct_a"]
pub extern "C" fn __Contract__create_struct_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_b` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_b(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_b` instead")]
#[export_name = "create_struct_b"]
pub extern "C" fn __Contract__create_struct_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_b__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_a` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_tuple_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_tuple_a(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_a` instead")]
#[export_name = "create_struct_tuple_a"]
pub extern "C" fn __Contract__create_struct_tuple_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_tuple_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_b` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_tuple_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_tuple_b(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_b` instead")]
#[export_name = "create_struct_tuple_b"]
pub extern "C" fn __Contract__create_struct_tuple_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_tuple_b__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_a` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::get_enum_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_a` instead")]
#[export_name = "get_enum_a"]
pub extern "C" fn __Contract__get_enum_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_b` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::get_enum_b(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_b` instead")]
#[export_name = "get_enum_b"]
pub extern "C" fn __Contract__get_enum_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_b__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_a` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_int_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::get_enum_int_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_a` instead")]
#[export_name = "get_enum_int_a"]
pub extern "C" fn __Contract__get_enum_int_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_int_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_b` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_int_b__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::get_enum_int_b(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_b` instead")]
#[export_name = "get_enum_int_b"]
pub extern "C" fn __Contract__get_enum_int_b__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_int_b__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_a` instead")]
#[allow(deprecated)]
pub fn __Contract__check_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::check_a(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_a` instead")]
#[export_name = "check_a"]
pub extern "C" fn __Contract__check_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__check_a__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_b` instead")]
#[allow(deprecated)]
pub fn __Contract__check_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::check_b(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_b` instead")]
#[export_name = "check_b"]
pub extern "C" fn __Contract__check_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__check_b__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_c` instead")]
#[allow(deprecated)]
pub fn __Contract__check_c__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::check_c(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_c` instead")]
#[export_name = "check_c"]
pub extern "C" fn __Contract__check_c__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__check_c__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_a` instead")]
#[allow(deprecated)]
pub fn __Contract__emit_event_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::emit_event_a(
            env.clone(),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_a` instead")]
#[export_name = "emit_event_a"]
pub extern "C" fn __Contract__emit_event_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__emit_event_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_b` instead")]
#[allow(deprecated)]
pub fn __Contract__emit_event_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::emit_event_b(
            env.clone(),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_b` instead")]
#[export_name = "emit_event_b"]
pub extern "C" fn __Contract__emit_event_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__emit_event_b__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
