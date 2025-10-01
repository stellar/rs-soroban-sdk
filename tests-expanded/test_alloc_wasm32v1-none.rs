#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Env};
extern crate alloc;
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
    pub fn num_list(env: Env, count: u32) -> soroban_sdk::Vec<u32> {
        let mut v1 = ::alloc::vec::Vec::new();
        (0..count).for_each(|i| v1.push(i));
        let mut v2 = ::soroban_sdk::Vec::new(&env);
        for i in v1 {
            v2.push_back(i);
        }
        v2
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__num_list__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_NUM_LIST: [u8; 56usize] = super::Contract::spec_xdr_num_list();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_num_list() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08num_list\0\0\0\x01\0\0\0\0\0\0\0\x05count\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xea\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn num_list(&self, count: &u32) -> soroban_sdk::Vec<u32> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("num_list");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [count.into_val(&self.env)]),
        );
        res
    }
    pub fn try_num_list(
        &self,
        count: &u32,
    ) -> Result<
        Result<
            soroban_sdk::Vec<u32>,
            <soroban_sdk::Vec<u32> as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("num_list");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [count.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn num_list<'i>(count: &'i u32) -> (&'i u32,) {
        (count,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__num_list {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).num_list` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::num_list(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).num_list` instead")]
    #[export_name = "num_list"]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
