#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, MuxedAddress};
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
pub struct Transfer<'a> {
    from: &'a Address,
    to: &'a Address,
    amount: &'a i128,
    to_muxed_id: Option<&'a u64>,
}
impl<'a> Transfer<'a> {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_events_ref::Transfer"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_TRANSFER: [u8; Transfer::spec_xdr_len()] = Transfer::spec_xdr();
impl<'a> Transfer<'a> {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::EventV0(soroban_sdk::xdr::ScSpecEventV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(Transfer::spec_name()),
            prefix_topics: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSymbolView(
                    soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"transfer"),
                ),
            ]),
            params: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"to"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"amount"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"to_muxed_id"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Option(
                        &soroban_sdk::xdr::ScSpecTypeOptionView {
                            value_type: &soroban_sdk::xdr::ScSpecTypeDefView::U64,
                        },
                    ),
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr_len() -> usize {
        const { Transfer::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; Transfer::spec_xdr_len()] {
        const { Transfer::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl<'a> soroban_sdk::SpecShakingMarker for Transfer<'a> {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <&'a Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <&'a Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <&'a i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Option<&'a u64> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Transfer::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl<'a> soroban_sdk::Event for Transfer<'a> {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            },
            {
                let v: soroban_sdk::Val = self.to.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 2usize] = ["amount", "to_muxed_id"];
        let vals: [soroban_sdk::Val; 2usize] =
            [self.amount.into_val(env), self.to_muxed_id.into_val(env)];
        env.sparse_map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl<'a> Transfer<'a> {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
impl Contract {
    pub fn transfer(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from: &from,
            to: &to.address(),
            amount: &amount,
            to_muxed_id: to.id().as_ref(),
        }
        .publish(&env);
    }
    pub fn failed_transfer(env: Env, from: Address, to: Address, amount: i128) {
        Transfer {
            from: &from,
            to: &to,
            amount: &amount,
            to_muxed_id: None,
        }
        .publish(&env);
        {
            ::core::panicking::panic_fmt(format_args!("fail"));
        };
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TRANSFER: [u8; super::Contract::spec_xdr_len_transfer()] =
        super::Contract::spec_xdr_transfer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_transfer: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"transfer"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"to"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::MuxedAddress,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"amount"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_transfer() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_transfer.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_transfer() -> [u8; Contract::spec_xdr_len_transfer()] {
        const { Contract::__SPEC_XDR_ENTRY_transfer.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__failed_transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FAILED_TRANSFER: [u8; super::Contract::spec_xdr_len_failed_transfer(
    )] = super::Contract::spec_xdr_failed_transfer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_failed_transfer: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"failed_transfer"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"to"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"amount"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_failed_transfer() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_failed_transfer.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_failed_transfer() -> [u8; Contract::spec_xdr_len_failed_transfer()] {
        const { Contract::__SPEC_XDR_ENTRY_failed_transfer.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn transfer(&self, from: &Address, to: impl Into<MuxedAddress>, amount: &i128) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into().into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_transfer(
        &self,
        from: &Address,
        to: impl Into<MuxedAddress>,
        amount: &i128,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into().into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn failed_transfer(&self, from: &Address, to: &Address, amount: &i128) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "failed_transfer") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_failed_transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: &i128,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "failed_transfer") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn transfer<'i>(
        from: &'i Address,
        to: &'i MuxedAddress,
        amount: &'i i128,
    ) -> (&'i Address, &'i MuxedAddress, &'i i128) {
        (from, to, amount)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn failed_transfer<'i>(
        from: &'i Address,
        to: &'i Address,
        amount: &'i i128,
    ) -> (&'i Address, &'i Address, &'i i128) {
        (from, to, amount)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
#[allow(deprecated)]
pub fn __Contract__transfer__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::transfer(
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
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
#[export_name = "transfer"]
pub extern "C" fn __Contract__transfer__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__transfer__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
#[allow(deprecated)]
pub fn __Contract__failed_transfer__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::failed_transfer(
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
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
#[export_name = "failed_transfer"]
pub extern "C" fn __Contract__failed_transfer__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__failed_transfer__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
