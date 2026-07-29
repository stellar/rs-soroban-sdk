#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
use test_workspace_lib::Value;
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
    pub fn value() -> Value {
        return Value { value: 13 };
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__value__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VALUE: [u8; super::Contract::__SPEC_XDR_REF_value.const_xdr_len()] =
        super::Contract::spec_xdr_value();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_REF_value: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::ScSymbolRef(soroban_sdk::xdr::StringMRef::new(b"value")),
            inputs: soroban_sdk::xdr::VecMRef::new(&[]),
            outputs: soroban_sdk::xdr::VecMRef::new(&[soroban_sdk::xdr::ScSpecTypeDefRef::UdtV2(
                soroban_sdk::xdr::ScSpecTypeUdtv2Ref {
                    id: <Value>::spec_type_id(),
                    name: soroban_sdk::xdr::StringMRef::new(b"Value"),
                },
            )]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_value() -> [u8; Contract::__SPEC_XDR_REF_value.const_xdr_len()] {
        Contract::__SPEC_XDR_REF_value.const_to_xdr()
    }
}
impl<'a> ContractClient<'a> {
    pub fn value(&self) -> Value {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("value");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_value(
        &self,
    ) -> Result<
        Result<
            Value,
            <Value as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("value");
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
    pub fn value<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).value` instead")]
#[allow(deprecated)]
pub fn __Contract__value__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::value(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).value` instead")]
#[export_name = "value"]
pub extern "C" fn __Contract__value__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__value__invoke_raw(soroban_sdk::Env::default())
}
