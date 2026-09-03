#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, contracttype, Error, Map, Symbol, Vec};
pub enum UdtEnum2 {
    A = 10,
    B = 15,
}
#[automatically_derived]
impl ::core::marker::Copy for UdtEnum2 {}
#[automatically_derived]
impl ::core::clone::Clone for UdtEnum2 {
    #[inline]
    fn clone(&self) -> UdtEnum2 {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtEnum2 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UdtEnum2::A => "A",
                UdtEnum2::B => "B",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtEnum2 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtEnum2 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtEnum2 {
    #[inline]
    fn eq(&self, other: &UdtEnum2) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
impl UdtEnum2 {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::UdtEnum2"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTENUM2: [u8; UdtEnum2::spec_xdr_len()] = UdtEnum2::spec_xdr();
impl UdtEnum2 {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtEnumV0(soroban_sdk::xdr::ScSpecUdtEnumV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(UdtEnum2::spec_name()),
            cases: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"A"),
                    value: 10u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"B"),
                    value: 15u32,
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { UdtEnum2::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; UdtEnum2::spec_xdr_len()] {
        const { UdtEnum2::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for UdtEnum2 {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &UdtEnum2::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtEnum2 {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            10u32 => Self::A,
            15u32 => Self::B,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum2> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtEnum2,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UdtEnum2::A => 10u32.into(),
            UdtEnum2::B => 15u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtEnum2> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtEnum2,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum2>>::try_from_val(env, *val)
    }
}
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
    UdtC(UdtEnum2),
    UdtD(UdtTuple),
}
#[automatically_derived]
impl ::core::clone::Clone for UdtEnum {
    #[inline]
    fn clone(&self) -> UdtEnum {
        match self {
            UdtEnum::UdtA => UdtEnum::UdtA,
            UdtEnum::UdtB(__self_0) => UdtEnum::UdtB(::core::clone::Clone::clone(__self_0)),
            UdtEnum::UdtC(__self_0) => UdtEnum::UdtC(::core::clone::Clone::clone(__self_0)),
            UdtEnum::UdtD(__self_0) => UdtEnum::UdtD(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            UdtEnum::UdtA => ::core::fmt::Formatter::write_str(f, "UdtA"),
            UdtEnum::UdtB(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtB", &__self_0)
            }
            UdtEnum::UdtC(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtC", &__self_0)
            }
            UdtEnum::UdtD(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtD", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UdtStruct>;
        let _: ::core::cmp::AssertParamIsEq<UdtEnum2>;
        let _: ::core::cmp::AssertParamIsEq<UdtTuple>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtEnum {
    #[inline]
    fn eq(&self, other: &UdtEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (UdtEnum::UdtB(__self_0), UdtEnum::UdtB(__arg1_0)) => __self_0 == __arg1_0,
                (UdtEnum::UdtC(__self_0), UdtEnum::UdtC(__arg1_0)) => __self_0 == __arg1_0,
                (UdtEnum::UdtD(__self_0), UdtEnum::UdtD(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
impl UdtEnum {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::UdtEnum"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTENUM: [u8; UdtEnum::spec_xdr_len()] = UdtEnum::spec_xdr();
impl UdtEnum {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtUnionV0(soroban_sdk::xdr::ScSpecUdtUnionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(UdtEnum::spec_name()),
            cases: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"UdtA"),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"UdtB"),
                        type_: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                soroban_sdk::xdr::ScSpecTypeUdtView {
                                    name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                        <UdtStruct>::spec_name(),
                                    ),
                                },
                            ),
                        ]),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"UdtC"),
                        type_: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                soroban_sdk::xdr::ScSpecTypeUdtView {
                                    name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                        <UdtEnum2>::spec_name(),
                                    ),
                                },
                            ),
                        ]),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"UdtD"),
                        type_: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                soroban_sdk::xdr::ScSpecTypeUdtView {
                                    name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                        <UdtTuple>::spec_name(),
                                    ),
                                },
                            ),
                        ]),
                    },
                ),
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { UdtEnum::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; UdtEnum::spec_xdr_len()] {
        const { UdtEnum::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for UdtEnum {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <UdtStruct as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <UdtEnum2 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <UdtTuple as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &UdtEnum::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["UdtA", "UdtB", "UdtC", "UdtD"];
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
                    Self::UdtA
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::UdtB(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::UdtC(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                3 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::UdtD(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            UdtEnum::UdtA => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"UdtA")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            UdtEnum::UdtB(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"UdtB")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            UdtEnum::UdtC(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"UdtC")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            UdtEnum::UdtD(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"UdtD")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum>>::try_from_val(env, *val)
    }
}
pub struct UdtTuple(pub i64, pub Vec<i64>);
#[automatically_derived]
impl ::core::clone::Clone for UdtTuple {
    #[inline]
    fn clone(&self) -> UdtTuple {
        UdtTuple(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtTuple {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "UdtTuple", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtTuple {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
        let _: ::core::cmp::AssertParamIsEq<Vec<i64>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtTuple {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtTuple {
    #[inline]
    fn eq(&self, other: &UdtTuple) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl UdtTuple {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::UdtTuple"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTTUPLE: [u8; UdtTuple::spec_xdr_len()] = UdtTuple::spec_xdr();
impl UdtTuple {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(UdtTuple::spec_name()),
            fields: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"0"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::I64,
                        },
                    ),
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { UdtTuple::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; UdtTuple::spec_xdr_len()] {
        const { UdtTuple::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for UdtTuple {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Vec<i64> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &UdtTuple::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtTuple {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtTuple> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtTuple,
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtTuple> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtTuple,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtTuple>>::try_from_val(env, *val)
    }
}
pub struct UdtStruct {
    a: i64,
    b: i64,
    pub c: Vec<i64>,
}
#[automatically_derived]
impl ::core::clone::Clone for UdtStruct {
    #[inline]
    fn clone(&self) -> UdtStruct {
        UdtStruct {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
            c: ::core::clone::Clone::clone(&self.c),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdtStruct",
            "a",
            &self.a,
            "b",
            &self.b,
            "c",
            &&self.c,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
        let _: ::core::cmp::AssertParamIsEq<Vec<i64>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtStruct {
    #[inline]
    fn eq(&self, other: &UdtStruct) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}
impl UdtStruct {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::UdtStruct"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTSTRUCT: [u8; UdtStruct::spec_xdr_len()] = UdtStruct::spec_xdr();
impl UdtStruct {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(UdtStruct::spec_name()),
            fields: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::I64,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"c"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::I64,
                        },
                    ),
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { UdtStruct::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; UdtStruct::spec_xdr_len()] {
        const { UdtStruct::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for UdtStruct {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <i64 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Vec<i64> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &UdtStruct::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 3usize] = ["a", "b", "c"];
        let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            a: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            b: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            c: vals[2]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 3usize] = ["a", "b", "c"];
        let vals: [Val; 3usize] = [
            (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.b).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.c).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtStruct>>::try_from_val(env, *val)
    }
}
pub struct UdtRecursive {
    pub a: Symbol,
    pub b: Vec<UdtRecursive>,
}
#[automatically_derived]
impl ::core::clone::Clone for UdtRecursive {
    #[inline]
    fn clone(&self) -> UdtRecursive {
        UdtRecursive {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtRecursive {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UdtRecursive",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtRecursive {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<Vec<UdtRecursive>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtRecursive {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtRecursive {
    #[inline]
    fn eq(&self, other: &UdtRecursive) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl UdtRecursive {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::UdtRecursive"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTRECURSIVE: [u8; UdtRecursive::spec_xdr_len()] =
    UdtRecursive::spec_xdr();
impl UdtRecursive {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(UdtRecursive::spec_name()),
            fields: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Symbol,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                soroban_sdk::xdr::ScSpecTypeUdtView {
                                    name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                        <UdtRecursive>::spec_name(),
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { UdtRecursive::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; UdtRecursive::spec_xdr_len()] {
        const { UdtRecursive::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for UdtRecursive {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Symbol as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Vec<UdtRecursive> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &UdtRecursive::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtRecursive {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            a: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            b: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtRecursive> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtRecursive,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let vals: [Val; 2usize] = [
            (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.b).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtRecursive> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtRecursive,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtRecursive>>::try_from_val(env, *val)
    }
}
pub struct RecursiveToEnum {
    pub a: Symbol,
    pub b: Map<u32, RecursiveEnum>,
}
#[automatically_derived]
impl ::core::clone::Clone for RecursiveToEnum {
    #[inline]
    fn clone(&self) -> RecursiveToEnum {
        RecursiveToEnum {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RecursiveToEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "RecursiveToEnum",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RecursiveToEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<Map<u32, RecursiveEnum>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RecursiveToEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RecursiveToEnum {
    #[inline]
    fn eq(&self, other: &RecursiveToEnum) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl RecursiveToEnum {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::RecursiveToEnum"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_RECURSIVETOENUM: [u8; RecursiveToEnum::spec_xdr_len()] =
    RecursiveToEnum::spec_xdr();
impl RecursiveToEnum {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(RecursiveToEnum::spec_name()),
            fields: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Symbol,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Map(
                        &soroban_sdk::xdr::ScSpecTypeMapView {
                            key_type: &soroban_sdk::xdr::ScSpecTypeDefView::U32,
                            value_type: &soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                soroban_sdk::xdr::ScSpecTypeUdtView {
                                    name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                        <RecursiveEnum>::spec_name(),
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { RecursiveToEnum::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; RecursiveToEnum::spec_xdr_len()] {
        const { RecursiveToEnum::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for RecursiveToEnum {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Symbol as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Map<u32, RecursiveEnum> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &RecursiveToEnum::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RecursiveToEnum {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            a: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            b: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveToEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &RecursiveToEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let vals: [Val; 2usize] = [
            (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.b).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RecursiveToEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&RecursiveToEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveToEnum>>::try_from_val(env, *val)
    }
}
pub enum RecursiveEnum {
    NotRecursive,
    Recursive(RecursiveToEnum),
}
#[automatically_derived]
impl ::core::clone::Clone for RecursiveEnum {
    #[inline]
    fn clone(&self) -> RecursiveEnum {
        match self {
            RecursiveEnum::NotRecursive => RecursiveEnum::NotRecursive,
            RecursiveEnum::Recursive(__self_0) => {
                RecursiveEnum::Recursive(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RecursiveEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            RecursiveEnum::NotRecursive => ::core::fmt::Formatter::write_str(f, "NotRecursive"),
            RecursiveEnum::Recursive(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Recursive", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RecursiveEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<RecursiveToEnum>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RecursiveEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RecursiveEnum {
    #[inline]
    fn eq(&self, other: &RecursiveEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (RecursiveEnum::Recursive(__self_0), RecursiveEnum::Recursive(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => true,
            }
    }
}
impl RecursiveEnum {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_udt::RecursiveEnum"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_RECURSIVEENUM: [u8; RecursiveEnum::spec_xdr_len()] =
    RecursiveEnum::spec_xdr();
impl RecursiveEnum {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtUnionV0(soroban_sdk::xdr::ScSpecUdtUnionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(RecursiveEnum::spec_name()),
            cases: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::VoidV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseVoidV0View {
                        doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(
                            b"NotRecursive",
                        ),
                    },
                ),
                soroban_sdk::xdr::ScSpecUdtUnionCaseV0View::TupleV0(
                    soroban_sdk::xdr::ScSpecUdtUnionCaseTupleV0View {
                        doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"Recursive"),
                        type_: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                soroban_sdk::xdr::ScSpecTypeUdtView {
                                    name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                        <RecursiveToEnum>::spec_name(),
                                    ),
                                },
                            ),
                        ]),
                    },
                ),
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { RecursiveEnum::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; RecursiveEnum::spec_xdr_len()] {
        const { RecursiveEnum::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for RecursiveEnum {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <RecursiveToEnum as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &RecursiveEnum::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RecursiveEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["NotRecursive", "Recursive"];
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
                    Self::NotRecursive
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Recursive(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &RecursiveEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            RecursiveEnum::NotRecursive => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"NotRecursive")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            RecursiveEnum::Recursive(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Recursive")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RecursiveEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&RecursiveEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveEnum>>::try_from_val(env, *val)
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
impl Contract {
    pub fn add(a: UdtEnum, b: UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        a + b
    }
    pub fn recursive(a: UdtRecursive) -> Option<UdtRecursive> {
        if a.b.is_empty() {
            None
        } else {
            Some(a.b.first_unchecked())
        }
    }
    pub fn recursive_enum(a: RecursiveEnum, key: u32) -> Result<Option<RecursiveEnum>, Error> {
        match a {
            RecursiveEnum::NotRecursive => Ok(None),
            RecursiveEnum::Recursive(router) => Ok(router.b.get(key)),
        }
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
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                        soroban_sdk::xdr::ScSpecTypeUdtView {
                            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                <UdtEnum>::spec_name(),
                            ),
                        },
                    ),
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                        soroban_sdk::xdr::ScSpecTypeUdtView {
                            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                <UdtEnum>::spec_name(),
                            ),
                        },
                    ),
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::I64,
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
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__recursive__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_RECURSIVE: [u8; super::Contract::spec_xdr_len_recursive()] =
        super::Contract::spec_xdr_recursive();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_recursive: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"recursive"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                        soroban_sdk::xdr::ScSpecTypeUdtView {
                            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                <UdtRecursive>::spec_name(),
                            ),
                        },
                    ),
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Option(
                    &soroban_sdk::xdr::ScSpecTypeOptionView {
                        value_type: &soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                            soroban_sdk::xdr::ScSpecTypeUdtView {
                                name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                    <UdtRecursive>::spec_name(),
                                ),
                            },
                        ),
                    },
                ),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_recursive() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_recursive.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_recursive() -> [u8; Contract::spec_xdr_len_recursive()] {
        const { Contract::__SPEC_XDR_ENTRY_recursive.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__recursive_enum__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_RECURSIVE_ENUM: [u8; super::Contract::spec_xdr_len_recursive_enum()] =
        super::Contract::spec_xdr_recursive_enum();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_recursive_enum: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"recursive_enum"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                        soroban_sdk::xdr::ScSpecTypeUdtView {
                            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                <RecursiveEnum>::spec_name(),
                            ),
                        },
                    ),
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"key"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U32,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Result(
                    &soroban_sdk::xdr::ScSpecTypeResultView {
                        ok_type: &soroban_sdk::xdr::ScSpecTypeDefView::Option(
                            &soroban_sdk::xdr::ScSpecTypeOptionView {
                                value_type: &soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                                    soroban_sdk::xdr::ScSpecTypeUdtView {
                                        name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                            <RecursiveEnum>::spec_name(),
                                        ),
                                    },
                                ),
                            },
                        ),
                        error_type: &soroban_sdk::xdr::ScSpecTypeDefView::Error,
                    },
                ),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_recursive_enum() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_recursive_enum.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_recursive_enum() -> [u8; Contract::spec_xdr_len_recursive_enum()] {
        const { Contract::__SPEC_XDR_ENTRY_recursive_enum.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn add(&self, a: &UdtEnum, b: &UdtEnum) -> i64 {
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
        a: &UdtEnum,
        b: &UdtEnum,
    ) -> Result<
        Result<i64, <i64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
    pub fn recursive(&self, a: &UdtRecursive) -> Option<UdtRecursive> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("recursive");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        res
    }
    pub fn try_recursive(
        &self,
        a: &UdtRecursive,
    ) -> Result<
        Result<
            Option<UdtRecursive>,
            <Option<
                UdtRecursive,
            > as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("recursive");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        res
    }
    pub fn recursive_enum(&self, a: &RecursiveEnum, key: &u32) -> Option<RecursiveEnum> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "recursive_enum") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), key.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_recursive_enum(
        &self,
        a: &RecursiveEnum,
        key: &u32,
    ) -> Result<
        Result<
            Option<RecursiveEnum>,
            <Option<RecursiveEnum> as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "recursive_enum") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), key.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn add<'i>(a: &'i UdtEnum, b: &'i UdtEnum) -> (&'i UdtEnum, &'i UdtEnum) {
        (a, b)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn recursive<'i>(a: &'i UdtRecursive) -> (&'i UdtRecursive,) {
        (a,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn recursive_enum<'i>(a: &'i RecursiveEnum, key: &'i u32) -> (&'i RecursiveEnum, &'i u32) {
        (a, key)
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive` instead")]
#[allow(deprecated)]
pub fn __Contract__recursive__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::recursive(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive` instead")]
#[export_name = "recursive"]
pub extern "C" fn __Contract__recursive__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__recursive__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive_enum` instead")]
#[allow(deprecated)]
pub fn __Contract__recursive_enum__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::recursive_enum(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive_enum` instead")]
#[export_name = "recursive_enum"]
pub extern "C" fn __Contract__recursive_enum__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__recursive_enum__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
