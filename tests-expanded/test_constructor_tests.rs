#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, contracttype, Env};
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
pub enum DataKey {
    Persistent(u32),
    Temp(u32),
    Instance(u32),
}
pub static __SPEC_XDR_TYPE_DATAKEY: [u8; 112usize] = DataKey::spec_xdr();
impl DataKey {
    pub const fn spec_xdr() -> [u8; 112usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07DataKey\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\nPersistent\0\0\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x04Temp\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x08Instance\0\0\0\x01\0\0\0\x04"
    }
}
impl soroban_sdk::IncludeSpecMarker for DataKey {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for DataKey {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["Persistent", "Temp", "Instance"];
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
                    Self::Persistent(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Temp(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Instance(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, DataKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &DataKey,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            DataKey::Persistent(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Persistent")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            DataKey::Temp(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Temp")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            DataKey::Instance(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Instance")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &DataKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&DataKey,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, DataKey>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for DataKey {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVec,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let vec = val;
        let mut iter = vec.iter();
        let discriminant: soroban_sdk::xdr::ScSymbol = iter
            .next()
            .ok_or(soroban_sdk::xdr::Error::Invalid)?
            .clone()
            .try_into()
            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
        let discriminant_name: &str = &discriminant.to_utf8_string()?;
        Ok(match discriminant_name {
            "Persistent" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::Persistent(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            "Temp" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::Temp(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            "Instance" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::Instance(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            _ => Err(soroban_sdk::xdr::Error::Invalid)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for DataKey {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVal,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
            <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
        } else {
            Err(soroban_sdk::xdr::Error::Invalid)
        }
    }
}
impl TryFrom<&DataKey> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &DataKey) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        Ok(match val {
            DataKey::Persistent(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "Persistent"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                ),
                value0,
            )
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            DataKey::Temp(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "Temp"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                ),
                value0,
            )
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            DataKey::Instance(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "Instance"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                ),
                value0,
            )
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
        })
    }
}
impl TryFrom<DataKey> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: DataKey) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&DataKey> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &DataKey) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
    }
}
impl TryFrom<DataKey> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: DataKey) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryDataKey {
        Persistent(<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
        Temp(<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
        Instance(<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryDataKey {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ArbitraryDataKey::Persistent(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Persistent", &__self_0)
                }
                ArbitraryDataKey::Temp(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Temp", &__self_0)
                }
                ArbitraryDataKey::Instance(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Instance", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryDataKey {
        #[inline]
        fn clone(&self) -> ArbitraryDataKey {
            match self {
                ArbitraryDataKey::Persistent(__self_0) => {
                    ArbitraryDataKey::Persistent(::core::clone::Clone::clone(__self_0))
                }
                ArbitraryDataKey::Temp(__self_0) => {
                    ArbitraryDataKey::Temp(::core::clone::Clone::clone(__self_0))
                }
                ArbitraryDataKey::Instance(__self_0) => {
                    ArbitraryDataKey::Instance(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryDataKey {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryDataKey {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryDataKey {
        #[inline]
        fn eq(&self, other: &ArbitraryDataKey) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ArbitraryDataKey::Persistent(__self_0),
                        ArbitraryDataKey::Persistent(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (ArbitraryDataKey::Temp(__self_0), ArbitraryDataKey::Temp(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (
                        ArbitraryDataKey::Instance(__self_0),
                        ArbitraryDataKey::Instance(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryDataKey {
        #[inline]
        fn cmp(&self, other: &ArbitraryDataKey) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (
                        ArbitraryDataKey::Persistent(__self_0),
                        ArbitraryDataKey::Persistent(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (ArbitraryDataKey::Temp(__self_0), ArbitraryDataKey::Temp(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (
                        ArbitraryDataKey::Instance(__self_0),
                        ArbitraryDataKey::Instance(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryDataKey {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryDataKey,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (
                    ArbitraryDataKey::Persistent(__self_0),
                    ArbitraryDataKey::Persistent(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (ArbitraryDataKey::Temp(__self_0), ArbitraryDataKey::Temp(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ArbitraryDataKey::Instance(__self_0), ArbitraryDataKey::Instance(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryDataKey: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryDataKey {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryDataKey.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64) >> 32
                        {
                            0u64 => {
                                ArbitraryDataKey::Persistent(arbitrary::Arbitrary::arbitrary(u)?)
                            }
                            1u64 => ArbitraryDataKey::Temp(arbitrary::Arbitrary::arbitrary(u)?),
                            2u64 => ArbitraryDataKey::Instance(arbitrary::Arbitrary::arbitrary(u)?),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryDataKey.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryDataKey.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?) * 3u64)
                            >> 32
                        {
                            0u64 => ArbitraryDataKey::Persistent(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            1u64 => ArbitraryDataKey::Temp(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            2u64 => ArbitraryDataKey::Instance(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryDataKey.with(|count| {
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
                        arbitrary::size_hint::or_all(
                                &[
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                depth,
                                            ),
                                        ],
                                    ),
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                depth,
                                            ),
                                        ],
                                    ),
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                depth,
                                            ),
                                        ],
                                    ),
                                ],
                            )
                    }),
                )
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for DataKey {
        type Prototype = ArbitraryDataKey;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryDataKey> for DataKey {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryDataKey,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryDataKey::Persistent(field_0) => {
                    DataKey::Persistent(soroban_sdk::IntoVal::into_val(field_0, env))
                }
                ArbitraryDataKey::Temp(field_0) => {
                    DataKey::Temp(soroban_sdk::IntoVal::into_val(field_0, env))
                }
                ArbitraryDataKey::Instance(field_0) => {
                    DataKey::Instance(soroban_sdk::IntoVal::into_val(field_0, env))
                }
            })
        }
    }
};
impl Contract {
    pub fn __constructor(env: Env, init_key: u32, init_value: i64) {
        env.storage()
            .persistent()
            .set(&DataKey::Persistent(init_key), &init_value);
        env.storage()
            .temporary()
            .set(&DataKey::Temp(init_key * 2), &(init_value * 2));
        env.storage()
            .instance()
            .set(&DataKey::Instance(init_key * 3), &(init_value * 3));
    }
    pub fn get_data(env: Env, key: DataKey) -> Option<i64> {
        match key {
            DataKey::Persistent(_) => env.storage().persistent().get(&key),
            DataKey::Temp(_) => env.storage().temporary().get(&key),
            DataKey::Instance(_) => env.storage().instance().get(&key),
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract____constructor__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN___CONSTRUCTOR: [u8; 80usize] =
        super::Contract::spec_xdr___constructor();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr___constructor() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\r__constructor\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x08init_key\0\0\0\x04\0\0\0\0\0\0\0\ninit_value\0\0\0\0\0\x07\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_data__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_GET_DATA: [u8; 64usize] = super::Contract::spec_xdr_get_data();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_data() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08get_data\0\0\0\x01\0\0\0\0\0\0\0\x03key\0\0\0\x07\xd0\0\0\0\x07DataKey\0\0\0\0\x01\0\0\x03\xe8\0\0\0\x07"
    }
}
impl<'a> ContractClient<'a> {
    pub fn get_data(&self, key: &DataKey) -> Option<i64> {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_data");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_get_data(
        &self,
        key: &DataKey,
    ) -> Result<
        Result<
            Option<i64>,
            <Option<i64> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_data");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
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
    pub fn __constructor<'i>(init_key: &'i u32, init_value: &'i i64) -> (&'i u32, &'i i64) {
        (init_key, init_value)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_data<'i>(key: &'i DataKey) -> (&'i DataKey,) {
        (key,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract____constructor {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::__constructor(
                env.clone(),
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 2usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    2usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize], args[1usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_data {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_data` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::get_data(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_data` instead")]
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_data` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__99dc7227b32e52c8d11ead5dec3dd80bafdad62d74493e7341c782fd8cb13593_ctor() {
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
                    __Contract__99dc7227b32e52c8d11ead5dec3dd80bafdad62d74493e7341c782fd8cb13593_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "__constructor",
            #[allow(deprecated)]
            &__Contract____constructor::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "get_data",
            #[allow(deprecated)]
            &__Contract__get_data::invoke_raw_slice,
        );
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_constructor"]
#[doc(hidden)]
pub const test_constructor: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_constructor"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/constructor/src/lib.rs",
        start_line: 38usize,
        start_col: 4usize,
        end_line: 38usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::UnitTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_constructor()),
    ),
};
fn test_constructor() {
    let env = Env::default();
    let contract_id = env.register(Contract, ContractArgs::__constructor(&100_u32, &1000_i64));
    let client = ContractClient::new(&env, &contract_id);
    match (&client.get_data(&DataKey::Persistent(100)), &Some(1000)) {
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
    match (&client.get_data(&DataKey::Temp(200)), &Some(2000)) {
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
    match (&client.get_data(&DataKey::Instance(300)), &Some(3000)) {
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
    match (&client.get_data(&DataKey::Persistent(10)), &None) {
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
    match (&client.get_data(&DataKey::Temp(20)), &None) {
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
    match (&client.get_data(&DataKey::Instance(30)), &None) {
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
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_passing_no_constructor_arguments_causes_panic"]
#[doc(hidden)]
pub const test_passing_no_constructor_arguments_causes_panic: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_passing_no_constructor_arguments_causes_panic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/constructor/src/lib.rs",
            start_line: 53usize,
            start_col: 4usize,
            end_line: 53usize,
            end_col: 54usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage(
                "constructor invocation has failed with error",
            ),
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_passing_no_constructor_arguments_causes_panic()),
        ),
    };
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_passing_no_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register(Contract, ());
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_missing_constructor_arguments_causes_panic"]
#[doc(hidden)]
pub const test_missing_constructor_arguments_causes_panic: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_missing_constructor_arguments_causes_panic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/constructor/src/lib.rs",
            start_line: 60usize,
            start_col: 4usize,
            end_line: 60usize,
            end_col: 51usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage(
                "constructor invocation has failed with error",
            ),
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_missing_constructor_arguments_causes_panic()),
        ),
    };
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_missing_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register(Contract, (100_u32,));
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_passing_extra_constructor_arguments_causes_panic"]
#[doc(hidden)]
pub const test_passing_extra_constructor_arguments_causes_panic: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_passing_extra_constructor_arguments_causes_panic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/constructor/src/lib.rs",
            start_line: 67usize,
            start_col: 4usize,
            end_line: 67usize,
            end_col: 57usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage(
                "constructor invocation has failed with error",
            ),
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_passing_extra_constructor_arguments_causes_panic()),
        ),
    };
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_passing_extra_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register(Contract, (100_u32, 1000_i64, 123_u32));
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_passing_incorrectly_typed_constructor_arguments_causes_panic"]
#[doc(hidden)]
pub const test_passing_incorrectly_typed_constructor_arguments_causes_panic: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "test_passing_incorrectly_typed_constructor_arguments_causes_panic",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/constructor/src/lib.rs",
            start_line: 74usize,
            start_col: 4usize,
            end_line: 74usize,
            end_col: 69usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage(
                "constructor invocation has failed with error",
            ),
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || {
                test::assert_test_result(
                    test_passing_incorrectly_typed_constructor_arguments_causes_panic(),
                )
            },
        ),
    };
#[should_panic(expected = "constructor invocation has failed with error")]
fn test_passing_incorrectly_typed_constructor_arguments_causes_panic() {
    let env = Env::default();
    let _ = env.register(Contract, (100_u32, 1000_u32));
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_constructor,
        &test_missing_constructor_arguments_causes_panic,
        &test_passing_extra_constructor_arguments_causes_panic,
        &test_passing_incorrectly_typed_constructor_arguments_causes_panic,
        &test_passing_no_constructor_arguments_causes_panic,
    ])
}
