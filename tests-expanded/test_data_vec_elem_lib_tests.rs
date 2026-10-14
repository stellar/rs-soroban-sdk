#![feature(prelude_import)]
//! A data library providing only a tuple struct for a contract to use as the
//! element type of a `Vec`, without converting any element, so that the
//! contract links this crate in only through the `Vec`'s spec anchor.
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::contracttype;
pub struct VecElem(pub u32);
impl soroban_sdk::SpecName for VecElem {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_vec_elem_lib::VecElem";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `VecElem` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_vec_elem_lib::VecElem`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for VecElem {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
static __SPEC_XDR_TYPE_VECELEM: [u8; VecElem::spec_xdr().len()] = VecElem::spec_xdr();
impl VecElem {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <VecElem as soroban_sdk::SpecName>::SPEC_NAME,
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
    pub const fn spec_xdr() -> [u8; VecElem::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { VecElem::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecAnchor for VecElem {
    #[inline(never)]
    fn spec_anchor() {
        soroban_sdk::spec_anchor_ref(<u32 as soroban_sdk::SpecAnchor>::spec_anchor);
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for VecElem {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <VecElem as soroban_sdk::SpecAnchor>::spec_anchor();
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, VecElem> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &VecElem,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <VecElem as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 1usize] = [(&val.0).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &VecElem> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&VecElem,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, VecElem>>::try_from_val(env, *val)
    }
}
