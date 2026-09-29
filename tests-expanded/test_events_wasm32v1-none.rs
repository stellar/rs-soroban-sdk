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
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_EVENT_TRANSFER: [u8; Transfer::spec_xdr().len()] = Transfer::spec_xdr();
impl Transfer {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"Transfer"),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"transfer"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"to"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"amount"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"to_muxed_id",
                    ),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Option(
                        &soroban_sdk::xdr::r#const::ScSpecTypeOption {
                            value_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                        },
                    ),
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; Transfer::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Transfer::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Transfer::spec_xdr(),
                );
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
        env.sparse_map_new_from_slices(&KEYS, &vals)
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
/// An event whose data is a single value, rather than a map.
pub struct SingleValue {
    from: Address,
    amount: i128,
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_EVENT_SINGLEVALUE: [u8; SingleValue::spec_xdr().len()] = SingleValue::spec_xdr();
impl SingleValue {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"An event whose data is a single value, rather than a map.",
            ),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"SingleValue"),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"single_value"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"amount"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::SingleValue,
        });
    pub const fn spec_xdr() -> [u8; SingleValue::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { SingleValue::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for SingleValue {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &SingleValue::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for SingleValue {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "single_value") }, {
            let v: soroban_sdk::Val = self.from.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::IntoVal;
        self.amount.into_val(env)
    }
}
impl SingleValue {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
/// An event whose data is a single value, and that has no data fields, and so
/// whose data is void.
pub struct SingleValueVoid {
    from: Address,
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_EVENT_SINGLEVALUEVOID: [u8; SingleValueVoid::spec_xdr().len()] =
    SingleValueVoid::spec_xdr();
impl SingleValueVoid {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
            b"An event whose data is a single value, and that has no data fields, and so\nwhose data is void.",
        ),
        lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
            b"SingleValueVoid",
        ),
        prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"single_value_void",
                    ),
                ),
            ],
        ),
        params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"from",
                    ),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
            ],
        ),
        data_format: soroban_sdk::xdr::ScSpecEventDataFormat::SingleValue,
    });
    pub const fn spec_xdr() -> [u8; SingleValueVoid::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { SingleValueVoid::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for SingleValueVoid {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &SingleValueVoid::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for SingleValueVoid {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "single_value_void") }, {
            let v: soroban_sdk::Val = self.from.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        soroban_sdk::Val::VOID.to_val()
    }
}
impl SingleValueVoid {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
/// An event whose data is a vec, rather than a map.
pub struct VecValues {
    from: Address,
    a: u32,
    b: u32,
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_EVENT_VECVALUES: [u8; VecValues::spec_xdr().len()] = VecValues::spec_xdr();
impl VecValues {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"An event whose data is a vec, rather than a map.",
            ),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"VecValues"),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"vec_values"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Vec,
        });
    pub const fn spec_xdr() -> [u8; VecValues::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { VecValues::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for VecValues {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &VecValues::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for VecValues {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "vec_values") }, {
            let v: soroban_sdk::Val = self.from.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::IntoVal;
        (
            {
                let v: soroban_sdk::Val = self.a.into_val(env);
                v
            },
            {
                let v: soroban_sdk::Val = self.b.into_val(env);
                v
            },
        )
            .into_val(env)
    }
}
impl VecValues {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
/// An event whose data is a map, which is the default.
pub struct MapValues {
    from: Address,
    a: u32,
    b: u32,
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_EVENT_MAPVALUES: [u8; MapValues::spec_xdr().len()] = MapValues::spec_xdr();
impl MapValues {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"An event whose data is a map, which is the default.",
            ),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"MapValues"),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"map_values"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; MapValues::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { MapValues::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for MapValues {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &MapValues::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for MapValues {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "map_values") }, {
            let v: soroban_sdk::Val = self.from.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let vals: [soroban_sdk::Val; 2usize] = [self.a.into_val(env), self.b.into_val(env)];
        env.sparse_map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl MapValues {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
impl Contract {
    pub fn single_value(env: Env, from: Address, amount: i128) {
        SingleValue { from, amount }.publish(&env);
    }
    pub fn single_value_void(env: Env, from: Address) {
        SingleValueVoid { from }.publish(&env);
    }
    pub fn vec_values(env: Env, from: Address, a: u32, b: u32) {
        VecValues { from, a, b }.publish(&env);
    }
    pub fn map_values(env: Env, from: Address, a: u32, b: u32) {
        MapValues { from, a, b }.publish(&env);
    }
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
#[allow(dead_code)]
mod __Contract__single_value__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_SINGLE_VALUE: [u8; super::Contract::spec_xdr_single_value().len()] =
        super::Contract::spec_xdr_single_value();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_single_value: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"single_value"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"amount",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_single_value(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_single_value.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_single_value.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__single_value_void__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_SINGLE_VALUE_VOID: [u8; super::Contract::spec_xdr_single_value_void()
        .len()] = super::Contract::spec_xdr_single_value_void();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_single_value_void: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"single_value_void",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_single_value_void(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_single_value_void.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_single_value_void.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__vec_values__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_VEC_VALUES: [u8; super::Contract::spec_xdr_vec_values().len()] =
        super::Contract::spec_xdr_vec_values();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_vec_values: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"vec_values"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_vec_values() -> [u8; Contract::__SPEC_XDR_ENTRY_vec_values.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_vec_values.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__map_values__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_MAP_VALUES: [u8; super::Contract::spec_xdr_map_values().len()] =
        super::Contract::spec_xdr_map_values();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_map_values: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"map_values"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_map_values() -> [u8; Contract::__SPEC_XDR_ENTRY_map_values.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_map_values.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_TRANSFER: [u8; super::Contract::spec_xdr_transfer().len()] =
        super::Contract::spec_xdr_transfer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_transfer: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"transfer"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"to"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::MuxedAddress,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"amount",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_transfer() -> [u8; Contract::__SPEC_XDR_ENTRY_transfer.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_transfer.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__failed_transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_FAILED_TRANSFER: [u8; super::Contract::spec_xdr_failed_transfer().len()] =
        super::Contract::spec_xdr_failed_transfer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_failed_transfer: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"failed_transfer"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"to"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"amount",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_failed_transfer(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_failed_transfer.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_failed_transfer.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn single_value(&self, from: &Address, amount: &i128) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "single_value") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [from.into_val(&self.env), amount.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_single_value(
        &self,
        from: &Address,
        amount: &i128,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "single_value") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [from.into_val(&self.env), amount.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn single_value_void(&self, from: &Address) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "single_value_void") },
            ::soroban_sdk::Vec::from_array(&self.env, [from.into_val(&self.env)]),
        );
        res
    }
    pub fn try_single_value_void(
        &self,
        from: &Address,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "single_value_void") },
            ::soroban_sdk::Vec::from_array(&self.env, [from.into_val(&self.env)]),
        );
        res
    }
    pub fn vec_values(&self, from: &Address, a: &u32, b: &u32) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "vec_values") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    a.into_val(&self.env),
                    b.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_vec_values(
        &self,
        from: &Address,
        a: &u32,
        b: &u32,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "vec_values") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    a.into_val(&self.env),
                    b.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn map_values(&self, from: &Address, a: &u32, b: &u32) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "map_values") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    a.into_val(&self.env),
                    b.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_map_values(
        &self,
        from: &Address,
        a: &u32,
        b: &u32,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "map_values") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    a.into_val(&self.env),
                    b.into_val(&self.env),
                ],
            ),
        );
        res
    }
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
    pub fn single_value<'i>(from: &'i Address, amount: &'i i128) -> (&'i Address, &'i i128) {
        (from, amount)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn single_value_void<'i>(from: &'i Address) -> (&'i Address,) {
        (from,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn vec_values<'i>(
        from: &'i Address,
        a: &'i u32,
        b: &'i u32,
    ) -> (&'i Address, &'i u32, &'i u32) {
        (from, a, b)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn map_values<'i>(
        from: &'i Address,
        a: &'i u32,
        b: &'i u32,
    ) -> (&'i Address, &'i u32, &'i u32) {
        (from, a, b)
    }
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).single_value` instead")]
#[allow(deprecated)]
pub fn __Contract__single_value__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::single_value(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).single_value` instead")]
#[export_name = "single_value"]
pub extern "C" fn __Contract__single_value__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__single_value__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).single_value_void` instead")]
#[allow(deprecated)]
pub fn __Contract__single_value_void__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::single_value_void(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).single_value_void` instead")]
#[export_name = "single_value_void"]
pub extern "C" fn __Contract__single_value_void__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__single_value_void__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).vec_values` instead")]
#[allow(deprecated)]
pub fn __Contract__vec_values__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::vec_values(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).vec_values` instead")]
#[export_name = "vec_values"]
pub extern "C" fn __Contract__vec_values__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__vec_values__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).map_values` instead")]
#[allow(deprecated)]
pub fn __Contract__map_values__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::map_values(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).map_values` instead")]
#[export_name = "map_values"]
pub extern "C" fn __Contract__map_values__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__map_values__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
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
