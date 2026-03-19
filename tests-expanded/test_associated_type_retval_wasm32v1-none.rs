#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, symbol_short, Env};
pub trait AssociatedType {
    type Val;
    fn set_val(env: Env, input: Self::Val);
    fn get_val(env: Env) -> Self::Val;
    fn both(input: Self::Val) -> Self::Val;
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
