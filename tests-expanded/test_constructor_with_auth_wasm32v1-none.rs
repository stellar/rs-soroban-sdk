#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Address, Env};
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
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&"admin", &admin);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract____constructor__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN___CONSTRUCTOR: [u8; super::Contract::spec_xdr_len___constructor()] =
        super::Contract::spec_xdr___constructor();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY___constructor: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"__constructor"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"admin"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len___constructor() -> usize {
        const { Contract::__SPEC_XDR_ENTRY___constructor.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr___constructor() -> [u8; Contract::spec_xdr_len___constructor()] {
        const { Contract::__SPEC_XDR_ENTRY___constructor.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn __constructor<'i>(admin: &'i Address) -> (&'i Address,) {
        (admin,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
#[allow(deprecated)]
pub fn __Contract____constructor__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::__constructor(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
#[export_name = "__constructor"]
pub extern "C" fn __Contract____constructor__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract____constructor__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
