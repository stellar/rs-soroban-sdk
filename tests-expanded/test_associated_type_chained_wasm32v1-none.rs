#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Error, Vec};
pub trait AssociatedType {
    type Val;
    type ValVal;
    fn set_val(env: Env, input: Self::Val);
    fn get_val(env: Env) -> Self::Val;
    fn both(input: Self::Val) -> Self::Val;
    fn wrapped(input: Vec<Self::Val>) -> Result<Self::Val, Error>;
    fn double_wrapped(input: Option<Vec<Self::Val>>) -> Result<Vec<Self::Val>, Error>;
    fn valval(input: Self::ValVal) -> Option<Self::ValVal>;
    fn tuple(input1: Self::Val) -> (Self::Val, Self::ValVal);
    fn valref(input: &Self::Val) -> Self::Val;
}
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
impl AssociatedType for Contract {
    type Val = u64;
    type ValVal = Self::Val;
    fn set_val(env: Env, input: Self::Val) {
        env.storage().instance().set(
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("val");
                SYMBOL
            },
            &input,
        );
    }
    fn get_val(env: Env) -> Self::Val {
        env.storage()
            .instance()
            .get(&{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("val");
                SYMBOL
            })
            .unwrap()
    }
    fn both(input: Self::Val) -> Self::Val {
        input + 1
    }
    fn wrapped(input: Vec<Self::Val>) -> Result<Self::Val, Error> {
        if input.is_empty() {
            Err(Error::from_contract_error(0))
        } else {
            let mut sum = 0;
            for val in input {
                sum += val;
            }
            Ok(sum)
        }
    }
    fn double_wrapped(input: Option<Vec<Self::Val>>) -> Result<Vec<Self::Val>, Error> {
        match input {
            Some(v) => Ok(v),
            None => Err(Error::from_contract_error(1)),
        }
    }
    fn valval(input: Self::ValVal) -> Option<Self::ValVal> {
        Some(input)
    }
    fn tuple(input: Self::Val) -> (Self::Val, Self::Val) {
        (input, input)
    }
    fn valref(input: &Self::Val) -> Self::Val {
        input.clone()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__set_val__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_SET_VAL: [u8; 48usize] = super::Contract::spec_xdr_set_val();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_set_val() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07set_val\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x06\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_val__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_VAL: [u8; 32usize] = super::Contract::spec_xdr_get_val();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_val() -> [u8; 32usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07get_val\0\0\0\0\0\0\0\0\x01\0\0\0\x06"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__both__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_BOTH: [u8; 48usize] = super::Contract::spec_xdr_both();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_both() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x04both\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrapped__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WRAPPED: [u8; 64usize] = super::Contract::spec_xdr_wrapped();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrapped() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07wrapped\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\x03\xea\0\0\0\x06\0\0\0\x01\0\0\x03\xe9\0\0\0\x06\0\0\0\x03"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__double_wrapped__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DOUBLE_WRAPPED: [u8; 80usize] =
        super::Contract::spec_xdr_double_wrapped();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_double_wrapped() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0edouble_wrapped\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\x03\xe8\0\0\x03\xea\0\0\0\x06\0\0\0\x01\0\0\x03\xe9\0\0\x03\xea\0\0\0\x06\0\0\0\x03"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__valval__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VALVAL: [u8; 56usize] = super::Contract::spec_xdr_valval();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_valval() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06valval\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x06\0\0\0\x01\0\0\x03\xe8\0\0\0\x06"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__tuple__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TUPLE: [u8; 64usize] = super::Contract::spec_xdr_tuple();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05tuple\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x06\0\0\0\x01\0\0\x03\xed\0\0\0\x02\0\0\0\x06\0\0\0\x06"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__valref__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VALREF: [u8; 52usize] = super::Contract::spec_xdr_valref();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_valref() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06valref\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractClient<'a> {
    pub fn set_val(&self, input: &u64) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("set_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_set_val(
        &self,
        input: &u64,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("set_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn get_val(&self) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_get_val(
        &self,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn both(&self, input: &u64) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("both");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_both(
        &self,
        input: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("both");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn wrapped(&self, input: &Vec<u64>) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("wrapped");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_wrapped(
        &self,
        input: &Vec<u64>,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("wrapped");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn double_wrapped(&self, input: &Option<Vec<u64>>) -> Vec<u64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "double_wrapped") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_double_wrapped(
        &self,
        input: &Option<Vec<u64>>,
    ) -> Result<
        Result<
            Vec<u64>,
            <Vec<u64> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "double_wrapped") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn valval(&self, input: &u64) -> Option<u64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valval");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_valval(
        &self,
        input: &u64,
    ) -> Result<
        Result<
            Option<u64>,
            <Option<u64> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valval");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn tuple(&self, input: &u64) -> (u64, u64) {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_tuple(
        &self,
        input: &u64,
    ) -> Result<
        Result<
            (u64, u64),
            <(u64, u64) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn valref(&self, input: &u64) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valref");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_valref(
        &self,
        input: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valref");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn set_val<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_val<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn both<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrapped<'i>(input: &'i Vec<u64>) -> (&'i Vec<u64>,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn double_wrapped<'i>(input: &'i Option<Vec<u64>>) -> (&'i Option<Vec<u64>>,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn valval<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn valref<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).set_val` instead")]
#[allow(deprecated)]
pub fn __Contract__set_val__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::set_val(
            env.clone(),
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).set_val` instead")]
#[export_name = "set_val"]
pub extern "C" fn __Contract__set_val__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__set_val__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_val` instead")]
#[allow(deprecated)]
pub fn __Contract__get_val__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::get_val(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_val` instead")]
#[export_name = "get_val"]
pub extern "C" fn __Contract__get_val__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_val__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).both` instead")]
#[allow(deprecated)]
pub fn __Contract__both__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::both(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).both` instead")]
#[export_name = "both"]
pub extern "C" fn __Contract__both__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__both__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapped` instead")]
#[allow(deprecated)]
pub fn __Contract__wrapped__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::wrapped(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapped` instead")]
#[export_name = "wrapped"]
pub extern "C" fn __Contract__wrapped__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__wrapped__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).double_wrapped` instead")]
#[allow(deprecated)]
pub fn __Contract__double_wrapped__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::double_wrapped(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).double_wrapped` instead")]
#[export_name = "double_wrapped"]
pub extern "C" fn __Contract__double_wrapped__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__double_wrapped__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valval` instead")]
#[allow(deprecated)]
pub fn __Contract__valval__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::valval(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valval` instead")]
#[export_name = "valval"]
pub extern "C" fn __Contract__valval__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__valval__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::tuple(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
#[export_name = "tuple"]
pub extern "C" fn __Contract__tuple__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valref` instead")]
#[allow(deprecated)]
pub fn __Contract__valref__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::valref(
            &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valref` instead")]
#[export_name = "valref"]
pub extern "C" fn __Contract__valref__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__valref__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
