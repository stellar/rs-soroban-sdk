#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bls12_381::{Bls12381Fp, Bls12381Fp2, Bls12381G1Affine, Bls12381G2Affine, Fr},
    log, Env,
};
pub struct DummyProof {
    pub fp: Bls12381Fp,
    pub fp2: Bls12381Fp2,
    pub g1: Bls12381G1Affine,
    pub g2: Bls12381G2Affine,
    pub fr: Fr,
}
pub static __SPEC_XDR_TYPE_DUMMYPROOF: [u8; 128usize] = DummyProof::spec_xdr();
impl DummyProof {
    pub const fn spec_xdr() -> [u8; 128usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\nDummyProof\0\0\0\0\0\x05\0\0\0\0\0\0\0\x02fp\0\0\0\0\x03\xee\0\0\00\0\0\0\0\0\0\0\x03fp2\0\0\0\x03\xee\0\0\0`\0\0\0\0\0\0\0\x02fr\0\0\0\0\0\x0c\0\0\0\0\0\0\0\x02g1\0\0\0\0\x03\xee\0\0\0`\0\0\0\0\0\0\0\x02g2\0\0\0\0\x03\xee\0\0\0\xc0"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for DummyProof {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 5usize] = ["fp", "fp2", "fr", "g1", "g2"];
        let mut vals: [Val; 5usize] = [Val::VOID.to_val(); 5usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            fp: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            fp2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            fr: vals[2]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            g1: vals[3]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            g2: vals[4]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, DummyProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &DummyProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 5usize] = ["fp", "fp2", "fr", "g1", "g2"];
        let vals: [Val; 5usize] = [
            (&val.fp).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.fp2).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.fr).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.g1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.g2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &DummyProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&DummyProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, DummyProof>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for DummyProof {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScMap,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let map = val;
        if map.len() != 5usize {
            return Err(soroban_sdk::xdr::Error::Invalid);
        }
        map.validate()?;
        Ok(Self {
            fp: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "fp".try_into()
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
            fp2: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "fp2"
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
            fr: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "fr".try_into()
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for DummyProof {
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
impl TryFrom<&DummyProof> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &DummyProof) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "fp".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.fp)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "fp2"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.fp2)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "fr".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.fr)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
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
impl TryFrom<DummyProof> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: DummyProof) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&DummyProof> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &DummyProof) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<DummyProof> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: DummyProof) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryDummyProof {
        fp: <Bls12381Fp as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        fp2: <Bls12381Fp2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        g1: <Bls12381G1Affine as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        g2: <Bls12381G2Affine as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        fr: <Fr as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryDummyProof {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "ArbitraryDummyProof",
                "fp",
                &self.fp,
                "fp2",
                &self.fp2,
                "g1",
                &self.g1,
                "g2",
                &self.g2,
                "fr",
                &&self.fr,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryDummyProof {
        #[inline]
        fn clone(&self) -> ArbitraryDummyProof {
            ArbitraryDummyProof {
                fp: ::core::clone::Clone::clone(&self.fp),
                fp2: ::core::clone::Clone::clone(&self.fp2),
                g1: ::core::clone::Clone::clone(&self.g1),
                g2: ::core::clone::Clone::clone(&self.g2),
                fr: ::core::clone::Clone::clone(&self.fr),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryDummyProof {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Bls12381Fp as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Bls12381Fp2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Bls12381G1Affine as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Bls12381G2Affine as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Fr as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryDummyProof {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryDummyProof {
        #[inline]
        fn eq(&self, other: &ArbitraryDummyProof) -> bool {
            self.fp == other.fp
                && self.fp2 == other.fp2
                && self.g1 == other.g1
                && self.g2 == other.g2
                && self.fr == other.fr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryDummyProof {
        #[inline]
        fn cmp(&self, other: &ArbitraryDummyProof) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.fp, &other.fp) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(&self.fp2, &other.fp2) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.g1, &other.g1) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(&self.g2, &other.g2) {
                                        ::core::cmp::Ordering::Equal => {
                                            ::core::cmp::Ord::cmp(&self.fr, &other.fr)
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryDummyProof {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryDummyProof,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.fp, &other.fp) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.fp2, &other.fp2) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.g1, &other.g1) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(&self.g2, &other.g2)
                                    {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => ::core::cmp::PartialOrd::partial_cmp(
                                            &self.fr, &other.fr,
                                        ),
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryDummyProof: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryDummyProof {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryDummyProof.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryDummyProof {
                        fp: arbitrary::Arbitrary::arbitrary(u)?,
                        fp2: arbitrary::Arbitrary::arbitrary(u)?,
                        g1: arbitrary::Arbitrary::arbitrary(u)?,
                        g2: arbitrary::Arbitrary::arbitrary(u)?,
                        fr: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryDummyProof.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryDummyProof.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryDummyProof {
                        fp: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        fp2: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        g1: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        g2: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        fr: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryDummyProof.with(|count| {
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
                            <<Bls12381Fp as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Bls12381Fp2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Bls12381G1Affine as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Bls12381G2Affine as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Fr as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for DummyProof {
        type Prototype = ArbitraryDummyProof;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryDummyProof> for DummyProof {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryDummyProof,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(DummyProof {
                fp: soroban_sdk::IntoVal::into_val(&v.fp, env),
                fp2: soroban_sdk::IntoVal::into_val(&v.fp2, env),
                g1: soroban_sdk::IntoVal::into_val(&v.g1, env),
                g2: soroban_sdk::IntoVal::into_val(&v.g2, env),
                fr: soroban_sdk::IntoVal::into_val(&v.fr, env),
            })
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for DummyProof {
    #[inline]
    fn clone(&self) -> DummyProof {
        DummyProof {
            fp: ::core::clone::Clone::clone(&self.fp),
            fp2: ::core::clone::Clone::clone(&self.fp2),
            g1: ::core::clone::Clone::clone(&self.g1),
            g2: ::core::clone::Clone::clone(&self.g2),
            fr: ::core::clone::Clone::clone(&self.fr),
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
    pub fn g1_mul(env: Env, p: Bls12381G1Affine, s: Fr) -> Bls12381G1Affine {
        env.crypto().bls12_381().g1_mul(&p, &s)
    }
    pub fn g2_mul(env: Env, p: Bls12381G2Affine, s: Fr) -> Bls12381G2Affine {
        env.crypto().bls12_381().g2_mul(&p, &s)
    }
    pub fn dummy_verify(env: Env, proof: DummyProof) -> bool {
        let g1 = env.crypto().bls12_381().map_fp_to_g1(&proof.fp);
        let in1 = env.crypto().bls12_381().g1_is_in_subgroup(&g1);
        if true {
            (&env).logs().add(
                "`map_fp_to_g1` result is in subgroup: ",
                &[<_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(&in1, &env)],
            );
        }
        let g2 = env.crypto().bls12_381().map_fp2_to_g2(&proof.fp2);
        let in2 = env.crypto().bls12_381().g2_is_in_subgroup(&g2);
        if true {
            (&env).logs().add(
                "`map_fp2_to_g2` result is in subgroup: ",
                &[<_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(&in2, &env)],
            );
        }
        let g1_mul = env.crypto().bls12_381().g1_mul(&proof.g1, &proof.fr);
        let g2_mul = env.crypto().bls12_381().g2_mul(&proof.g2, &proof.fr);
        let vp1 = soroban_sdk::Vec::from_array(&env, [g1_mul]);
        let vp2 = soroban_sdk::Vec::from_array(&env, [g2_mul]);
        env.crypto().bls12_381().pairing_check(vp1, vp2)
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
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g1_mul\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01p\0\0\0\0\0\x03\xee\0\0\0`\0\0\0\0\0\0\0\x01s\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\x03\xee\0\0\0`"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g2_mul__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_G2_MUL: [u8; 72usize] = super::Contract::spec_xdr_g2_mul();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g2_mul() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g2_mul\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01p\0\0\0\0\0\x03\xee\0\0\0\xc0\0\0\0\0\0\0\0\x01s\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\x03\xee\0\0\0\xc0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__dummy_verify__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_DUMMY_VERIFY: [u8; 72usize] = super::Contract::spec_xdr_dummy_verify();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_dummy_verify() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cdummy_verify\0\0\0\x01\0\0\0\0\0\0\0\x05proof\0\0\0\0\0\x07\xd0\0\0\0\nDummyProof\0\0\0\0\0\x01\0\0\0\x01"
    }
}
impl<'a> ContractClient<'a> {
    pub fn g1_mul(&self, p: &Bls12381G1Affine, s: &Fr) -> Bls12381G1Affine {
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
        p: &Bls12381G1Affine,
        s: &Fr,
    ) -> Result<
        Result<
            Bls12381G1Affine,
            <Bls12381G1Affine as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
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
    pub fn g2_mul(&self, p: &Bls12381G2Affine, s: &Fr) -> Bls12381G2Affine {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g2_mul");
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
    pub fn try_g2_mul(
        &self,
        p: &Bls12381G2Affine,
        s: &Fr,
    ) -> Result<
        Result<
            Bls12381G2Affine,
            <Bls12381G2Affine as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g2_mul");
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
    pub fn dummy_verify(&self, proof: &DummyProof) -> bool {
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
            &{ soroban_sdk::Symbol::new(&self.env, "dummy_verify") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_dummy_verify(
        &self,
        proof: &DummyProof,
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
            &{ soroban_sdk::Symbol::new(&self.env, "dummy_verify") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
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
    pub fn g1_mul<'i>(p: &'i Bls12381G1Affine, s: &'i Fr) -> (&'i Bls12381G1Affine, &'i Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g2_mul<'i>(p: &'i Bls12381G2Affine, s: &'i Fr) -> (&'i Bls12381G2Affine, &'i Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn dummy_verify<'i>(proof: &'i DummyProof) -> (&'i DummyProof,) {
        (proof,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
pub fn __Contract__g1_mul__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::g1_mul(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
pub fn __Contract__g1_mul__invoke_raw_slice(
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
    __Contract__g1_mul__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
pub extern "C" fn __Contract__g1_mul__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__g1_mul__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
pub fn __Contract__g2_mul__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::g2_mul(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
pub fn __Contract__g2_mul__invoke_raw_slice(
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
    __Contract__g2_mul__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
pub extern "C" fn __Contract__g2_mul__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__g2_mul__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
pub fn __Contract__dummy_verify__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::dummy_verify(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
pub fn __Contract__dummy_verify__invoke_raw_slice(
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
    __Contract__dummy_verify__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
pub extern "C" fn __Contract__dummy_verify__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__dummy_verify__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract_None_492d85ed5a5d2cb14995f41e785b3df9c7fcf4af92b6d50ce2c9fa7c9b1e350d_ctor() {
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
                    __Contract_None_492d85ed5a5d2cb14995f41e785b3df9c7fcf4af92b6d50ce2c9fa7c9b1e350d_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "g1_mul",
            #[allow(deprecated)]
            &__Contract__g1_mul__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "g2_mul",
            #[allow(deprecated)]
            &__Contract__g2_mul__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "dummy_verify",
            #[allow(deprecated)]
            &__Contract__dummy_verify__invoke_raw_slice,
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{Contract, ContractClient};
    use soroban_sdk::{bytesn, Env};
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_g1_mul"]
    #[doc(hidden)]
    pub const test_g1_mul: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_g1_mul"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/bls/src/lib.rs",
            start_line: 56usize,
            start_col: 8usize,
            end_line: 56usize,
            end_col: 19usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_g1_mul()),
        ),
    };
    fn test_g1_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let g1 = Bls12381G1Affine::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                23u8, 241u8, 211u8, 167u8, 49u8, 151u8, 215u8, 148u8, 38u8, 149u8, 99u8, 140u8,
                79u8, 169u8, 172u8, 15u8, 195u8, 104u8, 140u8, 79u8, 151u8, 116u8, 185u8, 5u8,
                161u8, 78u8, 58u8, 63u8, 23u8, 27u8, 172u8, 88u8, 108u8, 85u8, 232u8, 63u8, 249u8,
                122u8, 26u8, 239u8, 251u8, 58u8, 240u8, 10u8, 219u8, 34u8, 198u8, 187u8, 8u8,
                179u8, 244u8, 129u8, 227u8, 170u8, 160u8, 241u8, 160u8, 158u8, 48u8, 237u8, 116u8,
                29u8, 138u8, 228u8, 252u8, 245u8, 224u8, 149u8, 213u8, 208u8, 10u8, 246u8, 0u8,
                219u8, 24u8, 203u8, 44u8, 4u8, 179u8, 237u8, 208u8, 60u8, 199u8, 68u8, 162u8,
                136u8, 138u8, 228u8, 12u8, 170u8, 35u8, 41u8, 70u8, 197u8, 231u8, 225u8,
            ],
        ));
        let zero = Fr::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],
        ));
        let inf = Bls12381G1Affine::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],
        ));
        let res = client.g1_mul(&g1, &zero);
        match (&res, &inf) {
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
    #[rustc_test_marker = "test::test_g2_mul"]
    #[doc(hidden)]
    pub const test_g2_mul: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_g2_mul"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/bls/src/lib.rs",
            start_line: 73usize,
            start_col: 8usize,
            end_line: 73usize,
            end_col: 19usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_g2_mul()),
        ),
    };
    fn test_g2_mul() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let g2 = Bls12381G2Affine::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                19u8, 224u8, 43u8, 96u8, 82u8, 113u8, 159u8, 96u8, 125u8, 172u8, 211u8, 160u8,
                136u8, 39u8, 79u8, 101u8, 89u8, 107u8, 208u8, 208u8, 153u8, 32u8, 182u8, 26u8,
                181u8, 218u8, 97u8, 187u8, 220u8, 127u8, 80u8, 73u8, 51u8, 76u8, 241u8, 18u8, 19u8,
                148u8, 93u8, 87u8, 229u8, 172u8, 125u8, 5u8, 93u8, 4u8, 43u8, 126u8, 2u8, 74u8,
                162u8, 178u8, 240u8, 143u8, 10u8, 145u8, 38u8, 8u8, 5u8, 39u8, 45u8, 197u8, 16u8,
                81u8, 198u8, 228u8, 122u8, 212u8, 250u8, 64u8, 59u8, 2u8, 180u8, 81u8, 11u8, 100u8,
                122u8, 227u8, 209u8, 119u8, 11u8, 172u8, 3u8, 38u8, 168u8, 5u8, 187u8, 239u8,
                212u8, 128u8, 86u8, 200u8, 193u8, 33u8, 189u8, 184u8, 6u8, 6u8, 196u8, 160u8, 46u8,
                167u8, 52u8, 204u8, 50u8, 172u8, 210u8, 176u8, 43u8, 194u8, 139u8, 153u8, 203u8,
                62u8, 40u8, 126u8, 133u8, 167u8, 99u8, 175u8, 38u8, 116u8, 146u8, 171u8, 87u8,
                46u8, 153u8, 171u8, 63u8, 55u8, 13u8, 39u8, 92u8, 236u8, 29u8, 161u8, 170u8, 169u8,
                7u8, 95u8, 240u8, 95u8, 121u8, 190u8, 12u8, 229u8, 213u8, 39u8, 114u8, 125u8,
                110u8, 17u8, 140u8, 201u8, 205u8, 198u8, 218u8, 46u8, 53u8, 26u8, 173u8, 253u8,
                155u8, 170u8, 140u8, 189u8, 211u8, 167u8, 109u8, 66u8, 154u8, 105u8, 81u8, 96u8,
                209u8, 44u8, 146u8, 58u8, 201u8, 204u8, 59u8, 172u8, 162u8, 137u8, 225u8, 147u8,
                84u8, 134u8, 8u8, 184u8, 40u8, 1u8,
            ],
        ));
        let zero = Fr::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],
        ));
        let inf = Bls12381G2Affine::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],
        ));
        let res = client.g2_mul(&g2, &zero);
        match (&res, &inf) {
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
    #[rustc_test_marker = "test::test_dummy_verify"]
    #[doc(hidden)]
    pub const test_dummy_verify: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_dummy_verify"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/bls/src/lib.rs",
            start_line: 90usize,
            start_col: 8usize,
            end_line: 90usize,
            end_col: 25usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_dummy_verify()),
        ),
    };
    fn test_dummy_verify() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let fp = Bls12381Fp::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
            ],
        ));
        let fp2 = Bls12381Fp2::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
            ],
        ));
        let g1 = Bls12381G1Affine::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                23u8, 241u8, 211u8, 167u8, 49u8, 151u8, 215u8, 148u8, 38u8, 149u8, 99u8, 140u8,
                79u8, 169u8, 172u8, 15u8, 195u8, 104u8, 140u8, 79u8, 151u8, 116u8, 185u8, 5u8,
                161u8, 78u8, 58u8, 63u8, 23u8, 27u8, 172u8, 88u8, 108u8, 85u8, 232u8, 63u8, 249u8,
                122u8, 26u8, 239u8, 251u8, 58u8, 240u8, 10u8, 219u8, 34u8, 198u8, 187u8, 8u8,
                179u8, 244u8, 129u8, 227u8, 170u8, 160u8, 241u8, 160u8, 158u8, 48u8, 237u8, 116u8,
                29u8, 138u8, 228u8, 252u8, 245u8, 224u8, 149u8, 213u8, 208u8, 10u8, 246u8, 0u8,
                219u8, 24u8, 203u8, 44u8, 4u8, 179u8, 237u8, 208u8, 60u8, 199u8, 68u8, 162u8,
                136u8, 138u8, 228u8, 12u8, 170u8, 35u8, 41u8, 70u8, 197u8, 231u8, 225u8,
            ],
        ));
        let g2 = Bls12381G2Affine::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                19u8, 224u8, 43u8, 96u8, 82u8, 113u8, 159u8, 96u8, 125u8, 172u8, 211u8, 160u8,
                136u8, 39u8, 79u8, 101u8, 89u8, 107u8, 208u8, 208u8, 153u8, 32u8, 182u8, 26u8,
                181u8, 218u8, 97u8, 187u8, 220u8, 127u8, 80u8, 73u8, 51u8, 76u8, 241u8, 18u8, 19u8,
                148u8, 93u8, 87u8, 229u8, 172u8, 125u8, 5u8, 93u8, 4u8, 43u8, 126u8, 2u8, 74u8,
                162u8, 178u8, 240u8, 143u8, 10u8, 145u8, 38u8, 8u8, 5u8, 39u8, 45u8, 197u8, 16u8,
                81u8, 198u8, 228u8, 122u8, 212u8, 250u8, 64u8, 59u8, 2u8, 180u8, 81u8, 11u8, 100u8,
                122u8, 227u8, 209u8, 119u8, 11u8, 172u8, 3u8, 38u8, 168u8, 5u8, 187u8, 239u8,
                212u8, 128u8, 86u8, 200u8, 193u8, 33u8, 189u8, 184u8, 6u8, 6u8, 196u8, 160u8, 46u8,
                167u8, 52u8, 204u8, 50u8, 172u8, 210u8, 176u8, 43u8, 194u8, 139u8, 153u8, 203u8,
                62u8, 40u8, 126u8, 133u8, 167u8, 99u8, 175u8, 38u8, 116u8, 146u8, 171u8, 87u8,
                46u8, 153u8, 171u8, 63u8, 55u8, 13u8, 39u8, 92u8, 236u8, 29u8, 161u8, 170u8, 169u8,
                7u8, 95u8, 240u8, 95u8, 121u8, 190u8, 12u8, 229u8, 213u8, 39u8, 114u8, 125u8,
                110u8, 17u8, 140u8, 201u8, 205u8, 198u8, 218u8, 46u8, 53u8, 26u8, 173u8, 253u8,
                155u8, 170u8, 140u8, 189u8, 211u8, 167u8, 109u8, 66u8, 154u8, 105u8, 81u8, 96u8,
                209u8, 44u8, 146u8, 58u8, 201u8, 204u8, 59u8, 172u8, 162u8, 137u8, 225u8, 147u8,
                84u8, 134u8, 8u8, 184u8, 40u8, 1u8,
            ],
        ));
        let fr = Fr::from_bytes(::soroban_sdk::BytesN::from_array(
            &env,
            &[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
            ],
        ));
        let proof = DummyProof {
            fp,
            fp2,
            g1,
            g2,
            fr,
        };
        let res = client.dummy_verify(&proof);
        if !!res {
            ::core::panicking::panic("assertion failed: !res")
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_dummy_verify, &test_g1_mul, &test_g2_mul])
}
