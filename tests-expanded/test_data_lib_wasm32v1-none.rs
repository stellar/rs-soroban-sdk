#![feature(prelude_import)]
//! A data library: a crate that only provides types, for contracts to use.
//!
//! The types are only of the kinds whose conversions to and from `Val` are
//! inlined into the contract using them, so that the contract need not call
//! into this crate at all. A named-field struct is left out because its
//! conversions are not inlined.
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::contracttype;
pub enum IntEnum {
    A = 1,
    B = 2,
}
#[automatically_derived]
impl ::core::clone::Clone for IntEnum {
    #[inline]
    fn clone(&self) -> IntEnum {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for IntEnum {}
#[automatically_derived]
impl ::core::fmt::Debug for IntEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                IntEnum::A => "A",
                IntEnum::B => "B",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for IntEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for IntEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for IntEnum {
    #[inline]
    fn eq(&self, other: &IntEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
impl soroban_sdk::SpecName for IntEnum {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::IntEnum";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `IntEnum` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::IntEnum`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for IntEnum {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_INTENUM: [u8; IntEnum::spec_xdr().len()] = IntEnum::spec_xdr();
impl IntEnum {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtEnumV0(
            soroban_sdk::xdr::r#const::ScSpecUdtEnumV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <IntEnum as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"A"),
                        value: 1u32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"B"),
                        value: 2u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; IntEnum::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { IntEnum::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for IntEnum {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for IntEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <IntEnum as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::A,
            2u32 => Self::B,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, IntEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &IntEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <IntEnum as soroban_sdk::SpecAnchor>::spec_anchor();
        Ok(match val {
            IntEnum::A => 1u32.into(),
            IntEnum::B => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &IntEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&IntEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, IntEnum>>::try_from_val(env, *val)
    }
}
pub enum Enum {
    A(u32),
    B(u32),
}
#[automatically_derived]
impl ::core::clone::Clone for Enum {
    #[inline]
    fn clone(&self) -> Enum {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Enum {}
#[automatically_derived]
impl ::core::fmt::Debug for Enum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Enum::A(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A", &__self_0)
            }
            Enum::B(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Enum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Enum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum {
    #[inline]
    fn eq(&self, other: &Enum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Enum::A(__self_0), Enum::A(__arg1_0)) => __self_0 == __arg1_0,
                (Enum::B(__self_0), Enum::B(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
impl soroban_sdk::SpecName for Enum {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Enum";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Enum` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Enum`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Enum {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_ENUM: [u8; Enum::spec_xdr().len()] = Enum::spec_xdr();
impl Enum {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(
            soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Enum as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"A"),
                            type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                                <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                            ]),
                        },
                    ),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"B"),
                            type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                                <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                            ]),
                        },
                    ),
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Enum::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Enum::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for Enum {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Enum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Enum as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["A", "B"];
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
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::A(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::B(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Enum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Enum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Enum as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            Enum::A(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"A")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            Enum::B(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"B")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Enum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Enum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Enum>>::try_from_val(env, *val)
    }
}
pub struct Tuple(pub u32, pub u32);
#[automatically_derived]
impl ::core::clone::Clone for Tuple {
    #[inline]
    fn clone(&self) -> Tuple {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tuple {}
#[automatically_derived]
impl ::core::fmt::Debug for Tuple {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Tuple", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Tuple {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Tuple {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Tuple {
    #[inline]
    fn eq(&self, other: &Tuple) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl soroban_sdk::SpecName for Tuple {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Tuple";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Tuple` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Tuple`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Tuple {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_TUPLE: [u8; Tuple::spec_xdr().len()] = Tuple::spec_xdr();
impl Tuple {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Tuple as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"0"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"1"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Tuple::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Tuple::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for Tuple {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Tuple {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Tuple as soroban_sdk::SpecAnchor>::spec_anchor();
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Tuple> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Tuple,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Tuple as soroban_sdk::SpecAnchor>::spec_anchor();
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Tuple> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Tuple,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Tuple>>::try_from_val(env, *val)
    }
}
pub struct Inner(pub u32);
#[automatically_derived]
impl ::core::clone::Clone for Inner {
    #[inline]
    fn clone(&self) -> Inner {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Inner {}
#[automatically_derived]
impl ::core::fmt::Debug for Inner {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Inner", &&self.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Inner {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Inner {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Inner {
    #[inline]
    fn eq(&self, other: &Inner) -> bool {
        self.0 == other.0
    }
}
impl soroban_sdk::SpecName for Inner {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Inner";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Inner` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Inner`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Inner {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_INNER: [u8; Inner::spec_xdr().len()] = Inner::spec_xdr();
impl Inner {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Inner as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"0"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Inner::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Inner::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for Inner {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Inner {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Inner as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Inner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Inner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Inner as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 1usize] = [(&val.0).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Inner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Inner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Inner>>::try_from_val(env, *val)
    }
}
pub struct Outer(pub Inner);
#[automatically_derived]
impl ::core::clone::Clone for Outer {
    #[inline]
    fn clone(&self) -> Outer {
        let _: ::core::clone::AssertParamIsClone<Inner>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Outer {}
#[automatically_derived]
impl ::core::fmt::Debug for Outer {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Outer", &&self.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Outer {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Inner>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Outer {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Outer {
    #[inline]
    fn eq(&self, other: &Outer) -> bool {
        self.0 == other.0
    }
}
impl soroban_sdk::SpecName for Outer {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Outer";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Outer` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Outer`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Outer {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_OUTER: [u8; Outer::spec_xdr().len()] = Outer::spec_xdr();
impl Outer {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Outer as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"0"),
                        type_: <Inner as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Outer::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Outer::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for Outer {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Outer {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Outer as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Outer> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Outer,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Outer as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 1usize] = [(&val.0).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Outer> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Outer,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Outer>>::try_from_val(env, *val)
    }
}
pub struct Wrapped(pub u32);
#[automatically_derived]
impl ::core::clone::Clone for Wrapped {
    #[inline]
    fn clone(&self) -> Wrapped {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Wrapped {}
#[automatically_derived]
impl ::core::fmt::Debug for Wrapped {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wrapped", &&self.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Wrapped {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Wrapped {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Wrapped {
    #[inline]
    fn eq(&self, other: &Wrapped) -> bool {
        self.0 == other.0
    }
}
impl soroban_sdk::SpecName for Wrapped {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Wrapped";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Wrapped` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Wrapped`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Wrapped {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_WRAPPED: [u8; Wrapped::spec_xdr().len()] = Wrapped::spec_xdr();
impl Wrapped {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Wrapped as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"0"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Wrapped::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Wrapped::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for Wrapped {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Wrapped {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Wrapped as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Wrapped> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Wrapped,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <Wrapped as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 1usize] = [(&val.0).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Wrapped> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Wrapped,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Wrapped>>::try_from_val(env, *val)
    }
}
