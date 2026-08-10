#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
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
    fn assert_fields_are_eq(&self) {
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
impl StructA {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::StructA")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTA: [u8; StructA::__SPEC_XDR_VIEW.const_xdr_len()] =
    StructA::spec_xdr();
impl StructA {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"StructA"),
            fields: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U32,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Bool,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; StructA::__SPEC_XDR_VIEW.const_xdr_len()] {
        StructA::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for StructA {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <bool as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&StructA::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
impl StructB {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::StructB")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTB: [u8; StructB::__SPEC_XDR_VIEW.const_xdr_len()] =
    StructB::spec_xdr();
impl StructB {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"StructB"),
            fields: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::String,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; StructB::__SPEC_XDR_VIEW.const_xdr_len()] {
        StructB::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for StructB {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <soroban_sdk::String as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&StructB::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
impl StructC {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::StructC")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTC: [u8; StructC::__SPEC_XDR_VIEW.const_xdr_len()] =
    StructC::spec_xdr();
impl StructC {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"StructC"),
            fields: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::U32,
                        },
                    ),
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; StructC::__SPEC_XDR_VIEW.const_xdr_len()] {
        StructC::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for StructC {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Vec<u32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&StructC::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
impl StructTupleA {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::StructTupleA")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTTUPLEA: [u8; StructTupleA::__SPEC_XDR_VIEW.const_xdr_len()] =
    StructTupleA::spec_xdr();
impl StructTupleA {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"StructTupleA"),
            fields: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"0"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; StructTupleA::__SPEC_XDR_VIEW.const_xdr_len()] {
        StructTupleA::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for StructTupleA {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&StructTupleA::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
impl StructTupleB {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::StructTupleB")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTTUPLEB: [u8; StructTupleB::__SPEC_XDR_VIEW.const_xdr_len()] =
    StructTupleB::spec_xdr();
impl StructTupleB {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"StructTupleB"),
            fields: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"0"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U128,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U128,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; StructTupleB::__SPEC_XDR_VIEW.const_xdr_len()] {
        StructTupleB::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for StructTupleB {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <u128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&StructTupleB::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
        self.1 == other.1 && self.0 == other.0
    }
}
impl StructTupleC {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::StructTupleC")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_STRUCTTUPLEC: [u8; StructTupleC::__SPEC_XDR_VIEW.const_xdr_len()] =
    StructTupleC::spec_xdr();
impl StructTupleC {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"StructTupleC"),
            fields: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"0"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; StructTupleC::__SPEC_XDR_VIEW.const_xdr_len()] {
        StructTupleC::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for StructTupleC {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&StructTupleC::spec_xdr());
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
    fn assert_fields_are_eq(&self) {}
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
impl EnumA {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::EnumA")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMA: [u8; EnumA::__SPEC_XDR_VIEW.const_xdr_len()] = EnumA::spec_xdr();
impl EnumA {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtUnionV0(soroban_sdk::xdr::ScSpecUdtUnionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"EnumA"),
            cases: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V1"),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V2"),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V3"),
                    },
                ),
            ]),
        });
    pub const fn spec_xdr() -> [u8; EnumA::__SPEC_XDR_VIEW.const_xdr_len()] {
        EnumA::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EnumA {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EnumA::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
impl EnumB {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::EnumB")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMB: [u8; EnumB::__SPEC_XDR_VIEW.const_xdr_len()] = EnumB::spec_xdr();
impl EnumB {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtUnionV0(soroban_sdk::xdr::ScSpecUdtUnionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"EnumB"),
            cases: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V1"),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V2"),
                        type_: soroban_sdk::xdr::VecMView::new(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::I64,
                        ]),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V3"),
                        type_: soroban_sdk::xdr::VecMView::new(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::I64,
                            soroban_sdk::xdr::ScSpecTypeDefView::I64,
                        ]),
                    },
                ),
            ]),
        });
    pub const fn spec_xdr() -> [u8; EnumB::__SPEC_XDR_VIEW.const_xdr_len()] {
        EnumB::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EnumB {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EnumB::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
impl EnumC {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::EnumC")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMC: [u8; EnumC::__SPEC_XDR_VIEW.const_xdr_len()] = EnumC::spec_xdr();
impl EnumC {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtUnionV0(soroban_sdk::xdr::ScSpecUdtUnionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"EnumC"),
            cases: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V1"),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V2"),
                        type_: soroban_sdk::xdr::VecMView::new(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::UdtV2(
                                soroban_sdk::xdr::ScSpecTypeUdtv2View {
                                    name: soroban_sdk::xdr::StringMView::new(b"StructA"),
                                    id: <StructA>::spec_type_id(),
                                },
                            ),
                        ]),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"V3"),
                        type_: soroban_sdk::xdr::VecMView::new(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::UdtV2(
                                soroban_sdk::xdr::ScSpecTypeUdtv2View {
                                    name: soroban_sdk::xdr::StringMView::new(b"StructTupleA"),
                                    id: <StructTupleA>::spec_type_id(),
                                },
                            ),
                        ]),
                    },
                ),
            ]),
        });
    pub const fn spec_xdr() -> [u8; EnumC::__SPEC_XDR_VIEW.const_xdr_len()] {
        EnumC::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EnumC {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <StructA as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <StructTupleA as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EnumC::spec_xdr());
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
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for EnumIntA {}
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
    fn assert_fields_are_eq(&self) {}
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
impl EnumIntA {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::EnumIntA")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMINTA: [u8; EnumIntA::__SPEC_XDR_VIEW.const_xdr_len()] =
    EnumIntA::spec_xdr();
impl EnumIntA {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtEnumV0(soroban_sdk::xdr::ScSpecUdtEnumV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"EnumIntA"),
            cases: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V1"),
                    value: 1u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V2"),
                    value: 2u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V3"),
                    value: 3u32,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; EnumIntA::__SPEC_XDR_VIEW.const_xdr_len()] {
        EnumIntA::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EnumIntA {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EnumIntA::spec_xdr());
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
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for EnumIntB {}
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
    fn assert_fields_are_eq(&self) {}
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
impl EnumIntB {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::EnumIntB")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMINTB: [u8; EnumIntB::__SPEC_XDR_VIEW.const_xdr_len()] =
    EnumIntB::spec_xdr();
impl EnumIntB {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtEnumV0(soroban_sdk::xdr::ScSpecUdtEnumV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"EnumIntB"),
            cases: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V1"),
                    value: 10u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V2"),
                    value: 20u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V3"),
                    value: 30u32,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; EnumIntB::__SPEC_XDR_VIEW.const_xdr_len()] {
        EnumIntB::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EnumIntB {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EnumIntB::spec_xdr());
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
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for EnumIntC {}
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
    fn assert_fields_are_eq(&self) {}
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
impl EnumIntC {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::EnumIntC")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ENUMINTC: [u8; EnumIntC::__SPEC_XDR_VIEW.const_xdr_len()] =
    EnumIntC::spec_xdr();
impl EnumIntC {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtEnumV0(soroban_sdk::xdr::ScSpecUdtEnumV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::StringMView::new(b"EnumIntC"),
            cases: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V1"),
                    value: 100u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V2"),
                    value: 200u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"V3"),
                    value: 300u32,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; EnumIntC::__SPEC_XDR_VIEW.const_xdr_len()] {
        EnumIntC::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EnumIntC {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EnumIntC::spec_xdr());
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
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for ErrorA {}
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
    fn assert_fields_are_eq(&self) {}
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
impl ErrorA {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::ErrorA")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERRORA: [u8; ErrorA::__SPEC_XDR_VIEW.const_xdr_len()] =
    ErrorA::spec_xdr();
impl ErrorA {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtErrorEnumV0(
            soroban_sdk::xdr::ScSpecUdtErrorEnumV0View {
                doc: soroban_sdk::xdr::StringMView::new(b""),
                lib: soroban_sdk::xdr::StringMView::new(b""),
                name: soroban_sdk::xdr::StringMView::new(b"ErrorA"),
                cases: soroban_sdk::xdr::VecMView::new(&[
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E1"),
                        value: 1u32,
                    },
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E2"),
                        value: 2u32,
                    },
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E3"),
                        value: 3u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; ErrorA::__SPEC_XDR_VIEW.const_xdr_len()] {
        ErrorA::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for ErrorA {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&ErrorA::spec_xdr());
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
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for ErrorB {}
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
    fn assert_fields_are_eq(&self) {}
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
impl ErrorB {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::ErrorB")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERRORB: [u8; ErrorB::__SPEC_XDR_VIEW.const_xdr_len()] =
    ErrorB::spec_xdr();
impl ErrorB {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtErrorEnumV0(
            soroban_sdk::xdr::ScSpecUdtErrorEnumV0View {
                doc: soroban_sdk::xdr::StringMView::new(b""),
                lib: soroban_sdk::xdr::StringMView::new(b""),
                name: soroban_sdk::xdr::StringMView::new(b"ErrorB"),
                cases: soroban_sdk::xdr::VecMView::new(&[
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E1"),
                        value: 10u32,
                    },
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E2"),
                        value: 11u32,
                    },
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E3"),
                        value: 12u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; ErrorB::__SPEC_XDR_VIEW.const_xdr_len()] {
        ErrorB::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for ErrorB {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&ErrorB::spec_xdr());
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
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for ErrorC {}
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
    fn assert_fields_are_eq(&self) {}
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
impl ErrorC {
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        soroban_sdk::spec_type_id("test_spec_lib::ErrorC")
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERRORC: [u8; ErrorC::__SPEC_XDR_VIEW.const_xdr_len()] =
    ErrorC::spec_xdr();
impl ErrorC {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtErrorEnumV0(
            soroban_sdk::xdr::ScSpecUdtErrorEnumV0View {
                doc: soroban_sdk::xdr::StringMView::new(b""),
                lib: soroban_sdk::xdr::StringMView::new(b""),
                name: soroban_sdk::xdr::StringMView::new(b"ErrorC"),
                cases: soroban_sdk::xdr::VecMView::new(&[
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E1"),
                        value: 100u32,
                    },
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E2"),
                        value: 101u32,
                    },
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0View {
                        doc: soroban_sdk::xdr::StringMView::new(b""),
                        name: soroban_sdk::xdr::StringMView::new(b"E3"),
                        value: 102u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; ErrorC::__SPEC_XDR_VIEW.const_xdr_len()] {
        ErrorC::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for ErrorC {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&ErrorC::spec_xdr());
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
    fn assert_fields_are_eq(&self) {
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
pub static __SPEC_XDR_EVENT_EVENTA: [u8; EventA::__SPEC_XDR_VIEW.const_xdr_len()] =
    EventA::spec_xdr();
impl EventA {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::EventV0(soroban_sdk::xdr::ScSpecEventV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(b"EventA")),
            prefix_topics: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::new(b"event_a"),
            )]),
            params: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::String,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; EventA::__SPEC_XDR_VIEW.const_xdr_len()] {
        EventA::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EventA {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <soroban_sdk::String as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EventA::spec_xdr());
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
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
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
    fn assert_fields_are_eq(&self) {
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
        self.f3 == other.f3 && self.f1 == other.f1 && self.f2 == other.f2
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_EVENTB: [u8; EventB::__SPEC_XDR_VIEW.const_xdr_len()] =
    EventB::spec_xdr();
impl EventB {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::EventV0(soroban_sdk::xdr::ScSpecEventV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(b"EventB")),
            prefix_topics: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::new(b"event_b"),
            )]),
            params: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f3"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I128,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; EventB::__SPEC_XDR_VIEW.const_xdr_len()] {
        EventB::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EventB {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EventB::spec_xdr());
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
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
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
    fn assert_fields_are_eq(&self) {
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
        self.f2 == other.f2 && self.f3 == other.f3 && self.f1 == other.f1
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_EVENTC: [u8; EventC::__SPEC_XDR_VIEW.const_xdr_len()] =
    EventC::spec_xdr();
impl EventC {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::EventV0(soroban_sdk::xdr::ScSpecEventV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(b"EventC")),
            prefix_topics: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::new(b"event_c"),
            )]),
            params: soroban_sdk::xdr::VecMView::new(&[
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Symbol,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
                soroban_sdk::xdr::ScSpecEventParamV0View {
                    doc: soroban_sdk::xdr::StringMView::new(b""),
                    name: soroban_sdk::xdr::StringMView::new(b"f3"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; EventC::__SPEC_XDR_VIEW.const_xdr_len()] {
        EventC::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EventC {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <soroban_sdk::Symbol as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EventC::spec_xdr());
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
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct EventD;
#[automatically_derived]
impl ::core::clone::Clone for EventD {
    #[inline]
    fn clone(&self) -> EventD {
        EventD
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EventD {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "EventD")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EventD {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_fields_are_eq(&self) {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EventD {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EventD {
    #[inline]
    fn eq(&self, other: &EventD) -> bool {
        true
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_EVENTD: [u8; EventD::__SPEC_XDR_VIEW.const_xdr_len()] =
    EventD::spec_xdr();
impl EventD {
    const __SPEC_XDR_VIEW: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::EventV0(soroban_sdk::xdr::ScSpecEventV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            lib: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(b"EventD")),
            prefix_topics: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::new(b"event_d"),
            )]),
            params: soroban_sdk::xdr::VecMView::new(&[]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; EventD::__SPEC_XDR_VIEW.const_xdr_len()] {
        EventD::__SPEC_XDR_VIEW.const_to_xdr()
    }
}
impl soroban_sdk::SpecShakingMarker for EventD {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] = soroban_sdk::spec_marker(&EventD::spec_xdr());
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for EventD {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{
            #[allow(deprecated)]
            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_d");
            SYMBOL
        },)
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 0usize] = [];
        let vals: [soroban_sdk::Val; 0usize] = [];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl EventD {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
