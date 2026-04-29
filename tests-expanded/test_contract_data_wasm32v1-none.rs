#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Env, Symbol};
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
    pub fn put(e: Env, key: Symbol, val: Symbol) {
        e.storage().persistent().set(&key, &val)
    }
    pub fn get(e: Env, key: Symbol) -> Option<Symbol> {
        e.storage().persistent().get(&key)
    }
    pub fn del(e: Env, key: Symbol) {
        e.storage().persistent().remove(&key)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__put__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUT: [u8; 56usize] = super::Contract::spec_xdr_put();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_put() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03put\0\0\0\0\x02\0\0\0\0\0\0\0\x03key\0\0\0\0\x11\0\0\0\0\0\0\0\x03val\0\0\0\0\x11\0\0\0\0"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_PUT: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    42usize,
    0usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_FUNCTION,
    *b"e\x0c\0\x05\x14x\xd8\xa1\xf6a@`\xca\xaa\x92\xe3\x86C9J\xb4\xfd\xaa\xac,\t\xe3Y\x93\x9e\x03\xed",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET: [u8; 48usize] = super::Contract::spec_xdr_get();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03get\0\0\0\0\x01\0\0\0\0\0\0\0\x03key\0\0\0\0\x11\0\0\0\x01\0\0\x03\xe8\0\0\0\x11"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_GET: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    42usize,
    0usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_FUNCTION,
    *b"y\r\xaa\xcd\xc6b\xde\xa54\tZ3\xeen<6\x0f\xa9\xbf\x1e\xea\xf3\x1axp\x9d\xab\xc34\xe6F\xe7",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__del__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DEL: [u8; 40usize] = super::Contract::spec_xdr_del();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_del() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03del\0\0\0\0\x01\0\0\0\0\0\0\0\x03key\0\0\0\0\x11\0\0\0\0"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_DEL: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    42usize,
    0usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_FUNCTION,
    *b"\xff\xef\x99?\x06NE\xf9\x0f\x99z\xf6'\xe9@\x9ah\x8ch\xae\xff\xe8\xa3\xfa\xd6[=\x1f+W\xa8<",
    [],
);
impl<'a> ContractClient<'a> {
    pub fn put(&self, key: &Symbol, val: &Symbol) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("put");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [key.into_val(&self.env), val.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_put(
        &self,
        key: &Symbol,
        val: &Symbol,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("put");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [key.into_val(&self.env), val.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn get(&self, key: &Symbol) -> Option<Symbol> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn try_get(
        &self,
        key: &Symbol,
    ) -> Result<
        Result<
            Option<Symbol>,
            <Option<Symbol> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn del(&self, key: &Symbol) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("del");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn try_del(
        &self,
        key: &Symbol,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("del");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn put<'i>(key: &'i Symbol, val: &'i Symbol) -> (&'i Symbol, &'i Symbol) {
        (key, val)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get<'i>(key: &'i Symbol) -> (&'i Symbol,) {
        (key,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn del<'i>(key: &'i Symbol) -> (&'i Symbol,) {
        (key,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).put` instead")]
#[allow(deprecated)]
pub fn __Contract__put__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::put(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).put` instead")]
#[export_name = "put"]
pub extern "C" fn __Contract__put__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__put__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get` instead")]
#[allow(deprecated)]
pub fn __Contract__get__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::get(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get` instead")]
#[export_name = "get"]
pub extern "C" fn __Contract__get__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).del` instead")]
#[allow(deprecated)]
pub fn __Contract__del__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::del(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).del` instead")]
#[export_name = "del"]
pub extern "C" fn __Contract__del__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__del__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
