#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use test_spec_lib::{EnumA, EnumIntA, ErrorA, EventA, StructA, StructTupleA};
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
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fn_struct_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN_STRUCT_A: [u8; 80usize] = super::Contract::spec_xdr_fn_struct_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn_struct_a() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0bfn_struct_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fn_struct_tuple_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN_STRUCT_TUPLE_A: [u8; 92usize] =
        super::Contract::spec_xdr_fn_struct_tuple_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn_struct_tuple_a() -> [u8; 92usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11fn_struct_tuple_a\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fn_enum_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN_ENUM_A: [u8; 48usize] = super::Contract::spec_xdr_fn_enum_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn_enum_a() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\tfn_enum_a\0\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumA\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fn_enum_int_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN_ENUM_INT_A: [u8; 52usize] =
        super::Contract::spec_xdr_fn_enum_int_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn_enum_int_a() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rfn_enum_int_a\0\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fn_error_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN_ERROR_A: [u8; 76usize] = super::Contract::spec_xdr_fn_error_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn_error_a() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nfn_error_a\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorA\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fn_event_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN_EVENT_A: [u8; 64usize] = super::Contract::spec_xdr_fn_event_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn_event_a() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nfn_event_a\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn fn_struct_a(&self, f1: &u32, f2: &bool) -> StructA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_fn_struct_a(
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
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn fn_struct_tuple_a(&self, f1: &i64, f2: &i64) -> StructTupleA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_fn_struct_tuple_a(
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
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn fn_enum_a(&self) -> EnumA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn_enum_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_fn_enum_a(
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
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn_enum_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn fn_enum_int_a(&self) -> EnumIntA {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_fn_enum_int_a(
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
            &{ soroban_sdk::Symbol::new(&self.env, "fn_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn fn_error_a(&self, input: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_error_a") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_fn_error_a(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorA, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_error_a") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn fn_event_a(&self, f1: &Address, f2: &String) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_fn_event_a(
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
            &{ soroban_sdk::Symbol::new(&self.env, "fn_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_struct_a<'i>(f1: &'i u32, f2: &'i bool) -> (&'i u32, &'i bool) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_struct_tuple_a<'i>(f1: &'i i64, f2: &'i i64) -> (&'i i64, &'i i64) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_enum_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_enum_int_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_error_a<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_event_a<'i>(f1: &'i Address, f2: &'i String) -> (&'i Address, &'i String) {
        (f1, f2)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_struct_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_struct_a(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_a` instead")]
pub extern "C" fn __Contract__fn_struct_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_struct_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_tuple_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_struct_tuple_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_struct_tuple_a(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_tuple_a` instead")]
pub extern "C" fn __Contract__fn_struct_tuple_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_struct_tuple_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_enum_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::fn_enum_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_a` instead")]
pub extern "C" fn __Contract__fn_enum_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_enum_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_int_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_enum_int_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::fn_enum_int_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_int_a` instead")]
pub extern "C" fn __Contract__fn_enum_int_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_enum_int_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_error_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_error_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_error_a(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_error_a` instead")]
pub extern "C" fn __Contract__fn_error_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_error_a__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_event_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_event_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_event_a(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_event_a` instead")]
pub extern "C" fn __Contract__fn_event_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_event_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
