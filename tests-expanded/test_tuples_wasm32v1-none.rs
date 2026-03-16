#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl};
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
    pub fn void_fn(_void_arg: ()) -> () {}
    pub fn tuple1(arg: (u32,)) -> (u32,) {
        arg
    }
    pub fn tuple2(arg: (u32, i64)) -> (u32, i64) {
        arg
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__void_fn__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VOID_FN: [u8; 52usize] = super::Contract::spec_xdr_void_fn();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_void_fn() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07void_fn\0\0\0\0\x01\0\0\0\0\0\0\0\x08void_arg\0\0\0\x02\0\0\0\x01\0\0\0\x02"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__tuple1__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TUPLE1: [u8; 64usize] = super::Contract::spec_xdr_tuple1();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple1() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06tuple1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03arg\0\0\0\x03\xed\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\x03\xed\0\0\0\x01\0\0\0\x04"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__tuple2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TUPLE2: [u8; 72usize] = super::Contract::spec_xdr_tuple2();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple2() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06tuple2\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03arg\0\0\0\x03\xed\0\0\0\x02\0\0\0\x04\0\0\0\x07\0\0\0\x01\0\0\x03\xed\0\0\0\x02\0\0\0\x04\0\0\0\x07"
    }
}
impl<'a> ContractClient<'a> {
    pub fn void_fn(&self, _void_arg: &()) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("void_fn");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_void_arg.into_val(&self.env)]),
        );
        res
    }
    pub fn try_void_fn(
        &self,
        _void_arg: &(),
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("void_fn");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_void_arg.into_val(&self.env)]),
        );
        res
    }
    pub fn tuple1(&self, arg: &(u32,)) -> (u32,) {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        res
    }
    pub fn try_tuple1(
        &self,
        arg: &(u32,),
    ) -> Result<
        Result<
            (u32,),
            <(u32,) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        res
    }
    pub fn tuple2(&self, arg: &(u32, i64)) -> (u32, i64) {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        res
    }
    pub fn try_tuple2(
        &self,
        arg: &(u32, i64),
    ) -> Result<
        Result<
            (u32, i64),
            <(u32, i64) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn void_fn<'i>(_void_arg: &'i ()) -> (&'i (),) {
        (_void_arg,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple1<'i>(arg: &'i (u32,)) -> (&'i (u32,),) {
        (arg,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple2<'i>(arg: &'i (u32, i64)) -> (&'i (u32, i64),) {
        (arg,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).void_fn` instead")]
#[allow(deprecated)]
pub fn __Contract__void_fn__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::void_fn(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).void_fn` instead")]
#[export_name = "void_fn"]
pub extern "C" fn __Contract__void_fn__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__void_fn__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple1` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple1__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::tuple1(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple1` instead")]
#[export_name = "tuple1"]
pub extern "C" fn __Contract__tuple1__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple1__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple2` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple2__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::tuple2(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple2` instead")]
#[export_name = "tuple2"]
pub extern "C" fn __Contract__tuple2__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple2__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
