#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
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
    pub fn empty(_a: i32, _b: &i32) {}
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMPTY: [u8; 60usize] = super::Contract::spec_xdr_empty();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty() -> [u8; 60usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05empty\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x05\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn empty(&self, _a: &i32, _b: &i32) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [_a.into_val(&self.env), _b.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_empty(
        &self,
        _a: &i32,
        _b: &i32,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [_a.into_val(&self.env), _b.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn empty<'i>(_a: &'i i32, _b: &'i i32) -> (&'i i32, &'i i32) {
        (_a, _b)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::empty(
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty` instead")]
    #[export_name = "empty"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
