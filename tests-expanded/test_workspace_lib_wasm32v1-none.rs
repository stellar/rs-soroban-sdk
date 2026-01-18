#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::contracttype;
pub struct Value {
    pub value: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Value {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Value", "value", &&self.value)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Value {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Value {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Value {
    #[inline]
    fn eq(&self, other: &Value) -> bool {
        self.value == other.value
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_VALUE: [u8; 48usize] = Value::spec_xdr();
impl Value {
    pub const fn spec_xdr() -> [u8; 48usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x05Value\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x05"
    }
}
impl soroban_sdk::IncludeSpecMarker for Value {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <i32 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\x82\xf8t\xbe\t\x04b\\";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Value {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Value> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Value,
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Value> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Value,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Value>>::try_from_val(env, *val)
    }
}
