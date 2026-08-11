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
pub struct Transfer {
    from: Address,
    to: Address,
    amount: i128,
    to_muxed_id: Option<u64>,
}
impl Transfer {
    #[doc(hidden)]
    pub const fn spec_type_name() -> &'static str {
        "test_events::Transfer"
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_TRANSFER: [u8; Transfer::__SPEC_XDR_VIEW.const_xdr_len()] =
    Transfer::spec_xdr();
impl Transfer {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::EventV0(soroban_sdk::xdr::ScSpecEventV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new_str(Transfer::spec_type_name()),
            prefix_topics: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::new(b"transfer"),
            )]),
            params: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"from"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"to"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"amount"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"to_muxed_id"),
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
    pub const fn spec_xdr() -> [u8; Transfer::__SPEC_XDR_VIEW.const_xdr_len()] {
        Transfer::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for Transfer {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Option<u64> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&Transfer::spec_xdr());
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for Transfer {
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
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl Transfer {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
impl Contract {
    pub fn transfer(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from: from.clone(),
            to: to.address(),
            amount,
            to_muxed_id: to.id(),
        }
        .publish(&env);
    }
    pub fn failed_transfer(env: Env, from: Address, to: Address, amount: i128) {
        Transfer {
            from: from.clone(),
            to: to.clone(),
            amount,
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
    pub static __SPEC_XDR_FN_TRANSFER: [u8; super::Contract::__SPEC_XDR_VIEW_transfer
        .const_xdr_len()] = super::Contract::spec_xdr_transfer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_VIEW_transfer: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(b"transfer")),
            inputs: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"from"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"to"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::MuxedAddress,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"amount"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::new(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_transfer() -> [u8; Contract::__SPEC_XDR_VIEW_transfer.const_xdr_len()] {
        Contract::__SPEC_XDR_VIEW_transfer.const_to_xdr()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__failed_transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FAILED_TRANSFER: [u8;
        super::Contract::__SPEC_XDR_VIEW_failed_transfer.const_xdr_len()] =
        super::Contract::spec_xdr_failed_transfer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_VIEW_failed_transfer: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(
                b"failed_transfer",
            )),
            inputs: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"from"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"to"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"amount"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::new(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_failed_transfer(
    ) -> [u8; Contract::__SPEC_XDR_VIEW_failed_transfer.const_xdr_len()] {
        Contract::__SPEC_XDR_VIEW_failed_transfer.const_to_xdr()
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
