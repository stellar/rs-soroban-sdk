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
    pub fn add(a: i128, b: i128) -> i128 {
        a + b
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_ADD: [u8; super::Contract::spec_xdr_len_add()] =
        super::Contract::spec_xdr_add();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_add: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"add"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::I128,
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_add() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_add.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add() -> [u8; Contract::spec_xdr_len_add()] {
        const { Contract::__SPEC_XDR_ENTRY_add.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn add(&self, a: &i128, b: &i128) -> i128 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_add(
        &self,
        a: &i128,
        b: &i128,
    ) -> Result<
        Result<i128, <i128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn add<'i>(a: &'i i128, b: &'i i128) -> (&'i i128, &'i i128) {
        (a, b)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
#[allow(deprecated)]
pub fn __Contract__add__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::add(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
#[export_name = "add"]
pub extern "C" fn __Contract__add__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__add__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
