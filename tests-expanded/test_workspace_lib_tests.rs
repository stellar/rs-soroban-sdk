#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::contracttype;
pub struct Value {
    pub value: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Value {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Value", "value", &&self.value)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Value {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Value {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Value {
    #[inline]
    fn eq(&self, other: &Value) -> bool {
        self.value == other.value
    }
}
pub static __SPEC_XDR_TYPE_VALUE: [u8; 48usize] = Value::spec_xdr();
impl Value {
    pub const fn spec_xdr() -> [u8; 48usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x05Value\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x05"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Value {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, Value> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &Value,
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Value> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&Value,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Value>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for Value {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Value {
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
impl TryFrom<&Value> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &Value) -> Result<Self, soroban_sdk::xdr::Error> {
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
impl TryFrom<Value> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: Value) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&Value> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &Value) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<Value> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: Value) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryValue {
        value: <i32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryValue {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ArbitraryValue",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryValue {
        #[inline]
        fn clone(&self) -> ArbitraryValue {
            ArbitraryValue {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryValue {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <i32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryValue {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryValue {
        #[inline]
        fn eq(&self, other: &ArbitraryValue) -> bool {
            self.value == other.value
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryValue {
        #[inline]
        fn cmp(&self, other: &ArbitraryValue) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.value, &other.value)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryValue {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryValue,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.value, &other.value)
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryValue: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryValue {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryValue.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryValue {
                        value: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryValue.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryValue.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryValue {
                        value: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryValue.with(|count| {
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
                            <<i32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Value {
        type Prototype = ArbitraryValue;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryValue> for Value {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryValue,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(Value {
                value: soroban_sdk::IntoVal::into_val(&v.value, env),
            })
        }
    }
};
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
