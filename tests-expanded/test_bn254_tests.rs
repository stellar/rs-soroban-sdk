#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bn254::{Bn254G1Affine, Bn254G2Affine, Fr},
    Env, Vec,
};
pub struct MockProof {
    pub g1: Vec<Bn254G1Affine>,
    pub g2: Vec<Bn254G2Affine>,
}
pub static __SPEC_XDR_TYPE_MOCKPROOF: [u8; 80usize] = MockProof::spec_xdr();
impl MockProof {
    pub const fn spec_xdr() -> [u8; 80usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\tMockProof\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02g1\0\0\0\0\x03\xea\0\0\x03\xee\0\0\0@\0\0\0\0\0\0\0\x02g2\0\0\0\0\x03\xea\0\0\x03\xee\0\0\0\x80"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for MockProof {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["g1", "g2"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            g1: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            g2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, MockProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &MockProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["g1", "g2"];
        let vals: [Val; 2usize] = [
            (&val.g1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.g2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &MockProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&MockProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, MockProof>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for MockProof {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScMap,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let map = val;
        if map.len() != 2usize {
            return Err(soroban_sdk::xdr::Error::Invalid);
        }
        map.validate()?;
        Ok(Self {
            g1: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "g1".try_into()
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
            g2: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "g2".try_into()
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for MockProof {
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
impl TryFrom<&MockProof> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &MockProof) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "g1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.g1)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "g2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.g2)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
            ]),
        ))
    }
}
impl TryFrom<MockProof> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: MockProof) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&MockProof> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &MockProof) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<MockProof> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: MockProof) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryMockProof {
        g1: <Vec<Bn254G1Affine> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        g2: <Vec<Bn254G2Affine> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryMockProof {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ArbitraryMockProof",
                "g1",
                &self.g1,
                "g2",
                &&self.g2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryMockProof {
        #[inline]
        fn clone(&self) -> ArbitraryMockProof {
            ArbitraryMockProof {
                g1: ::core::clone::Clone::clone(&self.g1),
                g2: ::core::clone::Clone::clone(&self.g2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryMockProof {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Vec<
                    Bn254G1Affine,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Vec<
                    Bn254G2Affine,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryMockProof {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryMockProof {
        #[inline]
        fn eq(&self, other: &ArbitraryMockProof) -> bool {
            self.g1 == other.g1 && self.g2 == other.g2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryMockProof {
        #[inline]
        fn cmp(&self, other: &ArbitraryMockProof) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.g1, &other.g1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.g2, &other.g2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryMockProof {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryMockProof,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.g1, &other.g1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.g2, &other.g2)
                }
                cmp => cmp,
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryMockProof: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryMockProof {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMockProof.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryMockProof {
                        g1: arbitrary::Arbitrary::arbitrary(u)?,
                        g2: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMockProof.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryMockProof.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryMockProof {
                        g1: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        g2: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMockProof.with(|count| {
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
                            <<Vec<
                                Bn254G1Affine,
                            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Vec<
                                Bn254G2Affine,
                            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for MockProof {
        type Prototype = ArbitraryMockProof;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryMockProof> for MockProof {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryMockProof,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(MockProof {
                g1: soroban_sdk::IntoVal::into_val(&v.g1, env),
                g2: soroban_sdk::IntoVal::into_val(&v.g2, env),
            })
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for MockProof {
    #[inline]
    fn clone(&self) -> MockProof {
        MockProof {
            g1: ::core::clone::Clone::clone(&self.g1),
            g2: ::core::clone::Clone::clone(&self.g2),
        }
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
impl Contract {
    pub fn verify_pairing(env: Env, proof: MockProof) -> bool {
        env.crypto().bn254().pairing_check(proof.g1, proof.g2)
    }
    pub fn g1_add(a: Bn254G1Affine, b: Bn254G1Affine) -> Bn254G1Affine {
        a + b
    }
    pub fn g1_mul(p: Bn254G1Affine, s: Fr) -> Bn254G1Affine {
        p * s
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__verify_pairing__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_VERIFY_PAIRING: [u8; 76usize] =
        super::Contract::spec_xdr_verify_pairing();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_verify_pairing() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0everify_pairing\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05proof\0\0\0\0\0\x07\xd0\0\0\0\tMockProof\0\0\0\0\0\0\x01\0\0\0\x01"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_add__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_G1_ADD: [u8; 76usize] = super::Contract::spec_xdr_g1_add();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g1_add() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g1_add\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\x03\xee\0\0\0@\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x03\xee\0\0\0@\0\0\0\x01\0\0\x03\xee\0\0\0@"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_mul__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_G1_MUL: [u8; 72usize] = super::Contract::spec_xdr_g1_mul();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g1_mul() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g1_mul\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01p\0\0\0\0\0\x03\xee\0\0\0@\0\0\0\0\0\0\0\x01s\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\x03\xee\0\0\0@"
    }
}
impl<'a> ContractClient<'a> {
    pub fn verify_pairing(&self, proof: &MockProof) -> bool {
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
            &{ soroban_sdk::Symbol::new(&self.env, "verify_pairing") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_verify_pairing(
        &self,
        proof: &MockProof,
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
            &{ soroban_sdk::Symbol::new(&self.env, "verify_pairing") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn g1_add(&self, a: &Bn254G1Affine, b: &Bn254G1Affine) -> Bn254G1Affine {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_add");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_g1_add(
        &self,
        a: &Bn254G1Affine,
        b: &Bn254G1Affine,
    ) -> Result<
        Result<
            Bn254G1Affine,
            <Bn254G1Affine as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_add");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn g1_mul(&self, p: &Bn254G1Affine, s: &Fr) -> Bn254G1Affine {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_mul");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [p.into_val(&self.env), s.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_g1_mul(
        &self,
        p: &Bn254G1Affine,
        s: &Fr,
    ) -> Result<
        Result<
            Bn254G1Affine,
            <Bn254G1Affine as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_mul");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [p.into_val(&self.env), s.into_val(&self.env)],
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
    pub fn verify_pairing<'i>(proof: &'i MockProof) -> (&'i MockProof,) {
        (proof,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g1_add<'i>(
        a: &'i Bn254G1Affine,
        b: &'i Bn254G1Affine,
    ) -> (&'i Bn254G1Affine, &'i Bn254G1Affine) {
        (a, b)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g1_mul<'i>(p: &'i Bn254G1Affine, s: &'i Fr) -> (&'i Bn254G1Affine, &'i Fr) {
        (p, s)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__verify_pairing {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_pairing` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::verify_pairing(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_pairing` instead")]
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_pairing` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_add {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_add` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::g1_add(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_add` instead")]
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_add` instead")]
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
pub mod __Contract__g1_mul {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::g1_mul(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
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
#[allow(unused)]
fn __Contract__0bbda83869b96862f040597ce6f7f1153fcf92be3a6565cd371a4485bb1a9c8d_ctor() {
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
                    __Contract__0bbda83869b96862f040597ce6f7f1153fcf92be3a6565cd371a4485bb1a9c8d_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "verify_pairing",
            #[allow(deprecated)]
            &__Contract__verify_pairing::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "g1_add",
            #[allow(deprecated)]
            &__Contract__g1_add::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "g1_mul",
            #[allow(deprecated)]
            &__Contract__g1_mul::invoke_raw_slice,
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, Env, U256};
    extern crate std;
    use crate::{Contract, ContractClient};
    fn parse_ethereum_g1_add_input(input: &str) -> ([u8; 64], [u8; 64]) {
        let bytes = hex::decode(input).unwrap();
        match (&bytes.len(), &128) {
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
        let g1_1: [u8; 64] = bytes[0..64].try_into().unwrap();
        let g1_2: [u8; 64] = bytes[64..128].try_into().unwrap();
        (g1_1, g1_2)
    }
    fn parse_ethereum_pairing_input(
        input: &str,
    ) -> (std::vec::Vec<[u8; 64]>, std::vec::Vec<[u8; 128]>) {
        let bytes = hex::decode(input).unwrap();
        match (&(bytes.len() % 192), &0) {
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
        let num_pairs = bytes.len() / 192;
        let mut g1_points = std::vec::Vec::new();
        let mut g2_points = std::vec::Vec::new();
        for i in 0..num_pairs {
            let offset = i * 192;
            g1_points.push(bytes[offset..offset + 64].try_into().unwrap());
            g2_points.push(bytes[offset + 64..offset + 192].try_into().unwrap());
        }
        (g1_points, g2_points)
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_add_and_mul"]
    #[doc(hidden)]
    pub const test_add_and_mul: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_add_and_mul"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/bn254/src/lib.rs",
            start_line: 70usize,
            start_col: 8usize,
            end_line: 70usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_add_and_mul()),
        ),
    };
    fn test_add_and_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let add_input = "23f16f1bcc31bd002746da6fa3825209af9a356ccd99cf79604a430dd592bcd90a03caeda9c5aa40cdc9e4166e083492885dad36c72714e3697e34a4bc72ccaa21315394462f1a39f87462dbceb92718b220e4f80af516f727ad85380fadefbc2e4f40ea7bbe2d4d71f13c84fd2ae24a4a24d9638dd78349d0dee8435a67cca6";
        let (g1_x_bytes, g1_y_bytes) = parse_ethereum_g1_add_input(add_input);
        let x_bn254 = Bn254G1Affine::from_array(&env, &g1_x_bytes);
        let y_bn254 = Bn254G1Affine::from_array(&env, &g1_y_bytes);
        let expected_x_plus_y = hex::decode(
                "013f227997b410cbd96b137a114f5b12d5a3a53d7482797bcd1f116ff30ff1931effebc79dee208d036553beae8ca71afb3b4c00979560db3991c7e67c49103c",
            )
            .unwrap();
        match (
            &client.g1_add(&x_bn254, &y_bn254).to_array(),
            &expected_x_plus_y.as_slice(),
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
        let scalar: Fr = U256::from_u32(&env, 2).into();
        match (
            &client.g1_add(&x_bn254, &x_bn254),
            &client.g1_mul(&x_bn254, &scalar),
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
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_pairing"]
    #[doc(hidden)]
    pub const test_pairing: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_pairing"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/bn254/src/lib.rs",
            start_line: 98usize,
            start_col: 8usize,
            end_line: 98usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_pairing()),
        ),
    };
    fn test_pairing() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let pairing_input = "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";
        let (g1_points, g2_points) = parse_ethereum_pairing_input(pairing_input);
        let g1_vec = ::soroban_sdk::Vec::from_array(
            &env,
            [
                Bn254G1Affine::from_array(&env, &g1_points[0]),
                Bn254G1Affine::from_array(&env, &g1_points[1]),
            ],
        );
        let g2_vec = ::soroban_sdk::Vec::from_array(
            &env,
            [
                Bn254G2Affine::from_array(&env, &g2_points[0]),
                Bn254G2Affine::from_array(&env, &g2_points[1]),
            ],
        );
        let proof = MockProof {
            g1: g1_vec,
            g2: g2_vec,
        };
        if !client.verify_pairing(&proof) {
            ::core::panicking::panic("assertion failed: client.verify_pairing(&proof)")
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_g1_negation"]
    #[doc(hidden)]
    pub const test_g1_negation: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_g1_negation"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/bn254/src/lib.rs",
            start_line: 131usize,
            start_col: 8usize,
            end_line: 131usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_g1_negation()),
        ),
    };
    fn test_g1_negation() {
        let env = Env::default();
        let negated_input = "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000130644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45";
        let bytes = hex::decode(negated_input).unwrap();
        match (&bytes.len(), &128) {
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
        let g1_bytes: [u8; 64] = bytes[0..64].try_into().unwrap();
        let g1_negaed_bytes: [u8; 64] = bytes[64..128].try_into().unwrap();
        let g1 = Bn254G1Affine::from_array(&env, &g1_bytes);
        let g1_negated = Bn254G1Affine::from_array(&env, &g1_negaed_bytes);
        match (&-g1, &g1_negated) {
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
    test::test_main_static(&[&test_add_and_mul, &test_g1_negation, &test_pairing])
}
