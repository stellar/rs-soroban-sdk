#![feature(prelude_import)]
//! A data library providing only an enum with tuple variants, so that a contract
//! using it links this crate in only through the enum's spec anchor.
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::contracttype;
pub enum Enum {
    A(u32),
    B(u32),
}
impl soroban_sdk::SpecName for Enum {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_enum_lib::Enum";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Enum` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_enum_lib::Enum`",
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
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
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
