#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Val};
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
    pub fn val(v: Val) -> Val {
        v
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__val__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VAL: [u8; 44usize] = super::Contract::spec_xdr_val();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_val() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03val\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn val(&self, v: &Val) -> Val {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("val");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_val(
        &self,
        v: &Val,
    ) -> Result<
        Result<Val, <Val as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("val");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn val<'i>(v: &'i Val) -> (&'i Val,) {
        (v,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).val` instead")]
#[allow(deprecated)]
pub fn __Contract__val__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::val(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).val` instead")]
#[export_name = "val"]
pub extern "C" fn __Contract__val__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__val__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
