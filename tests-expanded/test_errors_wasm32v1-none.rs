#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, Env,
    Symbol,
};
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
pub enum Flag {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Flag {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Flag {
    #[inline]
    fn eq(&self, other: &Flag) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_FLAG: [u8; Flag::__SPEC_XDR_REF.const_xdr_len()] = Flag::spec_xdr();
impl Flag {
    const __SPEC_XDR_REF: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::UdtEnumV0(soroban_sdk::xdr::ScSpecUdtEnumV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            lib: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::StringMRef::new(b"Flag"),
            cases: soroban_sdk::xdr::VecMRef::new(&[
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"A"),
                    value: 0u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"B"),
                    value: 1u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"C"),
                    value: 2u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"D"),
                    value: 3u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"E"),
                    value: 4u32,
                },
            ]),
        });
    pub const fn spec_xdr() -> [u8; Flag::__SPEC_XDR_REF.const_xdr_len()] {
        Flag::__SPEC_XDR_REF.const_to_xdr()
    }
}
impl Flag {
    const __SPEC_XDR_CANONICAL_REF: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::UdtEnumV0(soroban_sdk::xdr::ScSpecUdtEnumV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            lib: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::StringMRef::new(b"Flag"),
            cases: soroban_sdk::xdr::VecMRef::new(&[
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"A"),
                    value: 0u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"B"),
                    value: 1u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"C"),
                    value: 2u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"D"),
                    value: 3u32,
                },
                soroban_sdk::xdr::ScSpecUdtEnumCaseV0Ref {
                    doc: soroban_sdk::xdr::StringMRef::new(b""),
                    name: soroban_sdk::xdr::StringMRef::new(b"E"),
                    value: 4u32,
                },
            ]),
        });
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        let xdr: [u8; Flag::__SPEC_XDR_CANONICAL_REF.const_xdr_len()] =
            Flag::__SPEC_XDR_CANONICAL_REF.const_to_xdr();
        let hash = soroban_sdk::reexports_for_macros::sha2_const::Sha256::new()
            .update(&xdr)
            .finalize();
        [
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
        ]
    }
}
impl soroban_sdk::SpecShakingMarker for Flag {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14usize] = *b"SpEcV1g\x19\x8d\xc6\x8aP\xeb\xb7";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Flag {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            0u32 => Self::A,
            1u32 => Self::B,
            2u32 => Self::C,
            3u32 => Self::D,
            4u32 => Self::E,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Flag> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Flag,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            Flag::A => 0u32.into(),
            Flag::B => 1u32.into(),
            Flag::C => 2u32.into(),
            Flag::D => 3u32.into(),
            Flag::E => 4u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Flag> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Flag,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Flag>>::try_from_val(env, *val)
    }
}
pub enum Error {
    AnError = 1,
}
#[automatically_derived]
impl ::core::fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "AnError")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Error {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Error {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        true
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_ERROR: [u8; Error::__SPEC_XDR_REF.const_xdr_len()] = Error::spec_xdr();
impl Error {
    const __SPEC_XDR_REF: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::UdtErrorEnumV0(
            soroban_sdk::xdr::ScSpecUdtErrorEnumV0Ref {
                doc: soroban_sdk::xdr::StringMRef::new(b""),
                lib: soroban_sdk::xdr::StringMRef::new(b""),
                name: soroban_sdk::xdr::StringMRef::new(b"Error"),
                cases: soroban_sdk::xdr::VecMRef::new(&[
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0Ref {
                        doc: soroban_sdk::xdr::StringMRef::new(b""),
                        name: soroban_sdk::xdr::StringMRef::new(b"AnError"),
                        value: 1u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Error::__SPEC_XDR_REF.const_xdr_len()] {
        Error::__SPEC_XDR_REF.const_to_xdr()
    }
}
impl Error {
    const __SPEC_XDR_CANONICAL_REF: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::UdtErrorEnumV0(
            soroban_sdk::xdr::ScSpecUdtErrorEnumV0Ref {
                doc: soroban_sdk::xdr::StringMRef::new(b""),
                lib: soroban_sdk::xdr::StringMRef::new(b""),
                name: soroban_sdk::xdr::StringMRef::new(b"Error"),
                cases: soroban_sdk::xdr::VecMRef::new(&[
                    soroban_sdk::xdr::ScSpecUdtErrorEnumCaseV0Ref {
                        doc: soroban_sdk::xdr::StringMRef::new(b""),
                        name: soroban_sdk::xdr::StringMRef::new(b"AnError"),
                        value: 1u32,
                    },
                ]),
            },
        );
    #[doc(hidden)]
    pub const fn spec_type_id() -> [u8; 8] {
        let xdr: [u8; Error::__SPEC_XDR_CANONICAL_REF.const_xdr_len()] =
            Error::__SPEC_XDR_CANONICAL_REF.const_to_xdr();
        let hash = soroban_sdk::reexports_for_macros::sha2_const::Sha256::new()
            .update(&xdr)
            .finalize();
        [
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
        ]
    }
}
impl soroban_sdk::SpecShakingMarker for Error {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: [u8; 14usize] = *b"SpEcV1\xbc\x04\x04\xea\xa4\x9e6(";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl TryFrom<soroban_sdk::Error> for Error {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::AnError,
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
        match val {
            Error::AnError => soroban_sdk::Error::from_contract_error(1u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for Error {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                1u32 => Self::AnError,
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
        match val {
            Error::AnError => soroban_sdk::InvokeError::Contract(1u32),
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
impl Contract {
    pub fn hello(env: Env, flag: Flag) -> Result<Symbol, Error> {
        env.storage().persistent().set(
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("persisted");
                SYMBOL
            },
            &true,
        );
        if flag == Flag::A {
            Ok({
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
                SYMBOL
            })
        } else if flag == Flag::B {
            Err(Error::AnError)
        } else if flag == Flag::C {
            {
                (&env).panic_with_error(Error::AnError);
            }
        } else if flag == Flag::D {
            {
                ::core::panicking::panic_fmt(format_args!("an error"));
            }
        } else if flag == Flag::E {
            {
                (&env).panic_with_error(soroban_sdk::Error::from_contract_error(9));
            }
        } else {
            ::core::panicking::panic("not implemented")
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__hello__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_HELLO: [u8; super::Contract::__SPEC_XDR_REF_hello.const_xdr_len()] =
        super::Contract::spec_xdr_hello();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_REF_hello: soroban_sdk::xdr::ScSpecEntryRef<'static> =
        soroban_sdk::xdr::ScSpecEntryRef::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0Ref {
            doc: soroban_sdk::xdr::StringMRef::new(b""),
            name: soroban_sdk::xdr::ScSymbolRef(soroban_sdk::xdr::StringMRef::new(b"hello")),
            inputs: soroban_sdk::xdr::VecMRef::new(&[soroban_sdk::xdr::ScSpecFunctionInputV0Ref {
                doc: soroban_sdk::xdr::StringMRef::new(b""),
                name: soroban_sdk::xdr::StringMRef::new(b"flag"),
                type_: soroban_sdk::xdr::ScSpecTypeDefRef::UdtV2(
                    soroban_sdk::xdr::ScSpecTypeUdtv2 {
                        id: <Flag>::spec_type_id(),
                    },
                ),
            }]),
            outputs: soroban_sdk::xdr::VecMRef::new(&[soroban_sdk::xdr::ScSpecTypeDefRef::Result(
                &soroban_sdk::xdr::ScSpecTypeResultRef {
                    ok_type: &soroban_sdk::xdr::ScSpecTypeDefRef::Symbol,
                    error_type: &soroban_sdk::xdr::ScSpecTypeDefRef::Error,
                },
            )]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_hello() -> [u8; Contract::__SPEC_XDR_REF_hello.const_xdr_len()] {
        Contract::__SPEC_XDR_REF_hello.const_to_xdr()
    }
}
impl Contract {}
impl<'a> ContractClient<'a> {
    pub fn hello(&self, flag: &Flag) -> Symbol {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [flag.into_val(&self.env)]),
        );
        res
    }
    pub fn try_hello(
        &self,
        flag: &Flag,
    ) -> Result<
        Result<
            Symbol,
            <Symbol as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [flag.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn hello<'i>(flag: &'i Flag) -> (&'i Flag,) {
        (flag,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
#[allow(deprecated)]
pub fn __Contract__hello__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::hello(
            env.clone(),
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
#[export_name = "hello"]
pub extern "C" fn __Contract__hello__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__hello__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
