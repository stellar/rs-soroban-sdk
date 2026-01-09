#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contracterror, contractevent, contracttype, Address, Vec};
pub struct StructA {
    pub f1: u32,
    pub f2: bool,
}
#[automatically_derived]
impl ::core::clone::Clone for StructA {
    #[inline]
    fn clone(&self) -> StructA {
        StructA {
            f1: ::core::clone::Clone::clone(&self.f1),
            f2: ::core::clone::Clone::clone(&self.f2),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "StructA", "f1", &self.f1, "f2", &&self.f2,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for StructA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<bool>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructA {
    #[inline]
    fn eq(&self, other: &StructA) -> bool {
        self.f1 == other.f1 && self.f2 == other.f2
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTA: [u8; 60usize] = StructA::spec_xdr();
impl StructA {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructA\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01"
    }
}
impl soroban_sdk::IncludeSpecMarker for StructA {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xb6\x1c\xfd\xdfhY-d";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructA {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["f1", "f2"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            f1: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            f2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &StructA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["f1", "f2"];
        let vals: [Val; 2usize] = [
            (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&StructA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructA>>::try_from_val(env, *val)
    }
}
pub struct StructB {
    pub f1: i64,
    pub f2: soroban_sdk::String,
}
#[automatically_derived]
impl ::core::clone::Clone for StructB {
    #[inline]
    fn clone(&self) -> StructB {
        StructB {
            f1: ::core::clone::Clone::clone(&self.f1),
            f2: ::core::clone::Clone::clone(&self.f2),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructB {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "StructB", "f1", &self.f1, "f2", &&self.f2,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for StructB {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
        let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructB {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructB {
    #[inline]
    fn eq(&self, other: &StructB) -> bool {
        self.f1 == other.f1 && self.f2 == other.f2
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTB: [u8; 60usize] = StructB::spec_xdr();
impl StructB {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructB\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10"
    }
}
impl soroban_sdk::IncludeSpecMarker for StructB {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xf3\xc4\xd3\x8c\xc1w\xe9\x18";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructB {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["f1", "f2"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            f1: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            f2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &StructB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["f1", "f2"];
        let vals: [Val; 2usize] = [
            (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&StructB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructB>>::try_from_val(env, *val)
    }
}
pub struct StructC {
    pub f1: Vec<u32>,
    pub f2: Address,
}
#[automatically_derived]
impl ::core::clone::Clone for StructC {
    #[inline]
    fn clone(&self) -> StructC {
        StructC {
            f1: ::core::clone::Clone::clone(&self.f1),
            f2: ::core::clone::Clone::clone(&self.f2),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "StructC", "f1", &self.f1, "f2", &&self.f2,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for StructC {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Vec<u32>>;
        let _: ::core::cmp::AssertParamIsEq<Address>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructC {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructC {
    #[inline]
    fn eq(&self, other: &StructC) -> bool {
        self.f1 == other.f1 && self.f2 == other.f2
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTC: [u8; 64usize] = StructC::spec_xdr();
impl StructC {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructC\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\x03\xea\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13"
    }
}
impl soroban_sdk::IncludeSpecMarker for StructC {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xa3\x16\n\x8f\xc9\x92\xd2\x11";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructC {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["f1", "f2"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            f1: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            f2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &StructC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["f1", "f2"];
        let vals: [Val; 2usize] = [
            (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&StructC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructC>>::try_from_val(env, *val)
    }
}
pub struct StructTupleA(pub i64, pub i64);
#[automatically_derived]
impl ::core::clone::Clone for StructTupleA {
    #[inline]
    fn clone(&self) -> StructTupleA {
        StructTupleA(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructTupleA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleA", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for StructTupleA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructTupleA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructTupleA {
    #[inline]
    fn eq(&self, other: &StructTupleA) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTTUPLEA: [u8; 64usize] = StructTupleA::spec_xdr();
impl StructTupleA {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleA\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\x07"
    }
}
impl soroban_sdk::IncludeSpecMarker for StructTupleA {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xcf)\x97]S\xb2\xfd)";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleA {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
            1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &StructTupleA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 2usize] = [
            (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&StructTupleA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleA>>::try_from_val(env, *val)
    }
}
pub struct StructTupleB(pub u128, pub u128);
#[automatically_derived]
impl ::core::clone::Clone for StructTupleB {
    #[inline]
    fn clone(&self) -> StructTupleB {
        StructTupleB(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructTupleB {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleB", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for StructTupleB {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u128>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructTupleB {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructTupleB {
    #[inline]
    fn eq(&self, other: &StructTupleB) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTTUPLEB: [u8; 64usize] = StructTupleB::spec_xdr();
impl StructTupleB {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleB\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\n\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\n"
    }
}
impl soroban_sdk::IncludeSpecMarker for StructTupleB {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEcx\xd98\x9c\x1ao\xac\x8c";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleB {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
            1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &StructTupleB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 2usize] = [
            (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&StructTupleB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleB>>::try_from_val(env, *val)
    }
}
pub struct StructTupleC(pub Address, pub i128);
#[automatically_derived]
impl ::core::clone::Clone for StructTupleC {
    #[inline]
    fn clone(&self) -> StructTupleC {
        StructTupleC(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for StructTupleC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleC", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for StructTupleC {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Address>;
        let _: ::core::cmp::AssertParamIsEq<i128>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for StructTupleC {}
#[automatically_derived]
impl ::core::cmp::PartialEq for StructTupleC {
    #[inline]
    fn eq(&self, other: &StructTupleC) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTTUPLEC: [u8; 64usize] = StructTupleC::spec_xdr();
impl StructTupleC {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleC\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\x0b"
    }
}
impl soroban_sdk::IncludeSpecMarker for StructTupleC {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xc5=\x81\xc1\"\xafT\xd9";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleC {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
            1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &StructTupleC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 2usize] = [
            (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&StructTupleC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleC>>::try_from_val(env, *val)
    }
}
pub enum EnumA {
    V1,
    V2,
    V3,
}
#[automatically_derived]
impl ::core::clone::Clone for EnumA {
    #[inline]
    fn clone(&self) -> EnumA {
        match self {
            EnumA::V1 => EnumA::V1,
            EnumA::V2 => EnumA::V2,
            EnumA::V3 => EnumA::V3,
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumA::V1 => "V1",
                EnumA::V2 => "V2",
                EnumA::V3 => "V3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumA {
    #[inline]
    fn eq(&self, other: &EnumA) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMA: [u8; 76usize] = EnumA::spec_xdr();
impl EnumA {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumA\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\0\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\0\0\0\0\0\0\0\0\x02V3\0\0"
    }
}
impl soroban_sdk::IncludeSpecMarker for EnumA {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xa2=N\xc1p\x95\x90\xb2";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumA {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
        let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
        let mut iter = vec.try_iter();
        let discriminant: soroban_sdk::Symbol = iter
            .next()
            .ok_or(soroban_sdk::ConversionError)??
            .try_into_val(env)
            .map_err(|_| soroban_sdk::ConversionError)?;
        Ok(
            match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?) as usize
            {
                0 => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V1
                }
                1 => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V2
                }
                2 => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V3
                }
                _ => Err(soroban_sdk::ConversionError {})?,
            },
        )
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &EnumA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            EnumA::V1 => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            EnumA::V2 => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            EnumA::V3 => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&EnumA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumA>>::try_from_val(env, *val)
    }
}
pub enum EnumB {
    V1,
    V2(i64),
    V3(i64, i64),
}
#[automatically_derived]
impl ::core::clone::Clone for EnumB {
    #[inline]
    fn clone(&self) -> EnumB {
        match self {
            EnumB::V1 => EnumB::V1,
            EnumB::V2(__self_0) => EnumB::V2(::core::clone::Clone::clone(__self_0)),
            EnumB::V3(__self_0, __self_1) => EnumB::V3(
                ::core::clone::Clone::clone(__self_0),
                ::core::clone::Clone::clone(__self_1),
            ),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumB {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumB::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
            EnumB::V2(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
            }
            EnumB::V3(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(f, "V3", __self_0, &__self_1)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumB {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumB {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumB {
    #[inline]
    fn eq(&self, other: &EnumB) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => __self_0 == __arg1_0,
                (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                _ => true,
            }
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMB: [u8; 96usize] = EnumB::spec_xdr();
impl EnumB {
    pub const fn spec_xdr() -> [u8; 96usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumB\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x01\0\0\0\x07\0\0\0\x01\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x02\0\0\0\x07\0\0\0\x07"
    }
}
impl soroban_sdk::IncludeSpecMarker for EnumB {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc'\x1b\0DSH^\xcc";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumB {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
        let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
        let mut iter = vec.try_iter();
        let discriminant: soroban_sdk::Symbol = iter
            .next()
            .ok_or(soroban_sdk::ConversionError)??
            .try_into_val(env)
            .map_err(|_| soroban_sdk::ConversionError)?;
        Ok(
            match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?) as usize
            {
                0 => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V1
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V2(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 2usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V3(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                _ => Err(soroban_sdk::ConversionError {})?,
            },
        )
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &EnumB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            EnumB::V1 => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            EnumB::V2(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            EnumB::V3(ref value0, ref value1) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),
                    value0.try_into_val(env)?,
                    value1.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&EnumB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumB>>::try_from_val(env, *val)
    }
}
pub enum EnumC {
    V1,
    V2(StructA),
    V3(StructTupleA),
}
#[automatically_derived]
impl ::core::clone::Clone for EnumC {
    #[inline]
    fn clone(&self) -> EnumC {
        match self {
            EnumC::V1 => EnumC::V1,
            EnumC::V2(__self_0) => EnumC::V2(::core::clone::Clone::clone(__self_0)),
            EnumC::V3(__self_0) => EnumC::V3(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumC::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
            EnumC::V2(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
            }
            EnumC::V3(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V3", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumC {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<StructA>;
        let _: ::core::cmp::AssertParamIsEq<StructTupleA>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumC {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumC {
    #[inline]
    fn eq(&self, other: &EnumC) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => __self_0 == __arg1_0,
                (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMC: [u8; 120usize] = EnumC::spec_xdr();
impl EnumC {
    pub const fn spec_xdr() -> [u8; 120usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumC\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0\0\0\0\x01\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA"
    }
}
impl soroban_sdk::IncludeSpecMarker for EnumC {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xa0\xdd\x8f\xdc\xc9W\xbe\xc2";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumC {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
        let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
        let mut iter = vec.try_iter();
        let discriminant: soroban_sdk::Symbol = iter
            .next()
            .ok_or(soroban_sdk::ConversionError)??
            .try_into_val(env)
            .map_err(|_| soroban_sdk::ConversionError)?;
        Ok(
            match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?) as usize
            {
                0 => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V1
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V2(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::V3(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                _ => Err(soroban_sdk::ConversionError {})?,
            },
        )
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &EnumC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            EnumC::V1 => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            EnumC::V2(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            EnumC::V3(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&EnumC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumC>>::try_from_val(env, *val)
    }
}
pub enum EnumIntA {
    V1 = 1,
    V2 = 2,
    V3 = 3,
}
#[automatically_derived]
impl ::core::marker::Copy for EnumIntA {}
#[automatically_derived]
impl ::core::clone::Clone for EnumIntA {
    #[inline]
    fn clone(&self) -> EnumIntA {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumIntA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumIntA::V1 => "V1",
                EnumIntA::V2 => "V2",
                EnumIntA::V3 => "V3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumIntA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumIntA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumIntA {
    #[inline]
    fn eq(&self, other: &EnumIntA) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMINTA: [u8; 76usize] = EnumIntA::spec_xdr();
impl EnumIntA {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntA\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x03"
    }
}
impl soroban_sdk::IncludeSpecMarker for EnumIntA {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEcV]\x80\\~\x1a\x08/";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntA {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::V1,
            2u32 => Self::V2,
            3u32 => Self::V3,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &EnumIntA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            EnumIntA::V1 => 1u32.into(),
            EnumIntA::V2 => 2u32.into(),
            EnumIntA::V3 => 3u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&EnumIntA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntA>>::try_from_val(env, *val)
    }
}
pub enum EnumIntB {
    V1 = 10,
    V2 = 20,
    V3 = 30,
}
#[automatically_derived]
impl ::core::marker::Copy for EnumIntB {}
#[automatically_derived]
impl ::core::clone::Clone for EnumIntB {
    #[inline]
    fn clone(&self) -> EnumIntB {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumIntB {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumIntB::V1 => "V1",
                EnumIntB::V2 => "V2",
                EnumIntB::V3 => "V3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumIntB {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumIntB {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumIntB {
    #[inline]
    fn eq(&self, other: &EnumIntB) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMINTB: [u8; 76usize] = EnumIntB::spec_xdr();
impl EnumIntB {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntB\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x14\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x1e"
    }
}
impl soroban_sdk::IncludeSpecMarker for EnumIntB {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc,\x9c\xc0_\xed_)\x85";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntB {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            10u32 => Self::V1,
            20u32 => Self::V2,
            30u32 => Self::V3,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &EnumIntB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            EnumIntB::V1 => 10u32.into(),
            EnumIntB::V2 => 20u32.into(),
            EnumIntB::V3 => 30u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&EnumIntB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntB>>::try_from_val(env, *val)
    }
}
pub enum EnumIntC {
    V1 = 100,
    V2 = 200,
    V3 = 300,
}
#[automatically_derived]
impl ::core::marker::Copy for EnumIntC {}
#[automatically_derived]
impl ::core::clone::Clone for EnumIntC {
    #[inline]
    fn clone(&self) -> EnumIntC {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EnumIntC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                EnumIntC::V1 => "V1",
                EnumIntC::V2 => "V2",
                EnumIntC::V3 => "V3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EnumIntC {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EnumIntC {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EnumIntC {
    #[inline]
    fn eq(&self, other: &EnumIntC) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMINTC: [u8; 76usize] = EnumIntC::spec_xdr();
impl EnumIntC {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntC\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0d\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\xc8\0\0\0\0\0\0\0\x02V3\0\0\0\0\x01,"
    }
}
impl soroban_sdk::IncludeSpecMarker for EnumIntC {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc`\xca\xda\x19\xb9c\xf0/";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntC {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            100u32 => Self::V1,
            200u32 => Self::V2,
            300u32 => Self::V3,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &EnumIntC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            EnumIntC::V1 => 100u32.into(),
            EnumIntC::V2 => 200u32.into(),
            EnumIntC::V3 => 300u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&EnumIntC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntC>>::try_from_val(env, *val)
    }
}
pub enum ErrorA {
    E1 = 1,
    E2 = 2,
    E3 = 3,
}
#[automatically_derived]
impl ::core::marker::Copy for ErrorA {}
#[automatically_derived]
impl ::core::clone::Clone for ErrorA {
    #[inline]
    fn clone(&self) -> ErrorA {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ErrorA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                ErrorA::E1 => "E1",
                ErrorA::E2 => "E2",
                ErrorA::E3 => "E3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ErrorA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ErrorA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ErrorA {
    #[inline]
    fn eq(&self, other: &ErrorA) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERRORA: [u8; 76usize] = ErrorA::spec_xdr();
impl ErrorA {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorA\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02E2\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02E3\0\0\0\0\0\x03"
    }
}
impl soroban_sdk::IncludeSpecMarker for ErrorA {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xe9R\xa7\xe8b\x99\xa2\xc3";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl TryFrom<soroban_sdk::Error> for ErrorA {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::E1,
                2u32 => Self::E2,
                3u32 => Self::E3,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for ErrorA {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<ErrorA> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: ErrorA) -> soroban_sdk::Error {
        <_ as From<&ErrorA>>::from(&val)
    }
}
impl From<&ErrorA> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &ErrorA) -> soroban_sdk::Error {
        match val {
            ErrorA::E1 => soroban_sdk::Error::from_contract_error(1u32),
            ErrorA::E2 => soroban_sdk::Error::from_contract_error(2u32),
            ErrorA::E3 => soroban_sdk::Error::from_contract_error(3u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for ErrorA {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                1u32 => Self::E1,
                2u32 => Self::E2,
                3u32 => Self::E3,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for ErrorA {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<ErrorA> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: ErrorA) -> soroban_sdk::InvokeError {
        <_ as From<&ErrorA>>::from(&val)
    }
}
impl From<&ErrorA> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &ErrorA) -> soroban_sdk::InvokeError {
        match val {
            ErrorA::E1 => soroban_sdk::InvokeError::Contract(1u32),
            ErrorA::E2 => soroban_sdk::InvokeError::Contract(2u32),
            ErrorA::E3 => soroban_sdk::InvokeError::Contract(3u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorA {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let error: soroban_sdk::Error = val.try_into_val(env)?;
        error.try_into().map_err(|_| soroban_sdk::ConversionError)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &ErrorA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorA> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&ErrorA,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorA>>::try_from_val(env, *val)
    }
}
pub enum ErrorB {
    E1 = 10,
    E2 = 11,
    E3 = 12,
}
#[automatically_derived]
impl ::core::marker::Copy for ErrorB {}
#[automatically_derived]
impl ::core::clone::Clone for ErrorB {
    #[inline]
    fn clone(&self) -> ErrorB {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ErrorB {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                ErrorB::E1 => "E1",
                ErrorB::E2 => "E2",
                ErrorB::E3 => "E3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ErrorB {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ErrorB {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ErrorB {
    #[inline]
    fn eq(&self, other: &ErrorB) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERRORB: [u8; 76usize] = ErrorB::spec_xdr();
impl ErrorB {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorB\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02E2\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02E3\0\0\0\0\0\x0c"
    }
}
impl soroban_sdk::IncludeSpecMarker for ErrorB {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\x1d1\xd6\xfb\x88\xd2=\xe3";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl TryFrom<soroban_sdk::Error> for ErrorB {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                10u32 => Self::E1,
                11u32 => Self::E2,
                12u32 => Self::E3,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for ErrorB {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<ErrorB> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: ErrorB) -> soroban_sdk::Error {
        <_ as From<&ErrorB>>::from(&val)
    }
}
impl From<&ErrorB> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &ErrorB) -> soroban_sdk::Error {
        match val {
            ErrorB::E1 => soroban_sdk::Error::from_contract_error(10u32),
            ErrorB::E2 => soroban_sdk::Error::from_contract_error(11u32),
            ErrorB::E3 => soroban_sdk::Error::from_contract_error(12u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for ErrorB {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                10u32 => Self::E1,
                11u32 => Self::E2,
                12u32 => Self::E3,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for ErrorB {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<ErrorB> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: ErrorB) -> soroban_sdk::InvokeError {
        <_ as From<&ErrorB>>::from(&val)
    }
}
impl From<&ErrorB> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &ErrorB) -> soroban_sdk::InvokeError {
        match val {
            ErrorB::E1 => soroban_sdk::InvokeError::Contract(10u32),
            ErrorB::E2 => soroban_sdk::InvokeError::Contract(11u32),
            ErrorB::E3 => soroban_sdk::InvokeError::Contract(12u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorB {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let error: soroban_sdk::Error = val.try_into_val(env)?;
        error.try_into().map_err(|_| soroban_sdk::ConversionError)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &ErrorB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorB> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&ErrorB,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorB>>::try_from_val(env, *val)
    }
}
pub enum ErrorC {
    E1 = 100,
    E2 = 101,
    E3 = 102,
}
#[automatically_derived]
impl ::core::marker::Copy for ErrorC {}
#[automatically_derived]
impl ::core::clone::Clone for ErrorC {
    #[inline]
    fn clone(&self) -> ErrorC {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ErrorC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                ErrorC::E1 => "E1",
                ErrorC::E2 => "E2",
                ErrorC::E3 => "E3",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ErrorC {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ErrorC {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ErrorC {
    #[inline]
    fn eq(&self, other: &ErrorC) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERRORC: [u8; 76usize] = ErrorC::spec_xdr();
impl ErrorC {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorC\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0d\0\0\0\0\0\0\0\x02E2\0\0\0\0\0e\0\0\0\0\0\0\0\x02E3\0\0\0\0\0f"
    }
}
impl soroban_sdk::IncludeSpecMarker for ErrorC {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xb9\x01\xafj\xe0c\xa3\r";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl TryFrom<soroban_sdk::Error> for ErrorC {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                100u32 => Self::E1,
                101u32 => Self::E2,
                102u32 => Self::E3,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for ErrorC {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<ErrorC> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: ErrorC) -> soroban_sdk::Error {
        <_ as From<&ErrorC>>::from(&val)
    }
}
impl From<&ErrorC> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &ErrorC) -> soroban_sdk::Error {
        match val {
            ErrorC::E1 => soroban_sdk::Error::from_contract_error(100u32),
            ErrorC::E2 => soroban_sdk::Error::from_contract_error(101u32),
            ErrorC::E3 => soroban_sdk::Error::from_contract_error(102u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for ErrorC {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                100u32 => Self::E1,
                101u32 => Self::E2,
                102u32 => Self::E3,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for ErrorC {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<ErrorC> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: ErrorC) -> soroban_sdk::InvokeError {
        <_ as From<&ErrorC>>::from(&val)
    }
}
impl From<&ErrorC> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &ErrorC) -> soroban_sdk::InvokeError {
        match val {
            ErrorC::E1 => soroban_sdk::InvokeError::Contract(100u32),
            ErrorC::E2 => soroban_sdk::InvokeError::Contract(101u32),
            ErrorC::E3 => soroban_sdk::InvokeError::Contract(102u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorC {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let error: soroban_sdk::Error = val.try_into_val(env)?;
        error.try_into().map_err(|_| soroban_sdk::ConversionError)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &ErrorC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorC> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&ErrorC,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorC>>::try_from_val(env, *val)
    }
}
pub struct EventA {
    pub f1: Address,
    pub f2: soroban_sdk::String,
}
#[automatically_derived]
impl ::core::clone::Clone for EventA {
    #[inline]
    fn clone(&self) -> EventA {
        EventA {
            f1: ::core::clone::Clone::clone(&self.f1),
            f2: ::core::clone::Clone::clone(&self.f2),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EventA {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "EventA", "f1", &self.f1, "f2", &&self.f2,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EventA {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Address>;
        let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EventA {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EventA {
    #[inline]
    fn eq(&self, other: &EventA) -> bool {
        self.f1 == other.f1 && self.f2 == other.f2
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_EVENTA: [u8; 88usize] = EventA::spec_xdr();
impl EventA {
    pub const fn spec_xdr() -> [u8; 88usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventA\0\0\0\0\0\x01\0\0\0\x07event_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::IncludeSpecMarker for EventA {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEcK\xe6\x8ej\x19\x9en\xbd";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for EventA {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_a");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.f1.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["f2"];
        let vals: [soroban_sdk::Val; 1usize] = [self.f2.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl EventA {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct EventB {
    pub f1: Address,
    pub f2: Address,
    pub f3: i128,
}
#[automatically_derived]
impl ::core::clone::Clone for EventB {
    #[inline]
    fn clone(&self) -> EventB {
        EventB {
            f1: ::core::clone::Clone::clone(&self.f1),
            f2: ::core::clone::Clone::clone(&self.f2),
            f3: ::core::clone::Clone::clone(&self.f3),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EventB {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f, "EventB", "f1", &self.f1, "f2", &self.f2, "f3", &&self.f3,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EventB {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Address>;
        let _: ::core::cmp::AssertParamIsEq<i128>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EventB {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EventB {
    #[inline]
    fn eq(&self, other: &EventB) -> bool {
        self.f1 == other.f1 && self.f2 == other.f2 && self.f3 == other.f3
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_EVENTB: [u8; 108usize] = EventB::spec_xdr();
impl EventB {
    pub const fn spec_xdr() -> [u8; 108usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventB\0\0\0\0\0\x01\0\0\0\x07event_b\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::IncludeSpecMarker for EventB {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xe6\xaa\xefz\x17i$\x15";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for EventB {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_b");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.f1.into_val(env);
                v
            },
            {
                let v: soroban_sdk::Val = self.f2.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["f3"];
        let vals: [soroban_sdk::Val; 1usize] = [self.f3.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl EventB {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct EventC {
    pub f1: soroban_sdk::Symbol,
    pub f2: i64,
    pub f3: i64,
}
#[automatically_derived]
impl ::core::clone::Clone for EventC {
    #[inline]
    fn clone(&self) -> EventC {
        EventC {
            f1: ::core::clone::Clone::clone(&self.f1),
            f2: ::core::clone::Clone::clone(&self.f2),
            f3: ::core::clone::Clone::clone(&self.f3),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EventC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f, "EventC", "f1", &self.f1, "f2", &self.f2, "f3", &&self.f3,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EventC {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EventC {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EventC {
    #[inline]
    fn eq(&self, other: &EventC) -> bool {
        self.f1 == other.f1 && self.f2 == other.f2 && self.f3 == other.f3
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_EVENTC: [u8; 108usize] = EventC::spec_xdr();
impl EventC {
    pub const fn spec_xdr() -> [u8; 108usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventC\0\0\0\0\0\x01\0\0\0\x07event_c\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::IncludeSpecMarker for EventC {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\x16\xd6\xdf\xe7\xdb\xb4W@";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for EventC {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_c");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.f1.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 2usize] = ["f2", "f3"];
        let vals: [soroban_sdk::Val; 2usize] = [self.f2.into_val(env), self.f3.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl EventC {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
