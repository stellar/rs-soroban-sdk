#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{
    contracttrait, contracttype, Address, Bytes, BytesN, Duration, Env, Map, String, Symbol,
    Timepoint, Vec, I256, U256,
};
pub struct MyStruct {
    pub a: i64,
    pub b: i64,
}
#[automatically_derived]
impl ::core::clone::Clone for MyStruct {
    #[inline]
    fn clone(&self) -> MyStruct {
        MyStruct {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f, "MyStruct", "a", &self.a, "b", &&self.b,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for MyStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyStruct {
    #[inline]
    fn eq(&self, other: &MyStruct) -> bool {
        self.a == other.a && self.b == other.b
    }
}
pub static __SPEC_XDR_TYPE_MYSTRUCT: [u8; 60usize] = MyStruct::spec_xdr();
impl MyStruct {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x08MyStruct\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x07"
    }
}
impl soroban_sdk::IncludeSpecMarker for MyStruct {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for MyStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            a: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            b: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, MyStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &MyStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "b"];
        let vals: [Val; 2usize] = [
            (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.b).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &MyStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&MyStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, MyStruct>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for MyStruct {
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
            a: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "a".try_into()
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
            b: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "b".try_into()
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for MyStruct {
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
impl TryFrom<&MyStruct> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &MyStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "a".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.a)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "b".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.b)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
            ]),
        ))
    }
}
impl TryFrom<MyStruct> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: MyStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&MyStruct> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &MyStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<MyStruct> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: MyStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryMyStruct {
        a: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        b: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryMyStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ArbitraryMyStruct",
                "a",
                &self.a,
                "b",
                &&self.b,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryMyStruct {
        #[inline]
        fn clone(&self) -> ArbitraryMyStruct {
            ArbitraryMyStruct {
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryMyStruct {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryMyStruct {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryMyStruct {
        #[inline]
        fn eq(&self, other: &ArbitraryMyStruct) -> bool {
            self.a == other.a && self.b == other.b
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryMyStruct {
        #[inline]
        fn cmp(&self, other: &ArbitraryMyStruct) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryMyStruct {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryMyStruct,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b)
                }
                cmp => cmp,
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryMyStruct: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryMyStruct {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyStruct.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryMyStruct {
                        a: arbitrary::Arbitrary::arbitrary(u)?,
                        b: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyStruct.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryMyStruct.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryMyStruct {
                        a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        b: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyStruct.with(|count| {
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
                            <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for MyStruct {
        type Prototype = ArbitraryMyStruct;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryMyStruct> for MyStruct {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryMyStruct,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(MyStruct {
                a: soroban_sdk::IntoVal::into_val(&v.a, env),
                b: soroban_sdk::IntoVal::into_val(&v.b, env),
            })
        }
    }
};
pub enum MyEnumUnit {
    A = 1,
    B = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for MyEnumUnit {}
#[automatically_derived]
impl ::core::clone::Clone for MyEnumUnit {
    #[inline]
    fn clone(&self) -> MyEnumUnit {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyEnumUnit {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                MyEnumUnit::A => "A",
                MyEnumUnit::B => "B",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for MyEnumUnit {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyEnumUnit {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyEnumUnit {
    #[inline]
    fn eq(&self, other: &MyEnumUnit) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
pub static __SPEC_XDR_TYPE_MYENUMUNIT: [u8; 64usize] = MyEnumUnit::spec_xdr();
impl MyEnumUnit {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\nMyEnumUnit\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::IncludeSpecMarker for MyEnumUnit {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for MyEnumUnit {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::A,
            2u32 => Self::B,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, MyEnumUnit> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &MyEnumUnit,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            MyEnumUnit::A => 1u32.into(),
            MyEnumUnit::B => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &MyEnumUnit> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&MyEnumUnit,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, MyEnumUnit>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for MyEnumUnit {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVal,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
            Ok(match *discriminant {
                1u32 => Self::A,
                2u32 => Self::B,
                _ => Err(soroban_sdk::xdr::Error::Invalid)?,
            })
        } else {
            Err(soroban_sdk::xdr::Error::Invalid)
        }
    }
}
impl TryInto<soroban_sdk::xdr::ScVal> for &MyEnumUnit {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
        Ok(match self {
            MyEnumUnit::A => 1u32.into(),
            MyEnumUnit::B => 2u32.into(),
        })
    }
}
impl TryInto<soroban_sdk::xdr::ScVal> for MyEnumUnit {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
        Ok(match self {
            MyEnumUnit::A => 1u32.into(),
            MyEnumUnit::B => 2u32.into(),
        })
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryMyEnumUnit {
        A,
        B,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryMyEnumUnit {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ArbitraryMyEnumUnit::A => "A",
                    ArbitraryMyEnumUnit::B => "B",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryMyEnumUnit {
        #[inline]
        fn clone(&self) -> ArbitraryMyEnumUnit {
            match self {
                ArbitraryMyEnumUnit::A => ArbitraryMyEnumUnit::A,
                ArbitraryMyEnumUnit::B => ArbitraryMyEnumUnit::B,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryMyEnumUnit {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryMyEnumUnit {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryMyEnumUnit {
        #[inline]
        fn eq(&self, other: &ArbitraryMyEnumUnit) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryMyEnumUnit {
        #[inline]
        fn cmp(&self, other: &ArbitraryMyEnumUnit) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryMyEnumUnit {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryMyEnumUnit,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryMyEnumUnit: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryMyEnumUnit {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyEnumUnit.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 2u64) >> 32
                        {
                            0u64 => ArbitraryMyEnumUnit::A,
                            1u64 => ArbitraryMyEnumUnit::B,
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyEnumUnit.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryMyEnumUnit.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?) * 2u64)
                            >> 32
                        {
                            0u64 => ArbitraryMyEnumUnit::A,
                            1u64 => ArbitraryMyEnumUnit::B,
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyEnumUnit.with(|count| {
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
                        ])
                    }),
                )
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for MyEnumUnit {
        type Prototype = ArbitraryMyEnumUnit;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryMyEnumUnit> for MyEnumUnit {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryMyEnumUnit,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryMyEnumUnit::A => MyEnumUnit::A,
                ArbitraryMyEnumUnit::B => MyEnumUnit::B,
            })
        }
    }
};
pub enum MyEnumVariants {
    VarA,
    VarB(MyStruct),
    VarC(MyEnumUnit),
}
#[automatically_derived]
impl ::core::clone::Clone for MyEnumVariants {
    #[inline]
    fn clone(&self) -> MyEnumVariants {
        match self {
            MyEnumVariants::VarA => MyEnumVariants::VarA,
            MyEnumVariants::VarB(__self_0) => {
                MyEnumVariants::VarB(::core::clone::Clone::clone(__self_0))
            }
            MyEnumVariants::VarC(__self_0) => {
                MyEnumVariants::VarC(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyEnumVariants {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MyEnumVariants::VarA => ::core::fmt::Formatter::write_str(f, "VarA"),
            MyEnumVariants::VarB(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "VarB", &__self_0)
            }
            MyEnumVariants::VarC(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "VarC", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for MyEnumVariants {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<MyStruct>;
        let _: ::core::cmp::AssertParamIsEq<MyEnumUnit>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyEnumVariants {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyEnumVariants {
    #[inline]
    fn eq(&self, other: &MyEnumVariants) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (MyEnumVariants::VarB(__self_0), MyEnumVariants::VarB(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (MyEnumVariants::VarC(__self_0), MyEnumVariants::VarC(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => true,
            }
    }
}
pub static __SPEC_XDR_TYPE_MYENUMVARIANTS: [u8; 128usize] = MyEnumVariants::spec_xdr();
impl MyEnumVariants {
    pub const fn spec_xdr() -> [u8; 128usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x0eMyEnumVariants\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x04VarA\0\0\0\x01\0\0\0\0\0\0\0\x04VarB\0\0\0\x01\0\0\x07\xd0\0\0\0\x08MyStruct\0\0\0\x01\0\0\0\0\0\0\0\x04VarC\0\0\0\x01\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0"
    }
}
impl soroban_sdk::IncludeSpecMarker for MyEnumVariants {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {}
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for MyEnumVariants {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["VarA", "VarB", "VarC"];
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
                    if iter.len() > 0 {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::VarA
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::VarB(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::VarC(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, MyEnumVariants> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &MyEnumVariants,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            MyEnumVariants::VarA => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"VarA")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            MyEnumVariants::VarB(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"VarB")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            MyEnumVariants::VarC(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"VarC")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &MyEnumVariants> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&MyEnumVariants,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, MyEnumVariants>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for MyEnumVariants {
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
            "VarA" => {
                if iter.len() > 0 {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                Self::VarA
            }
            "VarB" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::VarB(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            "VarC" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::VarC(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            _ => Err(soroban_sdk::xdr::Error::Invalid)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for MyEnumVariants {
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
impl TryFrom<&MyEnumVariants> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &MyEnumVariants) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        Ok(match val {
            MyEnumVariants::VarA => {
                let symbol = soroban_sdk::xdr::ScSymbol(
                    "VarA"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                );
                let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                (val,)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
            }
            MyEnumVariants::VarB(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "VarB"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                ),
                value0,
            )
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            MyEnumVariants::VarC(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "VarC"
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
impl TryFrom<MyEnumVariants> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: MyEnumVariants) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&MyEnumVariants> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &MyEnumVariants) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
    }
}
impl TryFrom<MyEnumVariants> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: MyEnumVariants) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryMyEnumVariants {
        VarA,
        VarB(<MyStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
        VarC(<MyEnumUnit as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryMyEnumVariants {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ArbitraryMyEnumVariants::VarA => ::core::fmt::Formatter::write_str(f, "VarA"),
                ArbitraryMyEnumVariants::VarB(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "VarB", &__self_0)
                }
                ArbitraryMyEnumVariants::VarC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "VarC", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryMyEnumVariants {
        #[inline]
        fn clone(&self) -> ArbitraryMyEnumVariants {
            match self {
                ArbitraryMyEnumVariants::VarA => ArbitraryMyEnumVariants::VarA,
                ArbitraryMyEnumVariants::VarB(__self_0) => {
                    ArbitraryMyEnumVariants::VarB(::core::clone::Clone::clone(__self_0))
                }
                ArbitraryMyEnumVariants::VarC(__self_0) => {
                    ArbitraryMyEnumVariants::VarC(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryMyEnumVariants {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <MyStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <MyEnumUnit as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryMyEnumVariants {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryMyEnumVariants {
        #[inline]
        fn eq(&self, other: &ArbitraryMyEnumVariants) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ArbitraryMyEnumVariants::VarB(__self_0),
                        ArbitraryMyEnumVariants::VarB(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        ArbitraryMyEnumVariants::VarC(__self_0),
                        ArbitraryMyEnumVariants::VarC(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryMyEnumVariants {
        #[inline]
        fn cmp(&self, other: &ArbitraryMyEnumVariants) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (
                        ArbitraryMyEnumVariants::VarB(__self_0),
                        ArbitraryMyEnumVariants::VarB(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (
                        ArbitraryMyEnumVariants::VarC(__self_0),
                        ArbitraryMyEnumVariants::VarC(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryMyEnumVariants {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryMyEnumVariants,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (
                    ArbitraryMyEnumVariants::VarB(__self_0),
                    ArbitraryMyEnumVariants::VarB(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (
                    ArbitraryMyEnumVariants::VarC(__self_0),
                    ArbitraryMyEnumVariants::VarC(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryMyEnumVariants: ::std::thread::LocalKey<
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryMyEnumVariants {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyEnumVariants.with(|count| {
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
                            0u64 => ArbitraryMyEnumVariants::VarA,
                            1u64 => {
                                ArbitraryMyEnumVariants::VarB(arbitrary::Arbitrary::arbitrary(u)?)
                            }
                            2u64 => {
                                ArbitraryMyEnumVariants::VarC(arbitrary::Arbitrary::arbitrary(u)?)
                            }
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyEnumVariants.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryMyEnumVariants.with(|count| {
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
                            0u64 => ArbitraryMyEnumVariants::VarA,
                            1u64 => ArbitraryMyEnumVariants::VarB(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            2u64 => ArbitraryMyEnumVariants::VarC(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryMyEnumVariants.with(|count| {
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
                                    arbitrary::size_hint::and_all(&[]),
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<MyStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                depth,
                                            ),
                                        ],
                                    ),
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<MyEnumUnit as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for MyEnumVariants {
        type Prototype = ArbitraryMyEnumVariants;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryMyEnumVariants> for MyEnumVariants {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryMyEnumVariants,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryMyEnumVariants::VarA => MyEnumVariants::VarA,
                ArbitraryMyEnumVariants::VarB(field_0) => {
                    MyEnumVariants::VarB(soroban_sdk::IntoVal::into_val(field_0, env))
                }
                ArbitraryMyEnumVariants::VarC(field_0) => {
                    MyEnumVariants::VarC(soroban_sdk::IntoVal::into_val(field_0, env))
                }
            })
        }
    }
};
pub struct AllTypesSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_all_types as AllTypes;
pub trait AllTypes {
    /// Test u32 values.
    /// Returns the input unchanged.
    fn test_u32(v: u32) -> u32 {
        v
    }
    /// Test i32 values.
    fn test_i32(v: i32) -> i32 {
        v
    }
    fn test_u64(v: u64) -> u64 {
        v
    }
    fn test_i64(v: i64) -> i64 {
        v
    }
    fn test_u128(v: u128) -> u128 {
        v
    }
    fn test_i128(v: i128) -> i128 {
        v
    }
    fn test_bool(v: bool) -> bool {
        v
    }
    fn test_address(v: Address) -> Address {
        v
    }
    fn test_bytes(v: Bytes) -> Bytes {
        v
    }
    fn test_bytes_n(v: BytesN<32>) -> BytesN<32> {
        v
    }
    fn test_string(v: String) -> String {
        v
    }
    fn test_symbol(v: Symbol) -> Symbol {
        v
    }
    fn test_vec(v: Vec<u32>) -> Vec<u32> {
        v
    }
    fn test_map(v: Map<u32, u32>) -> Map<u32, u32> {
        v
    }
    fn test_duration(v: Duration) -> Duration {
        v
    }
    fn test_timepoint(v: Timepoint) -> Timepoint {
        v
    }
    fn test_i256(v: I256) -> I256 {
        v
    }
    fn test_u256(v: U256) -> U256 {
        v
    }
    fn test_env_param(env: &Env) -> u32 {
        let _ = env;
        42
    }
    fn test_struct(v: MyStruct) -> MyStruct {
        v
    }
    fn test_enum_unit(v: MyEnumUnit) -> MyEnumUnit {
        v
    }
    fn test_enum_variants(v: MyEnumVariants) -> MyEnumVariants {
        v
    }
}
///AllTypesClient is a client for calling the contract defined in "AllTypes".
pub struct AllTypesClient<'a> {
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
impl<'a> AllTypesClient<'a> {
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
impl<'a> AllTypesClient<'a> {
    /// Test u32 values.
    /// Returns the input unchanged.
    pub fn test_u32(&self, v: &u32) -> u32 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    /// Test u32 values.
    /// Returns the input unchanged.
    pub fn try_test_u32(
        &self,
        v: &u32,
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
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    /// Test i32 values.
    pub fn test_i32(&self, v: &i32) -> i32 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    /// Test i32 values.
    pub fn try_test_i32(
        &self,
        v: &i32,
    ) -> Result<
        Result<i32, <i32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_u64(&self, v: &u64) -> u64 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_u64(
        &self,
        v: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_i64(&self, v: &i64) -> i64 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_i64(
        &self,
        v: &i64,
    ) -> Result<
        Result<i64, <i64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_u128(&self, v: &u128) -> u128 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_u128(
        &self,
        v: &u128,
    ) -> Result<
        Result<u128, <u128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_i128(&self, v: &i128) -> i128 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_i128(
        &self,
        v: &i128,
    ) -> Result<
        Result<i128, <i128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_bool(&self, v: &bool) -> bool {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_bool(
        &self,
        v: &bool,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_address(&self, v: &Address) -> Address {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_address(
        &self,
        v: &Address,
    ) -> Result<
        Result<
            Address,
            <Address as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_bytes(&self, v: &Bytes) -> Bytes {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_bytes(
        &self,
        v: &Bytes,
    ) -> Result<
        Result<
            Bytes,
            <Bytes as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_bytes_n(&self, v: &BytesN<32>) -> BytesN<32> {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_bytes_n(
        &self,
        v: &BytesN<32>,
    ) -> Result<
        Result<
            BytesN<32>,
            <BytesN<32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_string(&self, v: &String) -> String {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_string(
        &self,
        v: &String,
    ) -> Result<
        Result<
            String,
            <String as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_symbol(&self, v: &Symbol) -> Symbol {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_symbol(
        &self,
        v: &Symbol,
    ) -> Result<
        Result<
            Symbol,
            <Symbol as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_vec(&self, v: &Vec<u32>) -> Vec<u32> {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_vec(
        &self,
        v: &Vec<u32>,
    ) -> Result<
        Result<
            Vec<u32>,
            <Vec<u32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_map(&self, v: &Map<u32, u32>) -> Map<u32, u32> {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_map(
        &self,
        v: &Map<u32, u32>,
    ) -> Result<
        Result<
            Map<u32, u32>,
            <Map<u32, u32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_duration(&self, v: &Duration) -> Duration {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_duration(
        &self,
        v: &Duration,
    ) -> Result<
        Result<
            Duration,
            <Duration as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_timepoint(&self, v: &Timepoint) -> Timepoint {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_timepoint(
        &self,
        v: &Timepoint,
    ) -> Result<
        Result<
            Timepoint,
            <Timepoint as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_i256(&self, v: &I256) -> I256 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_i256(
        &self,
        v: &I256,
    ) -> Result<
        Result<I256, <I256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_u256(&self, v: &U256) -> U256 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_u256(
        &self,
        v: &U256,
    ) -> Result<
        Result<U256, <U256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_env_param(&self) -> u32 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_env_param(
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
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_struct(&self, v: &MyStruct) -> MyStruct {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_struct(
        &self,
        v: &MyStruct,
    ) -> Result<
        Result<
            MyStruct,
            <MyStruct as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_enum_unit(&self, v: &MyEnumUnit) -> MyEnumUnit {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_enum_unit(
        &self,
        v: &MyEnumUnit,
    ) -> Result<
        Result<
            MyEnumUnit,
            <MyEnumUnit as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn test_enum_variants(&self, v: &MyEnumVariants) -> MyEnumVariants {
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_test_enum_variants(
        &self,
        v: &MyEnumVariants,
    ) -> Result<
        Result<
            MyEnumVariants,
            <MyEnumVariants as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
}
///AllTypesArgs is a type for building arg lists for functions defined in "AllTypes".
pub struct AllTypesArgs;
impl AllTypesArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u32<'i>(v: &'i u32) -> (&'i u32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i32<'i>(v: &'i i32) -> (&'i i32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u64<'i>(v: &'i u64) -> (&'i u64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i64<'i>(v: &'i i64) -> (&'i i64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u128<'i>(v: &'i u128) -> (&'i u128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i128<'i>(v: &'i i128) -> (&'i i128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bool<'i>(v: &'i bool) -> (&'i bool,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_address<'i>(v: &'i Address) -> (&'i Address,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes<'i>(v: &'i Bytes) -> (&'i Bytes,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes_n<'i>(v: &'i BytesN<32>) -> (&'i BytesN<32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_string<'i>(v: &'i String) -> (&'i String,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_symbol<'i>(v: &'i Symbol) -> (&'i Symbol,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_vec<'i>(v: &'i Vec<u32>) -> (&'i Vec<u32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_map<'i>(v: &'i Map<u32, u32>) -> (&'i Map<u32, u32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_duration<'i>(v: &'i Duration) -> (&'i Duration,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_timepoint<'i>(v: &'i Timepoint) -> (&'i Timepoint,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i256<'i>(v: &'i I256) -> (&'i I256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u256<'i>(v: &'i U256) -> (&'i U256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_env_param<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_struct<'i>(v: &'i MyStruct) -> (&'i MyStruct,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_unit<'i>(v: &'i MyEnumUnit) -> (&'i MyEnumUnit,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_variants<'i>(v: &'i MyEnumVariants) -> (&'i MyEnumVariants,) {
        (v,)
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    /// Test u32 values.
    /// Returns the input unchanged.
    pub const fn spec_xdr_test_u32() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0-Test u32 values.\nReturns the input unchanged.\0\0\0\0\0\0\x08test_u32\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    /// Test i32 values.
    pub const fn spec_xdr_test_i32() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\x10Test i32 values.\0\0\0\x08test_i32\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x05\0\0\0\x01\0\0\0\x05"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_u64() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_u64\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_i64() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_i64\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x07\0\0\0\x01\0\0\0\x07"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_u128() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_u128\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\n\0\0\0\x01\0\0\0\n"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_i128() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_i128\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0b\0\0\0\x01\0\0\0\x0b"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_bool() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_bool\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_address() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ctest_address\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x13\0\0\0\x01\0\0\0\x13"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_bytes() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ntest_bytes\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0e\0\0\0\x01\0\0\0\x0e"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_bytes_n() -> [u8; 60usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ctest_bytes_n\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xee\0\0\0 \0\0\0\x01\0\0\x03\xee\0\0\0 "
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_string() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_string\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x10\0\0\0\x01\0\0\0\x10"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_symbol() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_symbol\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x11\0\0\0\x01\0\0\0\x11"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_vec() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_vec\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xea\0\0\0\x04\0\0\0\x01\0\0\x03\xea\0\0\0\x04"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_map() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_map\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xec\0\0\0\x04\0\0\0\x04\0\0\0\x01\0\0\x03\xec\0\0\0\x04\0\0\0\x04"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_duration() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rtest_duration\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\t\0\0\0\x01\0\0\0\t"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_timepoint() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_timepoint\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x08\0\0\0\x01\0\0\0\x08"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_i256() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_i256\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\r\0\0\0\x01\0\0\0\r"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_u256() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_u256\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\0\x0c"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_env_param() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_env_param\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_struct() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_struct\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\x08MyStruct\0\0\0\x01\0\0\x07\xd0\0\0\0\x08MyStruct"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_enum_unit() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_enum_unit\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0"
    }
}
impl AllTypesSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_enum_variants() -> [u8; 100usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x12test_enum_variants\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\x0eMyEnumVariants\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0eMyEnumVariants\0\0"
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        contract, contractimpl, map, symbol_short, testutils::Address as _, vec, Env,
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
    impl AllTypes for Contract {}
    impl<'a> ContractClient<'a> {}
    impl ContractArgs {}
    #[doc(hidden)]
    /// Test u32 values.
    /// Returns the input unchanged.
    #[allow(non_snake_case)]
    pub mod __Contract__test_u32 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u32` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_u32(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u32` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u32` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    /// Test i32 values.
    #[allow(non_snake_case)]
    pub mod __Contract__test_i32 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i32` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_i32(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i32` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i32` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_u64 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u64` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_u64(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u64` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u64` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_i64 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i64` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_i64(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i64` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i64` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_u128 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u128` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_u128(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u128` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u128` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_i128 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i128` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_i128(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i128` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i128` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_bool {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bool` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_bool(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bool` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bool` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_address {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_address` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_address(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_address` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_address` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_bytes {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_bytes(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_bytes_n {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes_n` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_bytes_n(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes_n` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes_n` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_string {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_string` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_string(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_string` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_string` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_symbol {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_symbol` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_symbol(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_symbol` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_symbol` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_vec {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_vec` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_vec(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_vec` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_vec` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_map {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_map` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_map(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_map` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_map` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_duration {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_duration` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_duration(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_duration` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_duration` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_timepoint {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_timepoint` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_timepoint(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_timepoint` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_timepoint` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_i256 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i256` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_i256(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i256` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i256` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_u256 {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u256` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_u256(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u256` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u256` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_env_param {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_env_param` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_env_param(&env),
                &env,
            )
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_env_param` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env)
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_env_param` instead")]
        pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default())
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_struct {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_struct` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_struct(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_struct` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_struct` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_enum_unit {
        use super::*;
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_unit` instead")]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_enum_unit(
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
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_unit` instead")]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_unit` instead")]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_enum_variants {
        use super::*;
        #[deprecated(
            note = "use `ContractClient::new(&env, &contract_id).test_enum_variants` instead"
        )]
        #[allow(deprecated)]
        pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            use super::AllTypes;
            soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
                <super::Contract>::test_enum_variants(
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
        #[deprecated(
            note = "use `ContractClient::new(&env, &contract_id).test_enum_variants` instead"
        )]
        pub fn invoke_raw_slice(
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
            invoke_raw(env, args[0usize])
        }
        #[deprecated(
            note = "use `ContractClient::new(&env, &contract_id).test_enum_variants` instead"
        )]
        pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
            #[allow(deprecated)]
            invoke_raw(soroban_sdk::Env::default(), arg_0)
        }
        use super::*;
    }
    #[doc(hidden)]
    /// Test u32 values.
    /// Returns the input unchanged.
    #[allow(non_snake_case)]
    pub mod __Contract__test_u32__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        /// Test u32 values.
        /// Returns the input unchanged.
        pub static __SPEC_XDR_FN_TEST_U32: [u8; 96usize] = super::Contract::spec_xdr_test_u32();
    }
    impl Contract {
        #[allow(non_snake_case)]
        /// Test u32 values.
        /// Returns the input unchanged.
        pub const fn spec_xdr_test_u32() -> [u8; 96usize] {
            *b"\0\0\0\0\0\0\0-Test u32 values.\nReturns the input unchanged.\0\0\0\0\0\0\x08test_u32\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
        }
    }
    #[doc(hidden)]
    /// Test i32 values.
    #[allow(non_snake_case)]
    pub mod __Contract__test_i32__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        /// Test i32 values.
        pub static __SPEC_XDR_FN_TEST_I32: [u8; 64usize] = super::Contract::spec_xdr_test_i32();
    }
    impl Contract {
        #[allow(non_snake_case)]
        /// Test i32 values.
        pub const fn spec_xdr_test_i32() -> [u8; 64usize] {
            *b"\0\0\0\0\0\0\0\x10Test i32 values.\0\0\0\x08test_i32\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x05\0\0\0\x01\0\0\0\x05"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_u64__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_U64: [u8; 48usize] = super::Contract::spec_xdr_test_u64();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_u64() -> [u8; 48usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_u64\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_i64__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_I64: [u8; 48usize] = super::Contract::spec_xdr_test_i64();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_i64() -> [u8; 48usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_i64\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x07\0\0\0\x01\0\0\0\x07"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_u128__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_U128: [u8; 52usize] = super::Contract::spec_xdr_test_u128();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_u128() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_u128\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\n\0\0\0\x01\0\0\0\n"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_i128__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_I128: [u8; 52usize] = super::Contract::spec_xdr_test_i128();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_i128() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_i128\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0b\0\0\0\x01\0\0\0\x0b"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_bool__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_BOOL: [u8; 52usize] = super::Contract::spec_xdr_test_bool();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_bool() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_bool\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_address__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_ADDRESS: [u8; 52usize] =
            super::Contract::spec_xdr_test_address();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_address() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0ctest_address\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x13\0\0\0\x01\0\0\0\x13"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_bytes__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_BYTES: [u8; 52usize] = super::Contract::spec_xdr_test_bytes();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_bytes() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\ntest_bytes\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0e\0\0\0\x01\0\0\0\x0e"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_bytes_n__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_BYTES_N: [u8; 60usize] =
            super::Contract::spec_xdr_test_bytes_n();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_bytes_n() -> [u8; 60usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0ctest_bytes_n\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xee\0\0\0 \0\0\0\x01\0\0\x03\xee\0\0\0 "
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_string__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_STRING: [u8; 52usize] =
            super::Contract::spec_xdr_test_string();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_string() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_string\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x10\0\0\0\x01\0\0\0\x10"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_symbol__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_SYMBOL: [u8; 52usize] =
            super::Contract::spec_xdr_test_symbol();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_symbol() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_symbol\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x11\0\0\0\x01\0\0\0\x11"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_vec__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_VEC: [u8; 56usize] = super::Contract::spec_xdr_test_vec();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_vec() -> [u8; 56usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_vec\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xea\0\0\0\x04\0\0\0\x01\0\0\x03\xea\0\0\0\x04"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_map__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_MAP: [u8; 64usize] = super::Contract::spec_xdr_test_map();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_map() -> [u8; 64usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_map\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xec\0\0\0\x04\0\0\0\x04\0\0\0\x01\0\0\x03\xec\0\0\0\x04\0\0\0\x04"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_duration__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_DURATION: [u8; 56usize] =
            super::Contract::spec_xdr_test_duration();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_duration() -> [u8; 56usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\rtest_duration\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\t\0\0\0\x01\0\0\0\t"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_timepoint__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_TIMEPOINT: [u8; 56usize] =
            super::Contract::spec_xdr_test_timepoint();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_timepoint() -> [u8; 56usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_timepoint\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x08\0\0\0\x01\0\0\0\x08"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_i256__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_I256: [u8; 52usize] = super::Contract::spec_xdr_test_i256();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_i256() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_i256\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\r\0\0\0\x01\0\0\0\r"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_u256__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_U256: [u8; 52usize] = super::Contract::spec_xdr_test_u256();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_u256() -> [u8; 52usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_u256\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\0\x0c"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_env_param__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_ENV_PARAM: [u8; 40usize] =
            super::Contract::spec_xdr_test_env_param();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_env_param() -> [u8; 40usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_env_param\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_struct__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_STRUCT: [u8; 76usize] =
            super::Contract::spec_xdr_test_struct();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_struct() -> [u8; 76usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_struct\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\x08MyStruct\0\0\0\x01\0\0\x07\xd0\0\0\0\x08MyStruct"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_enum_unit__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_ENUM_UNIT: [u8; 88usize] =
            super::Contract::spec_xdr_test_enum_unit();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_enum_unit() -> [u8; 88usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_enum_unit\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0"
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__test_enum_variants__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_XDR_FN_TEST_ENUM_VARIANTS: [u8; 100usize] =
            super::Contract::spec_xdr_test_enum_variants();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_test_enum_variants() -> [u8; 100usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x12test_enum_variants\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\x0eMyEnumVariants\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0eMyEnumVariants\0\0"
        }
    }
    impl<'a> ContractClient<'a> {
        /// Test u32 values.
        /// Returns the input unchanged.
        pub fn test_u32(&self, v: &u32) -> u32 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        /// Test u32 values.
        /// Returns the input unchanged.
        pub fn try_test_u32(
            &self,
            v: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        /// Test i32 values.
        pub fn test_i32(&self, v: &i32) -> i32 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        /// Test i32 values.
        pub fn try_test_i32(
            &self,
            v: &i32,
        ) -> Result<
            Result<
                i32,
                <i32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_u64(&self, v: &u64) -> u64 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_u64(
            &self,
            v: &u64,
        ) -> Result<
            Result<
                u64,
                <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_i64(&self, v: &i64) -> i64 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_i64(
            &self,
            v: &i64,
        ) -> Result<
            Result<
                i64,
                <i64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_u128(&self, v: &u128) -> u128 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_u128(
            &self,
            v: &u128,
        ) -> Result<
            Result<
                u128,
                <u128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_i128(&self, v: &i128) -> i128 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_i128(
            &self,
            v: &i128,
        ) -> Result<
            Result<
                i128,
                <i128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_bool(&self, v: &bool) -> bool {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_bool(
            &self,
            v: &bool,
        ) -> Result<
            Result<
                bool,
                <bool as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_address(&self, v: &Address) -> Address {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_address(
            &self,
            v: &Address,
        ) -> Result<
            Result<
                Address,
                <Address as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_bytes(&self, v: &Bytes) -> Bytes {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_bytes(
            &self,
            v: &Bytes,
        ) -> Result<
            Result<
                Bytes,
                <Bytes as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_bytes_n(&self, v: &BytesN<32>) -> BytesN<32> {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_bytes_n(
            &self,
            v: &BytesN<32>,
        ) -> Result<
            Result<
                BytesN<32>,
                <BytesN<32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_string(&self, v: &String) -> String {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_string(
            &self,
            v: &String,
        ) -> Result<
            Result<
                String,
                <String as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_symbol(&self, v: &Symbol) -> Symbol {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_symbol(
            &self,
            v: &Symbol,
        ) -> Result<
            Result<
                Symbol,
                <Symbol as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_vec(&self, v: &Vec<u32>) -> Vec<u32> {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_vec(
            &self,
            v: &Vec<u32>,
        ) -> Result<
            Result<
                Vec<u32>,
                <Vec<u32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_map(&self, v: &Map<u32, u32>) -> Map<u32, u32> {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_map(
            &self,
            v: &Map<u32, u32>,
        ) -> Result<
            Result<
                Map<u32, u32>,
                <Map<
                    u32,
                    u32,
                > as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_duration(&self, v: &Duration) -> Duration {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_duration(
            &self,
            v: &Duration,
        ) -> Result<
            Result<
                Duration,
                <Duration as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_timepoint(&self, v: &Timepoint) -> Timepoint {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_timepoint(
            &self,
            v: &Timepoint,
        ) -> Result<
            Result<
                Timepoint,
                <Timepoint as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_i256(&self, v: &I256) -> I256 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_i256(
            &self,
            v: &I256,
        ) -> Result<
            Result<
                I256,
                <I256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_u256(&self, v: &U256) -> U256 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_u256(
            &self,
            v: &U256,
        ) -> Result<
            Result<
                U256,
                <U256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_env_param(&self) -> u32 {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_env_param(
            &self,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_struct(&self, v: &MyStruct) -> MyStruct {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_struct(
            &self,
            v: &MyStruct,
        ) -> Result<
            Result<
                MyStruct,
                <MyStruct as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_enum_unit(&self, v: &MyEnumUnit) -> MyEnumUnit {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_enum_unit(
            &self,
            v: &MyEnumUnit,
        ) -> Result<
            Result<
                MyEnumUnit,
                <MyEnumUnit as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn test_enum_variants(&self, v: &MyEnumVariants) -> MyEnumVariants {
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_test_enum_variants(
            &self,
            v: &MyEnumVariants,
        ) -> Result<
            Result<
                MyEnumVariants,
                <MyEnumVariants as soroban_sdk::TryFromVal<
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
                &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
                ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        pub fn test_u32<'i>(v: &'i u32) -> (&'i u32,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_i32<'i>(v: &'i i32) -> (&'i i32,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_u64<'i>(v: &'i u64) -> (&'i u64,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_i64<'i>(v: &'i i64) -> (&'i i64,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_u128<'i>(v: &'i u128) -> (&'i u128,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_i128<'i>(v: &'i i128) -> (&'i i128,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_bool<'i>(v: &'i bool) -> (&'i bool,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_address<'i>(v: &'i Address) -> (&'i Address,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_bytes<'i>(v: &'i Bytes) -> (&'i Bytes,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_bytes_n<'i>(v: &'i BytesN<32>) -> (&'i BytesN<32>,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_string<'i>(v: &'i String) -> (&'i String,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_symbol<'i>(v: &'i Symbol) -> (&'i Symbol,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_vec<'i>(v: &'i Vec<u32>) -> (&'i Vec<u32>,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_map<'i>(v: &'i Map<u32, u32>) -> (&'i Map<u32, u32>,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_duration<'i>(v: &'i Duration) -> (&'i Duration,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_timepoint<'i>(v: &'i Timepoint) -> (&'i Timepoint,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_i256<'i>(v: &'i I256) -> (&'i I256,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_u256<'i>(v: &'i U256) -> (&'i U256,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_env_param<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_struct<'i>(v: &'i MyStruct) -> (&'i MyStruct,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_enum_unit<'i>(v: &'i MyEnumUnit) -> (&'i MyEnumUnit,) {
            (v,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn test_enum_variants<'i>(v: &'i MyEnumVariants) -> (&'i MyEnumVariants,) {
            (v,)
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(unused)]
    fn __Contract_AllTypes_959aee9d42336ade92416504111dfbb4e37b0472bbb1e487310c05a170c39d28_ctor() {
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
                        __Contract_AllTypes_959aee9d42336ade92416504111dfbb4e37b0472bbb1e487310c05a170c39d28_ctor();
                    };
                    core::default::Default::default()
                }
                f
            };
        }
        {
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_u32",
                #[allow(deprecated)]
                &__Contract__test_u32::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_i32",
                #[allow(deprecated)]
                &__Contract__test_i32::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_u64",
                #[allow(deprecated)]
                &__Contract__test_u64::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_i64",
                #[allow(deprecated)]
                &__Contract__test_i64::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_u128",
                #[allow(deprecated)]
                &__Contract__test_u128::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_i128",
                #[allow(deprecated)]
                &__Contract__test_i128::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_bool",
                #[allow(deprecated)]
                &__Contract__test_bool::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_address",
                #[allow(deprecated)]
                &__Contract__test_address::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_bytes",
                #[allow(deprecated)]
                &__Contract__test_bytes::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_bytes_n",
                #[allow(deprecated)]
                &__Contract__test_bytes_n::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_string",
                #[allow(deprecated)]
                &__Contract__test_string::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_symbol",
                #[allow(deprecated)]
                &__Contract__test_symbol::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_vec",
                #[allow(deprecated)]
                &__Contract__test_vec::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_map",
                #[allow(deprecated)]
                &__Contract__test_map::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_duration",
                #[allow(deprecated)]
                &__Contract__test_duration::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_timepoint",
                #[allow(deprecated)]
                &__Contract__test_timepoint::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_i256",
                #[allow(deprecated)]
                &__Contract__test_i256::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_u256",
                #[allow(deprecated)]
                &__Contract__test_u256::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_env_param",
                #[allow(deprecated)]
                &__Contract__test_env_param::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_struct",
                #[allow(deprecated)]
                &__Contract__test_struct::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_enum_unit",
                #[allow(deprecated)]
                &__Contract__test_enum_unit::invoke_raw_slice,
            );
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "test_enum_variants",
                #[allow(deprecated)]
                &__Contract__test_enum_variants::invoke_raw_slice,
            );
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(unused)]
    fn __Contract_AllTypes_e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor() {
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
                        __Contract_AllTypes_e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor();
                    };
                    core::default::Default::default()
                }
                f
            };
        }
        {}
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_types"]
    #[doc(hidden)]
    pub const test_types: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_types"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/contracttrait_trait/src/lib.rs",
            start_line: 126usize,
            start_col: 8usize,
            end_line: 126usize,
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
        match (&client.test_u32(&42u32), &42u32) {
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
        match (&client.test_i32(&-42i32), &-42i32) {
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
        match (&client.test_u64(&42u64), &42u64) {
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
        match (&client.test_i64(&-42i64), &-42i64) {
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
        match (&client.test_u128(&42u128), &42u128) {
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
        match (&client.test_i128(&-42i128), &-42i128) {
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
        match (&client.test_bool(&true), &true) {
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
        let addr = Address::generate(&e);
        match (&client.test_address(&addr), &addr) {
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
        let bytes = Bytes::from_slice(&e, &[1, 2, 3]);
        match (&client.test_bytes(&bytes), &bytes) {
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
        let bytes_n = BytesN::from_array(&e, &[0u8; 32]);
        match (&client.test_bytes_n(&bytes_n), &bytes_n) {
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
        let string = String::from_str(&e, "hello");
        match (&client.test_string(&string), &string) {
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
        let symbol = {
            #[allow(deprecated)]
            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test");
            SYMBOL
        };
        match (&client.test_symbol(&symbol), &symbol) {
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
        let vec_val = ::soroban_sdk::Vec::from_array(&e, [1u32, 2u32, 3u32]);
        match (&client.test_vec(&vec_val), &vec_val) {
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
        let map_val = ::soroban_sdk::Map::from_array(&e, [(1u32, 2u32), (3u32, 4u32)]);
        match (&client.test_map(&map_val), &map_val) {
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
        let duration_val = Duration::from_seconds(&e, 100);
        match (&client.test_duration(&duration_val), &duration_val) {
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
        let timepoint_val = Timepoint::from_unix(&e, 100);
        match (&client.test_timepoint(&timepoint_val), &timepoint_val) {
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
        let i256_val = I256::from_i128(&e, 42);
        match (&client.test_i256(&i256_val), &i256_val) {
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
        let u256_val = U256::from_u128(&e, 42);
        match (&client.test_u256(&u256_val), &u256_val) {
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
        match (&client.test_env_param(), &42) {
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
        let my_struct = MyStruct { a: 10, b: 20 };
        match (&client.test_struct(&my_struct), &my_struct) {
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
        match (&client.test_enum_unit(&MyEnumUnit::A), &MyEnumUnit::A) {
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
        let my_enum = MyEnumVariants::VarB(MyStruct { a: 1, b: 2 });
        match (&client.test_enum_variants(&my_enum), &my_enum) {
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
    #[rustc_test_marker = "test::test_spec_docs"]
    #[doc(hidden)]
    pub const test_spec_docs: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_spec_docs"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/contracttrait_trait/src/lib.rs",
            start_line: 189usize,
            start_col: 8usize,
            end_line: 189usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_spec_docs()),
        ),
    };
    fn test_spec_docs() {
        use stellar_xdr::curr as stellar_xdr;
        use stellar_xdr::{Limits, ReadXdr, ScSpecEntry};
        let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_test_u32(), Limits::none()).unwrap();
        let ScSpecEntry::FunctionV0(func) = entry else {
            {
                ::core::panicking::panic_fmt(format_args!("expected FunctionV0"));
            };
        };
        match (
            &func.doc.to_utf8_string().unwrap(),
            &"Test u32 values.\nReturns the input unchanged.",
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
        let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_test_i32(), Limits::none()).unwrap();
        let ScSpecEntry::FunctionV0(func) = entry else {
            {
                ::core::panicking::panic_fmt(format_args!("expected FunctionV0"));
            };
        };
        match (&func.doc.to_utf8_string().unwrap(), &"Test i32 values.") {
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
    test::test_main_static(&[&test_spec_docs, &test_types])
}
