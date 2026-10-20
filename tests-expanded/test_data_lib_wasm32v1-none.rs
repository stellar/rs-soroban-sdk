#![feature(prelude_import)]
//! A data library: a crate that only provides errors and events, for contracts
//! to use.
//!
//! The error, the event and the event with a lifetime are each used by a
//! separate contract, test_data_error_contract, test_data_event_contract and
//! test_data_ref_event_contract, because a contract that calls into this crate
//! for one of them links in the spec entries of all of them.
//!
//! The types are each in a data library of their own, used by
//! test_data_contract, for the same reason. They are only of the kinds whose
//! conversions to and from `Val` are inlined into the contract using them, so
//! that the contract need not call into their crates at all. A named-field
//! struct is left out because its conversions are not inlined.
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contracterror, contractevent};
#[repr(u32)]
pub enum Error {
    A = 1,
}
impl soroban_sdk::SpecName for Error {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Error";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Error` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Error`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Error {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
impl soroban_sdk::client::Type for Error {
    type Output = Self;
    type Ok = Self;
    type Err = soroban_sdk::Error;
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_ERROR: [u8; Error::spec_xdr().len()] = Error::spec_xdr();
impl Error {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtErrorEnumV0(
            soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Error as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"A"),
                        value: 1u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Error::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Error::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for Error {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Error::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::SpecAnchor for Error {
    #[inline(never)]
    fn spec_anchor() {}
}
impl TryFrom<soroban_sdk::Error> for Error {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <Error as soroban_sdk::SpecAnchor>::spec_anchor();
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::A,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for Error {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<Error> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: Error) -> soroban_sdk::Error {
        <_ as From<&Error>>::from(&val)
    }
}
impl From<&Error> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &Error) -> soroban_sdk::Error {
        <Error as soroban_sdk::SpecAnchor>::spec_anchor();
        match val {
            Error::A => soroban_sdk::Error::from_contract_error(1u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for Error {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <Error as soroban_sdk::SpecAnchor>::spec_anchor();
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                1u32 => Self::A,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for Error {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<Error> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: Error) -> soroban_sdk::InvokeError {
        <_ as From<&Error>>::from(&val)
    }
}
impl From<&Error> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &Error) -> soroban_sdk::InvokeError {
        <Error as soroban_sdk::SpecAnchor>::spec_anchor();
        match val {
            Error::A => soroban_sdk::InvokeError::Contract(1u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Error {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Error> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Error,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Error> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Error,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Error>>::try_from_val(env, *val)
    }
}
pub struct Event {
    pub v: u32,
}
impl soroban_sdk::SpecName for Event {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::Event";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Event` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::Event`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Event {
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
static __SPEC_XDR_EVENT_EVENT: [u8; Event::spec_xdr().len()] = Event::spec_xdr();
impl Event {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Event as soroban_sdk::SpecName>::SPEC_NAME,
            ),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"event"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                    type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; Event::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Event::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for Event {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Event::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::SpecAnchor for Event {
    #[inline(never)]
    fn spec_anchor() {}
}
impl soroban_sdk::Event for Event {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        <Self as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::IntoVal;
        (&{
            #[allow(deprecated)]
            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event");
            SYMBOL
        },)
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        <Self as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["v"];
        let vals: [soroban_sdk::Val; 1usize] = [self.v.into_val(env)];
        env.sparse_map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl Event {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct RefEvent<'a> {
    pub v: &'a u32,
}
impl<'a> soroban_sdk::SpecName for RefEvent<'a> {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_data_lib::RefEvent";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `RefEvent` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_data_lib::RefEvent`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl<'a> soroban_sdk::SpecTypeDef for RefEvent<'a> {
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
static __SPEC_XDR_EVENT_REFEVENT: [u8; RefEvent::spec_xdr().len()] = RefEvent::spec_xdr();
impl<'a> RefEvent<'a> {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <RefEvent as soroban_sdk::SpecName>::SPEC_NAME,
            ),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"ref_event"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                    type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; RefEvent::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { RefEvent::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl<'a> soroban_sdk::SpecShakingMarker for RefEvent<'a> {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &RefEvent::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl<'a> soroban_sdk::SpecAnchor for RefEvent<'a> {
    #[inline(never)]
    fn spec_anchor() {}
}
impl<'a> soroban_sdk::Event for RefEvent<'a> {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        <Self as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::IntoVal;
        (&{
            #[allow(deprecated)]
            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("ref_event");
            SYMBOL
        },)
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        <Self as soroban_sdk::SpecAnchor>::spec_anchor();
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["v"];
        let vals: [soroban_sdk::Val; 1usize] = [self.v.into_val(env)];
        env.sparse_map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl<'a> RefEvent<'a> {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
