#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use proc_macros::{check_fn_is_item_fn, parse_item_fn, parse_item_impl};
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
    pub fn empty() {}
    pub fn empty2() {
        let _ = core::any::type_name::<Self>();
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMPTY: [u8; 28usize] = super::Contract::spec_xdr_empty();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05empty\0\0\0\0\0\0\0\0\0\0\0"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_EMPTY: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\x02\xe7\xd0\x8a\xd9\x1eRo\x96A\x9b\x0b~\x9b\xdd\x94/\xdc\x8ai\xe5:\xd7\x8b\x93(h\x18\x84\xac\x95\x98",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMPTY2: [u8; 28usize] = super::Contract::spec_xdr_empty2();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty2() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06empty2\0\0\0\0\0\0\0\0\0\0"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_EMPTY2: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"s\x9dR\x0b\xe0\xfb\xe3g\x97\xfcu\xe1\xc5\xffE\x93\x96\xed\n~\x18\xea\x85\xd2\xdbC\x8a\x18@\x90\x04\x01",
    [],
);
impl<'a> ContractClient<'a> {
    pub fn empty(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_empty(
        &self,
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
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn empty2(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty2");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_empty2(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty2");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn empty<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn empty2<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty` instead")]
#[allow(deprecated)]
pub fn __Contract__empty__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::empty(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty` instead")]
#[export_name = "empty"]
pub extern "C" fn __Contract__empty__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__empty__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty2` instead")]
#[allow(deprecated)]
pub fn __Contract__empty2__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::empty2(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty2` instead")]
#[export_name = "empty2"]
pub extern "C" fn __Contract__empty2__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__empty2__invoke_raw(soroban_sdk::Env::default())
}
