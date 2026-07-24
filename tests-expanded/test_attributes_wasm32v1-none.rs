#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractevent, contractimpl, contracttrait, contracttype, Env};
pub struct AttributeType {
    pub value: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for AttributeType {
    #[inline]
    fn clone(&self) -> AttributeType {
        AttributeType {
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "AttributeType",
            "value",
            &&self.value,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for AttributeType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AttributeType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for AttributeType {
    #[inline]
    fn eq(&self, other: &AttributeType) -> bool {
        self.value == other.value
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ATTRIBUTETYPE: [u8; 56usize] = AttributeType::spec_xdr();
impl AttributeType {
    pub const fn spec_xdr() -> [u8; 56usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\rAttributeType\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04"
    }
}
impl AttributeType {
    #[doc(hidden)]
    pub const SPEC_XDR_ID: [u8; 8] = [105u8, 9u8, 183u8, 6u8, 42u8, 136u8, 215u8, 248u8];
}
impl soroban_sdk::SpecShakingMarker for AttributeType {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14usize] = *b"SpEcV1i\t\xb7\x06*\x88\xd7\xf8";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for AttributeType {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["value"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            value: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, AttributeType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &AttributeType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["value"];
        let vals: [Val; 1usize] = [(&val.value)
            .try_into_val(env)
            .map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &AttributeType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&AttributeType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, AttributeType>>::try_from_val(env, *val)
    }
}
pub struct AttributeEvent {
    topic: u32,
    value: u32,
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_ATTRIBUTEEVENT: [u8; 112usize] = AttributeEvent::spec_xdr();
impl AttributeEvent {
    pub const fn spec_xdr() -> [u8; 112usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x0eAttributeEvent\0\0\0\0\0\x01\0\0\0\x0fattribute_event\0\0\0\0\x02\0\0\0\0\0\0\0\x05topic\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::SpecShakingMarker for AttributeEvent {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14usize] = *b"SpEcV1\xbfO\xc3P\xd4\x14\xb5V";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for AttributeEvent {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "attribute_event") }, {
            let v: soroban_sdk::Val = self.topic.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["value"];
        let vals: [soroban_sdk::Val; 1usize] = [self.value.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl AttributeEvent {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
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
pub struct AttributeTraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_attribute_trait as AttributeTrait;
pub trait AttributeTrait {
    fn trait_override() -> u32 {
        1
    }
    fn trait_default() -> u32 {
        2
    }
    fn trait_default_stacked_cfg() -> u32 {
        5
    }
}
///AttributeTraitClient is a client for calling the contract defined in "AttributeTrait".
pub struct AttributeTraitClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> AttributeTraitClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> AttributeTraitClient<'a> {
    pub fn trait_override(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_trait_override(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn trait_default(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_trait_default(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn trait_default_stacked_cfg(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_trait_default_stacked_cfg(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
///AttributeTraitArgs is a type for building arg lists for functions defined in "AttributeTrait".
pub struct AttributeTraitArgs;
impl AttributeTraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_override<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default_stacked_cfg<'i>() -> () {
        ()
    }
}
impl AttributeTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_override() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etrait_override\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl AttributeTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rtrait_default\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl AttributeTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default_stacked_cfg() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x19trait_default_stacked_cfg\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl Contract {
    pub fn always(value: AttributeType) -> u32 {
        value.value
    }
    pub fn cfg_included(value: u32) -> u32 {
        value
    }
    pub fn publish(env: Env, topic: u32, value: u32) {
        AttributeEvent { topic, value }.publish(&env);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__always__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_ALWAYS: [u8; 80usize] = super::Contract::spec_xdr_always();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_always() -> [u8; 80usize] {
        let mut bytes = *b"\0\0\0\0\0\0\0\0\0\0\0\x06always\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\x07\xd1\0\0\0\0\0\0\0\0\0\0\0\rAttributeType\0\0\0\0\0\0\x01\0\0\0\x04";
        {
            let id = <AttributeType>::SPEC_XDR_ID;
            let mut i = 0usize;
            while i < 8 {
                bytes[44usize + i] = id[i];
                i += 1;
            }
        }
        bytes
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__cfg_included__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CFG_INCLUDED: [u8; 56usize] = super::Contract::spec_xdr_cfg_included();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_cfg_included() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ccfg_included\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
    }
}
impl Contract {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH: [u8; 68usize] = super::Contract::spec_xdr_publish();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish() -> [u8; 68usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07publish\0\0\0\0\x02\0\0\0\0\0\0\0\x05topic\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn always(&self, value: &AttributeType) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("always");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        res
    }
    pub fn try_always(
        &self,
        value: &AttributeType,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("always");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        res
    }
    pub fn cfg_included(&self, value: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "cfg_included") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        res
    }
    pub fn try_cfg_included(
        &self,
        value: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "cfg_included") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        res
    }
    pub fn publish(&self, topic: &u32, value: &u32) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("publish");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [topic.into_val(&self.env), value.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_publish(
        &self,
        topic: &u32,
        value: &u32,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("publish");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [topic.into_val(&self.env), value.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn always<'i>(value: &'i AttributeType) -> (&'i AttributeType,) {
        (value,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn cfg_included<'i>(value: &'i u32) -> (&'i u32,) {
        (value,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish<'i>(topic: &'i u32, value: &'i u32) -> (&'i u32, &'i u32) {
        (topic, value)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).always` instead")]
#[allow(deprecated)]
pub fn __Contract__always__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::always(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).always` instead")]
#[export_name = "always"]
pub extern "C" fn __Contract__always__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__always__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).cfg_included` instead")]
#[allow(deprecated)]
pub fn __Contract__cfg_included__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::cfg_included(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).cfg_included` instead")]
#[export_name = "cfg_included"]
pub extern "C" fn __Contract__cfg_included__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__cfg_included__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish` instead")]
#[allow(deprecated)]
pub fn __Contract__publish__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish` instead")]
#[export_name = "publish"]
pub extern "C" fn __Contract__publish__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
impl AttributeTrait for Contract {
    fn trait_override() -> u32 {
        3
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__trait_override__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TRAIT_OVERRIDE: [u8; 40usize] =
        super::Contract::spec_xdr_trait_override();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_override() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etrait_override\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl Contract {}
impl Contract {}
impl<'a> ContractClient<'a> {
    pub fn trait_override(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_trait_override(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_override<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_override` instead")]
#[allow(deprecated)]
pub fn __Contract__trait_override__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AttributeTrait>::trait_override(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_override` instead")]
#[export_name = "trait_override"]
pub extern "C" fn __Contract__trait_override__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__trait_override__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_default` instead")]
#[allow(deprecated)]
pub fn __Contract__trait_default__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AttributeTrait>::trait_default(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_default` instead")]
#[export_name = "trait_default"]
pub extern "C" fn __Contract__trait_default__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__trait_default__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractClient::new(&env, &contract_id).trait_default_stacked_cfg` instead"
)]
#[allow(deprecated)]
pub fn __Contract__trait_default_stacked_cfg__invoke_raw(
    env: soroban_sdk::Env,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AttributeTrait>::trait_default_stacked_cfg(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractClient::new(&env, &contract_id).trait_default_stacked_cfg` instead"
)]
#[export_name = "trait_default_stacked_cfg"]
pub extern "C" fn __Contract__trait_default_stacked_cfg__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__trait_default_stacked_cfg__invoke_raw(soroban_sdk::Env::default())
}
impl Contract {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__trait_default__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TRAIT_DEFAULT: [u8; 40usize] =
        super::Contract::spec_xdr_trait_default();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rtrait_default\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__trait_default_stacked_cfg__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TRAIT_DEFAULT_STACKED_CFG: [u8; 52usize] =
        super::Contract::spec_xdr_trait_default_stacked_cfg();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default_stacked_cfg() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x19trait_default_stacked_cfg\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn trait_default(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_trait_default(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn trait_default_stacked_cfg(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_trait_default_stacked_cfg(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default_stacked_cfg<'i>() -> () {
        ()
    }
}
