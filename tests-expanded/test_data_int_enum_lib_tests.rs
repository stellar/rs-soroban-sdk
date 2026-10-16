#![feature(prelude_import)]
//! A data library providing only an int enum, so that a contract using it links
//! this crate in only through the int enum's spec anchor.
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
impl soroban_sdk::SpecName for IntEnum {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_int_enum_lib::IntEnum";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `IntEnum` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_int_enum_lib::IntEnum`",
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
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
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
