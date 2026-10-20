#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, crypto::Hash, Bytes, Env, MuxedAddress};
pub mod traits {
    use soroban_sdk::{
        contracterror, contracttrait, contracttype, crypto::Hash, Address, Bytes, Env,
        MuxedAddress, Vec,
    };
    pub struct Point {
        pub x: u32,
        pub y: u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Point {
        #[inline]
        fn clone(&self) -> Point {
            Point {
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Point {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "Point", "x", &self.x, "y", &&self.y,
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Point {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Point {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Point {
        #[inline]
        fn eq(&self, other: &Point) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    impl soroban_sdk::SpecName for Point {
        const SPEC_NAME: &'static str = {
            const NAME: &str = "::test_contracttrait_types::traits::Point";
            const CHECKED_NAME: &str = {
                if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "type `Point` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_contracttrait_types::traits::Point`",
                            ),
                        );
                    }
                }
                NAME
            };
            CHECKED_NAME
        };
    }
    impl soroban_sdk::SpecTypeDef for Point {
        const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
            soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        <Self as soroban_sdk::SpecName>::SPEC_NAME,
                    ),
                },
            );
    }
    impl soroban_sdk::client::Type for Point {
        type ArgMethod = soroban_sdk::client::Borrowed;
        type Ret = Self;
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_POINT: [u8; Point::spec_xdr().len()] = Point::spec_xdr();
    impl Point {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        <Point as soroban_sdk::SpecName>::SPEC_NAME,
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"x"),
                            type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"y"),
                            type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                    ]),
                },
            );
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr() -> [u8; Point::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Point::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Point {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["x", "y"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                x: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                y: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, Point> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &Point,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["x", "y"];
            let vals: [Val; 2usize] = [
                (&val.x).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.y).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Point> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&Point,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Point>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for Point {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScMap,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let map = val;
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            map.validate()?;
            Ok(Self {
                x: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "x".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                y: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "y".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Point {
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
    impl TryFrom<&Point> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &Point) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "x".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.x)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "y".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.y)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
            ])))
        }
    }
    impl TryFrom<Point> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: Point) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&Point> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &Point) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
        }
    }
    impl TryFrom<Point> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: Point) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryPoint {
            x: <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            y: <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryPoint {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArbitraryPoint",
                    "x",
                    &self.x,
                    "y",
                    &&self.y,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryPoint {
            #[inline]
            fn clone(&self) -> ArbitraryPoint {
                ArbitraryPoint {
                    x: ::core::clone::Clone::clone(&self.x),
                    y: ::core::clone::Clone::clone(&self.y),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryPoint {
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
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryPoint {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryPoint {
            #[inline]
            fn eq(&self, other: &ArbitraryPoint) -> bool {
                self.x == other.x && self.y == other.y
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryPoint {
            #[inline]
            fn cmp(&self, other: &ArbitraryPoint) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.x, &other.x) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.y, &other.y),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryPoint {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryPoint,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.x, &other.x) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.y, &other.y)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryPoint: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryPoint {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryPoint.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryPoint {
                            x: arbitrary::Arbitrary::arbitrary(u)?,
                            y: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryPoint.with(|count| {
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
                        RECURSIVE_COUNT_ArbitraryPoint.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryPoint {
                            x: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            y: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryPoint.with(|count| {
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
                                <<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Point {
            type Prototype = ArbitraryPoint;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryPoint> for Point {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryPoint,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(Point {
                    x: soroban_sdk::IntoVal::into_val(&v.x, env),
                    y: soroban_sdk::IntoVal::into_val(&v.y, env),
                })
            }
        }
    };
    pub enum Error {
        Odd = 1,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Error {}
    #[automatically_derived]
    impl ::core::clone::Clone for Error {
        #[inline]
        fn clone(&self) -> Error {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Odd")
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
    impl soroban_sdk::SpecName for Error {
        const SPEC_NAME: &'static str = {
            const NAME: &str = "::test_contracttrait_types::traits::Error";
            const CHECKED_NAME: &str = {
                if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "type `Error` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_contracttrait_types::traits::Error`",
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
            soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        <Self as soroban_sdk::SpecName>::SPEC_NAME,
                    ),
                },
            );
    }
    impl soroban_sdk::client::Type for Error {
        type ArgMethod = soroban_sdk::client::Borrowed;
        type Ret = Self;
    }
    #[doc(hidden)]
    #[allow(dead_code)]
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
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"Odd",
                            ),
                            value: 1u32,
                        },
                    ]),
                },
            );
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr() -> [u8; Error::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Error::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for Error {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {}
    }
    impl TryFrom<soroban_sdk::Error> for Error {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    1u32 => Self::Odd,
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
                Error::Odd => soroban_sdk::Error::from_contract_error(1u32),
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
                    1u32 => Self::Odd,
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
                Error::Odd => soroban_sdk::InvokeError::Contract(1u32),
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
    pub struct TypesTraitSpec;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Env {
        type __TypesTrait_Env;
    }
    impl<__T: ?Sized> __TypesTrait_Env for __T {
        type __TypesTrait_Env = Env;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_u32 {
        type __TypesTrait_u32;
    }
    impl<__T: ?Sized> __TypesTrait_u32 for __T {
        type __TypesTrait_u32 = u32;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Point {
        type __TypesTrait_Point;
    }
    impl<__T: ?Sized> __TypesTrait_Point for __T {
        type __TypesTrait_Point = Point;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Vec {
        type __TypesTrait_Vec<T0>;
    }
    impl<__T: ?Sized> __TypesTrait_Vec for __T {
        type __TypesTrait_Vec<T0> = Vec<T0>;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Error {
        type __TypesTrait_Error;
    }
    impl<__T: ?Sized> __TypesTrait_Error for __T {
        type __TypesTrait_Error = Error;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Result {
        type __TypesTrait_Result<T0, T1>;
    }
    impl<__T: ?Sized> __TypesTrait_Result for __T {
        type __TypesTrait_Result<T0, T1> = Result<T0, T1>;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Address {
        type __TypesTrait_Address;
    }
    impl<__T: ?Sized> __TypesTrait_Address for __T {
        type __TypesTrait_Address = Address;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_MuxedAddress {
        type __TypesTrait_MuxedAddress;
    }
    impl<__T: ?Sized> __TypesTrait_MuxedAddress for __T {
        type __TypesTrait_MuxedAddress = MuxedAddress;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Bytes {
        type __TypesTrait_Bytes;
    }
    impl<__T: ?Sized> __TypesTrait_Bytes for __T {
        type __TypesTrait_Bytes = Bytes;
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __TypesTrait_Hash {
        type __TypesTrait_Hash<const N0: usize>;
    }
    impl<__T: ?Sized> __TypesTrait_Hash for __T {
        type __TypesTrait_Hash<const N0: usize> = Hash<N0>;
    }
    /// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
    pub use __contractimpl_for_types_trait as TypesTrait;
    pub trait TypesTrait:
        __TypesTrait_Env
        + __TypesTrait_u32
        + __TypesTrait_Point
        + __TypesTrait_Vec
        + __TypesTrait_Error
        + __TypesTrait_Result
        + __TypesTrait_Address
        + __TypesTrait_MuxedAddress
        + __TypesTrait_Bytes
        + __TypesTrait_Hash
    {
        fn point(_env: &Env, x: u32, y: u32) -> Point {
            Point { x, y }
        }
        fn points(env: &Env, p: Point) -> Vec<Point> {
            Vec::from_array(env, [p.clone(), p])
        }
        fn even(_env: &Env, v: u32) -> Result<u32, Error> {
            if v % 2 == 0 {
                Ok(v)
            } else {
                Err(Error::Odd)
            }
        }
        fn address(_env: &Env, a: Address, _m: MuxedAddress) -> Address {
            a
        }
        fn hash(env: &Env, b: Bytes) -> Hash<32> {
            env.crypto().sha256(&b)
        }
    }
    ///TypesTraitClient is a client for calling the contract defined in "TypesTrait".
    pub struct TypesTraitClient<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        config: soroban_sdk::testutils::ClientInternalConfig<'a>,
    }
    impl<'a> TypesTraitClient<'a> {
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
    impl<'a> TypesTraitClient<'a> {
        /// Call `point` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_point`] to receive errors instead of panicking.
        pub fn point(
            &self,
            x: impl soroban_sdk::IntoArg<u32>,
            y: impl soroban_sdk::IntoArg<u32>,
        ) -> soroban_sdk::client::Ret<Point> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("point");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        soroban_sdk::IntoArg::<u32>::into_arg(x, &self.env),
                        soroban_sdk::IntoArg::<u32>::into_arg(y, &self.env),
                    ],
                ),
            );
            res
        }
        /// Call `point` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_point(
            &self,
            x: impl soroban_sdk::IntoArg<u32>,
            y: impl soroban_sdk::IntoArg<u32>,
        ) -> soroban_sdk::client::TryRet<Point> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("point");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        soroban_sdk::IntoArg::<u32>::into_arg(x, &self.env),
                        soroban_sdk::IntoArg::<u32>::into_arg(y, &self.env),
                    ],
                ),
            );
            res
        }
        /// Call `points` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_points`] to receive errors instead of panicking.
        pub fn points(
            &self,
            p: impl soroban_sdk::IntoArg<Point>,
        ) -> soroban_sdk::client::Ret<Vec<Point>> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("points");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [soroban_sdk::IntoArg::<Point>::into_arg(p, &self.env)],
                ),
            );
            res
        }
        /// Call `points` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_points(
            &self,
            p: impl soroban_sdk::IntoArg<Point>,
        ) -> soroban_sdk::client::TryRet<Vec<Point>> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("points");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [soroban_sdk::IntoArg::<Point>::into_arg(p, &self.env)],
                ),
            );
            res
        }
        /// Call `even` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_even`] to receive errors instead of panicking.
        pub fn even(
            &self,
            v: impl soroban_sdk::IntoArg<u32>,
        ) -> soroban_sdk::client::Ret<Result<u32, Error>> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("even");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [soroban_sdk::IntoArg::<u32>::into_arg(v, &self.env)],
                ),
            );
            res
        }
        /// Call `even` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_even(
            &self,
            v: impl soroban_sdk::IntoArg<u32>,
        ) -> soroban_sdk::client::TryRet<Result<u32, Error>> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("even");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [soroban_sdk::IntoArg::<u32>::into_arg(v, &self.env)],
                ),
            );
            res
        }
        /// Call `address` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_address`] to receive errors instead of panicking.
        pub fn address(
            &self,
            a: impl soroban_sdk::IntoArg<Address>,
            _m: impl soroban_sdk::IntoArg<MuxedAddress>,
        ) -> soroban_sdk::client::Ret<Address> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("address");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        soroban_sdk::IntoArg::<Address>::into_arg(a, &self.env),
                        soroban_sdk::IntoArg::<MuxedAddress>::into_arg(_m, &self.env),
                    ],
                ),
            );
            res
        }
        /// Call `address` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_address(
            &self,
            a: impl soroban_sdk::IntoArg<Address>,
            _m: impl soroban_sdk::IntoArg<MuxedAddress>,
        ) -> soroban_sdk::client::TryRet<Address> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("address");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        soroban_sdk::IntoArg::<Address>::into_arg(a, &self.env),
                        soroban_sdk::IntoArg::<MuxedAddress>::into_arg(_m, &self.env),
                    ],
                ),
            );
            res
        }
        /// Call `hash` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_hash`] to receive errors instead of panicking.
        pub fn hash(
            &self,
            b: impl soroban_sdk::IntoArg<Bytes>,
        ) -> soroban_sdk::client::Ret<Hash<32>> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hash");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [soroban_sdk::IntoArg::<Bytes>::into_arg(b, &self.env)],
                ),
            );
            res
        }
        /// Call `hash` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_hash(
            &self,
            b: impl soroban_sdk::IntoArg<Bytes>,
        ) -> soroban_sdk::client::TryRet<Hash<32>> {
            let _call_scope =
                soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hash");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [soroban_sdk::IntoArg::<Bytes>::into_arg(b, &self.env)],
                ),
            );
            res
        }
    }
    ///TypesTraitArgs is a type for building arg lists for functions defined in "TypesTrait".
    pub struct TypesTraitArgs;
    impl TypesTraitArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn point<'i>(x: &'i u32, y: &'i u32) -> (&'i u32, &'i u32) {
            (x, y)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn points<'i>(p: &'i Point) -> (&'i Point,) {
            (p,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn even<'i>(v: &'i u32) -> (&'i u32,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn address<'i>(
            a: &'i Address,
            _m: &'i MuxedAddress,
        ) -> (&'i Address, &'i MuxedAddress) {
            (a, _m)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn hash<'i>(b: &'i Bytes) -> (&'i Bytes,) {
            (b,)
        }
    }
    impl TypesTraitSpec {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_point: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
                soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::ScSymbol(
                        soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"point"),
                    ),
                    inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"x"),
                            type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"y"),
                            type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                    ]),
                    outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        <Point as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    ]),
                },
            );
        #[allow(non_snake_case)]
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr_point() -> [u8; TypesTraitSpec::__SPEC_XDR_ENTRY_point.const_xdr_len()]
        {
            const { TypesTraitSpec::__SPEC_XDR_ENTRY_point.const_to_xdr() }
        }
    }
    impl TypesTraitSpec {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_points: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
                soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::ScSymbol(
                        soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"points"),
                    ),
                    inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"p"),
                            type_: <Point as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                    ]),
                    outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        <Vec<Point> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    ]),
                },
            );
        #[allow(non_snake_case)]
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr_points(
        ) -> [u8; TypesTraitSpec::__SPEC_XDR_ENTRY_points.const_xdr_len()] {
            const { TypesTraitSpec::__SPEC_XDR_ENTRY_points.const_to_xdr() }
        }
    }
    impl TypesTraitSpec {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_even: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
                soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::ScSymbol(
                        soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"even"),
                    ),
                    inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                            type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                    ]),
                    outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        <Result<u32, Error> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    ]),
                },
            );
        #[allow(non_snake_case)]
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr_even() -> [u8; TypesTraitSpec::__SPEC_XDR_ENTRY_even.const_xdr_len()]
        {
            const { TypesTraitSpec::__SPEC_XDR_ENTRY_even.const_to_xdr() }
        }
    }
    impl TypesTraitSpec {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_address: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
                soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::ScSymbol(
                        soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"address"),
                    ),
                    inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                            type_: <Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"m"),
                            type_: <MuxedAddress as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                    ]),
                    outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        <Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    ]),
                },
            );
        #[allow(non_snake_case)]
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr_address(
        ) -> [u8; TypesTraitSpec::__SPEC_XDR_ENTRY_address.const_xdr_len()] {
            const { TypesTraitSpec::__SPEC_XDR_ENTRY_address.const_to_xdr() }
        }
    }
    impl TypesTraitSpec {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_hash: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
                soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::ScSymbol(
                        soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"hash"),
                    ),
                    inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                            type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                        },
                    ]),
                    outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        <Hash<32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    ]),
                },
            );
        #[allow(non_snake_case)]
        /// Returns the XDR of this item's contract spec entry, as it is
        /// embedded in the contract.
        ///
        /// The entry names user-defined types and events by their fully
        /// qualified path, such as `::mycrate::MyType`, which `stellar contract
        /// build` reduces to simple names in the spec of a built contract. A
        /// spec assembled from this entry outside of a contract build must be
        /// reduced with `soroban_spec::reduce::reduce` before it is published or
        /// given to tools.
        pub const fn spec_xdr_hash() -> [u8; TypesTraitSpec::__SPEC_XDR_ENTRY_hash.const_xdr_len()]
        {
            const { TypesTraitSpec::__SPEC_XDR_ENTRY_hash.const_to_xdr() }
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
impl traits::TypesTrait for Contract {}
impl<'a> ContractClient<'a> {}
impl ContractArgs {}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract_TypesTrait_project {
    type Env;
    type u32;
    type Point;
    type Vec<T0>;
    type Result<T0, T1>;
    type Error;
    type Address;
    type MuxedAddress;
    type Bytes;
    type Hash<const N0: usize>;
}
impl<__T: traits::TypesTrait> __Contract_TypesTrait_project for __T {
    type Env = __T::__TypesTrait_Env;
    type u32 = __T::__TypesTrait_u32;
    type Point = __T::__TypesTrait_Point;
    type Vec<T0> = __T::__TypesTrait_Vec<T0>;
    type Result<T0, T1> = __T::__TypesTrait_Result<T0, T1>;
    type Error = __T::__TypesTrait_Error;
    type Address = __T::__TypesTrait_Address;
    type MuxedAddress = __T::__TypesTrait_MuxedAddress;
    type Bytes = __T::__TypesTrait_Bytes;
    type Hash<const N0: usize> = __T::__TypesTrait_Hash<N0>;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract_TypesTrait_types {
    #[allow(non_camel_case_types)]
    pub type Env = <super::Contract as super::__Contract_TypesTrait_project>::Env;
    #[allow(non_camel_case_types)]
    pub type u32 = <super::Contract as super::__Contract_TypesTrait_project>::u32;
    #[allow(non_camel_case_types)]
    pub type Point = <super::Contract as super::__Contract_TypesTrait_project>::Point;
    #[allow(non_camel_case_types)]
    pub type Vec<T0> = <super::Contract as super::__Contract_TypesTrait_project>::Vec<T0>;
    #[allow(non_camel_case_types)]
    pub type Result<T0, T1> =
        <super::Contract as super::__Contract_TypesTrait_project>::Result<T0, T1>;
    #[allow(non_camel_case_types)]
    pub type Error = <super::Contract as super::__Contract_TypesTrait_project>::Error;
    #[allow(non_camel_case_types)]
    pub type Address = <super::Contract as super::__Contract_TypesTrait_project>::Address;
    #[allow(non_camel_case_types)]
    pub type MuxedAddress = <super::Contract as super::__Contract_TypesTrait_project>::MuxedAddress;
    #[allow(non_camel_case_types)]
    pub type Bytes = <super::Contract as super::__Contract_TypesTrait_project>::Bytes;
    #[allow(non_camel_case_types)]
    pub type Hash<const N0: usize> =
        <super::Contract as super::__Contract_TypesTrait_project>::Hash<N0>;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).point` instead")]
#[allow(deprecated)]
pub fn __Contract__point__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as traits::TypesTrait>::point(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).point` instead")]
pub fn __Contract__point__invoke_raw_slice(
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
    __Contract__point__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).point` instead")]
pub extern "C" fn __Contract__point__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__point__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).points` instead")]
#[allow(deprecated)]
pub fn __Contract__points__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as traits::TypesTrait>::points(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).points` instead")]
pub fn __Contract__points__invoke_raw_slice(
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
    __Contract__points__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).points` instead")]
pub extern "C" fn __Contract__points__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__points__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).even` instead")]
#[allow(deprecated)]
pub fn __Contract__even__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as traits::TypesTrait>::even(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).even` instead")]
pub fn __Contract__even__invoke_raw_slice(
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
    __Contract__even__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).even` instead")]
pub extern "C" fn __Contract__even__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__even__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).address` instead")]
#[allow(deprecated)]
pub fn __Contract__address__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as traits::TypesTrait>::address(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).address` instead")]
pub fn __Contract__address__invoke_raw_slice(
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
    __Contract__address__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).address` instead")]
pub extern "C" fn __Contract__address__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__address__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hash` instead")]
#[allow(deprecated)]
pub fn __Contract__hash__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as traits::TypesTrait>::hash(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hash` instead")]
pub fn __Contract__hash__invoke_raw_slice(
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
    __Contract__hash__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hash` instead")]
pub extern "C" fn __Contract__hash__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__hash__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__point__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_POINT: [u8; super::Contract::spec_xdr_point().len()] =
        super::Contract::spec_xdr_point();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_point: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"point"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"x",
                    ),
                    type_: <__Contract_TypesTrait_types::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"y",
                    ),
                    type_: <__Contract_TypesTrait_types::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_TypesTrait_types::Point as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_point() -> [u8; Contract::__SPEC_XDR_ENTRY_point.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_point.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__points__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_POINTS: [u8; super::Contract::spec_xdr_points().len()] =
        super::Contract::spec_xdr_points();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_points: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"points"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"p",
                    ),
                    type_: <__Contract_TypesTrait_types::Point as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_TypesTrait_types::Vec<
                    __Contract_TypesTrait_types::Point,
                > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_points() -> [u8; Contract::__SPEC_XDR_ENTRY_points.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_points.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__even__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_EVEN: [u8; super::Contract::spec_xdr_even().len()] =
        super::Contract::spec_xdr_even();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_even: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"even"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <__Contract_TypesTrait_types::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_TypesTrait_types::Result<
                    __Contract_TypesTrait_types::u32,
                    __Contract_TypesTrait_types::Error,
                > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_even() -> [u8; Contract::__SPEC_XDR_ENTRY_even.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_even.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__address__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_ADDRESS: [u8; super::Contract::spec_xdr_address().len()] =
        super::Contract::spec_xdr_address();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_address: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"address"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"a",
                    ),
                    type_: <__Contract_TypesTrait_types::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"m",
                    ),
                    type_: <__Contract_TypesTrait_types::MuxedAddress as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_TypesTrait_types::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_address() -> [u8; Contract::__SPEC_XDR_ENTRY_address.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_address.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__hash__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_HASH: [u8; super::Contract::spec_xdr_hash().len()] =
        super::Contract::spec_xdr_hash();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_hash: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"hash"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"b",
                    ),
                    type_: <__Contract_TypesTrait_types::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_TypesTrait_types::Hash<
                    32,
                > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_hash() -> [u8; Contract::__SPEC_XDR_ENTRY_hash.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_hash.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    /// Call `point` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_point`] to receive errors instead of panicking.
    pub fn point(
        &self,
        x: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::u32>,
        y: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::Ret<__Contract_TypesTrait_types::Point> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("point");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        x, &self.env,
                    ),
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        y, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `point` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_point(
        &self,
        x: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::u32>,
        y: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::TryRet<__Contract_TypesTrait_types::Point> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("point");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        x, &self.env,
                    ),
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        y, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `points` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_points`] to receive errors instead of panicking.
    pub fn points(
        &self,
        p: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::Point>,
    ) -> soroban_sdk::client::Ret<
        __Contract_TypesTrait_types::Vec<__Contract_TypesTrait_types::Point>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("points");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::Point>::into_arg(
                        p, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `points` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_points(
        &self,
        p: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::Point>,
    ) -> soroban_sdk::client::TryRet<
        __Contract_TypesTrait_types::Vec<__Contract_TypesTrait_types::Point>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("points");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::Point>::into_arg(
                        p, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `even` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_even`] to receive errors instead of panicking.
    pub fn even(
        &self,
        v: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::Ret<
        __Contract_TypesTrait_types::Result<
            __Contract_TypesTrait_types::u32,
            __Contract_TypesTrait_types::Error,
        >,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("even");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        v, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `even` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_even(
        &self,
        v: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::TryRet<
        __Contract_TypesTrait_types::Result<
            __Contract_TypesTrait_types::u32,
            __Contract_TypesTrait_types::Error,
        >,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("even");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        v, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `address` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_address`] to receive errors instead of panicking.
    pub fn address(
        &self,
        a: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::Address>,
        _m: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::MuxedAddress>,
    ) -> soroban_sdk::client::Ret<__Contract_TypesTrait_types::Address> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("address");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::Address>::into_arg(
                        a, &self.env,
                    ),
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::MuxedAddress>::into_arg(
                        _m, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `address` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_address(
        &self,
        a: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::Address>,
        _m: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::MuxedAddress>,
    ) -> soroban_sdk::client::TryRet<__Contract_TypesTrait_types::Address> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("address");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::Address>::into_arg(
                        a, &self.env,
                    ),
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::MuxedAddress>::into_arg(
                        _m, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `hash` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_hash`] to receive errors instead of panicking.
    pub fn hash(
        &self,
        b: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::Bytes>,
    ) -> soroban_sdk::client::Ret<__Contract_TypesTrait_types::Hash<32>> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hash");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::Bytes>::into_arg(
                        b, &self.env,
                    ),
                ],
            ),
        );
        res
    }
    /// Call `hash` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_hash(
        &self,
        b: impl soroban_sdk::IntoArg<__Contract_TypesTrait_types::Bytes>,
    ) -> soroban_sdk::client::TryRet<__Contract_TypesTrait_types::Hash<32>> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hash");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::IntoArg::<__Contract_TypesTrait_types::Bytes>::into_arg(
                        b, &self.env,
                    ),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn point<'i>(
        x: &'i __Contract_TypesTrait_types::u32,
        y: &'i __Contract_TypesTrait_types::u32,
    ) -> (
        &'i __Contract_TypesTrait_types::u32,
        &'i __Contract_TypesTrait_types::u32,
    ) {
        (x, y)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn points<'i>(
        p: &'i __Contract_TypesTrait_types::Point,
    ) -> (&'i __Contract_TypesTrait_types::Point,) {
        (p,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn even<'i>(
        v: &'i __Contract_TypesTrait_types::u32,
    ) -> (&'i __Contract_TypesTrait_types::u32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn address<'i>(
        a: &'i __Contract_TypesTrait_types::Address,
        _m: &'i __Contract_TypesTrait_types::MuxedAddress,
    ) -> (
        &'i __Contract_TypesTrait_types::Address,
        &'i __Contract_TypesTrait_types::MuxedAddress,
    ) {
        (a, _m)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn hash<'i>(
        b: &'i __Contract_TypesTrait_types::Bytes,
    ) -> (&'i __Contract_TypesTrait_types::Bytes,) {
        (b,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__traits_TypesTrait__eae13a0850ed3dec147eb36ad6535bcb907e2c1959f71f3f3e728d59000c7f58_ctor(
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
                    __Contract__traits_TypesTrait__eae13a0850ed3dec147eb36ad6535bcb907e2c1959f71f3f3e728d59000c7f58_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "point",
            #[allow(deprecated)]
            &__Contract__point__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "points",
            #[allow(deprecated)]
            &__Contract__points__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "even",
            #[allow(deprecated)]
            &__Contract__even__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "address",
            #[allow(deprecated)]
            &__Contract__address__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "hash",
            #[allow(deprecated)]
            &__Contract__hash__invoke_raw_slice,
        );
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__traits_TypesTrait__e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor(
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
                    __Contract__traits_TypesTrait__e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {}
}
impl Contract {
    pub fn hash_direct(env: &Env, b: Bytes) -> Hash<32> {
        env.crypto().sha256(&b)
    }
    pub fn muxed_direct(_env: &Env, m: MuxedAddress) -> MuxedAddress {
        m
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__hash_direct__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_HASH_DIRECT: [u8; super::Contract::spec_xdr_hash_direct().len()] =
        super::Contract::spec_xdr_hash_direct();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_hash_direct: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"hash_direct"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Hash<32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_hash_direct(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_hash_direct.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_hash_direct.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__muxed_direct__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_MUXED_DIRECT: [u8; super::Contract::spec_xdr_muxed_direct().len()] =
        super::Contract::spec_xdr_muxed_direct();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_muxed_direct: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"muxed_direct"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"m"),
                        type_: <MuxedAddress as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <MuxedAddress as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_muxed_direct(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_muxed_direct.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_muxed_direct.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    /// Call `hash_direct` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_hash_direct`] to receive errors instead of panicking.
    pub fn hash_direct(
        &self,
        b: impl soroban_sdk::IntoArg<Bytes>,
    ) -> soroban_sdk::client::Ret<Hash<32>> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "hash_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Bytes>::into_arg(b, &self.env)],
            ),
        );
        res
    }
    /// Call `hash_direct` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_hash_direct(
        &self,
        b: impl soroban_sdk::IntoArg<Bytes>,
    ) -> soroban_sdk::client::TryRet<Hash<32>> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "hash_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Bytes>::into_arg(b, &self.env)],
            ),
        );
        res
    }
    /// Call `muxed_direct` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_muxed_direct`] to receive errors instead of panicking.
    pub fn muxed_direct(
        &self,
        m: impl soroban_sdk::IntoArg<MuxedAddress>,
    ) -> soroban_sdk::client::Ret<MuxedAddress> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "muxed_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MuxedAddress>::into_arg(m, &self.env)],
            ),
        );
        res
    }
    /// Call `muxed_direct` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_muxed_direct(
        &self,
        m: impl soroban_sdk::IntoArg<MuxedAddress>,
    ) -> soroban_sdk::client::TryRet<MuxedAddress> {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "muxed_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MuxedAddress>::into_arg(m, &self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn hash_direct<'i>(b: &'i Bytes) -> (&'i Bytes,) {
        (b,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn muxed_direct<'i>(m: &'i MuxedAddress) -> (&'i MuxedAddress,) {
        (m,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hash_direct` instead")]
#[allow(deprecated)]
pub fn __Contract__hash_direct__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::hash_direct(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hash_direct` instead")]
pub fn __Contract__hash_direct__invoke_raw_slice(
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
    __Contract__hash_direct__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).hash_direct` instead")]
pub extern "C" fn __Contract__hash_direct__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__hash_direct__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).muxed_direct` instead")]
#[allow(deprecated)]
pub fn __Contract__muxed_direct__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::muxed_direct(
            &env,
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).muxed_direct` instead")]
pub fn __Contract__muxed_direct__invoke_raw_slice(
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
    __Contract__muxed_direct__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).muxed_direct` instead")]
pub extern "C" fn __Contract__muxed_direct__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__muxed_direct__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____3824a1bad62a4a8ceae1249ae3d7dc5b6ef9715668605b81d16da950d807cf69_ctor() {
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
                    __Contract____3824a1bad62a4a8ceae1249ae3d7dc5b6ef9715668605b81d16da950d807cf69_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "hash_direct",
            #[allow(deprecated)]
            &__Contract__hash_direct__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "muxed_direct",
            #[allow(deprecated)]
            &__Contract__muxed_direct__invoke_raw_slice,
        );
    }
}
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, MuxedAddress};
    extern crate test;
    #[rustc_test_marker = "test::test_types"]
    #[doc(hidden)]
    pub const test_types: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_types"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/contracttrait_types/src/lib.rs",
            start_line: 80usize,
            start_col: 8usize,
            end_line: 80usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_types()),
        ),
    };
    fn test_types() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let p = traits::Point { x: 1, y: 2 };
        match (&client.point(&1, &2), &p) {
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
        match (&client.points(&p).len(), &2) {
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
        match (&client.even(&2), &2) {
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
        match (&client.try_even(&3), &Err(Ok(traits::Error::Odd))) {
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
        let a = Address::generate(&e);
        match (&client.address(&a, &a), &a) {
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
        match (&client.address(&a, a.clone()), &a) {
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
        let m: MuxedAddress = a.clone().into();
        match (&client.address(&a, &m), &a) {
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
        match (&client.muxed_direct(&a).address(), &a) {
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
        let b = Bytes::from_array(&e, &[1, 2, 3]);
        let expected: BytesN<32> = e.crypto().sha256(&b).into();
        let h: BytesN<32> = client.hash(&b);
        match (&h, &expected) {
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
        let h: BytesN<32> = client.hash_direct(&b);
        match (&h, &expected) {
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
    test::test_main_static(&[&test_types])
}
