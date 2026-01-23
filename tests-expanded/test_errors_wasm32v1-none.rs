#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
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
pub static __SPEC_XDR_TYPE_FLAG: [u8; 104usize] = Flag::spec_xdr();
impl Flag {
    pub const fn spec_xdr() -> [u8; 104usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x04Flag\0\0\0\x05\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01C\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01D\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x01E\0\0\0\0\0\0\x04"
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
pub static __SPEC_XDR_TYPE_ERROR: [u8; 48usize] = Error::spec_xdr();
impl Error {
    pub const fn spec_xdr() -> [u8; 48usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x05Error\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x07AnError\0\0\0\0\x01"
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
    pub static __SPEC_XDR_FN_HELLO: [u8; 64usize] = super::Contract::spec_xdr_hello();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_hello() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05hello\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04flag\0\0\x07\xd0\0\0\0\x04Flag\0\0\0\x01\0\0\x03\xe9\0\0\0\x11\0\0\0\x03"
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
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn persisted<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
pub fn __Contract__hello__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::hello(
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
