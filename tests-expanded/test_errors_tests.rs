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
    set_auths: Option<&'a [soroban_sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [soroban_sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
mod __contract_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = soroban_sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(
        name: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl soroban_sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for Contract {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Flag {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVal,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
            Ok(match *discriminant {
                0u32 => Self::A,
                1u32 => Self::B,
                2u32 => Self::C,
                3u32 => Self::D,
                4u32 => Self::E,
                _ => Err(soroban_sdk::xdr::Error::Invalid)?,
            })
        } else {
            Err(soroban_sdk::xdr::Error::Invalid)
        }
    }
}
impl TryInto<soroban_sdk::xdr::ScVal> for &Flag {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
        Ok(match self {
            Flag::A => 0u32.into(),
            Flag::B => 1u32.into(),
            Flag::C => 2u32.into(),
            Flag::D => 3u32.into(),
            Flag::E => 4u32.into(),
        })
    }
}
impl TryInto<soroban_sdk::xdr::ScVal> for Flag {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
        Ok(match self {
            Flag::A => 0u32.into(),
            Flag::B => 1u32.into(),
            Flag::C => 2u32.into(),
            Flag::D => 3u32.into(),
            Flag::E => 4u32.into(),
        })
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryFlag {
        A,
        B,
        C,
        D,
        E,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryFlag {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ArbitraryFlag::A => "A",
                    ArbitraryFlag::B => "B",
                    ArbitraryFlag::C => "C",
                    ArbitraryFlag::D => "D",
                    ArbitraryFlag::E => "E",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryFlag {
        #[inline]
        fn clone(&self) -> ArbitraryFlag {
            match self {
                ArbitraryFlag::A => ArbitraryFlag::A,
                ArbitraryFlag::B => ArbitraryFlag::B,
                ArbitraryFlag::C => ArbitraryFlag::C,
                ArbitraryFlag::D => ArbitraryFlag::D,
                ArbitraryFlag::E => ArbitraryFlag::E,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryFlag {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryFlag {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryFlag {
        #[inline]
        fn eq(&self, other: &ArbitraryFlag) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryFlag {
        #[inline]
        fn cmp(&self, other: &ArbitraryFlag) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryFlag {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryFlag,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryFlag: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
            #[inline]
            fn __init() -> std::cell::Cell<u32> {
                std::cell::Cell::new(0)
            }
            unsafe {
                ::std::thread::LocalKey::new(
                    const {
                        if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                            |init| {
                                #[thread_local]
                                static VAL: ::std::thread::local_impl::LazyStorage<
                                    std::cell::Cell<u32>,
                                    (),
                                > = ::std::thread::local_impl::LazyStorage::new();
                                VAL.get_or_init(init, __init)
                            }
                        } else {
                            |init| {
                                #[thread_local]
                                static VAL: ::std::thread::local_impl::LazyStorage<
                                    std::cell::Cell<u32>,
                                    !,
                                > = ::std::thread::local_impl::LazyStorage::new();
                                VAL.get_or_init(init, __init)
                            }
                        }
                    },
                )
            }
        };
        #[automatically_derived]
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryFlag {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryFlag.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 5u64) >> 32
                        {
                            0u64 => ArbitraryFlag::A,
                            1u64 => ArbitraryFlag::B,
                            2u64 => ArbitraryFlag::C,
                            3u64 => ArbitraryFlag::D,
                            4u64 => ArbitraryFlag::E,
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryFlag.with(|count| {
                        count.set(count.get() - 1);
                    });
                }
                result
            }
            fn arbitrary_take_rest(
                mut u: arbitrary::Unstructured<'arbitrary>,
            ) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryFlag.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?) * 5u64)
                            >> 32
                        {
                            0u64 => ArbitraryFlag::A,
                            1u64 => ArbitraryFlag::B,
                            2u64 => ArbitraryFlag::C,
                            3u64 => ArbitraryFlag::D,
                            4u64 => ArbitraryFlag::E,
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryFlag.with(|count| {
                        count.set(count.get() - 1);
                    });
                }
                result
            }
            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                arbitrary::size_hint::and(
                    <u32 as arbitrary::Arbitrary>::size_hint(depth),
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::or_all(&[
                            arbitrary::size_hint::and_all(&[]),
                            arbitrary::size_hint::and_all(&[]),
                            arbitrary::size_hint::and_all(&[]),
                            arbitrary::size_hint::and_all(&[]),
                            arbitrary::size_hint::and_all(&[]),
                        ])
                    }),
                )
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Flag {
        type Prototype = ArbitraryFlag;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryFlag> for Flag {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryFlag,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryFlag::A => Flag::A,
                ArbitraryFlag::B => Flag::B,
                ArbitraryFlag::C => Flag::C,
                ArbitraryFlag::D => Flag::D,
                ArbitraryFlag::E => Flag::E,
            })
        }
    }
};
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
    pub fn persisted(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("persisted");
                SYMBOL
            })
            .unwrap_or(false)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__hello__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_HELLO: [u8; 64usize] = super::Contract::spec_xdr_hello();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_hello() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05hello\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04flag\0\0\x07\xd0\0\0\0\x04Flag\0\0\0\x01\0\0\x03\xe9\0\0\0\x11\0\0\0\x03"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__persisted__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_PERSISTED: [u8; 36usize] = super::Contract::spec_xdr_persisted();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_persisted() -> [u8; 36usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\tpersisted\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01"
    }
}
impl<'a> ContractClient<'a> {
    pub fn hello(&self, flag: &Flag) -> Symbol {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn persisted(&self) -> bool {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("persisted");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_persisted(
        &self,
    ) -> Result<
        Result<bool, <bool as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("persisted");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
pub mod __Contract__hello {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::hello(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 1usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    1usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__persisted {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).persisted` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::persisted(env.clone()),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).persisted` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 0usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    0usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env)
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).persisted` instead")]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__dc66cfa30fdb08b17ba29ed3da0a0be599deef8db57bfb9cd9b3dcbf8c3be498_ctor() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::ctor::__support::CtorRetType {
                unsafe {
                    __Contract__dc66cfa30fdb08b17ba29ed3da0a0be599deef8db57bfb9cd9b3dcbf8c3be498_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "hello",
            #[allow(deprecated)]
            &__Contract__hello::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "persisted",
            #[allow(deprecated)]
            &__Contract__persisted::invoke_raw_slice,
        );
    }
}
mod test {
    use crate::{Contract, ContractClient, Error, Flag};
    use soroban_sdk::{symbol_short, xdr, Env, InvokeError};
    extern crate test;
    #[rustc_test_marker = "test::hello_ok"]
    #[doc(hidden)]
    pub const hello_ok: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::hello_ok"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/errors/src/lib.rs",
            start_line: 63usize,
            start_col: 8usize,
            end_line: 63usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(hello_ok()),
        ),
    };
    fn hello_ok() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let res = client.hello(&Flag::A);
        match (&res, &{
            #[allow(deprecated)]
            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
            SYMBOL
        }) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !client.persisted() {
            ::core::panicking::panic("assertion failed: client.persisted()")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::try_hello_ok"]
    #[doc(hidden)]
    pub const try_hello_ok: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::try_hello_ok"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/errors/src/lib.rs",
            start_line: 74usize,
            start_col: 8usize,
            end_line: 74usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(try_hello_ok()),
        ),
    };
    fn try_hello_ok() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let res = client.try_hello(&Flag::A);
        match (
            &res,
            &Ok(Ok({
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
                SYMBOL
            })),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !client.persisted() {
            ::core::panicking::panic("assertion failed: client.persisted()")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::try_hello_error"]
    #[doc(hidden)]
    pub const try_hello_error: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::try_hello_error"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/errors/src/lib.rs",
            start_line: 85usize,
            start_col: 8usize,
            end_line: 85usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(try_hello_error()),
        ),
    };
    fn try_hello_error() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let res = client.try_hello(&Flag::B);
        match (&res, &Err(Ok(Error::AnError))) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !!client.persisted() {
            ::core::panicking::panic("assertion failed: !client.persisted()")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::try_hello_error_panic"]
    #[doc(hidden)]
    pub const try_hello_error_panic: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::try_hello_error_panic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/errors/src/lib.rs",
            start_line: 96usize,
            start_col: 8usize,
            end_line: 96usize,
            end_col: 29usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(try_hello_error_panic()),
        ),
    };
    fn try_hello_error_panic() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let res = client.try_hello(&Flag::C);
        match (&res, &Err(Ok(Error::AnError))) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !!client.persisted() {
            ::core::panicking::panic("assertion failed: !client.persisted()")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::try_hello_error_panic_string"]
    #[doc(hidden)]
    pub const try_hello_error_panic_string: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::try_hello_error_panic_string"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/errors/src/lib.rs",
            start_line: 107usize,
            start_col: 8usize,
            end_line: 107usize,
            end_col: 36usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(try_hello_error_panic_string()),
        ),
    };
    fn try_hello_error_panic_string() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let res = client.try_hello(&Flag::D);
        match (&res, &Err(Err(InvokeError::Abort))) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !!client.persisted() {
            ::core::panicking::panic("assertion failed: !client.persisted()")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::try_hello_error_unexpected_contract_error"]
    #[doc(hidden)]
    pub const try_hello_error_unexpected_contract_error: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("test::try_hello_error_unexpected_contract_error"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/errors/src/lib.rs",
                start_line: 118usize,
                start_col: 8usize,
                end_line: 118usize,
                end_col: 49usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(try_hello_error_unexpected_contract_error()),
            ),
        };
    fn try_hello_error_unexpected_contract_error() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let res = client.try_hello(&Flag::E);
        match (&res, &Err(Err(InvokeError::Contract(9)))) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        if !!client.persisted() {
            ::core::panicking::panic("assertion failed: !client.persisted()")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::type_conversion"]
    #[doc(hidden)]
    pub const type_conversion: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::type_conversion"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/errors/src/lib.rs",
            start_line: 129usize,
            start_col: 8usize,
            end_line: 129usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(type_conversion()),
        ),
    };
    fn type_conversion() {
        match (
            &<_ as Into<InvokeError>>::into(Error::AnError),
            &InvokeError::Contract(1),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as TryInto<Error>>::try_into(InvokeError::Contract(1)),
            &Ok(Error::AnError),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as TryInto<Error>>::try_into(InvokeError::Contract(2)),
            &Err(InvokeError::Contract(2)),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as TryInto<Error>>::try_into(InvokeError::Abort),
            &Err(InvokeError::Abort),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as Into<soroban_sdk::Error>>::into(Error::AnError),
            &soroban_sdk::Error::from_contract_error(1),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as TryInto<Error>>::try_into(soroban_sdk::Error::from_contract_error(1)),
            &Ok(Error::AnError),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as TryInto<Error>>::try_into(soroban_sdk::Error::from_contract_error(2)),
            &Err(soroban_sdk::Error::from_contract_error(2)),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as TryInto<Error>>::try_into(soroban_sdk::Error::from_type_and_code(
                xdr::ScErrorType::Context,
                xdr::ScErrorCode::InvalidAction,
            )),
            &Err(soroban_sdk::Error::from_type_and_code(
                xdr::ScErrorType::Context,
                xdr::ScErrorCode::InvalidAction,
            )),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as Into<InvokeError>>::into(soroban_sdk::Error::from_contract_error(1)),
            &InvokeError::Contract(1),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &<_ as Into<InvokeError>>::into(soroban_sdk::Error::from_type_and_code(
                xdr::ScErrorType::Context,
                xdr::ScErrorCode::InvalidAction,
            )),
            &InvokeError::Abort,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &hello_ok,
        &try_hello_error,
        &try_hello_error_panic,
        &try_hello_error_panic_string,
        &try_hello_error_unexpected_contract_error,
        &try_hello_ok,
        &type_conversion,
    ])
}
