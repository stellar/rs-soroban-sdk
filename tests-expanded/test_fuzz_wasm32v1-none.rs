#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, U256};
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
    pub fn run(a: U256, b: U256) {
        if a < b {
            {
                ::core::panicking::panic_fmt(format_args!("unexpected"));
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__run__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_RUN: [u8; super::Contract::__SPEC_XDR_VIEW_run.const_xdr_len()] =
        super::Contract::spec_xdr_run();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_VIEW_run: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::V2(soroban_sdk::xdr::ScSpecEntryV2View {
            id: soroban_sdk::spec_type_id("test_fuzz::Contract::run"),
            body: soroban_sdk::xdr::ScSpecEntryV2BodyView::FunctionV0(
                soroban_sdk::xdr::ScSpecFunctionV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(
                        b"run",
                    )),
                    inputs: soroban_sdk::xdr::VecMView::new(&[
                        soroban_sdk::xdr::ScSpecFunctionInputV0View {
                            doc: soroban_sdk::xdr::StringMView::new(b""),
                            name: soroban_sdk::xdr::StringMView::new(b"a"),
                            type_: soroban_sdk::xdr::ScSpecTypeDefView::U256,
                        },
                        soroban_sdk::xdr::ScSpecFunctionInputV0View {
                            doc: soroban_sdk::xdr::StringMView::new(b""),
                            name: soroban_sdk::xdr::StringMView::new(b"b"),
                            type_: soroban_sdk::xdr::ScSpecTypeDefView::U256,
                        },
                    ]),
                    outputs: soroban_sdk::xdr::VecMView::new(&[]),
                },
            ),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_run() -> [u8; Contract::__SPEC_XDR_VIEW_run.const_xdr_len()] {
        Contract::__SPEC_XDR_VIEW_run.const_to_xdr()
    }
}
impl<'a> ContractClient<'a> {
    pub fn run(&self, a: &U256, b: &U256) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("run");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_run(
        &self,
        a: &U256,
        b: &U256,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("run");
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
    pub fn run<'i>(a: &'i U256, b: &'i U256) -> (&'i U256, &'i U256) {
        (a, b)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).run` instead")]
#[allow(deprecated)]
pub fn __Contract__run__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::run(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).run` instead")]
#[export_name = "run"]
pub extern "C" fn __Contract__run__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__run__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
