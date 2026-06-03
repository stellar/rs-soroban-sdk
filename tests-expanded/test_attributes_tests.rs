#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractevent, contractimpl, contracttrait, contracttype, Env};
pub struct AttributeType {
    pub value: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for AttributeType {
    #[inline]
    fn clone(&self) -> AttributeType {
        AttributeType {
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AttributeType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "AttributeType",
            "value",
            &&self.value,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for AttributeType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AttributeType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for AttributeType {
    #[inline]
    fn eq(&self, other: &AttributeType) -> bool {
        self.value == other.value
    }
}
pub static __SPEC_XDR_TYPE_ATTRIBUTETYPE: [u8; 56usize] = AttributeType::spec_xdr();
impl AttributeType {
    pub const fn spec_xdr() -> [u8; 56usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\rAttributeType\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04"
    }
}
impl soroban_sdk::SpecShakingMarker for AttributeType {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for AttributeType {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["value"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            value: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, AttributeType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &AttributeType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["value"];
        let vals: [Val; 1usize] = [(&val.value)
            .try_into_val(env)
            .map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &AttributeType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&AttributeType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, AttributeType>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for AttributeType {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScMap,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let map = val;
        if map.len() != 1usize {
            return Err(soroban_sdk::xdr::Error::Invalid);
        }
        map.validate()?;
        Ok(Self {
            value: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "value"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
                .into();
                let idx = map
                    .binary_search_by_key(&key, |entry| entry.key.clone())
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                let rv: soroban_sdk::Val = (&map[idx].val.clone())
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                rv.try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
            },
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for AttributeType {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVal,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
            <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
        } else {
            Err(soroban_sdk::xdr::Error::Invalid)
        }
    }
}
impl TryFrom<&AttributeType> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &AttributeType) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
            soroban_sdk::xdr::ScMapEntry {
                key: soroban_sdk::xdr::ScSymbol(
                    "value"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
                .into(),
                val: (&val.value)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            },
        ])))
    }
}
impl TryFrom<AttributeType> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: AttributeType) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&AttributeType> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &AttributeType) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<AttributeType> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: AttributeType) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryAttributeType {
        value: <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryAttributeType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ArbitraryAttributeType",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryAttributeType {
        #[inline]
        fn clone(&self) -> ArbitraryAttributeType {
            ArbitraryAttributeType {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryAttributeType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryAttributeType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryAttributeType {
        #[inline]
        fn eq(&self, other: &ArbitraryAttributeType) -> bool {
            self.value == other.value
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryAttributeType {
        #[inline]
        fn cmp(&self, other: &ArbitraryAttributeType) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.value, &other.value)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryAttributeType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryAttributeType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.value, &other.value)
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryAttributeType: ::std::thread::LocalKey<
            std::cell::Cell<u32>,
        > = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryAttributeType {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryAttributeType.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryAttributeType {
                        value: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryAttributeType.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryAttributeType.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryAttributeType {
                        value: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryAttributeType.with(|count| {
                        count.set(count.get() - 1);
                    });
                }
                result
            }
            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                arbitrary::size_hint::recursion_guard(depth, |depth| {
                    arbitrary::size_hint::and_all(
                        &[
                            <<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for AttributeType {
        type Prototype = ArbitraryAttributeType;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryAttributeType> for AttributeType {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryAttributeType,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(AttributeType {
                value: soroban_sdk::IntoVal::into_val(&v.value, env),
            })
        }
    }
};
pub struct AttributeEvent {
    topic: u32,
    value: u32,
}
pub static __SPEC_XDR_EVENT_ATTRIBUTEEVENT: [u8; 112usize] = AttributeEvent::spec_xdr();
impl AttributeEvent {
    pub const fn spec_xdr() -> [u8; 112usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x0eAttributeEvent\0\0\0\0\0\x01\0\0\0\x0fattribute_event\0\0\0\0\x02\0\0\0\0\0\0\0\x05topic\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::SpecShakingMarker for AttributeEvent {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
    }
}
impl soroban_sdk::Event for AttributeEvent {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "attribute_event") }, {
            let v: soroban_sdk::Val = self.topic.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["value"];
        let vals: [soroban_sdk::Val; 1usize] = [self.value.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl AttributeEvent {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
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
pub struct AttributeTraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_attribute_trait as AttributeTrait;
pub trait AttributeTrait {
    fn trait_override() -> u32 {
        1
    }
    fn trait_default() -> u32 {
        2
    }
    fn trait_default_stacked_cfg() -> u32 {
        5
    }
}
///AttributeTraitClient is a client for calling the contract defined in "AttributeTrait".
pub struct AttributeTraitClient<'a> {
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
impl<'a> AttributeTraitClient<'a> {
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
impl<'a> AttributeTraitClient<'a> {
    pub fn trait_override(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_trait_override(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn trait_default(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_trait_default(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn trait_default_stacked_cfg(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_trait_default_stacked_cfg(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
}
///AttributeTraitArgs is a type for building arg lists for functions defined in "AttributeTrait".
pub struct AttributeTraitArgs;
impl AttributeTraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_override<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default_stacked_cfg<'i>() -> () {
        ()
    }
}
impl AttributeTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_override() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etrait_override\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl AttributeTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rtrait_default\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl AttributeTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default_stacked_cfg() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x19trait_default_stacked_cfg\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl Contract {
    pub fn always(value: AttributeType) -> u32 {
        value.value
    }
    pub fn cfg_included(value: u32) -> u32 {
        value
    }
    pub fn publish(env: Env, topic: u32, value: u32) {
        AttributeEvent { topic, value }.publish(&env);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__always__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_ALWAYS: [u8; 72usize] = super::Contract::spec_xdr_always();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_always() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06always\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\x07\xd0\0\0\0\rAttributeType\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__cfg_included__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CFG_INCLUDED: [u8; 56usize] = super::Contract::spec_xdr_cfg_included();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_cfg_included() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ccfg_included\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
    }
}
impl Contract {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_PUBLISH: [u8; 68usize] = super::Contract::spec_xdr_publish();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish() -> [u8; 68usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07publish\0\0\0\0\x02\0\0\0\0\0\0\0\x05topic\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x04\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn always(&self, value: &AttributeType) -> u32 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("always");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_always(
        &self,
        value: &AttributeType,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("always");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn cfg_included(&self, value: &u32) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "cfg_included") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_cfg_included(
        &self,
        value: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "cfg_included") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn publish(&self, topic: &u32, value: &u32) -> () {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("publish");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [topic.into_val(&self.env), value.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_publish(
        &self,
        topic: &u32,
        value: &u32,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("publish");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [topic.into_val(&self.env), value.into_val(&self.env)],
            ),
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
    pub fn always<'i>(value: &'i AttributeType) -> (&'i AttributeType,) {
        (value,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn cfg_included<'i>(value: &'i u32) -> (&'i u32,) {
        (value,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish<'i>(topic: &'i u32, value: &'i u32) -> (&'i u32, &'i u32) {
        (topic, value)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).always` instead")]
#[allow(deprecated)]
pub fn __Contract__always__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::always(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).always` instead")]
pub fn __Contract__always__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__always__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).always` instead")]
pub extern "C" fn __Contract__always__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__always__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).cfg_included` instead")]
#[allow(deprecated)]
pub fn __Contract__cfg_included__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::cfg_included(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).cfg_included` instead")]
pub fn __Contract__cfg_included__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__cfg_included__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).cfg_included` instead")]
pub extern "C" fn __Contract__cfg_included__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__cfg_included__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish` instead")]
#[allow(deprecated)]
pub fn __Contract__publish__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish` instead")]
pub fn __Contract__publish__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__publish__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish` instead")]
pub extern "C" fn __Contract__publish__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____2246acd8c5a7d7a27c1bbb827466a98b2233215f0f3fdd39122db001505b94ef_ctor() {
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
                    __Contract____2246acd8c5a7d7a27c1bbb827466a98b2233215f0f3fdd39122db001505b94ef_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "always",
            #[allow(deprecated)]
            &__Contract__always__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "cfg_included",
            #[allow(deprecated)]
            &__Contract__cfg_included__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "publish",
            #[allow(deprecated)]
            &__Contract__publish__invoke_raw_slice,
        );
    }
}
impl AttributeTrait for Contract {
    fn trait_override() -> u32 {
        3
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__trait_override__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_TRAIT_OVERRIDE: [u8; 40usize] =
        super::Contract::spec_xdr_trait_override();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_override() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etrait_override\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl Contract {}
impl Contract {}
impl<'a> ContractClient<'a> {
    pub fn trait_override(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_trait_override(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_override") },
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
    pub fn trait_override<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_override` instead")]
#[allow(deprecated)]
pub fn __Contract__trait_override__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AttributeTrait>::trait_override(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_override` instead")]
pub fn __Contract__trait_override__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__trait_override__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_override` instead")]
pub extern "C" fn __Contract__trait_override__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__trait_override__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_default` instead")]
#[allow(deprecated)]
pub fn __Contract__trait_default__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AttributeTrait>::trait_default(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_default` instead")]
pub fn __Contract__trait_default__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__trait_default__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).trait_default` instead")]
pub extern "C" fn __Contract__trait_default__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__trait_default__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractClient::new(&env, &contract_id).trait_default_stacked_cfg` instead"
)]
#[allow(deprecated)]
pub fn __Contract__trait_default_stacked_cfg__invoke_raw(
    env: soroban_sdk::Env,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AttributeTrait>::trait_default_stacked_cfg(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractClient::new(&env, &contract_id).trait_default_stacked_cfg` instead"
)]
pub fn __Contract__trait_default_stacked_cfg__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__trait_default_stacked_cfg__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractClient::new(&env, &contract_id).trait_default_stacked_cfg` instead"
)]
pub extern "C" fn __Contract__trait_default_stacked_cfg__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__trait_default_stacked_cfg__invoke_raw(soroban_sdk::Env::default())
}
impl Contract {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__trait_default__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_TRAIT_DEFAULT: [u8; 40usize] =
        super::Contract::spec_xdr_trait_default();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rtrait_default\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__trait_default_stacked_cfg__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_TRAIT_DEFAULT_STACKED_CFG: [u8; 52usize] =
        super::Contract::spec_xdr_trait_default_stacked_cfg();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_trait_default_stacked_cfg() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x19trait_default_stacked_cfg\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn trait_default(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_trait_default(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn trait_default_stacked_cfg(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_trait_default_stacked_cfg(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "trait_default_stacked_cfg") },
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
    pub fn trait_default<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn trait_default_stacked_cfg<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__AttributeTrait__5c3dc6ba0d711665380c5f56dcc72fd5dd342c89561cb9e2302a58b4f4cac9dd_ctor(
) {
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
                    __Contract__AttributeTrait__5c3dc6ba0d711665380c5f56dcc72fd5dd342c89561cb9e2302a58b4f4cac9dd_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "trait_default",
            #[allow(deprecated)]
            &__Contract__trait_default__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "trait_default_stacked_cfg",
            #[allow(deprecated)]
            &__Contract__trait_default_stacked_cfg__invoke_raw_slice,
        );
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__AttributeTrait__969988e5ad11b5d89373fc419647b603f4e9508cb73e2783cdc5572a30929b70_ctor(
) {
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
                    __Contract__AttributeTrait__969988e5ad11b5d89373fc419647b603f4e9508cb73e2783cdc5572a30929b70_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "trait_override",
            #[allow(deprecated)]
            &__Contract__trait_override__invoke_raw_slice,
        );
    }
}
mod test {
    extern crate std;
    use super::*;
    use soroban_sdk::{testutils::Events, Env};
    fn attribute_value() -> AttributeType {
        AttributeType { value: 1 }
    }
    extern crate test;
    #[rustc_test_marker = "test::test_cfg_items"]
    #[doc(hidden)]
    pub const test_cfg_items: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_cfg_items"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/attributes/src/lib.rs",
            start_line: 89usize,
            start_col: 8usize,
            end_line: 89usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_cfg_items()),
        ),
    };
    fn test_cfg_items() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        match (&client.always(&attribute_value()), &1) {
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
        client.publish(&1, &2);
        match (&env.events().all().events().len(), &1) {
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
    extern crate test;
    #[rustc_test_marker = "test::test_cfg_methods"]
    #[doc(hidden)]
    pub const test_cfg_methods: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_cfg_methods"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/attributes/src/lib.rs",
            start_line: 101usize,
            start_col: 8usize,
            end_line: 101usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_cfg_methods()),
        ),
    };
    fn test_cfg_methods() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        match (&client.cfg_included(&3), &3) {
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
    extern crate test;
    #[rustc_test_marker = "test::test_contracttrait_cfg_override_and_default"]
    #[doc(hidden)]
    pub const test_contracttrait_cfg_override_and_default: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("test::test_contracttrait_cfg_override_and_default"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/attributes/src/lib.rs",
                start_line: 110usize,
                start_col: 8usize,
                end_line: 110usize,
                end_col: 51usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test_contracttrait_cfg_override_and_default()),
            ),
        };
    fn test_contracttrait_cfg_override_and_default() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        match (&client.trait_override(), &3) {
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
        match (&client.trait_default(), &2) {
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
        match (&client.trait_default_stacked_cfg(), &5) {
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
    extern crate test;
    #[rustc_test_marker = "test::test_specs_include_active_attribute_items"]
    #[doc(hidden)]
    pub const test_specs_include_active_attribute_items: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("test::test_specs_include_active_attribute_items"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/attributes/src/lib.rs",
                start_line: 121usize,
                start_col: 8usize,
                end_line: 121usize,
                end_col: 49usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test_specs_include_active_attribute_items()),
            ),
        };
    fn test_specs_include_active_attribute_items() {
        use soroban_sdk::xdr::{
            Limits, ReadXdr, ScSpecEntry, ScSpecEventParamLocationV0, ScSpecFunctionV0,
            ScSpecTypeDef,
        };
        use std::collections::HashSet;
        let type_entry = ScSpecEntry::from_xdr(AttributeType::spec_xdr(), Limits::none()).unwrap();
        let ScSpecEntry::UdtStructV0(type_spec) = type_entry else {
            {
                ::core::panicking::panic_fmt(format_args!("expected struct spec"));
            };
        };
        match (&type_spec.fields[0].name.to_utf8_string_lossy(), &"value") {
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
        match (&type_spec.fields[0].type_, &ScSpecTypeDef::U32) {
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
        let event_entry =
            ScSpecEntry::from_xdr(AttributeEvent::spec_xdr(), Limits::none()).unwrap();
        let ScSpecEntry::EventV0(event_spec) = event_entry else {
            {
                ::core::panicking::panic_fmt(format_args!("expected event spec"));
            };
        };
        let event_params = event_spec
            .params
            .iter()
            .map(|p| {
                (
                    p.name.to_utf8_string_lossy(),
                    p.type_.clone(),
                    p.location.clone(),
                )
            })
            .collect::<std::vec::Vec<_>>();
        match (
            &event_params,
            &[
                (
                    "topic".into(),
                    ScSpecTypeDef::U32,
                    ScSpecEventParamLocationV0::TopicList,
                ),
                (
                    "value".into(),
                    ScSpecTypeDef::U32,
                    ScSpecEventParamLocationV0::Data,
                ),
            ],
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
        let method_entry =
            ScSpecEntry::from_xdr(Contract::spec_xdr_cfg_included(), Limits::none()).unwrap();
        let ScSpecEntry::FunctionV0(ScSpecFunctionV0 { name, inputs, .. }) = method_entry else {
            {
                ::core::panicking::panic_fmt(format_args!("expected function spec"));
            };
        };
        match (&name.to_utf8_string_lossy(), &"cfg_included") {
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
        match (&inputs[0].type_, &ScSpecTypeDef::U32) {
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
        let wasm = b"\x00asm\x01\x00\x00\x00\x01\x1f\x05`\x04~~~~\x01~`\x02~~\x01~`\x03~~~\x01~`\x01~\x01~`\x00\x01~\x02\x1f\x05\x01m\x01a\x00\x00\x01b\x01j\x00\x01\x01v\x01g\x00\x01\x01m\x019\x00\x02\x01x\x011\x00\x01\x03\x07\x06\x03\x03\x01\x04\x04\x04\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\xbb\x80\xc0\x00\x0b\x7f\x00A\xbb\x80\xc0\x00\x0b\x7f\x00A\xc0\x80\xc0\x00\x0b\x07\x88\x01\n\x06memory\x02\x00\x06always\x00\x05\x0ccfg_included\x00\x06\x07publish\x00\x07\rtrait_default\x00\x08\x19trait_default_stacked_cfg\x00\t\x0etrait_override\x00\n\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xdd\x03\x06\x85\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00A\x00-\x00\x80\x80\xc0\x80\x00\x1a \x01B\x027\x03\x08\x02@\x02@ \x00B\xff\x01\x83B\xcc\x00R\r\x00 \x00A\xa4\x80\xc0\x80\x00\xadB \x86B\x04\x84 \x01A\x08j\xadB \x86B\x04\x84B\x84\x80\x80\x80\x10\x10\x80\x80\x80\x80\x00\x1a \x01)\x03\x08\"\x00B\xff\x01\x83B\x04Q\r\x01\x0b\x00\x0b \x01A\x10j$\x80\x80\x80\x80\x00 \x00B\x84\x80\x80\x80p\x83\x0b\x1a\x00\x02@ \x00B\xff\x01\x83B\x04Q\r\x00\x00\x0b \x00B\x84\x80\x80\x80p\x83\x0b\x9c\x02\x02\x02\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\x04R\r\x00 \x01B\xff\x01\x83B\x04R\r\x00A\x00!\x03A\x00-\x00\x8e\x80\xc0\x80\x00\x1aA\xac\x80\xc0\x80\x00\xadB \x86B\x04\x84B\x84\x80\x80\x80\xf0\x01\x10\x81\x80\x80\x80\x00!\x04 \x02 \x00B\x84\x80\x80\x80p\x837\x03\x08 \x02 \x047\x03\x00\x03@\x02@ \x03A\x10G\r\x00A\x00!\x03\x02@\x03@ \x03A\x10F\r\x01 \x02A\x10j \x03j \x02 \x03j)\x03\x007\x03\x00 \x03A\x08j!\x03\x0c\x00\x0b\x0b \x02A\x10j\xadB \x86B\x04\x84\"\x00B\x84\x80\x80\x80 \x10\x82\x80\x80\x80\x00!\x04 \x02 \x01B\x84\x80\x80\x80p\x837\x03\x10 \x04A\xa4\x80\xc0\x80\x00\xadB \x86B\x04\x84 \x00B\x84\x80\x80\x80\x10\x10\x83\x80\x80\x80\x00\x10\x84\x80\x80\x80\x00\x1a \x02A j$\x80\x80\x80\x80\x00B\x02\x0f\x0b \x02A\x10j \x03jB\x027\x03\x00 \x03A\x08j!\x03\x0c\x00\x0b\x0b\x00\x0b\x08\x00B\x84\x80\x80\x80 \x0b\t\x00B\x84\x80\x80\x80\xd0\x00\x0b\x08\x00B\x84\x80\x80\x800\x0b\x0bD\x01\x00A\x80\x80\xc0\x00\x0b;SpEcV1i\t\xb7\x06*\x88\xd7\xf8SpEcV1\xbfO\xc3P\xd4\x14\xb5Vvalue\x00\x00\x00\x1c\x00\x10\x00\x05\x00\x00\x00attribute_event\x00\x93\x19\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06always\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05value\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\rAttributeType\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07publish\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05topic\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x05value\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\rAttributeType\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05value\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0ccfg_included\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05value\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\rtrait_default\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0eAttributeEvent\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0fattribute_event\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05topic\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05value\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0etrait_override\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x19trait_default_stacked_cfg\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x07Context\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nExecutable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
        let fn_names: HashSet<std::string::String> = soroban_spec::read::from_wasm(wasm)
            .unwrap()
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::FunctionV0(f) = e {
                    Some(f.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();
        if !fn_names.contains("always") {
            ::core::panicking::panic("assertion failed: fn_names.contains(\"always\")")
        }
        if !fn_names.contains("cfg_included") {
            ::core::panicking::panic("assertion failed: fn_names.contains(\"cfg_included\")")
        }
        if !fn_names.contains("publish") {
            ::core::panicking::panic("assertion failed: fn_names.contains(\"publish\")")
        }
        if !fn_names.contains("trait_override") {
            ::core::panicking::panic("assertion failed: fn_names.contains(\"trait_override\")")
        }
        if !fn_names.contains("trait_default") {
            ::core::panicking::panic("assertion failed: fn_names.contains(\"trait_default\")")
        }
        if !fn_names.contains("trait_default_stacked_cfg") {
            ::core::panicking::panic(
                "assertion failed: fn_names.contains(\"trait_default_stacked_cfg\")",
            )
        }
        if !!fn_names.contains("cfg_excluded") {
            ::core::panicking::panic("assertion failed: !fn_names.contains(\"cfg_excluded\")")
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_cfg_items,
        &test_cfg_methods,
        &test_contracttrait_cfg_override_and_default,
        &test_specs_include_active_attribute_items,
    ])
}
