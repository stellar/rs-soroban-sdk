#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, contracttype};
use test_spec_lib_no_lto::{Enum, IntEnum, Outer, Tuple, Wrapped};
pub struct Wrapper(pub Wrapped);
#[automatically_derived]
impl ::core::clone::Clone for Wrapper {
    #[inline]
    fn clone(&self) -> Wrapper {
        let _: ::core::clone::AssertParamIsClone<Wrapped>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Wrapper {}
#[automatically_derived]
impl ::core::fmt::Debug for Wrapper {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wrapper", &&self.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Wrapper {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Wrapped>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Wrapper {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Wrapper {
    #[inline]
    fn eq(&self, other: &Wrapper) -> bool {
        self.0 == other.0
    }
}
impl soroban_sdk::SpecName for Wrapper {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_spec_no_lto::Wrapper";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `Wrapper` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_spec_no_lto::Wrapper`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for Wrapper {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
#[doc(hidden)]
#[allow(dead_code)]
static __SPEC_XDR_TYPE_WRAPPER: [u8; Wrapper::spec_xdr().len()] = Wrapper::spec_xdr();
impl Wrapper {
    #[doc(hidden)]
    #[inline(never)]
    pub fn __spec_link() {}
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <Wrapper as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"0"),
                        type_: <Wrapped as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Wrapper::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Wrapper::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Wrapper {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Wrapper::__spec_link();
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Wrapper> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Wrapper,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Wrapper::__spec_link();
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 1usize] = [(&val.0).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Wrapper> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Wrapper,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Wrapper>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for Wrapper {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVec,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let vec = val;
        if vec.len() != 1usize {
            return Err(soroban_sdk::xdr::Error::Invalid);
        }
        Ok(Self {
            0: {
                let rv: soroban_sdk::Val = (&vec[0].clone())
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                rv.try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
            },
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Wrapper {
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
impl TryFrom<&Wrapper> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &Wrapper) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        Ok(soroban_sdk::xdr::ScVec(
            <[_]>::into_vec(::alloc::boxed::box_new([(&val.0)
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?]))
            .try_into()?,
        ))
    }
}
impl TryFrom<Wrapper> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: Wrapper) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&Wrapper> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &Wrapper) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
    }
}
impl TryFrom<Wrapper> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: Wrapper) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryWrapper(
        <Wrapped as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    );
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryWrapper {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ArbitraryWrapper", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryWrapper {
        #[inline]
        fn clone(&self) -> ArbitraryWrapper {
            ArbitraryWrapper(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryWrapper {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Wrapped as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryWrapper {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryWrapper {
        #[inline]
        fn eq(&self, other: &ArbitraryWrapper) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryWrapper {
        #[inline]
        fn cmp(&self, other: &ArbitraryWrapper) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryWrapper {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryWrapper,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryWrapper: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryWrapper {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryWrapper.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| Ok(ArbitraryWrapper(arbitrary::Arbitrary::arbitrary(u)?)))();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryWrapper.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryWrapper.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryWrapper(arbitrary::Arbitrary::arbitrary_take_rest(
                        u,
                    )?))
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryWrapper.with(|count| {
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
                            <<Wrapped as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Wrapper {
        type Prototype = ArbitraryWrapper;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryWrapper> for Wrapper {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryWrapper,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(Wrapper(soroban_sdk::IntoVal::into_val(&v.0, env)))
        }
    }
};
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    config: soroban_sdk::testutils::ClientInternalConfig<'a>,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            config: soroban_sdk::testutils::ClientInternalConfig::default(),
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
            config: self.config.set_auths(auths),
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
            config: self.config.mock_auths(mock_auths),
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
            config: self.config.mock_all_auths(),
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
            config: self.config.mock_all_auths_allowing_non_root_auth(),
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
impl Contract {
    pub fn int_enum(v: IntEnum) -> IntEnum {
        v
    }
    pub fn enum_(v: Enum) -> Enum {
        v
    }
    pub fn tuple(v: Tuple) -> Tuple {
        v
    }
    pub fn outer(v: Outer) -> u32 {
        v.0 .0
    }
    pub fn wrapper(v: Wrapper) -> u32 {
        v.0 .0
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__int_enum__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_INT_ENUM: [u8; super::Contract::spec_xdr_int_enum().len()] =
        super::Contract::spec_xdr_int_enum();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_int_enum: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"int_enum"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <IntEnum as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <IntEnum as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_int_enum() -> [u8; Contract::__SPEC_XDR_ENTRY_int_enum.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_int_enum.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__enum___spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_ENUM_: [u8; super::Contract::spec_xdr_enum_().len()] =
        super::Contract::spec_xdr_enum_();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_enum_: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"enum_"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Enum as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Enum as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_enum_() -> [u8; Contract::__SPEC_XDR_ENTRY_enum_.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_enum_.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__tuple__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TUPLE: [u8; super::Contract::spec_xdr_tuple().len()] =
        super::Contract::spec_xdr_tuple();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_tuple: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"tuple"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Tuple as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Tuple as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple() -> [u8; Contract::__SPEC_XDR_ENTRY_tuple.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_tuple.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__outer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_OUTER: [u8; super::Contract::spec_xdr_outer().len()] =
        super::Contract::spec_xdr_outer();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_outer: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"outer"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Outer as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_outer() -> [u8; Contract::__SPEC_XDR_ENTRY_outer.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_outer.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__wrapper__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_WRAPPER: [u8; super::Contract::spec_xdr_wrapper().len()] =
        super::Contract::spec_xdr_wrapper();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_wrapper: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"wrapper"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Wrapper as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrapper() -> [u8; Contract::__SPEC_XDR_ENTRY_wrapper.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_wrapper.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn int_enum(&self, v: &IntEnum) -> IntEnum {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("int_enum");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_int_enum(
        &self,
        v: &IntEnum,
    ) -> Result<
        Result<
            IntEnum,
            <IntEnum as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("int_enum");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn enum_(&self, v: &Enum) -> Enum {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("enum_");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_enum_(
        &self,
        v: &Enum,
    ) -> Result<
        Result<Enum, <Enum as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("enum_");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn tuple(&self, v: &Tuple) -> Tuple {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_tuple(
        &self,
        v: &Tuple,
    ) -> Result<
        Result<
            Tuple,
            <Tuple as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn outer(&self, v: &Outer) -> u32 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("outer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_outer(
        &self,
        v: &Outer,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("outer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn wrapper(&self, v: &Wrapper) -> u32 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("wrapper");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_wrapper(
        &self,
        v: &Wrapper,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("wrapper");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn int_enum<'i>(v: &'i IntEnum) -> (&'i IntEnum,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn enum_<'i>(v: &'i Enum) -> (&'i Enum,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple<'i>(v: &'i Tuple) -> (&'i Tuple,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn outer<'i>(v: &'i Outer) -> (&'i Outer,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrapper<'i>(v: &'i Wrapper) -> (&'i Wrapper,) {
        (v,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).int_enum` instead")]
#[allow(deprecated)]
pub fn __Contract__int_enum__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::int_enum(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).int_enum` instead")]
pub fn __Contract__int_enum__invoke_raw_slice(
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
    __Contract__int_enum__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).int_enum` instead")]
pub extern "C" fn __Contract__int_enum__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__int_enum__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).enum_` instead")]
#[allow(deprecated)]
pub fn __Contract__enum___invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::enum_(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).enum_` instead")]
pub fn __Contract__enum___invoke_raw_slice(
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
    __Contract__enum___invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).enum_` instead")]
pub extern "C" fn __Contract__enum___invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__enum___invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::tuple(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
pub fn __Contract__tuple__invoke_raw_slice(
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
    __Contract__tuple__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
pub extern "C" fn __Contract__tuple__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).outer` instead")]
#[allow(deprecated)]
pub fn __Contract__outer__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::outer(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).outer` instead")]
pub fn __Contract__outer__invoke_raw_slice(
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
    __Contract__outer__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).outer` instead")]
pub extern "C" fn __Contract__outer__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__outer__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapper` instead")]
#[allow(deprecated)]
pub fn __Contract__wrapper__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::wrapper(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapper` instead")]
pub fn __Contract__wrapper__invoke_raw_slice(
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
    __Contract__wrapper__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapper` instead")]
pub extern "C" fn __Contract__wrapper__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__wrapper__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____9fb815ed53569d126533831dfb389fab7cfcc7d77ebec551a30c986e5f8f4c1f_ctor() {
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
                    __Contract____9fb815ed53569d126533831dfb389fab7cfcc7d77ebec551a30c986e5f8f4c1f_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "int_enum",
            #[allow(deprecated)]
            &__Contract__int_enum__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "enum_",
            #[allow(deprecated)]
            &__Contract__enum___invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "tuple",
            #[allow(deprecated)]
            &__Contract__tuple__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "outer",
            #[allow(deprecated)]
            &__Contract__outer__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrapper",
            #[allow(deprecated)]
            &__Contract__wrapper__invoke_raw_slice,
        );
    }
}
mod test {
    extern crate std;
    use std::collections::HashSet;
    use std::string::String;
    use stellar_xdr::ScSpecEntry;
    const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01[\x10`\x02~~\x01~`\x03~~~\x01~`\x01~\x01~`\x02\x7f\x7f\x01~`\x00\x00`\x01\x7f\x00`\x03\x7f\x7f\x7f\x00`\x01\x7f\x01\x7f`\x01\x7f\x01~`\x02\x7f~\x00`\x02\x7f\x7f\x00`\x03\x7f\x7f\x7f\x01~`\x04\x7f~\x7f\x7f\x01~`\x03\x7f~~\x01~`\x02\x7f~\x01~`\x01~\x01\x7f\x02%\x06\x01v\x01g\x00\x00\x01v\x01h\x00\x01\x01b\x01m\x00\x01\x01b\x01j\x00\x00\x01v\x011\x00\x00\x01v\x013\x00\x02\x03)(\x03\x02\x02\x02\x02\x02\x04\x05\x06\x07\x06\x06\x08\t\x06\x06\x06\n\x03\x04\x08\x06\x06\x06\x0b\x0c\x0c\r\x0e\x0b\x0c\x0c\x0b\x06\x0f\n\x08\x0f\n\x05\x04\x05\x01p\x01\x01\x01\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\xa4\x80\xc0\x00\x0b\x7f\x00A\xf8\x80\xc0\x00\x0b\x7f\x00A\x80\x81\xc0\x00\x0b\x07V\t\x06memory\x02\x00\x05enum_\x00\x07\x08int_enum\x00\x08\x05outer\x00\t\x05tuple\x00\n\x07wrapper\x00\x0b\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\x93\x16(:\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x006\x02\x0c \x02A\x0cj \x01\x10\x98\x80\x80\x80\x00!\x03 \x02A\x10j$\x80\x80\x80\x80\x00 \x03\x0b\x9b\x05\x01\x03\x7f#\x80\x80\x80\x80\x00A\xc0\x00k\"\x01$\x80\x80\x80\x80\x00\x10\x99\x80\x80\x80\x00 \x01 \x007\x03\x00\x10\x8c\x80\x80\x80\x00 \x01A(j \x01 \x01A?j\x10\x9b\x80\x80\x80\x00\x02@ \x01(\x02(\r\x00 \x01 \x01)\x0307\x03\x08 \x01A\x10j \x01A\x08j\x10\x9a\x80\x80\x80\x00\x10\x93\x80\x80\x80\x00 \x01A(j \x01A\x10j\x10\x97\x80\x80\x80\x00 \x01)\x03(\"\x00B\x02Q\r\x00 \x00\xa7A\x01q\r\x00 \x01 \x01)\x0307\x03  \x01A(j \x01A j \x01A?j\x10\x9c\x80\x80\x80\x00 \x01(\x02(\r\x00\x02@\x02@\x02@ \x01A?j \x01)\x030A\x84\x80\xc0\x80\x00A\x02\x10\xa0\x80\x80\x80\x00\x10\xa8\x80\x80\x80\x00\"\x02\x0e\x02\x00\x01\x03\x0b \x01A\x10j\x10\x8f\x80\x80\x80\x00A\x01K\r\x02 \x01A(j \x01A\x10j\x10\x97\x80\x80\x80\x00 \x01)\x03(\"\x00B\x02Q\r\x02 \x00\xa7A\x01q\r\x02 \x01)\x030\"\x00B\xff\x01\x83B\x04Q\r\x01\x0c\x02\x0b \x01A\x10j\x10\x8f\x80\x80\x80\x00A\x01K\r\x01 \x01A(j \x01A\x10j\x10\x97\x80\x80\x80\x00 \x01)\x03(\"\x00B\x02Q\r\x01 \x00\xa7A\x01q\r\x01 \x01)\x030\"\x00B\xff\x01\x83B\x04R\r\x01\x0b \x01 \x00B \x88>\x02\x0c \x01 \x026\x02\x08 \x01A\x0cj!\x03\x10\x8c\x80\x80\x80\x00\x02@\x02@ \x02A\x01qE\r\x00 \x01A(j \x01A?jA\x9c\x80\xc0\x80\x00\x10\x94\x80\x80\x80\x00 \x01(\x02(\r\x02 \x01 \x01)\x0307\x03  \x01A j\x10\x92\x80\x80\x80\x00!\x00 \x01A(j \x01A?j \x03\x10\x8e\x80\x80\x80\x00 \x01(\x02(\r\x02 \x01 \x01)\x0307\x03\x18 \x01 \x007\x03\x10 \x01A(j \x01A\x10j \x01A?j\x10\x9d\x80\x80\x80\x00\x0c\x01\x0b \x01A(j \x01A?jA\x94\x80\xc0\x80\x00\x10\x94\x80\x80\x80\x00 \x01(\x02(\r\x01 \x01 \x01)\x0307\x03  \x01A j\x10\x92\x80\x80\x80\x00!\x00 \x01A(j \x01A?j \x03\x10\x8e\x80\x80\x80\x00 \x01(\x02(\r\x01 \x01 \x01)\x0307\x03\x18 \x01 \x007\x03\x10 \x01A(j \x01A\x10j \x01A?j\x10\x9d\x80\x80\x80\x00\x0b \x01)\x030!\x00 \x01)\x03(PE\r\x00 \x01A\xc0\x00j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0bH\x00\x10\x99\x80\x80\x80\x00\x10\x8c\x80\x80\x80\x00\x02@\x02@ \x00B\xff\x01\x83B\x04R\r\x00 \x00B \x88\"\x00\xa7A}jA}K\r\x01\x0b\x00\x0b\x10\x8c\x80\x80\x80\x00B\x84\x80\x80\x80\x10B\x84\x80\x80\x80  \x00B\x01Q\x1b\x0b\xb3\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A k\"\x01$\x80\x80\x80\x80\x00\x10\x99\x80\x80\x80\x00\x10\x8c\x80\x80\x80\x00\x02@\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00 \x01B\x027\x03\x08 \x01A\x1fj \x00 \x01A\x08jA\x01\x10\x9f\x80\x80\x80\x00\x1a\x10\x8c\x80\x80\x80\x00 \x01)\x03\x08\"\x00B\xff\x01\x83B\xcb\x00R\r\x00 \x01B\x027\x03\x10 \x01A\x1fj \x00 \x01A\x10jA\x01\x10\x9f\x80\x80\x80\x00\x1a \x01)\x03\x10\"\x00B\xff\x01\x83B\x04Q\r\x01\x0b\x00\x0b \x00B \x88\xa7 \x01A\x1fj\x10\x86\x80\x80\x80\x00!\x00 \x01A j$\x80\x80\x80\x80\x00 \x00\x0b\x9c\x02\x02\x02\x7f\x01~#\x80\x80\x80\x80\x00A0k\"\x01$\x80\x80\x80\x80\x00\x10\x99\x80\x80\x80\x00\x10\x8c\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00A\x00!\x02\x02@\x03@ \x02A\x10F\r\x01 \x01A\x18j \x02jB\x027\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b \x01A/j \x00 \x01A\x18jA\x02\x10\x9f\x80\x80\x80\x00\x1a \x01)\x03\x18\"\x00B\xff\x01\x83B\x04R\r\x00 \x01)\x03 \"\x03B\xff\x01\x83B\x04R\r\x00 \x01 \x03B \x88>\x02\x04 \x01 \x00B \x88\xa76\x02\x00\x10\x8c\x80\x80\x80\x00 \x01A\x18j \x01A/j \x01\x10\x8e\x80\x80\x80\x00 \x01(\x02\x18\r\x00 \x01)\x03 !\x00 \x01A\x18j \x01A/j \x01A\x04j\x10\x8e\x80\x80\x80\x00 \x01(\x02\x18A\x01F\r\x00 \x01 \x01)\x03 7\x03\x10 \x01 \x007\x03\x08 \x01A/j \x01A\x08jA\x02\x10\x9e\x80\x80\x80\x00!\x00 \x01A0j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\xad\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A k\"\x01$\x80\x80\x80\x80\x00\x10\x99\x80\x80\x80\x00\x02@\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00 \x01B\x027\x03\x08 \x01A\x1fj \x00 \x01A\x08jA\x01\x10\x9f\x80\x80\x80\x00\x1a\x10\x8c\x80\x80\x80\x00 \x01)\x03\x08\"\x00B\xff\x01\x83B\xcb\x00R\r\x00 \x01B\x027\x03\x10 \x01A\x1fj \x00 \x01A\x10jA\x01\x10\x9f\x80\x80\x80\x00\x1a \x01)\x03\x10\"\x00B\xff\x01\x83B\x04Q\r\x01\x0b\x00\x0b \x00B \x88\xa7 \x01A\x1fj\x10\x86\x80\x80\x80\x00!\x00 \x01A j$\x80\x80\x80\x80\x00 \x00\x0b\x02\x00\x0b\x03\x00\x00\x0b\x19\x00 \x00B\x007\x03\x00 \x00 \x025\x02\x00B \x86B\x04\x847\x03\x08\x0b+\x01\x01\x7f\x02@ \x00(\x02\x0c\"\x01 \x00(\x02\x08\"\x00I\r\x00 \x01 \x00k\x0f\x0bA\xbc\x80\xc0\x80\x00\x10\xad\x80\x80\x80\x00\x00\x0b9\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00 \x03 \x02)\x02\x007\x02\x08 \x00 \x01 \x03A\x08j\x10\x91\x80\x80\x80\x00 \x03A\x10j$\x80\x80\x80\x80\x00\x0bm\x02\x02\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00 \x03 \x02(\x02\x00\"\x04 \x02(\x02\x04\"\x02\x10\xa7\x80\x80\x80\x00\x02@\x02@ \x03(\x02\x00A\x01G\r\x00 \x01 \x04 \x02\x10\xa6\x80\x80\x80\x00!\x05\x0c\x01\x0b \x03)\x03\x08!\x05\x0b \x00B\x007\x03\x00 \x00 \x057\x03\x08 \x03A\x10j$\x80\x80\x80\x80\x00\x0b\x07\x00 \x00)\x03\x00\x0bM\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x017\x03\x08 \x00 \x02A\x10j \x01\x10\xa2\x80\x80\x80\x00\x10\xa8\x80\x80\x80\x006\x02\x0c \x00A\x006\x02\x08 \x00 \x017\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0bQ\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00 \x03 \x01 \x02\x10\x90\x80\x80\x80\x00B\x01!\x04\x02@ \x03(\x02\x00\r\x00 \x00 \x03)\x03\x087\x03\x08B\x00!\x04\x0b \x00 \x047\x03\x00 \x03A\x10j$\x80\x80\x80\x80\x00\x0b-\x01\x02~B\x01!\x03\x02@ \x02)\x03\x00\"\x04\x10\xab\x80\x80\x80\x00E\r\x00 \x00 \x047\x03\x08B\x00!\x03\x0b \x00 \x037\x03\x00\x0bR\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00 \x03 \x02)\x03\x087\x03\x08 \x03 \x02)\x03\x007\x03\x00 \x01 \x03A\x02\x10\xa3\x80\x80\x80\x00!\x04 \x00B\x007\x03\x00 \x00 \x047\x03\x08 \x03A\x10j$\x80\x80\x80\x80\x00\x0bN\x02\x01~\x01\x7fB\x02!\x02\x02@ \x01(\x02\x08\"\x03 \x01(\x02\x0cO\r\x00 \x00 \x01A\x08j \x01)\x03\x00 \x03\x10\xaa\x80\x80\x80\x00\x10\xa1\x80\x80\x80\x007\x03\x08 \x01 \x03A\x01j6\x02\x08B\x00!\x02\x0b \x00 \x027\x03\x00\x0b\r\x00 \x005\x02\x00B \x86B\x04\x84\x0b\x02\x00\x0b\x07\x00 \x00)\x03\x00\x0b.\x01\x02~B\x01!\x03\x02@ \x01)\x03\x00\"\x04B\xff\x01\x83B\xcb\x00R\r\x00 \x00 \x047\x03\x08B\x00!\x03\x0b \x00 \x037\x03\x00\x0b\x0e\x00 \x00 \x01 \x01\x10\x95\x80\x80\x80\x00\x0b\x0e\x00 \x00 \x02 \x01\x10\x96\x80\x80\x80\x00\x0b\x0e\x00 \x00 \x01 \x02\x10\xa3\x80\x80\x80\x00\x0b\x10\x00 \x00 \x01 \x02 \x03\x10\xa4\x80\x80\x80\x00\x0b\x10\x00 \x00 \x01 \x02 \x03\x10\xa5\x80\x80\x80\x00\x0b\x0c\x00 \x01 \x02\x10\x84\x80\x80\x80\x00\x0b\n\x00 \x01\x10\x85\x80\x80\x80\x00\x0b\x1a\x00 \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x80\x80\x80\x80\x00\x0b\x1c\x00 \x01 \x02\xadB \x86B\x04\x84 \x03\xadB \x86B\x04\x84\x10\x81\x80\x80\x80\x00\x0b\x1c\x00 \x01 \x02\xadB \x86B\x04\x84 \x03\xadB \x86B\x04\x84\x10\x82\x80\x80\x80\x00\x0b\x1a\x00 \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x00\x0b\xb5\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00\x02@\x02@ \x02A\tK\r\x00B\x00!\x04\x03@\x02@ \x02\r\x00 \x00A\x006\x02\x00 \x00 \x04B\x08\x86B\x0e\x847\x03\x08\x0c\x03\x0b \x03A\x08j \x01-\x00\x00\x10\xa9\x80\x80\x80\x00\x02@ \x03-\x00\x08A\x03F\r\x00 \x00 \x03)\x03\x087\x02\x04 \x00A\x016\x02\x00\x0c\x03\x0b \x01A\x01j!\x01 \x02A\x7fj!\x02 \x04B\x06\x86 \x031\x00\t\x84!\x04\x0c\x00\x0b\x0b \x00 \x026\x02\x08 \x00A\x00:\x00\x04 \x00A\x016\x02\x00\x0b \x03A\x10j$\x80\x80\x80\x80\x00\x0b\x08\x00 \x00B \x88\xa7\x0b\x82\x01\x01\x01\x7fA\x01!\x02\x02@ \x01A\xff\x01qA\xdf\x00F\r\x00\x02@\x02@ \x01APjA\xff\x01qA\nI\r\x00 \x01A\xbf\x7fjA\xff\x01qA\x1aI\r\x01\x02@ \x01A\x9f\x7fjA\xff\x01qA\x1aI\r\x00 \x00 \x01:\x00\x01 \x00A\x01:\x00\x00\x0f\x0b \x01AEj!\x02\x0c\x02\x0b \x01ARj!\x02\x0c\x01\x0b \x01AKj!\x02\x0b \x00A\x03:\x00\x00 \x00 \x02:\x00\x01\x0b\x0b\x00 \x00\xadB \x86B\x04\x84\x0b\x17\x01\x01\x7f \x00\xa7A\xff\x01q\"\x01A\x0eF \x01A\xca\x00Fr\x0b6\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02A\x01;\x01\x0c \x02 \x016\x02\x08 \x02 \x006\x02\x04 \x02A\x04j\x10\x8d\x80\x80\x80\x00\x00\x0bC\x01\x01\x7f#\x80\x80\x80\x80\x00A k\"\x01$\x80\x80\x80\x80\x00 \x01A\x006\x02\x18 \x01A\x016\x02\x0c \x01A\xf0\x80\xc0\x80\x006\x02\x08 \x01B\x047\x02\x10 \x01A\x08j \x00\x10\xac\x80\x80\x80\x00\x00\x0b\x0b\x81\x01\x01\x00A\x80\x80\xc0\x00\x0bxAB\x00\x00\x00\x00\x10\x00\x01\x00\x00\x00\x01\x00\x10\x00\x01\x00\x00\x00\x00\x00\x10\x00\x01\x00\x00\x00\x01\x00\x10\x00\x01\x00\x00\x00soroban-sdk/src/vec.rs\x00\x00$\x00\x10\x00\x16\x00\x00\x002\x04\x00\x00\t\x00\x00\x00attempt to subtract with overflow\x00\x00\x00L\x00\x10\x00!\x00\x00\x00\x00\xa7%\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05enum_\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01v\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1c::test_spec_lib_no_lto::Enum\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1c::test_spec_lib_no_lto::Enum\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05outer\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01v\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1d::test_spec_lib_no_lto::Outer\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05tuple\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01v\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1d::test_spec_lib_no_lto::Tuple\x00\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1d::test_spec_lib_no_lto::Tuple\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1b::test_spec_no_lto::Wrapper\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1f::test_spec_lib_no_lto::Wrapped\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07wrapper\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01v\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1b::test_spec_no_lto::Wrapper\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08int_enum\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01v\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1f::test_spec_lib_no_lto::IntEnum\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1f::test_spec_lib_no_lto::IntEnum\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1c::test_spec_lib_no_lto::Enum\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01A\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01B\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1d::test_spec_lib_no_lto::Inner\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1d::test_spec_lib_no_lto::Outer\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x1d::test_spec_lib_no_lto::Inner\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1d::test_spec_lib_no_lto::Tuple\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x011\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f::test_spec_lib_no_lto::IntEnum\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01A\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01B\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f::test_spec_lib_no_lto::Wrapped\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00xExecutable specified by the contract instance as a specific Wasm contract code entry identified by its Wasm sha256 hash.\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00_Executable reference via a persistent storage entry owned by this contract or another contract.\x00\x00\x00\x00\x0bExternalRef\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::ContractExecutableRef\x00\x00\x00\x01\x00\x00\x00\xc0Executable referenced via a persistent storage entry owned by a contract,\neither this contract or another contract.\n\nThe persistent storage entry owned by the `owner` has the `tag` as its key.\x00\x00\x00\x00\x00\x00\x00$::soroban_sdk::ContractExecutableRef\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05owner\x00\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x03tag\x00\x00\x00\x00\x10\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x1c::soroban_sdk::auth::Context\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00*::soroban_sdk::auth::SubContractInvocation\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00-::soroban_sdk::auth::InvokerContractAuthEntry\x00\x00\x00\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00-::soroban_sdk::auth::InvokerContractAuthEntry\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*::soroban_sdk::auth::SubContractInvocation\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\"::soroban_sdk::address::Executable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1d\x00\x00\x00\x00\x00G\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08rssdkver\x00\x00\x00\x0630.0.0\x00\x00";
    extern crate test;
    #[rustc_test_marker = "test::test_spec_entries_of_types_in_other_crates_are_linked_without_lto"]
    #[doc(hidden)]
    pub const test_spec_entries_of_types_in_other_crates_are_linked_without_lto:
        test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "test::test_spec_entries_of_types_in_other_crates_are_linked_without_lto",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_no_lto/src/test.rs",
            start_line: 11usize,
            start_col: 4usize,
            end_line: 11usize,
            end_col: 69usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || {
                test::assert_test_result(
                    test_spec_entries_of_types_in_other_crates_are_linked_without_lto(),
                )
            },
        ),
    };
    fn test_spec_entries_of_types_in_other_crates_are_linked_without_lto() {
        let entries = soroban_spec::read::from_wasm(WASM).unwrap();
        let defined: HashSet<String> = entries
            .iter()
            .filter_map(|e| match e {
                ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
                ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
                ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
                _ => None,
            })
            .collect();
        if !defined.contains("::test_spec_lib_no_lto::IntEnum") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_lib_no_lto::IntEnum\")",
            )
        }
        if !defined.contains("::test_spec_lib_no_lto::Enum") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_lib_no_lto::Enum\")",
            )
        }
        if !defined.contains("::test_spec_lib_no_lto::Tuple") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_lib_no_lto::Tuple\")",
            )
        }
        if !defined.contains("::test_spec_lib_no_lto::Outer") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_lib_no_lto::Outer\")",
            )
        }
        if !defined.contains("::test_spec_lib_no_lto::Inner") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_lib_no_lto::Inner\")",
            )
        }
        if !defined.contains("::test_spec_lib_no_lto::Wrapped") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_lib_no_lto::Wrapped\")",
            )
        }
        if !defined.contains("::test_spec_no_lto::Wrapper") {
            ::core::panicking::panic(
                "assertion failed: defined.contains(\"::test_spec_no_lto::Wrapper\")",
            )
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_spec_entries_of_types_in_other_crates_are_linked_without_lto])
}
