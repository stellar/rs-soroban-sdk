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
    pub static __SPEC_XDR_FN_PUT: [u8; super::Contract::__SPEC_XDR_REF_put.const_xdr_len()] =
        super::Contract::spec_xdr_put();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_REF_put: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::ScSymbolRef(soroban_sdk::xdr::StringMRef::new(b"put")),
            inputs: soroban_sdk::xdr::VecMRef::new(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"key"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefRef::Symbol,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"val"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefRef::Symbol,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMRef::new(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_put() -> [u8; Contract::__SPEC_XDR_REF_put.const_xdr_len()] {
        Contract::__SPEC_XDR_REF_put.const_to_xdr()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET: [u8; super::Contract::__SPEC_XDR_REF_get.const_xdr_len()] =
        super::Contract::spec_xdr_get();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_REF_get: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::ScSymbolRef(soroban_sdk::xdr::StringMRef::new(b"get")),
            inputs: soroban_sdk::xdr::VecMRef::new(&[soroban_sdk::xdr::ScSpecFunctionInputV0Ref {
                doc: soroban_sdk::xdr::StringMRef::new(b""),
                name: soroban_sdk::xdr::StringMRef::new(b"key"),
                type_: soroban_sdk::xdr::ScSpecTypeDefRef::Symbol,
            }]),
            outputs: soroban_sdk::xdr::VecMRef::new(&[soroban_sdk::xdr::ScSpecTypeDefRef::Option(
                &soroban_sdk::xdr::ScSpecTypeOptionRef {
                    value_type: &soroban_sdk::xdr::ScSpecTypeDefRef::Symbol,
                },
            )]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get() -> [u8; Contract::__SPEC_XDR_REF_get.const_xdr_len()] {
        Contract::__SPEC_XDR_REF_get.const_to_xdr()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__del__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DEL: [u8; super::Contract::__SPEC_XDR_REF_del.const_xdr_len()] =
        super::Contract::spec_xdr_del();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_REF_del: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::ScSymbolRef(soroban_sdk::xdr::StringMRef::new(b"del")),
            inputs: soroban_sdk::xdr::VecMRef::new(&[soroban_sdk::xdr::ScSpecFunctionInputV0Ref {
                doc: soroban_sdk::xdr::StringMRef::new(b""),
                name: soroban_sdk::xdr::StringMRef::new(b"key"),
                type_: soroban_sdk::xdr::ScSpecTypeDefRef::Symbol,
            }]),
            outputs: soroban_sdk::xdr::VecMRef::new(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_del() -> [u8; Contract::__SPEC_XDR_REF_del.const_xdr_len()] {
        Contract::__SPEC_XDR_REF_del.const_to_xdr()
    }
}
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
