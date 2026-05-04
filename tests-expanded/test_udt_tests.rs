#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, contracttype, Error, Map, Symbol, Vec};
pub enum UdtEnum2 {
    A = 10,
    B = 15,
}
#[automatically_derived]
impl ::core::marker::Copy for UdtEnum2 {}
#[automatically_derived]
impl ::core::clone::Clone for UdtEnum2 {
    #[inline]
    fn clone(&self) -> UdtEnum2 {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtEnum2 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UdtEnum2::A => "A",
                UdtEnum2::B => "B",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtEnum2 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtEnum2 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtEnum2 {
    #[inline]
    fn eq(&self, other: &UdtEnum2) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
pub static __SPEC_XDR_TYPE_UDTENUM2: [u8; 60usize] = UdtEnum2::spec_xdr();
impl UdtEnum2 {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08UdtEnum2\0\0\0\x02\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\n\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x0f"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for UdtEnum2 {
    const SPEC_TYPE_ID: [u8; 32] = *b"\xaf\xf7\x93\xba\x9eM\xde\x9a?'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTENUM2: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    2,
    *b"\xaf\xf7\x93\xba\x9eM\xde\x9a?'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e",
    [],
);
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtEnum2 {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            10u32 => Self::A,
            15u32 => Self::B,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum2> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtEnum2,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UdtEnum2::A => 10u32.into(),
            UdtEnum2::B => 15u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtEnum2> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtEnum2,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum2>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtEnum2 {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVal,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
            Ok(match *discriminant {
                10u32 => Self::A,
                15u32 => Self::B,
                _ => Err(soroban_sdk::xdr::Error::Invalid)?,
            })
        } else {
            Err(soroban_sdk::xdr::Error::Invalid)
        }
    }
}
impl TryInto<soroban_sdk::xdr::ScVal> for &UdtEnum2 {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
        Ok(match self {
            UdtEnum2::A => 10u32.into(),
            UdtEnum2::B => 15u32.into(),
        })
    }
}
impl TryInto<soroban_sdk::xdr::ScVal> for UdtEnum2 {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
        Ok(match self {
            UdtEnum2::A => 10u32.into(),
            UdtEnum2::B => 15u32.into(),
        })
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryUdtEnum2 {
        A,
        B,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryUdtEnum2 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ArbitraryUdtEnum2::A => "A",
                    ArbitraryUdtEnum2::B => "B",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryUdtEnum2 {
        #[inline]
        fn clone(&self) -> ArbitraryUdtEnum2 {
            match self {
                ArbitraryUdtEnum2::A => ArbitraryUdtEnum2::A,
                ArbitraryUdtEnum2::B => ArbitraryUdtEnum2::B,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryUdtEnum2 {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryUdtEnum2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryUdtEnum2 {
        #[inline]
        fn eq(&self, other: &ArbitraryUdtEnum2) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryUdtEnum2 {
        #[inline]
        fn cmp(&self, other: &ArbitraryUdtEnum2) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryUdtEnum2 {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryUdtEnum2,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryUdtEnum2: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtEnum2 {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
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
                            0u64 => ArbitraryUdtEnum2::A,
                            1u64 => ArbitraryUdtEnum2::B,
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
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
                            0u64 => ArbitraryUdtEnum2::A,
                            1u64 => ArbitraryUdtEnum2::B,
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
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
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtEnum2 {
        type Prototype = ArbitraryUdtEnum2;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtEnum2> for UdtEnum2 {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryUdtEnum2,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryUdtEnum2::A => UdtEnum2::A,
                ArbitraryUdtEnum2::B => UdtEnum2::B,
            })
        }
    }
};
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
    UdtC(UdtEnum2),
    UdtD(UdtTuple),
}
#[automatically_derived]
impl ::core::clone::Clone for UdtEnum {
    #[inline]
    fn clone(&self) -> UdtEnum {
        match self {
            UdtEnum::UdtA => UdtEnum::UdtA,
            UdtEnum::UdtB(__self_0) => UdtEnum::UdtB(::core::clone::Clone::clone(__self_0)),
            UdtEnum::UdtC(__self_0) => UdtEnum::UdtC(::core::clone::Clone::clone(__self_0)),
            UdtEnum::UdtD(__self_0) => UdtEnum::UdtD(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            UdtEnum::UdtA => ::core::fmt::Formatter::write_str(f, "UdtA"),
            UdtEnum::UdtB(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtB", &__self_0)
            }
            UdtEnum::UdtC(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtC", &__self_0)
            }
            UdtEnum::UdtD(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtD", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UdtStruct>;
        let _: ::core::cmp::AssertParamIsEq<UdtEnum2>;
        let _: ::core::cmp::AssertParamIsEq<UdtTuple>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtEnum {
    #[inline]
    fn eq(&self, other: &UdtEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (UdtEnum::UdtB(__self_0), UdtEnum::UdtB(__arg1_0)) => __self_0 == __arg1_0,
                (UdtEnum::UdtC(__self_0), UdtEnum::UdtC(__arg1_0)) => __self_0 == __arg1_0,
                (UdtEnum::UdtD(__self_0), UdtEnum::UdtD(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
pub static __SPEC_XDR_TYPE_UDTENUM: [u8; 156usize] = UdtEnum::spec_xdr();
impl UdtEnum {
    pub const fn spec_xdr() -> [u8; 156usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07UdtEnum\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x04UdtA\0\0\0\x01\0\0\0\0\0\0\0\x04UdtB\0\0\0\x01\0\0\x07\xd0\0\0\0\tUdtStruct\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04UdtC\0\0\0\x01\0\0\x07\xd0\0\0\0\x08UdtEnum2\0\0\0\x01\0\0\0\0\0\0\0\x04UdtD\0\0\0\x01\0\0\x07\xd0\0\0\0\x08UdtTuple"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for UdtEnum {
    const SPEC_TYPE_ID: [u8; 32] = *b"\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTENUM: [u8; 138usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    138usize,
    3usize,
>(
    2,
    *b"\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2",
    [
        <UdtStruct as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        <UdtEnum2 as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        <UdtTuple as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
    ],
);
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["UdtA", "UdtB", "UdtC", "UdtD"];
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
                    Self::UdtA
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::UdtB(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::UdtC(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                3 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::UdtD(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            UdtEnum::UdtA => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"UdtA")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            UdtEnum::UdtB(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"UdtB")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            UdtEnum::UdtC(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"UdtC")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            UdtEnum::UdtD(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"UdtD")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for UdtEnum {
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
            "UdtA" => {
                if iter.len() > 0 {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                Self::UdtA
            }
            "UdtB" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::UdtB(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            "UdtC" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::UdtC(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            "UdtD" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::UdtD(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            _ => Err(soroban_sdk::xdr::Error::Invalid)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtEnum {
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
impl TryFrom<&UdtEnum> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        Ok(match val {
            UdtEnum::UdtA => {
                let symbol = soroban_sdk::xdr::ScSymbol(
                    "UdtA"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                );
                let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                (val,)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
            }
            UdtEnum::UdtB(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "UdtB"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                ),
                value0,
            )
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            UdtEnum::UdtC(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "UdtC"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                ),
                value0,
            )
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            UdtEnum::UdtD(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "UdtD"
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
impl TryFrom<UdtEnum> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&UdtEnum> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
    }
}
impl TryFrom<UdtEnum> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryUdtEnum {
        UdtA,
        UdtB(<UdtStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
        UdtC(<UdtEnum2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
        UdtD(<UdtTuple as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryUdtEnum {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ArbitraryUdtEnum::UdtA => ::core::fmt::Formatter::write_str(f, "UdtA"),
                ArbitraryUdtEnum::UdtB(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtB", &__self_0)
                }
                ArbitraryUdtEnum::UdtC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtC", &__self_0)
                }
                ArbitraryUdtEnum::UdtD(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtD", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryUdtEnum {
        #[inline]
        fn clone(&self) -> ArbitraryUdtEnum {
            match self {
                ArbitraryUdtEnum::UdtA => ArbitraryUdtEnum::UdtA,
                ArbitraryUdtEnum::UdtB(__self_0) => {
                    ArbitraryUdtEnum::UdtB(::core::clone::Clone::clone(__self_0))
                }
                ArbitraryUdtEnum::UdtC(__self_0) => {
                    ArbitraryUdtEnum::UdtC(::core::clone::Clone::clone(__self_0))
                }
                ArbitraryUdtEnum::UdtD(__self_0) => {
                    ArbitraryUdtEnum::UdtD(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryUdtEnum {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <UdtStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <UdtEnum2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <UdtTuple as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryUdtEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryUdtEnum {
        #[inline]
        fn eq(&self, other: &ArbitraryUdtEnum) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (ArbitraryUdtEnum::UdtB(__self_0), ArbitraryUdtEnum::UdtB(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ArbitraryUdtEnum::UdtC(__self_0), ArbitraryUdtEnum::UdtC(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ArbitraryUdtEnum::UdtD(__self_0), ArbitraryUdtEnum::UdtD(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryUdtEnum {
        #[inline]
        fn cmp(&self, other: &ArbitraryUdtEnum) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (ArbitraryUdtEnum::UdtB(__self_0), ArbitraryUdtEnum::UdtB(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (ArbitraryUdtEnum::UdtC(__self_0), ArbitraryUdtEnum::UdtC(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (ArbitraryUdtEnum::UdtD(__self_0), ArbitraryUdtEnum::UdtD(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryUdtEnum {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryUdtEnum,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (ArbitraryUdtEnum::UdtB(__self_0), ArbitraryUdtEnum::UdtB(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ArbitraryUdtEnum::UdtC(__self_0), ArbitraryUdtEnum::UdtC(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (ArbitraryUdtEnum::UdtD(__self_0), ArbitraryUdtEnum::UdtD(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryUdtEnum: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtEnum {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 4u64) >> 32
                        {
                            0u64 => ArbitraryUdtEnum::UdtA,
                            1u64 => ArbitraryUdtEnum::UdtB(arbitrary::Arbitrary::arbitrary(u)?),
                            2u64 => ArbitraryUdtEnum::UdtC(arbitrary::Arbitrary::arbitrary(u)?),
                            3u64 => ArbitraryUdtEnum::UdtD(arbitrary::Arbitrary::arbitrary(u)?),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(
                        match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?) * 4u64)
                            >> 32
                        {
                            0u64 => ArbitraryUdtEnum::UdtA,
                            1u64 => ArbitraryUdtEnum::UdtB(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            2u64 => ArbitraryUdtEnum::UdtC(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            3u64 => ArbitraryUdtEnum::UdtD(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
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
                                            <<UdtStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                depth,
                                            ),
                                        ],
                                    ),
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<UdtEnum2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                depth,
                                            ),
                                        ],
                                    ),
                                    arbitrary::size_hint::and_all(
                                        &[
                                            <<UdtTuple as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtEnum {
        type Prototype = ArbitraryUdtEnum;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtEnum> for UdtEnum {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryUdtEnum,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryUdtEnum::UdtA => UdtEnum::UdtA,
                ArbitraryUdtEnum::UdtB(field_0) => {
                    UdtEnum::UdtB(soroban_sdk::IntoVal::into_val(field_0, env))
                }
                ArbitraryUdtEnum::UdtC(field_0) => {
                    UdtEnum::UdtC(soroban_sdk::IntoVal::into_val(field_0, env))
                }
                ArbitraryUdtEnum::UdtD(field_0) => {
                    UdtEnum::UdtD(soroban_sdk::IntoVal::into_val(field_0, env))
                }
            })
        }
    }
};
pub struct UdtTuple(pub i64, pub Vec<i64>);
#[automatically_derived]
impl ::core::clone::Clone for UdtTuple {
    #[inline]
    fn clone(&self) -> UdtTuple {
        UdtTuple(
            ::core::clone::Clone::clone(&self.0),
            ::core::clone::Clone::clone(&self.1),
        )
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtTuple {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "UdtTuple", &self.0, &&self.1)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtTuple {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
        let _: ::core::cmp::AssertParamIsEq<Vec<i64>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtTuple {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtTuple {
    #[inline]
    fn eq(&self, other: &UdtTuple) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
pub static __SPEC_XDR_TYPE_UDTTUPLE: [u8; 64usize] = UdtTuple::spec_xdr();
impl UdtTuple {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x08UdtTuple\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x011\0\0\0\0\0\x03\xea\0\0\0\x07"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for UdtTuple {
    const SPEC_TYPE_ID: [u8; 32] = *b"\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTTUPLE: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    2,
    *b"\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb",
    [],
);
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtTuple {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
        let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        env.vec_unpack_to_slice(vec, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
            1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtTuple> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtTuple,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        let vals: [Val; 2usize] = [
            (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .vec_new_from_slice(&vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtTuple> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtTuple,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtTuple>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for UdtTuple {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScVec,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let vec = val;
        if vec.len() != 2usize {
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
            1: {
                let rv: soroban_sdk::Val = (&vec[1].clone())
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                rv.try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
            },
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtTuple {
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
impl TryFrom<&UdtTuple> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        Ok(soroban_sdk::xdr::ScVec(
            <[_]>::into_vec(::alloc::boxed::box_new([
                (&val.0)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                (&val.1)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            ]))
            .try_into()?,
        ))
    }
}
impl TryFrom<UdtTuple> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&UdtTuple> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
    }
}
impl TryFrom<UdtTuple> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryUdtTuple(
        <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        <Vec<i64> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    );
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryUdtTuple {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(
                f,
                "ArbitraryUdtTuple",
                &self.0,
                &&self.1,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryUdtTuple {
        #[inline]
        fn clone(&self) -> ArbitraryUdtTuple {
            ArbitraryUdtTuple(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryUdtTuple {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Vec<i64> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryUdtTuple {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryUdtTuple {
        #[inline]
        fn eq(&self, other: &ArbitraryUdtTuple) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryUdtTuple {
        #[inline]
        fn cmp(&self, other: &ArbitraryUdtTuple) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryUdtTuple {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryUdtTuple,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryUdtTuple: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtTuple {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryUdtTuple(
                        arbitrary::Arbitrary::arbitrary(u)?,
                        arbitrary::Arbitrary::arbitrary(u)?,
                    ))
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryUdtTuple(
                        arbitrary::Arbitrary::arbitrary(&mut u)?,
                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    ))
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
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
                            <<Vec<
                                i64,
                            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtTuple {
        type Prototype = ArbitraryUdtTuple;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtTuple> for UdtTuple {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryUdtTuple,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(UdtTuple(
                soroban_sdk::IntoVal::into_val(&v.0, env),
                soroban_sdk::IntoVal::into_val(&v.1, env),
            ))
        }
    }
};
pub struct UdtStruct {
    a: i64,
    b: i64,
    pub c: Vec<i64>,
}
#[automatically_derived]
impl ::core::clone::Clone for UdtStruct {
    #[inline]
    fn clone(&self) -> UdtStruct {
        UdtStruct {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
            c: ::core::clone::Clone::clone(&self.c),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "UdtStruct",
            "a",
            &self.a,
            "b",
            &self.b,
            "c",
            &&self.c,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
        let _: ::core::cmp::AssertParamIsEq<Vec<i64>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtStruct {
    #[inline]
    fn eq(&self, other: &UdtStruct) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}
pub static __SPEC_XDR_TYPE_UDTSTRUCT: [u8; 84usize] = UdtStruct::spec_xdr();
impl UdtStruct {
    pub const fn spec_xdr() -> [u8; 84usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\tUdtStruct\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01c\0\0\0\0\0\x03\xea\0\0\0\x07"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for UdtStruct {
    const SPEC_TYPE_ID: [u8; 32] = *b"\x16'd8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTSTRUCT: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    2,
    *b"\x16'd8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87",
    [],
);
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 3usize] = ["a", "b", "c"];
        let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
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
            c: vals[2]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 3usize] = ["a", "b", "c"];
        let vals: [Val; 3usize] = [
            (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.b).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.c).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtStruct>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for UdtStruct {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::xdr::ScMap,
    ) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::Validate;
        use soroban_sdk::TryIntoVal;
        let map = val;
        if map.len() != 3usize {
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
            c: {
                let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                    "c".try_into()
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtStruct {
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
impl TryFrom<&UdtStruct> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
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
            soroban_sdk::xdr::ScMapEntry {
                key: soroban_sdk::xdr::ScSymbol(
                    "c".try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
                .into(),
                val: (&val.c)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            },
        ])))
    }
}
impl TryFrom<UdtStruct> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&UdtStruct> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<UdtStruct> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryUdtStruct {
        a: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        b: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        c: <Vec<i64> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryUdtStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ArbitraryUdtStruct",
                "a",
                &self.a,
                "b",
                &self.b,
                "c",
                &&self.c,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryUdtStruct {
        #[inline]
        fn clone(&self) -> ArbitraryUdtStruct {
            ArbitraryUdtStruct {
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
                c: ::core::clone::Clone::clone(&self.c),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryUdtStruct {
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
            let _: ::core::cmp::AssertParamIsEq<
                <Vec<i64> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryUdtStruct {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryUdtStruct {
        #[inline]
        fn eq(&self, other: &ArbitraryUdtStruct) -> bool {
            self.a == other.a && self.b == other.b && self.c == other.c
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryUdtStruct {
        #[inline]
        fn cmp(&self, other: &ArbitraryUdtStruct) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.b, &other.b) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.c, &other.c),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryUdtStruct {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryUdtStruct,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.c, &other.c)
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
        const RECURSIVE_COUNT_ArbitraryUdtStruct: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtStruct {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryUdtStruct {
                        a: arbitrary::Arbitrary::arbitrary(u)?,
                        b: arbitrary::Arbitrary::arbitrary(u)?,
                        c: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryUdtStruct {
                        a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        b: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        c: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
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
                            <<Vec<
                                i64,
                            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtStruct {
        type Prototype = ArbitraryUdtStruct;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtStruct> for UdtStruct {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryUdtStruct,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(UdtStruct {
                a: soroban_sdk::IntoVal::into_val(&v.a, env),
                b: soroban_sdk::IntoVal::into_val(&v.b, env),
                c: soroban_sdk::IntoVal::into_val(&v.c, env),
            })
        }
    }
};
pub struct UdtRecursive {
    pub a: Symbol,
    pub b: Vec<UdtRecursive>,
}
#[automatically_derived]
impl ::core::clone::Clone for UdtRecursive {
    #[inline]
    fn clone(&self) -> UdtRecursive {
        UdtRecursive {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UdtRecursive {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UdtRecursive",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UdtRecursive {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<Vec<UdtRecursive>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UdtRecursive {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UdtRecursive {
    #[inline]
    fn eq(&self, other: &UdtRecursive) -> bool {
        self.a == other.a && self.b == other.b
    }
}
pub static __SPEC_XDR_TYPE_UDTRECURSIVE: [u8; 84usize] = UdtRecursive::spec_xdr();
impl UdtRecursive {
    pub const fn spec_xdr() -> [u8; 84usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cUdtRecursive\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x0cUdtRecursive"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for UdtRecursive {
    const SPEC_TYPE_ID: [u8; 32] = *b"\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTRECURSIVE: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    74usize,
    1usize,
>(
    2,
    *b"\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05",
    [<UdtRecursive as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
);
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtRecursive {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtRecursive> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UdtRecursive,
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtRecursive> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UdtRecursive,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtRecursive>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for UdtRecursive {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtRecursive {
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
impl TryFrom<&UdtRecursive> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
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
        ])))
    }
}
impl TryFrom<UdtRecursive> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&UdtRecursive> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<UdtRecursive> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryUdtRecursive {
        a: <Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        b: <Vec<UdtRecursive> as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryUdtRecursive {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ArbitraryUdtRecursive",
                "a",
                &self.a,
                "b",
                &&self.b,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryUdtRecursive {
        #[inline]
        fn clone(&self) -> ArbitraryUdtRecursive {
            ArbitraryUdtRecursive {
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryUdtRecursive {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Vec<
                    UdtRecursive,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryUdtRecursive {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryUdtRecursive {
        #[inline]
        fn eq(&self, other: &ArbitraryUdtRecursive) -> bool {
            self.a == other.a && self.b == other.b
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryUdtRecursive {
        #[inline]
        fn cmp(&self, other: &ArbitraryUdtRecursive) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryUdtRecursive {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryUdtRecursive,
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
        const RECURSIVE_COUNT_ArbitraryUdtRecursive: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtRecursive {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryUdtRecursive {
                        a: arbitrary::Arbitrary::arbitrary(u)?,
                        b: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryUdtRecursive {
                        a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        b: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
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
                            <<Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Vec<
                                UdtRecursive,
                            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtRecursive {
        type Prototype = ArbitraryUdtRecursive;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtRecursive> for UdtRecursive {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryUdtRecursive,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(UdtRecursive {
                a: soroban_sdk::IntoVal::into_val(&v.a, env),
                b: soroban_sdk::IntoVal::into_val(&v.b, env),
            })
        }
    }
};
pub struct RecursiveToEnum {
    pub a: Symbol,
    pub b: Map<u32, RecursiveEnum>,
}
#[automatically_derived]
impl ::core::clone::Clone for RecursiveToEnum {
    #[inline]
    fn clone(&self) -> RecursiveToEnum {
        RecursiveToEnum {
            a: ::core::clone::Clone::clone(&self.a),
            b: ::core::clone::Clone::clone(&self.b),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RecursiveToEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "RecursiveToEnum",
            "a",
            &self.a,
            "b",
            &&self.b,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RecursiveToEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<Map<u32, RecursiveEnum>>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RecursiveToEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RecursiveToEnum {
    #[inline]
    fn eq(&self, other: &RecursiveToEnum) -> bool {
        self.a == other.a && self.b == other.b
    }
}
pub static __SPEC_XDR_TYPE_RECURSIVETOENUM: [u8; 96usize] = RecursiveToEnum::spec_xdr();
impl RecursiveToEnum {
    pub const fn spec_xdr() -> [u8; 96usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fRecursiveToEnum\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x03\xec\0\0\0\x04\0\0\x07\xd0\0\0\0\rRecursiveEnum\0\0\0"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for RecursiveToEnum {
    const SPEC_TYPE_ID: [u8; 32] = *b"\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_RECURSIVETOENUM: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    74usize,
    1usize,
>(
    2,
    *b"\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci",
    [<RecursiveEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
);
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RecursiveToEnum {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveToEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &RecursiveToEnum,
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RecursiveToEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&RecursiveToEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveToEnum>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for RecursiveToEnum {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for RecursiveToEnum {
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
impl TryFrom<&RecursiveToEnum> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        use soroban_sdk::TryFromVal;
        soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
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
        ])))
    }
}
impl TryFrom<RecursiveToEnum> for soroban_sdk::xdr::ScMap {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&RecursiveToEnum> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
    }
}
impl TryFrom<RecursiveToEnum> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub struct ArbitraryRecursiveToEnum {
        a: <Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        b: <Map<
            u32,
            RecursiveEnum,
        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryRecursiveToEnum {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ArbitraryRecursiveToEnum",
                "a",
                &self.a,
                "b",
                &&self.b,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryRecursiveToEnum {
        #[inline]
        fn clone(&self) -> ArbitraryRecursiveToEnum {
            ArbitraryRecursiveToEnum {
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryRecursiveToEnum {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                <Map<
                    u32,
                    RecursiveEnum,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryRecursiveToEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryRecursiveToEnum {
        #[inline]
        fn eq(&self, other: &ArbitraryRecursiveToEnum) -> bool {
            self.a == other.a && self.b == other.b
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryRecursiveToEnum {
        #[inline]
        fn cmp(&self, other: &ArbitraryRecursiveToEnum) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryRecursiveToEnum {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryRecursiveToEnum,
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
        const RECURSIVE_COUNT_ArbitraryRecursiveToEnum: ::std::thread::LocalKey<
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryRecursiveToEnum {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryRecursiveToEnum {
                        a: arbitrary::Arbitrary::arbitrary(u)?,
                        b: arbitrary::Arbitrary::arbitrary(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
                        if count.get() > 0 {
                            return Err(arbitrary::Error::NotEnoughData);
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    })?;
                }
                let result = (|| {
                    Ok(ArbitraryRecursiveToEnum {
                        a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                        b: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                    })
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
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
                            <<Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                            <<Map<
                                u32,
                                RecursiveEnum,
                            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                depth,
                            ),
                        ],
                    )
                })
            }
        }
    };
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for RecursiveToEnum {
        type Prototype = ArbitraryRecursiveToEnum;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryRecursiveToEnum> for RecursiveToEnum {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryRecursiveToEnum,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(RecursiveToEnum {
                a: soroban_sdk::IntoVal::into_val(&v.a, env),
                b: soroban_sdk::IntoVal::into_val(&v.b, env),
            })
        }
    }
};
pub enum RecursiveEnum {
    NotRecursive,
    Recursive(RecursiveToEnum),
}
#[automatically_derived]
impl ::core::clone::Clone for RecursiveEnum {
    #[inline]
    fn clone(&self) -> RecursiveEnum {
        match self {
            RecursiveEnum::NotRecursive => RecursiveEnum::NotRecursive,
            RecursiveEnum::Recursive(__self_0) => {
                RecursiveEnum::Recursive(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RecursiveEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            RecursiveEnum::NotRecursive => ::core::fmt::Formatter::write_str(f, "NotRecursive"),
            RecursiveEnum::Recursive(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Recursive", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for RecursiveEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<RecursiveToEnum>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for RecursiveEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for RecursiveEnum {
    #[inline]
    fn eq(&self, other: &RecursiveEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (RecursiveEnum::Recursive(__self_0), RecursiveEnum::Recursive(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => true,
            }
    }
}
pub static __SPEC_XDR_TYPE_RECURSIVEENUM: [u8; 112usize] = RecursiveEnum::spec_xdr();
impl RecursiveEnum {
    pub const fn spec_xdr() -> [u8; 112usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\rRecursiveEnum\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x0cNotRecursive\0\0\0\x01\0\0\0\0\0\0\0\tRecursive\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fRecursiveToEnum\0"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for RecursiveEnum {
    const SPEC_TYPE_ID: [u8; 32] =
        *b"\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\0M";
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_RECURSIVEENUM: [u8; 74usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<74usize, 1usize>(
        2,
        *b"\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\0M",
        [<RecursiveToEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
    );
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RecursiveEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["NotRecursive", "Recursive"];
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
                    Self::NotRecursive
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Recursive(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &RecursiveEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            RecursiveEnum::NotRecursive => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"NotRecursive")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            RecursiveEnum::Recursive(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Recursive")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RecursiveEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&RecursiveEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveEnum>>::try_from_val(env, *val)
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for RecursiveEnum {
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
            "NotRecursive" => {
                if iter.len() > 0 {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                Self::NotRecursive
            }
            "Recursive" => {
                if iter.len() > 1usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                let rv0: soroban_sdk::Val = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Self::Recursive(
                    rv0.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                )
            }
            _ => Err(soroban_sdk::xdr::Error::Invalid)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for RecursiveEnum {
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
impl TryFrom<&RecursiveEnum> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate alloc;
        Ok(match val {
            RecursiveEnum::NotRecursive => {
                let symbol = soroban_sdk::xdr::ScSymbol(
                    "NotRecursive"
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                );
                let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                (val,)
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
            }
            RecursiveEnum::Recursive(value0) => (
                soroban_sdk::xdr::ScSymbol(
                    "Recursive"
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
impl TryFrom<RecursiveEnum> for soroban_sdk::xdr::ScVec {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
impl TryFrom<&RecursiveEnum> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: &RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
    }
}
impl TryFrom<RecursiveEnum> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    #[inline(always)]
    fn try_from(val: RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
        (&val).try_into()
    }
}
const _: () = {
    use soroban_sdk::testutils::arbitrary::arbitrary;
    use soroban_sdk::testutils::arbitrary::std;
    pub enum ArbitraryRecursiveEnum {
        NotRecursive,
        Recursive(
            <RecursiveToEnum as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        ),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ArbitraryRecursiveEnum {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ArbitraryRecursiveEnum::NotRecursive => {
                    ::core::fmt::Formatter::write_str(f, "NotRecursive")
                }
                ArbitraryRecursiveEnum::Recursive(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Recursive", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ArbitraryRecursiveEnum {
        #[inline]
        fn clone(&self) -> ArbitraryRecursiveEnum {
            match self {
                ArbitraryRecursiveEnum::NotRecursive => ArbitraryRecursiveEnum::NotRecursive,
                ArbitraryRecursiveEnum::Recursive(__self_0) => {
                    ArbitraryRecursiveEnum::Recursive(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ArbitraryRecursiveEnum {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <RecursiveToEnum as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ArbitraryRecursiveEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ArbitraryRecursiveEnum {
        #[inline]
        fn eq(&self, other: &ArbitraryRecursiveEnum) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ArbitraryRecursiveEnum::Recursive(__self_0),
                        ArbitraryRecursiveEnum::Recursive(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ArbitraryRecursiveEnum {
        #[inline]
        fn cmp(&self, other: &ArbitraryRecursiveEnum) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (
                        ArbitraryRecursiveEnum::Recursive(__self_0),
                        ArbitraryRecursiveEnum::Recursive(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ArbitraryRecursiveEnum {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ArbitraryRecursiveEnum,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (
                    ArbitraryRecursiveEnum::Recursive(__self_0),
                    ArbitraryRecursiveEnum::Recursive(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    const _: () = {
        #[allow(non_upper_case_globals)]
        const RECURSIVE_COUNT_ArbitraryRecursiveEnum: ::std::thread::LocalKey<
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
        impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryRecursiveEnum {
            fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self> {
                let guard_against_recursion = u.is_empty();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
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
                            0u64 => ArbitraryRecursiveEnum::NotRecursive,
                            1u64 => ArbitraryRecursiveEnum::Recursive(
                                arbitrary::Arbitrary::arbitrary(u)?,
                            ),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
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
                    RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
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
                            0u64 => ArbitraryRecursiveEnum::NotRecursive,
                            1u64 => ArbitraryRecursiveEnum::Recursive(
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ),
                            _ => {
                                ::core::panicking::panic("internal error: entered unreachable code")
                            }
                        },
                    )
                })();
                if guard_against_recursion {
                    RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
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
                                            <<RecursiveToEnum as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
    impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for RecursiveEnum {
        type Prototype = ArbitraryRecursiveEnum;
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryRecursiveEnum> for RecursiveEnum {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            v: &ArbitraryRecursiveEnum,
        ) -> std::result::Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryRecursiveEnum::NotRecursive => RecursiveEnum::NotRecursive,
                ArbitraryRecursiveEnum::Recursive(field_0) => {
                    RecursiveEnum::Recursive(soroban_sdk::IntoVal::into_val(field_0, env))
                }
            })
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
    pub fn add(a: UdtEnum, b: UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        a + b
    }
    pub fn recursive(a: UdtRecursive) -> Option<UdtRecursive> {
        if a.b.is_empty() {
            None
        } else {
            Some(a.b.first_unchecked())
        }
    }
    pub fn recursive_enum(a: RecursiveEnum, key: u32) -> Result<Option<RecursiveEnum>, Error> {
        match a {
            RecursiveEnum::NotRecursive => Ok(None),
            RecursiveEnum::Recursive(router) => Ok(router.b.get(key)),
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_ADD: [u8; 84usize] = super::Contract::spec_xdr_add();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03add\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\x07\xd0\0\0\0\x07UdtEnum\0\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x07\xd0\0\0\0\x07UdtEnum\0\0\0\0\x01\0\0\0\x07"
    }
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_ADD: [u8; 106usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    106usize,
    2usize,
>(
    0,
    *b"\xeb\xb9m\xe34\x1d[[\xe4K\xe7\xe3\xf4.\x99\x9b\xf2\x1a\xe15\xa1D+\xa8\x1b\x1cV\n\xed\xc1\xa4\x89",
    [
        <UdtEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        <UdtEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
    ],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__recursive__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_RECURSIVE: [u8; 88usize] = super::Contract::spec_xdr_recursive();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_recursive() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\trecursive\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01a\0\0\0\0\0\x07\xd0\0\0\0\x0cUdtRecursive\0\0\0\x01\0\0\x03\xe8\0\0\x07\xd0\0\0\0\x0cUdtRecursive"
    }
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_RECURSIVE: [u8; 106usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<106usize, 2usize>(
        0,
        *b"(`\x83Z;\x970\xd8\xdaZp\xcf\x9e\xbf\x82\x86|0\xb6\x90\x10Mf\x13\xcf\xd76\x0cDn\xdb\xb2",
        [
            <UdtRecursive as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            <UdtRecursive as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        ],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__recursive_enum__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_RECURSIVE_ENUM: [u8; 124usize] =
        super::Contract::spec_xdr_recursive_enum();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_recursive_enum() -> [u8; 124usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0erecursive_enum\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\x07\xd0\0\0\0\rRecursiveEnum\0\0\0\0\0\0\0\0\0\0\x03key\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\x03\xe8\0\0\x07\xd0\0\0\0\rRecursiveEnum\0\0\0\0\0\0\x03"
    }
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_RECURSIVE_ENUM: [u8; 106usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    106usize,
    2usize,
>(
    0,
    *b"\x84H!\x0e\xfc\xdbM6\x02\xaaN\xe4\xee\x99J\x08\x94\x08\xa9\xc0D\x88Ci\xc9\x07~\xb9\xa6\xc5\xec\xaa",
    [
        <RecursiveEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        <RecursiveEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
    ],
);
impl<'a> ContractClient<'a> {
    pub fn add(&self, a: &UdtEnum, b: &UdtEnum) -> i64 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
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
    pub fn try_add(
        &self,
        a: &UdtEnum,
        b: &UdtEnum,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
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
    pub fn recursive(&self, a: &UdtRecursive) -> Option<UdtRecursive> {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("recursive");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_recursive(
        &self,
        a: &UdtRecursive,
    ) -> Result<
        Result<
            Option<UdtRecursive>,
            <Option<
                UdtRecursive,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("recursive");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn recursive_enum(&self, a: &RecursiveEnum, key: &u32) -> Option<RecursiveEnum> {
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
            &{ soroban_sdk::Symbol::new(&self.env, "recursive_enum") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), key.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_recursive_enum(
        &self,
        a: &RecursiveEnum,
        key: &u32,
    ) -> Result<
        Result<
            Option<RecursiveEnum>,
            <Option<RecursiveEnum> as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "recursive_enum") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), key.into_val(&self.env)],
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
    pub fn add<'i>(a: &'i UdtEnum, b: &'i UdtEnum) -> (&'i UdtEnum, &'i UdtEnum) {
        (a, b)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn recursive<'i>(a: &'i UdtRecursive) -> (&'i UdtRecursive,) {
        (a,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn recursive_enum<'i>(a: &'i RecursiveEnum, key: &'i u32) -> (&'i RecursiveEnum, &'i u32) {
        (a, key)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
#[allow(deprecated)]
pub fn __Contract__add__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::add(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
pub fn __Contract__add__invoke_raw_slice(
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
    __Contract__add__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
pub extern "C" fn __Contract__add__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__add__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive` instead")]
#[allow(deprecated)]
pub fn __Contract__recursive__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::recursive(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive` instead")]
pub fn __Contract__recursive__invoke_raw_slice(
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
    __Contract__recursive__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive` instead")]
pub extern "C" fn __Contract__recursive__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__recursive__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive_enum` instead")]
#[allow(deprecated)]
pub fn __Contract__recursive_enum__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::recursive_enum(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive_enum` instead")]
pub fn __Contract__recursive_enum__invoke_raw_slice(
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
    __Contract__recursive_enum__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).recursive_enum` instead")]
pub extern "C" fn __Contract__recursive_enum__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__recursive_enum__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____ca538446cb58e8272191ab7091913766c05361045f91847da7c92c7de8846af4_ctor() {
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
                    __Contract____ca538446cb58e8272191ab7091913766c05361045f91847da7c92c7de8846af4_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "add",
            #[allow(deprecated)]
            &__Contract__add__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "recursive",
            #[allow(deprecated)]
            &__Contract__recursive__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "recursive_enum",
            #[allow(deprecated)]
            &__Contract__recursive_enum__invoke_raw_slice,
        );
    }
}
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, vec, xdr::ScVal, Bytes, Env, TryFromVal};
    extern crate test;
    #[rustc_test_marker = "test::test_serializing"]
    #[doc(hidden)]
    pub const test_serializing: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_serializing"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 96usize,
            start_col: 8usize,
            end_line: 96usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_serializing()),
        ),
    };
    fn test_serializing() {
        use soroban_sdk::xdr::ToXdr;
        let e = Env::default();
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: ::soroban_sdk::Vec::from_array(&e, [1]),
        };
        let bin = udt.to_xdr(&e);
        let expected_bytes = [
            0u8, 0, 0, 17, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 15, 0, 0, 0, 1, 97, 0, 0, 0, 0, 0, 0,
            6, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 15, 0, 0, 0, 1, 98, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0,
            0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 1, 99, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let expected_bytes = Bytes::from_array(&e, &expected_bytes);
        match (&bin, &expected_bytes) {
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
    #[rustc_test_marker = "test::test_add"]
    #[doc(hidden)]
    pub const test_add: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_add"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 116usize,
            start_col: 8usize,
            end_line: 116usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_add()),
        ),
    };
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: ::soroban_sdk::Vec::from_array(&e, [1]),
        };
        let z = client.add(&UdtEnum::UdtA, &UdtEnum::UdtB(udt));
        match (&z, &22) {
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
        let udt1 = UdtEnum2::A;
        let udt2 = UdtTuple(1, ::soroban_sdk::Vec::from_array(&e, [2, 3]));
        let z = client.add(&UdtEnum::UdtC(udt1), &UdtEnum::UdtD(udt2));
        match (&z, &16) {
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
    #[rustc_test_marker = "test::test_scval_accessibility_from_udt_types"]
    #[doc(hidden)]
    pub const test_scval_accessibility_from_udt_types: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_scval_accessibility_from_udt_types"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 136usize,
            start_col: 8usize,
            end_line: 136usize,
            end_col: 47usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_scval_accessibility_from_udt_types()),
        ),
    };
    fn test_scval_accessibility_from_udt_types() {
        let e = Env::default();
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: ::soroban_sdk::Vec::from_array(&e, [1]),
        };
        let val: ScVal = udt.clone().try_into().unwrap();
        let roundtrip = UdtStruct::try_from_val(&e, &val).unwrap();
        match (&udt, &roundtrip) {
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
    #[rustc_test_marker = "test::test_recursive"]
    #[doc(hidden)]
    pub const test_recursive: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_recursive"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 149usize,
            start_col: 8usize,
            end_line: 149usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_recursive()),
        ),
    };
    fn test_recursive() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let recursive_udt_0 = UdtRecursive {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("0");
                SYMBOL
            },
            b: ::soroban_sdk::Vec::new(&e),
        };
        let recursive_udt_1 = UdtRecursive {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("1");
                SYMBOL
            },
            b: ::soroban_sdk::Vec::from_array(&e, [recursive_udt_0.clone()]),
        };
        let recursive_udt_2 = UdtRecursive {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("2");
                SYMBOL
            },
            b: ::soroban_sdk::Vec::from_array(&e, [recursive_udt_1.clone()]),
        };
        let result_0 = client.recursive(&recursive_udt_2);
        match (&result_0, &Some(recursive_udt_1)) {
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
        let result_1 = client.recursive(&result_0.unwrap());
        match (&result_1, &Some(recursive_udt_0)) {
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
        let result_2 = client.recursive(&result_1.unwrap());
        match (&result_2, &None) {
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
    #[rustc_test_marker = "test::test_recursive_enum"]
    #[doc(hidden)]
    pub const test_recursive_enum: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_recursive_enum"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 178usize,
            start_col: 8usize,
            end_line: 178usize,
            end_col: 27usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_recursive_enum()),
        ),
    };
    fn test_recursive_enum() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let entry = RecursiveEnum::Recursive(RecursiveToEnum {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test");
                SYMBOL
            },
            b: Map::from_array(&e, [(42u32, RecursiveEnum::NotRecursive)]),
        });
        let result = client.recursive_enum(&entry, &42);
        match (&result, &Some(RecursiveEnum::NotRecursive)) {
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
        let none_result = client.recursive_enum(&entry, &43);
        match (&none_result, &None) {
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
mod test_with_wasm {
    use soroban_sdk::{symbol_short, vec, Env, Map};
    mod contract {
        pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01N\r`\x01~\x01~`\x03~~~\x01~`\x02~~\x01~`\x04~~~~\x01~`\x02\x7f~\x00`\x02\x7f\x7f\x00`\x03~\x7f\x7f\x01~`\x02\x7f\x7f\x01\x7f`\x05~\x7f\x7f\x7f\x7f\x00`\x00\x00`\x01\x7f\x01~`\x03\x7f\x7f\x7f\x00`\x02\x7f\x7f\x01~\x02O\r\x01v\x013\x00\x00\x01v\x01h\x00\x01\x01i\x012\x00\x00\x01v\x011\x00\x02\x01i\x011\x00\x00\x01v\x018\x00\x00\x01m\x014\x00\x02\x01m\x011\x00\x02\x01v\x01g\x00\x02\x01b\x01j\x00\x02\x01m\x019\x00\x01\x01m\x01a\x00\x03\x01b\x01m\x00\x01\x03\x12\x11\x04\x05\x06\x07\x08\x04\x04\x04\x05\x02\t\x00\n\x02\x0b\x0c\t\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x84\x81\xc0\x00\x0b\x7f\x00A\x84\x81\xc0\x00\x0b\x7f\x00A\x90\x81\xc0\x00\x0b\x07L\x07\x06memory\x02\x00\x03add\x00\x16\trecursive\x00\x18\x0erecursive_enum\x00\x1a\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xaa\x1a\x11\xb6\x06\x04\x01\x7f\x01~\x02\x7f\x01~#\x80\x80\x80\x80\x00A\xc0\x00k\"\x02$\x80\x80\x80\x80\x00\x02@\x02@ \x01B\xff\x01\x83B\xcb\x00Q\r\x00 \x00A\x04:\x00\x00\x0c\x01\x0b \x01\x10\x80\x80\x80\x80\x00!\x03 \x02A\x006\x02\x10 \x02 \x017\x03\x08 \x02 \x03B \x88>\x02\x14 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00\x02@\x02@\x02@\x02@ \x02)\x03\x18\"\x01B\x02Q\r\x00 \x01\xa7A\x01q\r\x00\x02@ \x02)\x03 \"\x01\xa7A\xff\x01q\"\x04A\xca\x00F\r\x00 \x04A\x0eG\r\x01\x0b\x02@\x02@\x02@\x02@ \x01A\x90\x80\xc0\x80\x00A\x04\x10\x8f\x80\x80\x80\x00B \x88\xa7\x0e\x04\x00\x01\x02\x03\x05\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00\r\x04A\x00!\x05\x0c\x05\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x03 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x01B\x02Q\r\x03 \x01\xa7A\x01q\r\x03 \x02)\x03 !\x01A\x00!\x04\x02@\x03@ \x04A\x18F\r\x01 \x02A\x18j \x04jB\x027\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b \x01B\xff\x01\x83B\xcc\x00R\r\x03 \x01A\xb4\x80\xc0\x80\x00A\x03 \x02A\x18jA\x03\x10\x91\x80\x80\x80\x00 \x02A0j \x02)\x03\x18\x10\x92\x80\x80\x80\x00 \x02(\x020\r\x03 \x02)\x038!\x03 \x02A0j \x02)\x03 \x10\x92\x80\x80\x80\x00 \x02(\x020\r\x03 \x02)\x03(\"\x06B\xff\x01\x83B\xcb\x00R\r\x03 \x02)\x038!\x01A\x01!\x05\x0c\x04\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x02 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x01B\x02Q\r\x02 \x01\xa7A\x01q\r\x02 \x02)\x03 \"\x01B\xff\x01\x83B\x04R\r\x02A\nA\x0fA\t \x01B \x88\xa7\"\x04A\x0fF\x1b \x04A\nF\x1b\"\x04A\tF\r\x02A\x02!\x05\x0c\x04\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x01 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x01B\x02Q\r\x01 \x01\xa7A\x01q\r\x01 \x02)\x03 \"\x01B\xff\x01\x83B\xcb\x00R\r\x01A\x00!\x04\x02@\x03@ \x04A\x10F\r\x01 \x02A0j \x04jB\x027\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b \x01 \x02A0j\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x81\x80\x80\x80\x00\x1a \x02A\x18j \x02)\x030\x10\x92\x80\x80\x80\x00 \x02(\x02\x18A\x01F\r\x01 \x02)\x038\"\x01B\xff\x01\x83B\xcb\x00R\r\x01 \x02)\x03 !\x03A\x03!\x05\x0c\x02\x0b \x00A\x04:\x00\x00\x0c\x03\x0b \x00A\x04:\x00\x00\x0c\x02\x0b\x0b \x00 \x067\x03\x18 \x00 \x017\x03\x10 \x00 \x037\x03\x08 \x00 \x04:\x00\x01 \x00 \x05:\x00\x00\x0b \x02A\xc0\x00j$\x80\x80\x80\x80\x00\x0bJ\x02\x01~\x01\x7fB\x02!\x02\x02@ \x01(\x02\x08\"\x03 \x01(\x02\x0cO\r\x00 \x00 \x01)\x03\x00 \x03\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x007\x03\x08 \x01 \x03A\x01j6\x02\x08B\x00!\x02\x0b \x00 \x027\x03\x00\x0b\x1c\x00 \x00 \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x8c\x80\x80\x80\x00\x0b\x19\x00\x02@ \x01 \x00I\r\x00 \x01 \x00k\x0f\x0b\x10\x97\x80\x80\x80\x00\x00\x0b1\x00\x02@ \x02 \x04F\r\x00\x00\x0b \x00 \x01\xadB \x86B\x04\x84 \x03\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x8b\x80\x80\x80\x00\x1a\x0b]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc1\x00F\r\x00\x02@ \x02A\x07F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x87!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x82\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0b\xb5\x01\x02\x02\x7f\x02~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00A\x00!\x03\x02@\x03@ \x03A\x10F\r\x01 \x02 \x03jB\x027\x03\x00 \x03A\x08j!\x03\x0c\x00\x0b\x0bB\x01!\x04\x02@ \x01B\xff\x01\x83B\xcc\x00R\r\x00 \x01A\xcc\x80\xc0\x80\x00A\x02 \x02A\x02\x10\x91\x80\x80\x80\x00\x02@ \x02)\x03\x00\"\x01\xa7A\xff\x01q\"\x03A\xca\x00F\r\x00 \x03A\x0eG\r\x01\x0b \x02)\x03\x08\"\x05B\xff\x01\x83B\xcb\x00R\r\x00 \x00 \x057\x03\x10 \x00 \x017\x03\x08B\x00!\x04\x0b \x00 \x047\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0b\xb5\x01\x02\x02\x7f\x02~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00A\x00!\x03\x02@\x03@ \x03A\x10F\r\x01 \x02 \x03jB\x027\x03\x00 \x03A\x08j!\x03\x0c\x00\x0b\x0bB\x01!\x04\x02@ \x01B\xff\x01\x83B\xcc\x00R\r\x00 \x01A\xcc\x80\xc0\x80\x00A\x02 \x02A\x02\x10\x91\x80\x80\x80\x00\x02@ \x02)\x03\x00\"\x01\xa7A\xff\x01q\"\x03A\xca\x00F\r\x00 \x03A\x0eG\r\x01\x0b \x02)\x03\x08\"\x05B\xff\x01\x83B\xcc\x00R\r\x00 \x00 \x057\x03\x10 \x00 \x017\x03\x08B\x00!\x04\x0b \x00 \x047\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0bx\x03\x01\x7f\x01~\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00B\x02!\x03\x02@ \x01(\x02\x08\"\x04 \x01(\x02\x0cO\r\x00 \x02 \x01)\x03\x00 \x04\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x00\x10\x92\x80\x80\x80\x00 \x02)\x03\x00!\x03 \x00 \x02)\x03\x087\x03\x08 \x01 \x04A\x01j6\x02\x08\x0b \x00 \x037\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0b\xb7\x04\x04\x02\x7f\x02~\x01\x7f\x05~#\x80\x80\x80\x80\x00A0k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x8d\x80\x80\x80\x00\x02@\x02@\x02@ \x02-\x00\x00\"\x03A\x04F\r\x00 \x02)\x03\x10!\x00 \x02)\x03\x08!\x04 \x021\x00\x01!\x05 \x02 \x01\x10\x8d\x80\x80\x80\x00 \x02-\x00\x00\"\x06A\x04F\r\x00 \x02)\x03\x10!\x07 \x02)\x03\x08!\x08 \x021\x00\x01!\tB\x00!\nB\x00!\x0b\x02@\x02@\x02@\x02@ \x03\x0e\x04\x05\x02\x01\x00\x05\x0b \x00\x10\x80\x80\x80\x80\x00!\x01 \x02A\x006\x02( \x02 \x007\x03  \x02 \x01B \x88>\x02,B\x00!\x00\x03@ \x02 \x02A j\x10\x95\x80\x80\x80\x00 \x02)\x03\x00\"\x01B\x02Q\r\x03 \x01\xa7A\x01q\r\x06 \x02)\x03\x08\"\x01B\x00S \x00 \x01|\"\x01 \x00SG\r\x06 \x01!\x00\x0c\x00\x0b\x0b \x05!\x0b\x0c\x03\x0b \x00B\x00S \x04 \x00|\"\x0b \x04SsE\r\x02\x0c\x03\x0b \x00B\x00S \x04 \x00|\"\x0b \x04SsE\r\x01\x0c\x02\x0b\x00\x0b\x02@\x02@\x02@\x02@\x02@ \x06\x0e\x04\x04\x02\x01\x00\x04\x0b \x07\x10\x80\x80\x80\x80\x00!\x00 \x02A\x006\x02( \x02 \x077\x03  \x02 \x00B \x88>\x02,B\x00!\x00\x03@ \x02 \x02A j\x10\x95\x80\x80\x80\x00 \x02)\x03\x00\"\x01B\x02Q\r\x03 \x01\xa7A\x01q\r\x05 \x02)\x03\x08\"\x01B\x00S \x00 \x01|\"\x01 \x00SG\r\x05 \x01!\x00\x0c\x00\x0b\x0b \t!\n\x0c\x02\x0b \x07B\x00S \x08 \x07|\"\n \x08Ss\r\x02\x0c\x01\x0b \x00B\x00S \x08 \x00|\"\n \x08Ss\r\x01\x0b \nB\x00S \x0b \n|\"\x00 \x0bSs\r\x00\x02@\x02@ \x00B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00B\x08\x86B\x07\x84!\x00\x0c\x01\x0b \x00\x10\x84\x80\x80\x80\x00!\x00\x0b \x02A0j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x10\x97\x80\x80\x80\x00\x00\x0b\t\x00\x10\x9d\x80\x80\x80\x00\x00\x0b\x9f\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x01$\x80\x80\x80\x80\x00 \x01A\x08j \x00\x10\x93\x80\x80\x80\x00\x02@ \x01(\x02\x08A\x01F\r\x00B\x02!\x00\x02@ \x01)\x03\x18\"\x02\x10\x80\x80\x80\x80\x00B\x80\x80\x80\x80\x10T\r\x00 \x01A\x08j \x02\x10\x85\x80\x80\x80\x00\x10\x93\x80\x80\x80\x00 \x01(\x02\x08A\x01F\r\x01 \x01)\x03\x10!\x00 \x01 \x01)\x03\x187\x03\x10 \x01 \x007\x03\x08 \x01A\x08j\x10\x99\x80\x80\x80\x00!\x00\x0b \x01A j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b$\x00A\xcc\x80\xc0\x80\x00\xadB \x86B\x04\x84 \x00\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x8a\x80\x80\x80\x00\x0b\xf6\x05\x03\x01\x7f\x01~\x01\x7f#\x80\x80\x80\x80\x00A\xc0\x00k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00 \x00\x10\x80\x80\x80\x80\x00!\x03 \x02A\x006\x02\x10 \x02 \x007\x03\x08 \x02 \x03B \x88>\x02\x14 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x00B\x02Q\r\x00 \x00\xa7A\x01q\r\x00\x02@ \x02)\x03 \"\x00\xa7A\xff\x01q\"\x04A\xca\x00F\r\x00 \x04A\x0eG\r\x01\x0b\x02@\x02@\x02@\x02@\x02@\x02@ \x00A\xf4\x80\xc0\x80\x00A\x02\x10\x8f\x80\x80\x80\x00B \x88\xa7\x0e\x02\x01\x00\x06\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x05 \x02A0j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x030\"\x00B\x02Q\r\x05 \x00\xa7A\x01q\r\x05 \x02A\x18j \x02)\x038\x10\x94\x80\x80\x80\x00 \x02(\x02\x18A\x01F\r\x05 \x01B\xff\x01\x83B\x04R\r\x05 \x02)\x03(\"\x00 \x01B\x84\x80\x80\x80p\x83\"\x01\x10\x86\x80\x80\x80\x00B\x01R\r\x01 \x00 \x01\x10\x87\x80\x80\x80\x00\"\x00B\xff\x01\x83B\xcb\x00R\r\x05 \x00\x10\x80\x80\x80\x80\x00!\x01 \x02A\x006\x02\x10 \x02 \x007\x03\x08 \x02 \x01B \x88>\x02\x14 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x00B\x02Q\r\x05 \x00\xa7A\x01q\r\x05\x02@ \x02)\x03 \"\x00\xa7A\xff\x01q\"\x04A\xca\x00F\r\x00 \x04A\x0eG\r\x06\x0b \x00A\xf4\x80\xc0\x80\x00A\x02\x10\x8f\x80\x80\x80\x00B \x88\xa7\x0e\x02\x03\x02\x05\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00\r\x04 \x01B\xff\x01\x83B\x04R\r\x04\x0bB\x02!\x00\x0c\x02\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x02 \x02A0j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x030\"\x00B\x02Q\r\x02 \x00\xa7A\x01q\r\x02 \x02A\x18j \x02)\x038\x10\x94\x80\x80\x80\x00 \x02(\x02\x18A\x01F\r\x02 \x02)\x03(!\x00 \x02)\x03 !\x01 \x02A\x18jA\xe8\x80\xc0\x80\x00A\t\x10\x9b\x80\x80\x80\x00 \x02(\x02\x18\r\x02 \x02)\x03 !\x03 \x02 \x007\x03  \x02 \x017\x03\x18 \x02 \x02A\x18j\x10\x99\x80\x80\x80\x007\x03  \x02 \x037\x03\x18 \x02A\x18jA\x02\x10\x9c\x80\x80\x80\x00!\x00\x0c\x01\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00\r\x01 \x02A\x18jA\xdc\x80\xc0\x80\x00A\x0c\x10\x9b\x80\x80\x80\x00 \x02(\x02\x18\r\x01 \x02 \x02)\x03 7\x03\x18 \x02A\x18jA\x01\x10\x9c\x80\x80\x80\x00!\x00\x0b \x02A\xc0\x00j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\xd6\x01\x02\x01~\x03\x7f\x02@\x02@ \x02A\tK\r\x00B\x00!\x03A\x00!\x04\x03@\x02@ \x04A\tG\r\x00 \x03B\x08\x86B\x0e\x84!\x03\x0c\x03\x0bA\x01!\x05\x02@ \x01 \x04j-\x00\x00\"\x06A\xdf\x00F\r\x00\x02@\x02@ \x06APjA\xff\x01qA\nI\r\x00 \x06A\xbf\x7fjA\xff\x01qA\x1aI\r\x01 \x06A\x9f\x7fjA\xff\x01qA\x1aO\r\x04 \x06AEj!\x05\x0c\x02\x0b \x06ARj!\x05\x0c\x01\x0b \x06AKj!\x05\x0b \x03B\x06\x86 \x05\xadB\xff\x01\x83\x84!\x03 \x04A\x01j!\x04\x0c\x00\x0b\x0b \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x89\x80\x80\x80\x00!\x03\x0b \x00B\x007\x03\x00 \x00 \x037\x03\x08\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x88\x80\x80\x80\x00\x0b\x03\x00\x00\x0b\x0b\x8e\x01\x01\x00A\x80\x80\xc0\x00\x0b\x84\x01UdtAUdtBUdtCUdtD\x00\x00\x10\x00\x04\x00\x00\x00\x04\x00\x10\x00\x04\x00\x00\x00\x08\x00\x10\x00\x04\x00\x00\x00\x0c\x00\x10\x00\x04\x00\x00\x00abc\x000\x00\x10\x00\x01\x00\x00\x001\x00\x10\x00\x01\x00\x00\x002\x00\x10\x00\x01\x00\x00\x000\x00\x10\x00\x01\x00\x00\x001\x00\x10\x00\x01\x00\x00\x00NotRecursiveRecursive\x00\x00\x00\\\x00\x10\x00\x0c\x00\x00\x00h\x00\x10\x00\t\x00\x00\x00\x00\xdb\x1c\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03add\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x07UdtEnum\x00\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x07UdtEnum\x00\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07UdtEnum\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04UdtA\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04UdtB\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\tUdtStruct\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04UdtC\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x08UdtEnum2\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04UdtD\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x08UdtTuple\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08UdtEnum2\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01A\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x01B\x00\x00\x00\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08UdtTuple\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x011\x00\x00\x00\x00\x00\x03\xea\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\tUdtStruct\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x01c\x00\x00\x00\x00\x00\x03\xea\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\trecursive\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x01\x00\x00\x03\xe8\x00\x00\x07\xd0\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cNotRecursive\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\tRecursive\x00\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fRecursiveToEnum\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fRecursiveToEnum\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x03\xec\x00\x00\x00\x04\x00\x00\x07\xd0\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0erecursive_enum\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03key\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x03\xe8\x00\x00\x07\xd0\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x07Context\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nExecutable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\xd1\x0b\x1ccontractspecv0.rssdk.graphv0SpGrV\x01\x00\x02\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2\x00\x03\x16\'d8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87\xaf\xf7\x93\xba\x9eM\xde\x9a?\'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xebSpGrV\x01\x00\x02\xaf\xf7\x93\xba\x9eM\xde\x9a?\'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e\x00\x00SpGrV\x01\x00\x02\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb\x00\x00SpGrV\x01\x00\x02\x16\'d8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87\x00\x00SpGrV\x01\x00\x00\xeb\xb9m\xe34\x1d[[\xe4K\xe7\xe3\xf4.\x99\x9b\xf2\x1a\xe15\xa1D+\xa8\x1b\x1cV\n\xed\xc1\xa4\x89\x00\x02\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2SpGrV\x01\x00\x02\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05\x00\x01\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05SpGrV\x01\x00\x02\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\x00\x01\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97CiSpGrV\x01\x00\x02\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci\x00\x01\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00MSpGrV\x01\x00\x00(`\x83Z;\x970\xd8\xdaZp\xcf\x9e\xbf\x82\x86|0\xb6\x90\x10Mf\x13\xcf\xd76\x0cDn\xdb\xb2\x00\x02\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05SpGrV\x01\x00\x00\x84H!\x0e\xfc\xdbM6\x02\xaaN\xe4\xee\x99J\x08\x94\x08\xa9\xc0D\x88Ci\xc9\x07~\xb9\xa6\xc5\xec\xaa\x00\x02\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00MSpGrV\x01\x00\x02\xa3J\xcf\xf7D\x93\x0bB]\x95\xeb\xfe\x03y\x83e5\\\x16\xeb\x94Ne\xe6Xw\x1f&\xf7\xc0pT\x00\x03\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xec\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9ns\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xafSpGrV\x01\x00\x02\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xec\x00\x00SpGrV\x01\x00\x02\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xcc\x00\x00SpGrV\x01\x00\x02\x9e)H\x8e\xf0\x01{{\xce\x9fdO\x0eD\xc0,\x0f\xe8\xee\'\x845r\x9f\xeb`\xd0\x12H\x17\x96g\x00\x02\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xecULqD\xd3\xfa:\x1f\x0c\xa5\xb7\x04\xe5H\x8b\x91J\x9e\x0fe\x7f\x9f[\xdbG#\xc7o\xb0\xf4\xcf\xe6SpGrV\x01\x00\x02ULqD\xd3\xfa:\x1f\x0c\xa5\xb7\x04\xe5H\x8b\x91J\x9e\x0fe\x7f\x9f[\xdbG#\xc7o\xb0\xf4\xcf\xe6\x00\x03\x9e)H\x8e\xf0\x01{{\xce\x9fdO\x0eD\xc0,\x0f\xe8\xee\'\x845r\x9f\xeb`\xd0\x12H\x17\x96g\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9ns\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xafSpGrV\x01\x00\x02\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9n\x00\x01\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xccSpGrV\x01\x00\x02s\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xaf\x00\x01\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xccSpGrV\x01\x00\x02L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e\x00\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
        pub trait Contract {
            fn add(env: soroban_sdk::Env, a: UdtEnum, b: UdtEnum) -> i64;
            fn recursive(env: soroban_sdk::Env, a: UdtRecursive) -> Option<UdtRecursive>;
            fn recursive_enum(
                env: soroban_sdk::Env,
                a: RecursiveEnum,
                key: u32,
            ) -> Result<Option<RecursiveEnum>, soroban_sdk::Error>;
        }
        ///Client is a client for calling the contract defined in "Contract".
        pub struct Client<'a> {
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
        impl<'a> Client<'a> {
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
            pub fn set_auths(
                &self,
                auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry],
            ) -> Self {
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
            pub fn mock_auths(
                &self,
                mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>],
            ) -> Self {
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
        impl<'a> Client<'a> {
            pub fn add(&self, a: &UdtEnum, b: &UdtEnum) -> i64 {
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
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
            pub fn try_add(
                &self,
                a: &UdtEnum,
                b: &UdtEnum,
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
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
            pub fn recursive(&self, a: &UdtRecursive) -> Option<UdtRecursive> {
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("recursive");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn try_recursive(
                &self,
                a: &UdtRecursive,
            ) -> Result<
                Result<
                    Option<UdtRecursive>,
                    <Option<UdtRecursive> as soroban_sdk::TryFromVal<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::Error,
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("recursive");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn recursive_enum(&self, a: &RecursiveEnum, key: &u32) -> Option<RecursiveEnum> {
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
                    &{ soroban_sdk::Symbol::new(&self.env, "recursive_enum") },
                    ::soroban_sdk::Vec::from_array(
                        &self.env,
                        [a.into_val(&self.env), key.into_val(&self.env)],
                    ),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn try_recursive_enum(
                &self,
                a: &RecursiveEnum,
                key: &u32,
            ) -> Result<
                Result<
                    Option<RecursiveEnum>,
                    <Option<RecursiveEnum> as soroban_sdk::TryFromVal<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::Error,
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
                    &{ soroban_sdk::Symbol::new(&self.env, "recursive_enum") },
                    ::soroban_sdk::Vec::from_array(
                        &self.env,
                        [a.into_val(&self.env), key.into_val(&self.env)],
                    ),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
        }
        ///Args is a type for building arg lists for functions defined in "Contract".
        pub struct Args;
        impl Args {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn add<'i>(a: &'i UdtEnum, b: &'i UdtEnum) -> (&'i UdtEnum, &'i UdtEnum) {
                (a, b)
            }
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn recursive<'i>(a: &'i UdtRecursive) -> (&'i UdtRecursive,) {
                (a,)
            }
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn recursive_enum<'i>(
                a: &'i RecursiveEnum,
                key: &'i u32,
            ) -> (&'i RecursiveEnum, &'i u32) {
                (a, key)
            }
        }
        pub struct UdtTuple(pub i64, pub soroban_sdk::Vec<i64>);
        #[automatically_derived]
        impl ::core::fmt::Debug for UdtTuple {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(f, "UdtTuple", &self.0, &&self.1)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UdtTuple {
            #[inline]
            fn clone(&self) -> UdtTuple {
                UdtTuple(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for UdtTuple {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i64>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<i64>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UdtTuple {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UdtTuple {
            #[inline]
            fn eq(&self, other: &UdtTuple) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for UdtTuple {
            #[inline]
            fn cmp(&self, other: &UdtTuple) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for UdtTuple {
            #[inline]
            fn partial_cmp(
                &self,
                other: &UdtTuple,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_UDTTUPLE: [u8; 64usize] = UdtTuple::spec_xdr();
        impl UdtTuple {
            pub const fn spec_xdr() -> [u8; 64usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x08UdtTuple\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x011\0\0\0\0\0\x03\xea\0\0\0\x07"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for UdtTuple {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTTUPLE: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtTuple {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
                let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
                let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
                env.vec_unpack_to_slice(vec, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
                    1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtTuple> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &UdtTuple,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                let vals: [Val; 2usize] = [
                    (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
                    (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .vec_new_from_slice(&vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtTuple> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&UdtTuple,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtTuple>>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for UdtTuple {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVec,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let vec = val;
                if vec.len() != 2usize {
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
                    1: {
                        let rv: soroban_sdk::Val = (&vec[1].clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtTuple {
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
        impl TryFrom<&UdtTuple> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                Ok(soroban_sdk::xdr::ScVec(
                    <[_]>::into_vec(::alloc::boxed::box_new([
                        (&val.0)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        (&val.1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ]))
                    .try_into()?,
                ))
            }
        }
        impl TryFrom<UdtTuple> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&UdtTuple> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<UdtTuple> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtTuple) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryUdtTuple(
                <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                <soroban_sdk::Vec<
                    i64,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            );
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryUdtTuple {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "ArbitraryUdtTuple",
                        &self.0,
                        &&self.1,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryUdtTuple {
                #[inline]
                fn clone(&self) -> ArbitraryUdtTuple {
                    ArbitraryUdtTuple(
                        ::core::clone::Clone::clone(&self.0),
                        ::core::clone::Clone::clone(&self.1),
                    )
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryUdtTuple {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            i64,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryUdtTuple {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryUdtTuple {
                #[inline]
                fn eq(&self, other: &ArbitraryUdtTuple) -> bool {
                    self.0 == other.0 && self.1 == other.1
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryUdtTuple {
                #[inline]
                fn cmp(&self, other: &ArbitraryUdtTuple) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                        ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryUdtTuple {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryUdtTuple,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryUdtTuple: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtTuple {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryUdtTuple(
                                arbitrary::Arbitrary::arbitrary(u)?,
                                arbitrary::Arbitrary::arbitrary(u)?,
                            ))
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryUdtTuple(
                                arbitrary::Arbitrary::arbitrary(&mut u)?,
                                arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            ))
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtTuple.with(|count| {
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
                                    <<soroban_sdk::Vec<
                                        i64,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtTuple {
                type Prototype = ArbitraryUdtTuple;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtTuple> for UdtTuple {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryUdtTuple,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(UdtTuple(
                        soroban_sdk::IntoVal::into_val(&v.0, env),
                        soroban_sdk::IntoVal::into_val(&v.1, env),
                    ))
                }
            }
        };
        pub struct UdtStruct {
            pub a: i64,
            pub b: i64,
            pub c: soroban_sdk::Vec<i64>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UdtStruct {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "UdtStruct",
                    "a",
                    &self.a,
                    "b",
                    &self.b,
                    "c",
                    &&self.c,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UdtStruct {
            #[inline]
            fn clone(&self) -> UdtStruct {
                UdtStruct {
                    a: ::core::clone::Clone::clone(&self.a),
                    b: ::core::clone::Clone::clone(&self.b),
                    c: ::core::clone::Clone::clone(&self.c),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for UdtStruct {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i64>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<i64>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UdtStruct {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UdtStruct {
            #[inline]
            fn eq(&self, other: &UdtStruct) -> bool {
                self.a == other.a && self.b == other.b && self.c == other.c
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for UdtStruct {
            #[inline]
            fn cmp(&self, other: &UdtStruct) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.b, &other.b) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.c, &other.c)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for UdtStruct {
            #[inline]
            fn partial_cmp(
                &self,
                other: &UdtStruct,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(&self.c, &other.c)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_UDTSTRUCT: [u8; 84usize] = UdtStruct::spec_xdr();
        impl UdtStruct {
            pub const fn spec_xdr() -> [u8; 84usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\tUdtStruct\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01c\0\0\0\0\0\x03\xea\0\0\0\x07"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for UdtStruct {
            const SPEC_TYPE_ID: [u8; 32] = *b"\x16'd8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTSTRUCT: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"\x16'd8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtStruct {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["a", "b", "c"];
                let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
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
                    c: vals[2]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtStruct> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &UdtStruct,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["a", "b", "c"];
                let vals: [Val; 3usize] = [
                    (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
                    (&val.b).try_into_val(env).map_err(|_| ConversionError)?,
                    (&val.c).try_into_val(env).map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtStruct> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&UdtStruct,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtStruct>>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for UdtStruct {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 3usize {
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
                    c: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "c".try_into()
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtStruct {
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
        impl TryFrom<&UdtStruct> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
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
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "c".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.c)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<UdtStruct> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&UdtStruct> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<UdtStruct> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtStruct) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryUdtStruct {
                a: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                b: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                c: <soroban_sdk::Vec<
                    i64,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryUdtStruct {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ArbitraryUdtStruct",
                        "a",
                        &self.a,
                        "b",
                        &self.b,
                        "c",
                        &&self.c,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryUdtStruct {
                #[inline]
                fn clone(&self) -> ArbitraryUdtStruct {
                    ArbitraryUdtStruct {
                        a: ::core::clone::Clone::clone(&self.a),
                        b: ::core::clone::Clone::clone(&self.b),
                        c: ::core::clone::Clone::clone(&self.c),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryUdtStruct {
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
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            i64,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryUdtStruct {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryUdtStruct {
                #[inline]
                fn eq(&self, other: &ArbitraryUdtStruct) -> bool {
                    self.a == other.a && self.b == other.b && self.c == other.c
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryUdtStruct {
                #[inline]
                fn cmp(&self, other: &ArbitraryUdtStruct) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.b, &other.b) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(&self.c, &other.c)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryUdtStruct {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryUdtStruct,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::cmp::PartialOrd::partial_cmp(&self.c, &other.c)
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
                const RECURSIVE_COUNT_ArbitraryUdtStruct: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtStruct {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryUdtStruct {
                                a: arbitrary::Arbitrary::arbitrary(u)?,
                                b: arbitrary::Arbitrary::arbitrary(u)?,
                                c: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryUdtStruct {
                                a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                b: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                c: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtStruct.with(|count| {
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
                                    <<soroban_sdk::Vec<
                                        i64,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtStruct {
                type Prototype = ArbitraryUdtStruct;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtStruct> for UdtStruct {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryUdtStruct,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(UdtStruct {
                        a: soroban_sdk::IntoVal::into_val(&v.a, env),
                        b: soroban_sdk::IntoVal::into_val(&v.b, env),
                        c: soroban_sdk::IntoVal::into_val(&v.c, env),
                    })
                }
            }
        };
        pub struct UdtRecursive {
            pub a: soroban_sdk::Symbol,
            pub b: soroban_sdk::Vec<UdtRecursive>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UdtRecursive {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UdtRecursive",
                    "a",
                    &self.a,
                    "b",
                    &&self.b,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UdtRecursive {
            #[inline]
            fn clone(&self) -> UdtRecursive {
                UdtRecursive {
                    a: ::core::clone::Clone::clone(&self.a),
                    b: ::core::clone::Clone::clone(&self.b),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for UdtRecursive {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<UdtRecursive>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UdtRecursive {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UdtRecursive {
            #[inline]
            fn eq(&self, other: &UdtRecursive) -> bool {
                self.a == other.a && self.b == other.b
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for UdtRecursive {
            #[inline]
            fn cmp(&self, other: &UdtRecursive) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for UdtRecursive {
            #[inline]
            fn partial_cmp(
                &self,
                other: &UdtRecursive,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b)
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_UDTRECURSIVE: [u8; 84usize] = UdtRecursive::spec_xdr();
        impl UdtRecursive {
            pub const fn spec_xdr() -> [u8; 84usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cUdtRecursive\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x0cUdtRecursive"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for UdtRecursive {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTRECURSIVE: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05",
            [<UdtRecursive as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtRecursive {
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtRecursive> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &UdtRecursive,
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtRecursive> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&UdtRecursive,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtRecursive>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for UdtRecursive {
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtRecursive {
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
        impl TryFrom<&UdtRecursive> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
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
                ])))
            }
        }
        impl TryFrom<UdtRecursive> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&UdtRecursive> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<UdtRecursive> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtRecursive) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryUdtRecursive {
                a: <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                b: <soroban_sdk::Vec<
                    UdtRecursive,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryUdtRecursive {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ArbitraryUdtRecursive",
                        "a",
                        &self.a,
                        "b",
                        &&self.b,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryUdtRecursive {
                #[inline]
                fn clone(&self) -> ArbitraryUdtRecursive {
                    ArbitraryUdtRecursive {
                        a: ::core::clone::Clone::clone(&self.a),
                        b: ::core::clone::Clone::clone(&self.b),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryUdtRecursive {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            UdtRecursive,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryUdtRecursive {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryUdtRecursive {
                #[inline]
                fn eq(&self, other: &ArbitraryUdtRecursive) -> bool {
                    self.a == other.a && self.b == other.b
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryUdtRecursive {
                #[inline]
                fn cmp(&self, other: &ArbitraryUdtRecursive) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                        ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryUdtRecursive {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryUdtRecursive,
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
                const RECURSIVE_COUNT_ArbitraryUdtRecursive: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtRecursive {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryUdtRecursive {
                                a: arbitrary::Arbitrary::arbitrary(u)?,
                                b: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryUdtRecursive {
                                a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                b: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtRecursive.with(|count| {
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
                                    <<soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Vec<
                                        UdtRecursive,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtRecursive {
                type Prototype = ArbitraryUdtRecursive;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtRecursive> for UdtRecursive {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryUdtRecursive,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(UdtRecursive {
                        a: soroban_sdk::IntoVal::into_val(&v.a, env),
                        b: soroban_sdk::IntoVal::into_val(&v.b, env),
                    })
                }
            }
        };
        pub struct RecursiveToEnum {
            pub a: soroban_sdk::Symbol,
            pub b: soroban_sdk::Map<u32, RecursiveEnum>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RecursiveToEnum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RecursiveToEnum",
                    "a",
                    &self.a,
                    "b",
                    &&self.b,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RecursiveToEnum {
            #[inline]
            fn clone(&self) -> RecursiveToEnum {
                RecursiveToEnum {
                    a: ::core::clone::Clone::clone(&self.a),
                    b: ::core::clone::Clone::clone(&self.b),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for RecursiveToEnum {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Map<u32, RecursiveEnum>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RecursiveToEnum {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RecursiveToEnum {
            #[inline]
            fn eq(&self, other: &RecursiveToEnum) -> bool {
                self.a == other.a && self.b == other.b
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for RecursiveToEnum {
            #[inline]
            fn cmp(&self, other: &RecursiveToEnum) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for RecursiveToEnum {
            #[inline]
            fn partial_cmp(
                &self,
                other: &RecursiveToEnum,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b)
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_RECURSIVETOENUM: [u8; 96usize] = RecursiveToEnum::spec_xdr();
        impl RecursiveToEnum {
            pub const fn spec_xdr() -> [u8; 96usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fRecursiveToEnum\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x11\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x03\xec\0\0\0\x04\0\0\x07\xd0\0\0\0\rRecursiveEnum\0\0\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for RecursiveToEnum {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_RECURSIVETOENUM: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci",
            [<RecursiveEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RecursiveToEnum {
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveToEnum> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &RecursiveToEnum,
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RecursiveToEnum> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&RecursiveToEnum,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveToEnum>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for RecursiveToEnum {
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for RecursiveToEnum {
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
        impl TryFrom<&RecursiveToEnum> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
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
                ])))
            }
        }
        impl TryFrom<RecursiveToEnum> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&RecursiveToEnum> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<RecursiveToEnum> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: RecursiveToEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryRecursiveToEnum {
                a: <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                b: <soroban_sdk::Map<
                    u32,
                    RecursiveEnum,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryRecursiveToEnum {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ArbitraryRecursiveToEnum",
                        "a",
                        &self.a,
                        "b",
                        &&self.b,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryRecursiveToEnum {
                #[inline]
                fn clone(&self) -> ArbitraryRecursiveToEnum {
                    ArbitraryRecursiveToEnum {
                        a: ::core::clone::Clone::clone(&self.a),
                        b: ::core::clone::Clone::clone(&self.b),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryRecursiveToEnum {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Map<
                            u32,
                            RecursiveEnum,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryRecursiveToEnum {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryRecursiveToEnum {
                #[inline]
                fn eq(&self, other: &ArbitraryRecursiveToEnum) -> bool {
                    self.a == other.a && self.b == other.b
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryRecursiveToEnum {
                #[inline]
                fn cmp(&self, other: &ArbitraryRecursiveToEnum) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                        ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryRecursiveToEnum {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryRecursiveToEnum,
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
                const RECURSIVE_COUNT_ArbitraryRecursiveToEnum: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryRecursiveToEnum {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryRecursiveToEnum {
                                a: arbitrary::Arbitrary::arbitrary(u)?,
                                b: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryRecursiveToEnum {
                                a: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                b: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryRecursiveToEnum.with(|count| {
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
                                    <<soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Map<
                                        u32,
                                        RecursiveEnum,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for RecursiveToEnum {
                type Prototype = ArbitraryRecursiveToEnum;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryRecursiveToEnum> for RecursiveToEnum {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryRecursiveToEnum,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(RecursiveToEnum {
                        a: soroban_sdk::IntoVal::into_val(&v.a, env),
                        b: soroban_sdk::IntoVal::into_val(&v.b, env),
                    })
                }
            }
        };
        pub struct ContractContext {
            pub args: soroban_sdk::Vec<soroban_sdk::Val>,
            pub contract: soroban_sdk::Address,
            pub fn_name: soroban_sdk::Symbol,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ContractContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ContractContext",
                    "args",
                    &self.args,
                    "contract",
                    &self.contract,
                    "fn_name",
                    &&self.fn_name,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ContractContext {
            #[inline]
            fn clone(&self) -> ContractContext {
                ContractContext {
                    args: ::core::clone::Clone::clone(&self.args),
                    contract: ::core::clone::Clone::clone(&self.contract),
                    fn_name: ::core::clone::Clone::clone(&self.fn_name),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ContractContext {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<soroban_sdk::Val>>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ContractContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ContractContext {
            #[inline]
            fn eq(&self, other: &ContractContext) -> bool {
                self.args == other.args
                    && self.contract == other.contract
                    && self.fn_name == other.fn_name
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ContractContext {
            #[inline]
            fn cmp(&self, other: &ContractContext) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.args, &other.args) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.contract, &other.contract) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.fn_name, &other.fn_name)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ContractContext {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ContractContext,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.args, &other.args) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&self.contract, &other.contract)
                        {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(&self.fn_name, &other.fn_name)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; 96usize] = ContractContext::spec_xdr();
        impl ContractContext {
            pub const fn spec_xdr() -> [u8; 96usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fContractContext\0\0\0\0\x03\0\0\0\0\0\0\0\x04args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\x08contract\0\0\0\x13\0\0\0\0\0\0\0\x07fn_name\0\0\0\0\x11"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for ContractContext {
            const SPEC_TYPE_ID: [u8; 32] = *b"\x03\x04uN\xea\xd7[\x13V\x9f\xd4\xbd\xc1\x8a\xd6\x7f\xd8iD\xa5B\x89qT\x0b'\xad(\xb8\x9f\x8f\x19";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CONTRACTCONTEXT: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"\x03\x04uN\xea\xd7[\x13V\x9f\xd4\xbd\xc1\x8a\xd6\x7f\xd8iD\xa5B\x89qT\x0b'\xad(\xb8\x9f\x8f\x19",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractContext {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["args", "contract", "fn_name"];
                let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    args: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    contract: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    fn_name: vals[2]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ContractContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &ContractContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["args", "contract", "fn_name"];
                let vals: [Val; 3usize] = [
                    (&val.args).try_into_val(env).map_err(|_| ConversionError)?,
                    (&val.contract)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.fn_name)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ContractContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&ContractContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ContractContext>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for ContractContext {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 3usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self {
                    args: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "args"
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
                    contract: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "contract"
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
                    fn_name: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "fn_name"
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for ContractContext {
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
        impl TryFrom<&ContractContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "args"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.args)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.contract)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "fn_name"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.fn_name)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<ContractContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&ContractContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<ContractContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryContractContext {
                args: <soroban_sdk::Vec<
                    soroban_sdk::Val,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                contract: <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                fn_name: <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryContractContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ArbitraryContractContext",
                        "args",
                        &self.args,
                        "contract",
                        &self.contract,
                        "fn_name",
                        &&self.fn_name,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryContractContext {
                #[inline]
                fn clone(&self) -> ArbitraryContractContext {
                    ArbitraryContractContext {
                        args: ::core::clone::Clone::clone(&self.args),
                        contract: ::core::clone::Clone::clone(&self.contract),
                        fn_name: ::core::clone::Clone::clone(&self.fn_name),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryContractContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            soroban_sdk::Val,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryContractContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryContractContext {
                #[inline]
                fn eq(&self, other: &ArbitraryContractContext) -> bool {
                    self.args == other.args
                        && self.contract == other.contract
                        && self.fn_name == other.fn_name
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryContractContext {
                #[inline]
                fn cmp(&self, other: &ArbitraryContractContext) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.args, &other.args) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.contract, &other.contract) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(&self.fn_name, &other.fn_name)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryContractContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryContractContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.args, &other.args) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.contract,
                                &other.contract,
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &self.fn_name,
                                        &other.fn_name,
                                    )
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
                const RECURSIVE_COUNT_ArbitraryContractContext: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContractContext {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryContractContext {
                                args: arbitrary::Arbitrary::arbitrary(u)?,
                                contract: arbitrary::Arbitrary::arbitrary(u)?,
                                fn_name: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryContractContext {
                                args: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                contract: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                fn_name: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
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
                                    <<soroban_sdk::Vec<
                                        soroban_sdk::Val,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for ContractContext {
                type Prototype = ArbitraryContractContext;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContractContext> for ContractContext {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryContractContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(ContractContext {
                        args: soroban_sdk::IntoVal::into_val(&v.args, env),
                        contract: soroban_sdk::IntoVal::into_val(&v.contract, env),
                        fn_name: soroban_sdk::IntoVal::into_val(&v.fn_name, env),
                    })
                }
            }
        };
        pub struct SubContractInvocation {
            pub context: ContractContext,
            pub sub_invocations: soroban_sdk::Vec<InvokerContractAuthEntry>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SubContractInvocation {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SubContractInvocation",
                    "context",
                    &self.context,
                    "sub_invocations",
                    &&self.sub_invocations,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SubContractInvocation {
            #[inline]
            fn clone(&self) -> SubContractInvocation {
                SubContractInvocation {
                    context: ::core::clone::Clone::clone(&self.context),
                    sub_invocations: ::core::clone::Clone::clone(&self.sub_invocations),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SubContractInvocation {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ContractContext>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<InvokerContractAuthEntry>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SubContractInvocation {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SubContractInvocation {
            #[inline]
            fn eq(&self, other: &SubContractInvocation) -> bool {
                self.context == other.context && self.sub_invocations == other.sub_invocations
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for SubContractInvocation {
            #[inline]
            fn cmp(&self, other: &SubContractInvocation) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.context, &other.context) {
                    ::core::cmp::Ordering::Equal => {
                        ::core::cmp::Ord::cmp(&self.sub_invocations, &other.sub_invocations)
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for SubContractInvocation {
            #[inline]
            fn partial_cmp(
                &self,
                other: &SubContractInvocation,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.context, &other.context) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &self.sub_invocations,
                            &other.sub_invocations,
                        )
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_SUBCONTRACTINVOCATION: [u8; 144usize] =
            SubContractInvocation::spec_xdr();
        impl SubContractInvocation {
            pub const fn spec_xdr() -> [u8; 144usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x07context\0\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\0\0\0\0\x0fsub_invocations\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x18InvokerContractAuthEntry"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for SubContractInvocation {
            const SPEC_TYPE_ID: [u8; 32] = *b" \x9d\xc5_\xba\x8fv\x18\x95\x02\xbdJ}\x97\x01KN\xd6\0\xf8\xb6\xefq\xa8j\x11\\\xc7\xd7\xd4\xcf\xf0";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_SUBCONTRACTINVOCATION: [u8; 106usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            106usize,
            2usize,
        >(
            2,
            *b" \x9d\xc5_\xba\x8fv\x18\x95\x02\xbdJ}\x97\x01KN\xd6\0\xf8\xb6\xefq\xa8j\x11\\\xc7\xd7\xd4\xcf\xf0",
            [
                <ContractContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <InvokerContractAuthEntry as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for SubContractInvocation {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["context", "sub_invocations"];
                let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    context: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    sub_invocations: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, SubContractInvocation> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &SubContractInvocation,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["context", "sub_invocations"];
                let vals: [Val; 2usize] = [
                    (&val.context)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.sub_invocations)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &SubContractInvocation> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&SubContractInvocation,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    SubContractInvocation,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for SubContractInvocation {
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
                    context: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "context"
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
                    sub_invocations: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "sub_invocations"
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for SubContractInvocation {
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
        impl TryFrom<&SubContractInvocation> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "context"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.context)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "sub_invocations"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.sub_invocations)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<SubContractInvocation> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&SubContractInvocation> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<SubContractInvocation> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitrarySubContractInvocation {
                context: <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                sub_invocations: <soroban_sdk::Vec<
                    InvokerContractAuthEntry,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitrarySubContractInvocation {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ArbitrarySubContractInvocation",
                        "context",
                        &self.context,
                        "sub_invocations",
                        &&self.sub_invocations,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitrarySubContractInvocation {
                #[inline]
                fn clone(&self) -> ArbitrarySubContractInvocation {
                    ArbitrarySubContractInvocation {
                        context: ::core::clone::Clone::clone(&self.context),
                        sub_invocations: ::core::clone::Clone::clone(&self.sub_invocations),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitrarySubContractInvocation {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            InvokerContractAuthEntry,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitrarySubContractInvocation {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitrarySubContractInvocation {
                #[inline]
                fn eq(&self, other: &ArbitrarySubContractInvocation) -> bool {
                    self.context == other.context && self.sub_invocations == other.sub_invocations
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitrarySubContractInvocation {
                #[inline]
                fn cmp(&self, other: &ArbitrarySubContractInvocation) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.context, &other.context) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(&self.sub_invocations, &other.sub_invocations)
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitrarySubContractInvocation {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitrarySubContractInvocation,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.context, &other.context) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(
                                &self.sub_invocations,
                                &other.sub_invocations,
                            )
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitrarySubContractInvocation: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitrarySubContractInvocation {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitrarySubContractInvocation {
                                context: arbitrary::Arbitrary::arbitrary(u)?,
                                sub_invocations: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
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
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitrarySubContractInvocation {
                                context: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                sub_invocations: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
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
                                    <<ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Vec<
                                        InvokerContractAuthEntry,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for SubContractInvocation {
                type Prototype = ArbitrarySubContractInvocation;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitrarySubContractInvocation>
                for SubContractInvocation
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitrarySubContractInvocation,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(SubContractInvocation {
                        context: soroban_sdk::IntoVal::into_val(&v.context, env),
                        sub_invocations: soroban_sdk::IntoVal::into_val(&v.sub_invocations, env),
                    })
                }
            }
        };
        pub struct CreateContractHostFnContext {
            pub executable: ContractExecutable,
            pub salt: soroban_sdk::BytesN<32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateContractHostFnContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CreateContractHostFnContext",
                    "executable",
                    &self.executable,
                    "salt",
                    &&self.salt,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateContractHostFnContext {
            #[inline]
            fn clone(&self) -> CreateContractHostFnContext {
                CreateContractHostFnContext {
                    executable: ::core::clone::Clone::clone(&self.executable),
                    salt: ::core::clone::Clone::clone(&self.salt),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CreateContractHostFnContext {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ContractExecutable>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CreateContractHostFnContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CreateContractHostFnContext {
            #[inline]
            fn eq(&self, other: &CreateContractHostFnContext) -> bool {
                self.executable == other.executable && self.salt == other.salt
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for CreateContractHostFnContext {
            #[inline]
            fn cmp(&self, other: &CreateContractHostFnContext) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.salt, &other.salt),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for CreateContractHostFnContext {
            #[inline]
            fn partial_cmp(
                &self,
                other: &CreateContractHostFnContext,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8; 116usize] =
            CreateContractHostFnContext::spec_xdr();
        impl CreateContractHostFnContext {
            pub const fn spec_xdr() -> [u8; 116usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x02\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for CreateContractHostFnContext {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xe1\"T\xf0&\x19?P\xad\xa3\xa0\xd2\xf1\xea\xf8~\xde\xe7\x12\xe5_&\xb62Cl\xc8x\xcc.\xd4\xcd";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xe1\"T\xf0&\x19?P\xad\xa3\xa0\xd2\xf1\xea\xf8~\xde\xe7\x12\xe5_&\xb62Cl\xc8x\xcc.\xd4\xcd",
            [<ContractExecutable as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for CreateContractHostFnContext {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["executable", "salt"];
                let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    executable: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    salt: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, CreateContractHostFnContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["executable", "salt"];
                let vals: [Val; 2usize] = [
                    (&val.executable)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.salt).try_into_val(env).map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &CreateContractHostFnContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    CreateContractHostFnContext,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap>
            for CreateContractHostFnContext
        {
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
                    executable: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "executable"
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
                    salt: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "salt"
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal>
            for CreateContractHostFnContext
        {
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
        impl TryFrom<&CreateContractHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "executable"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.executable)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "salt"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.salt)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<CreateContractHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&CreateContractHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<CreateContractHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryCreateContractHostFnContext {
                executable: <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                salt: <soroban_sdk::BytesN<
                    32,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ArbitraryCreateContractHostFnContext",
                        "executable",
                        &self.executable,
                        "salt",
                        &&self.salt,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn clone(&self) -> ArbitraryCreateContractHostFnContext {
                    ArbitraryCreateContractHostFnContext {
                        executable: ::core::clone::Clone::clone(&self.executable),
                        salt: ::core::clone::Clone::clone(&self.salt),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryCreateContractHostFnContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryCreateContractHostFnContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn eq(&self, other: &ArbitraryCreateContractHostFnContext) -> bool {
                    self.executable == other.executable && self.salt == other.salt
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn cmp(
                    &self,
                    other: &ArbitraryCreateContractHostFnContext,
                ) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(&self.salt, &other.salt)
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryCreateContractHostFnContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable)
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext:
                    ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryCreateContractHostFnContext {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractHostFnContext {
                                executable: arbitrary::Arbitrary::arbitrary(u)?,
                                salt: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractHostFnContext {
                                executable: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                salt: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
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
                                    <<ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::BytesN<
                                        32,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for CreateContractHostFnContext {
                type Prototype = ArbitraryCreateContractHostFnContext;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryCreateContractHostFnContext>
                for CreateContractHostFnContext
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryCreateContractHostFnContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(CreateContractHostFnContext {
                        executable: soroban_sdk::IntoVal::into_val(&v.executable, env),
                        salt: soroban_sdk::IntoVal::into_val(&v.salt, env),
                    })
                }
            }
        };
        pub struct CreateContractWithConstructorHostFnContext {
            pub constructor_args: soroban_sdk::Vec<soroban_sdk::Val>,
            pub executable: ContractExecutable,
            pub salt: soroban_sdk::BytesN<32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CreateContractWithConstructorHostFnContext",
                    "constructor_args",
                    &self.constructor_args,
                    "executable",
                    &self.executable,
                    "salt",
                    &&self.salt,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn clone(&self) -> CreateContractWithConstructorHostFnContext {
                CreateContractWithConstructorHostFnContext {
                    constructor_args: ::core::clone::Clone::clone(&self.constructor_args),
                    executable: ::core::clone::Clone::clone(&self.executable),
                    salt: ::core::clone::Clone::clone(&self.salt),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CreateContractWithConstructorHostFnContext {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<soroban_sdk::Val>>;
                let _: ::core::cmp::AssertParamIsEq<ContractExecutable>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CreateContractWithConstructorHostFnContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn eq(&self, other: &CreateContractWithConstructorHostFnContext) -> bool {
                self.constructor_args == other.constructor_args
                    && self.executable == other.executable
                    && self.salt == other.salt
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn cmp(
                &self,
                other: &CreateContractWithConstructorHostFnContext,
            ) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.constructor_args, &other.constructor_args) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.salt, &other.salt)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn partial_cmp(
                &self,
                other: &CreateContractWithConstructorHostFnContext,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(
                    &self.constructor_args,
                    &other.constructor_args,
                ) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(
                            &self.executable,
                            &other.executable,
                        ) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8; 164usize] =
            CreateContractWithConstructorHostFnContext::spec_xdr();
        impl CreateContractWithConstructorHostFnContext {
            pub const fn spec_xdr() -> [u8; 164usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0*CreateContractWithConstructorHostFnContext\0\0\0\0\0\x03\0\0\0\0\0\0\0\x10constructor_args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for CreateContractWithConstructorHostFnContext {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xd2;\xff\xe6\x97\xda;\x83c$F\x15Z\xf1r\xf4\xc18\xfda!\x0b\r\x87\x88\xa0\x9a\x08Yu\xccS";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xd2;\xff\xe6\x97\xda;\x83c$F\x15Z\xf1r\xf4\xc18\xfda!\x0b\r\x87\x88\xa0\x9a\x08Yu\xccS",
            [<ContractExecutable as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>
            for CreateContractWithConstructorHostFnContext
        {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["constructor_args", "executable", "salt"];
                let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    constructor_args: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    executable: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    salt: vals[2]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, CreateContractWithConstructorHostFnContext>
            for soroban_sdk::Val
        {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["constructor_args", "executable", "salt"];
                let vals: [Val; 3usize] = [
                    (&val.constructor_args)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.executable)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.salt).try_into_val(env).map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &CreateContractWithConstructorHostFnContext>
            for soroban_sdk::Val
        {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    CreateContractWithConstructorHostFnContext,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap>
            for CreateContractWithConstructorHostFnContext
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 3usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self {
                    constructor_args: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "constructor_args"
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
                    executable: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "executable"
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
                    salt: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "salt"
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal>
            for CreateContractWithConstructorHostFnContext
        {
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
        impl TryFrom<&CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "constructor_args"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.constructor_args)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "executable"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.executable)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "salt"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.salt)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryCreateContractWithConstructorHostFnContext {
                constructor_args: <soroban_sdk::Vec<
                    soroban_sdk::Val,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                executable: <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                salt: <soroban_sdk::BytesN<
                    32,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ArbitraryCreateContractWithConstructorHostFnContext",
                        "constructor_args",
                        &self.constructor_args,
                        "executable",
                        &self.executable,
                        "salt",
                        &&self.salt,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn clone(&self) -> ArbitraryCreateContractWithConstructorHostFnContext {
                    ArbitraryCreateContractWithConstructorHostFnContext {
                        constructor_args: ::core::clone::Clone::clone(&self.constructor_args),
                        executable: ::core::clone::Clone::clone(&self.executable),
                        salt: ::core::clone::Clone::clone(&self.salt),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            soroban_sdk::Val,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryCreateContractWithConstructorHostFnContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn eq(&self, other: &ArbitraryCreateContractWithConstructorHostFnContext) -> bool {
                    self.constructor_args == other.constructor_args
                        && self.executable == other.executable
                        && self.salt == other.salt
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn cmp(
                    &self,
                    other: &ArbitraryCreateContractWithConstructorHostFnContext,
                ) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.constructor_args, &other.constructor_args) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(&self.salt, &other.salt)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryCreateContractWithConstructorHostFnContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.constructor_args,
                        &other.constructor_args,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.executable,
                                &other.executable,
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
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
                const RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext:
                    ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary>
                    for ArbitraryCreateContractWithConstructorHostFnContext
                {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
                                    if count.get() > 0 {
                                        return Err(arbitrary::Error::NotEnoughData);
                                    }
                                    count.set(count.get() + 1);
                                    Ok(())
                                })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                                constructor_args: arbitrary::Arbitrary::arbitrary(u)?,
                                executable: arbitrary::Arbitrary::arbitrary(u)?,
                                salt: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
                                    if count.get() > 0 {
                                        return Err(arbitrary::Error::NotEnoughData);
                                    }
                                    count.set(count.get() + 1);
                                    Ok(())
                                })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                                constructor_args: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                executable: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                salt: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
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
                                    <<soroban_sdk::Vec<
                                        soroban_sdk::Val,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::BytesN<
                                        32,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary
                for CreateContractWithConstructorHostFnContext
            {
                type Prototype = ArbitraryCreateContractWithConstructorHostFnContext;
            }
            impl
                soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    ArbitraryCreateContractWithConstructorHostFnContext,
                > for CreateContractWithConstructorHostFnContext
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryCreateContractWithConstructorHostFnContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(CreateContractWithConstructorHostFnContext {
                        constructor_args: soroban_sdk::IntoVal::into_val(&v.constructor_args, env),
                        executable: soroban_sdk::IntoVal::into_val(&v.executable, env),
                        salt: soroban_sdk::IntoVal::into_val(&v.salt, env),
                    })
                }
            }
        };
        pub enum UdtEnum {
            UdtA,
            UdtB(UdtStruct),
            UdtC(UdtEnum2),
            UdtD(UdtTuple),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UdtEnum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    UdtEnum::UdtA => ::core::fmt::Formatter::write_str(f, "UdtA"),
                    UdtEnum::UdtB(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtB", &__self_0)
                    }
                    UdtEnum::UdtC(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtC", &__self_0)
                    }
                    UdtEnum::UdtD(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtD", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UdtEnum {
            #[inline]
            fn clone(&self) -> UdtEnum {
                match self {
                    UdtEnum::UdtA => UdtEnum::UdtA,
                    UdtEnum::UdtB(__self_0) => UdtEnum::UdtB(::core::clone::Clone::clone(__self_0)),
                    UdtEnum::UdtC(__self_0) => UdtEnum::UdtC(::core::clone::Clone::clone(__self_0)),
                    UdtEnum::UdtD(__self_0) => UdtEnum::UdtD(::core::clone::Clone::clone(__self_0)),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for UdtEnum {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<UdtStruct>;
                let _: ::core::cmp::AssertParamIsEq<UdtEnum2>;
                let _: ::core::cmp::AssertParamIsEq<UdtTuple>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UdtEnum {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UdtEnum {
            #[inline]
            fn eq(&self, other: &UdtEnum) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (UdtEnum::UdtB(__self_0), UdtEnum::UdtB(__arg1_0)) => __self_0 == __arg1_0,
                        (UdtEnum::UdtC(__self_0), UdtEnum::UdtC(__arg1_0)) => __self_0 == __arg1_0,
                        (UdtEnum::UdtD(__self_0), UdtEnum::UdtD(__arg1_0)) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for UdtEnum {
            #[inline]
            fn cmp(&self, other: &UdtEnum) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (UdtEnum::UdtB(__self_0), UdtEnum::UdtB(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (UdtEnum::UdtC(__self_0), UdtEnum::UdtC(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (UdtEnum::UdtD(__self_0), UdtEnum::UdtD(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for UdtEnum {
            #[inline]
            fn partial_cmp(
                &self,
                other: &UdtEnum,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (UdtEnum::UdtB(__self_0), UdtEnum::UdtB(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (UdtEnum::UdtC(__self_0), UdtEnum::UdtC(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (UdtEnum::UdtD(__self_0), UdtEnum::UdtD(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_UDTENUM: [u8; 156usize] = UdtEnum::spec_xdr();
        impl UdtEnum {
            pub const fn spec_xdr() -> [u8; 156usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07UdtEnum\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x04UdtA\0\0\0\x01\0\0\0\0\0\0\0\x04UdtB\0\0\0\x01\0\0\x07\xd0\0\0\0\tUdtStruct\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04UdtC\0\0\0\x01\0\0\x07\xd0\0\0\0\x08UdtEnum2\0\0\0\x01\0\0\0\0\0\0\0\x04UdtD\0\0\0\x01\0\0\x07\xd0\0\0\0\x08UdtTuple"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for UdtEnum {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTENUM: [u8; 138usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            138usize,
            3usize,
        >(
            2,
            *b"\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2",
            [
                <UdtStruct as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <UdtEnum2 as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <UdtTuple as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtEnum {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &["UdtA", "UdtB", "UdtC", "UdtD"];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 0 {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::UdtA
                        }
                        1 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::UdtB(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        2 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::UdtC(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        3 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::UdtD(
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &UdtEnum,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    UdtEnum::UdtA => {
                        let tup: (soroban_sdk::Val,) =
                            (soroban_sdk::Symbol::try_from_val(env, &"UdtA")?.to_val(),);
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    UdtEnum::UdtB(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"UdtB")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    UdtEnum::UdtC(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"UdtC")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    UdtEnum::UdtD(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"UdtD")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtEnum> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&UdtEnum,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum>>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for UdtEnum {
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
                    "UdtA" => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::UdtA
                    }
                    "UdtB" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::UdtB(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "UdtC" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::UdtC(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "UdtD" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::UdtD(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtEnum {
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
        impl TryFrom<&UdtEnum> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    UdtEnum::UdtA => {
                        let symbol = soroban_sdk::xdr::ScSymbol(
                            "UdtA"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        );
                        let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                        (val,)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    }
                    UdtEnum::UdtB(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "UdtB"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    UdtEnum::UdtC(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "UdtC"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    UdtEnum::UdtD(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "UdtD"
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
        impl TryFrom<UdtEnum> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&UdtEnum> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<UdtEnum> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: UdtEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryUdtEnum {
                UdtA,
                UdtB(<UdtStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
                UdtC(<UdtEnum2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
                UdtD(<UdtTuple as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryUdtEnum {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryUdtEnum::UdtA => ::core::fmt::Formatter::write_str(f, "UdtA"),
                        ArbitraryUdtEnum::UdtB(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtB", &__self_0)
                        }
                        ArbitraryUdtEnum::UdtC(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtC", &__self_0)
                        }
                        ArbitraryUdtEnum::UdtD(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UdtD", &__self_0)
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryUdtEnum {
                #[inline]
                fn clone(&self) -> ArbitraryUdtEnum {
                    match self {
                        ArbitraryUdtEnum::UdtA => ArbitraryUdtEnum::UdtA,
                        ArbitraryUdtEnum::UdtB(__self_0) => {
                            ArbitraryUdtEnum::UdtB(::core::clone::Clone::clone(__self_0))
                        }
                        ArbitraryUdtEnum::UdtC(__self_0) => {
                            ArbitraryUdtEnum::UdtC(::core::clone::Clone::clone(__self_0))
                        }
                        ArbitraryUdtEnum::UdtD(__self_0) => {
                            ArbitraryUdtEnum::UdtD(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryUdtEnum {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <UdtStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <UdtEnum2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <UdtTuple as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryUdtEnum {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryUdtEnum {
                #[inline]
                fn eq(&self, other: &ArbitraryUdtEnum) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryUdtEnum::UdtB(__self_0),
                                ArbitraryUdtEnum::UdtB(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryUdtEnum::UdtC(__self_0),
                                ArbitraryUdtEnum::UdtC(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryUdtEnum::UdtD(__self_0),
                                ArbitraryUdtEnum::UdtD(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryUdtEnum {
                #[inline]
                fn cmp(&self, other: &ArbitraryUdtEnum) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryUdtEnum::UdtB(__self_0),
                                ArbitraryUdtEnum::UdtB(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryUdtEnum::UdtC(__self_0),
                                ArbitraryUdtEnum::UdtC(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryUdtEnum::UdtD(__self_0),
                                ArbitraryUdtEnum::UdtD(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => ::core::cmp::Ordering::Equal,
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryUdtEnum {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryUdtEnum,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (ArbitraryUdtEnum::UdtB(__self_0), ArbitraryUdtEnum::UdtB(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (ArbitraryUdtEnum::UdtC(__self_0), ArbitraryUdtEnum::UdtC(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (ArbitraryUdtEnum::UdtD(__self_0), ArbitraryUdtEnum::UdtD(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryUdtEnum: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtEnum {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 4u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryUdtEnum::UdtA,
                                    1u64 => {
                                        ArbitraryUdtEnum::UdtB(arbitrary::Arbitrary::arbitrary(u)?)
                                    }
                                    2u64 => {
                                        ArbitraryUdtEnum::UdtC(arbitrary::Arbitrary::arbitrary(u)?)
                                    }
                                    3u64 => {
                                        ArbitraryUdtEnum::UdtD(arbitrary::Arbitrary::arbitrary(u)?)
                                    }
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 4u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryUdtEnum::UdtA,
                                    1u64 => ArbitraryUdtEnum::UdtB(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    2u64 => ArbitraryUdtEnum::UdtC(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    3u64 => ArbitraryUdtEnum::UdtD(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtEnum.with(|count| {
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
                                                    <<UdtStruct as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<UdtEnum2 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<UdtTuple as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtEnum {
                type Prototype = ArbitraryUdtEnum;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtEnum> for UdtEnum {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryUdtEnum,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryUdtEnum::UdtA => UdtEnum::UdtA,
                        ArbitraryUdtEnum::UdtB(field_0) => {
                            UdtEnum::UdtB(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                        ArbitraryUdtEnum::UdtC(field_0) => {
                            UdtEnum::UdtC(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                        ArbitraryUdtEnum::UdtD(field_0) => {
                            UdtEnum::UdtD(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                    })
                }
            }
        };
        pub enum RecursiveEnum {
            NotRecursive,
            Recursive(RecursiveToEnum),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RecursiveEnum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    RecursiveEnum::NotRecursive => {
                        ::core::fmt::Formatter::write_str(f, "NotRecursive")
                    }
                    RecursiveEnum::Recursive(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Recursive", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RecursiveEnum {
            #[inline]
            fn clone(&self) -> RecursiveEnum {
                match self {
                    RecursiveEnum::NotRecursive => RecursiveEnum::NotRecursive,
                    RecursiveEnum::Recursive(__self_0) => {
                        RecursiveEnum::Recursive(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for RecursiveEnum {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<RecursiveToEnum>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RecursiveEnum {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RecursiveEnum {
            #[inline]
            fn eq(&self, other: &RecursiveEnum) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            RecursiveEnum::Recursive(__self_0),
                            RecursiveEnum::Recursive(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for RecursiveEnum {
            #[inline]
            fn cmp(&self, other: &RecursiveEnum) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (
                            RecursiveEnum::Recursive(__self_0),
                            RecursiveEnum::Recursive(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for RecursiveEnum {
            #[inline]
            fn partial_cmp(
                &self,
                other: &RecursiveEnum,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (RecursiveEnum::Recursive(__self_0), RecursiveEnum::Recursive(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_RECURSIVEENUM: [u8; 112usize] = RecursiveEnum::spec_xdr();
        impl RecursiveEnum {
            pub const fn spec_xdr() -> [u8; 112usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\rRecursiveEnum\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x0cNotRecursive\0\0\0\x01\0\0\0\0\0\0\0\tRecursive\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fRecursiveToEnum\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for RecursiveEnum {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\0M";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_RECURSIVEENUM: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\0M",
            [<RecursiveToEnum as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RecursiveEnum {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &["NotRecursive", "Recursive"];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 0 {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::NotRecursive
                        }
                        1 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Recursive(
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveEnum> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &RecursiveEnum,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    RecursiveEnum::NotRecursive => {
                        let tup: (soroban_sdk::Val,) =
                            (soroban_sdk::Symbol::try_from_val(env, &"NotRecursive")?.to_val(),);
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    RecursiveEnum::Recursive(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Recursive")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RecursiveEnum> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&RecursiveEnum,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, RecursiveEnum>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for RecursiveEnum {
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
                    "NotRecursive" => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::NotRecursive
                    }
                    "Recursive" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Recursive(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for RecursiveEnum {
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
        impl TryFrom<&RecursiveEnum> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    RecursiveEnum::NotRecursive => {
                        let symbol = soroban_sdk::xdr::ScSymbol(
                            "NotRecursive"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        );
                        let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                        (val,)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    }
                    RecursiveEnum::Recursive(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Recursive"
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
        impl TryFrom<RecursiveEnum> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&RecursiveEnum> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<RecursiveEnum> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: RecursiveEnum) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryRecursiveEnum {
                NotRecursive,
                Recursive(
                    <RecursiveToEnum as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryRecursiveEnum {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryRecursiveEnum::NotRecursive => {
                            ::core::fmt::Formatter::write_str(f, "NotRecursive")
                        }
                        ArbitraryRecursiveEnum::Recursive(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Recursive",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryRecursiveEnum {
                #[inline]
                fn clone(&self) -> ArbitraryRecursiveEnum {
                    match self {
                        ArbitraryRecursiveEnum::NotRecursive => {
                            ArbitraryRecursiveEnum::NotRecursive
                        }
                        ArbitraryRecursiveEnum::Recursive(__self_0) => {
                            ArbitraryRecursiveEnum::Recursive(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryRecursiveEnum {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <RecursiveToEnum as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryRecursiveEnum {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryRecursiveEnum {
                #[inline]
                fn eq(&self, other: &ArbitraryRecursiveEnum) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryRecursiveEnum::Recursive(__self_0),
                                ArbitraryRecursiveEnum::Recursive(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryRecursiveEnum {
                #[inline]
                fn cmp(&self, other: &ArbitraryRecursiveEnum) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryRecursiveEnum::Recursive(__self_0),
                                ArbitraryRecursiveEnum::Recursive(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => ::core::cmp::Ordering::Equal,
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryRecursiveEnum {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryRecursiveEnum,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryRecursiveEnum::Recursive(__self_0),
                            ArbitraryRecursiveEnum::Recursive(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryRecursiveEnum: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryRecursiveEnum {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 2u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryRecursiveEnum::NotRecursive,
                                    1u64 => ArbitraryRecursiveEnum::Recursive(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 2u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryRecursiveEnum::NotRecursive,
                                    1u64 => ArbitraryRecursiveEnum::Recursive(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryRecursiveEnum.with(|count| {
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
                                                    <<RecursiveToEnum as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for RecursiveEnum {
                type Prototype = ArbitraryRecursiveEnum;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryRecursiveEnum> for RecursiveEnum {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryRecursiveEnum,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryRecursiveEnum::NotRecursive => RecursiveEnum::NotRecursive,
                        ArbitraryRecursiveEnum::Recursive(field_0) => {
                            RecursiveEnum::Recursive(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                    })
                }
            }
        };
        pub enum Context {
            Contract(ContractContext),
            CreateContractHostFn(CreateContractHostFnContext),
            CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Context {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Context::Contract(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
                    }
                    Context::CreateContractHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractHostFn",
                            &__self_0,
                        )
                    }
                    Context::CreateContractWithCtorHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractWithCtorHostFn",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Context {
            #[inline]
            fn clone(&self) -> Context {
                match self {
                    Context::Contract(__self_0) => {
                        Context::Contract(::core::clone::Clone::clone(__self_0))
                    }
                    Context::CreateContractHostFn(__self_0) => {
                        Context::CreateContractHostFn(::core::clone::Clone::clone(__self_0))
                    }
                    Context::CreateContractWithCtorHostFn(__self_0) => {
                        Context::CreateContractWithCtorHostFn(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Context {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ContractContext>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractHostFnContext>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractWithConstructorHostFnContext>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Context {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Context {
            #[inline]
            fn eq(&self, other: &Context) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Context::Contract(__self_0), Context::Contract(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Context::CreateContractHostFn(__self_0),
                            Context::CreateContractHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Context::CreateContractWithCtorHostFn(__self_0),
                            Context::CreateContractWithCtorHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Context {
            #[inline]
            fn cmp(&self, other: &Context) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (Context::Contract(__self_0), Context::Contract(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (
                            Context::CreateContractHostFn(__self_0),
                            Context::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            Context::CreateContractWithCtorHostFn(__self_0),
                            Context::CreateContractWithCtorHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Context {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Context,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (Context::Contract(__self_0), Context::Contract(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (
                        Context::CreateContractHostFn(__self_0),
                        Context::CreateContractHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        Context::CreateContractWithCtorHostFn(__self_0),
                        Context::CreateContractWithCtorHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CONTEXT: [u8; 244usize] = Context::spec_xdr();
        impl Context {
            pub const fn spec_xdr() -> [u8; 244usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07Context\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for Context {
            const SPEC_TYPE_ID: [u8; 32] = *b"\r\xb6\x0b\xec\x8f\xd04l1\xb3-\xa0{\x90\xa3\xc2\xab\x93\xd4\x82x\xe1_\x8a\xa8N?.\xcd\xc1\xfc\x08";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CONTEXT: [u8; 138usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            138usize,
            3usize,
        >(
            2,
            *b"\r\xb6\x0b\xec\x8f\xd04l1\xb3-\xa0{\x90\xa3\xc2\xab\x93\xd4\x82x\xe1_\x8a\xa8N?.\xcd\xc1\xfc\x08",
            [
                <ContractContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractWithConstructorHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Context {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &[
                    "Contract",
                    "CreateContractHostFn",
                    "CreateContractWithCtorHostFn",
                ];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Contract(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        1 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractHostFn(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        2 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractWithCtorHostFn(
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, Context> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &Context,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    Context::Contract(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Contract")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Context::CreateContractHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?
                                .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Context::CreateContractWithCtorHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(
                                env,
                                &"CreateContractWithCtorHostFn",
                            )?
                            .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Context> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&Context,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Context>>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for Context {
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
                    "Contract" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Contract(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractWithCtorHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractWithCtorHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Context {
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
        impl TryFrom<&Context> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Context) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    Context::Contract(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    Context::CreateContractHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractHostFn"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    Context::CreateContractWithCtorHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractWithCtorHostFn"
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
        impl TryFrom<Context> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Context) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&Context> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Context) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<Context> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Context) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryContext {
                Contract(
                    <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractHostFn(
                    <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractWithCtorHostFn(
                    <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryContext::Contract(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Contract", &__self_0,
                            )
                        }
                        ArbitraryContext::CreateContractHostFn(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "CreateContractHostFn",
                                &__self_0,
                            )
                        }
                        ArbitraryContext::CreateContractWithCtorHostFn(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "CreateContractWithCtorHostFn",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryContext {
                #[inline]
                fn clone(&self) -> ArbitraryContext {
                    match self {
                        ArbitraryContext::Contract(__self_0) => {
                            ArbitraryContext::Contract(::core::clone::Clone::clone(__self_0))
                        }
                        ArbitraryContext::CreateContractHostFn(__self_0) => {
                            ArbitraryContext::CreateContractHostFn(::core::clone::Clone::clone(
                                __self_0,
                            ))
                        }
                        ArbitraryContext::CreateContractWithCtorHostFn(__self_0) => {
                            ArbitraryContext::CreateContractWithCtorHostFn(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryContext {
                #[inline]
                fn eq(&self, other: &ArbitraryContext) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryContext::Contract(__self_0),
                                ArbitraryContext::Contract(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryContext::CreateContractHostFn(__self_0),
                                ArbitraryContext::CreateContractHostFn(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryContext::CreateContractWithCtorHostFn(__self_0),
                                ArbitraryContext::CreateContractWithCtorHostFn(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryContext {
                #[inline]
                fn cmp(&self, other: &ArbitraryContext) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryContext::Contract(__self_0),
                                ArbitraryContext::Contract(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryContext::CreateContractHostFn(__self_0),
                                ArbitraryContext::CreateContractHostFn(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryContext::CreateContractWithCtorHostFn(__self_0),
                                ArbitraryContext::CreateContractWithCtorHostFn(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryContext::Contract(__self_0),
                            ArbitraryContext::Contract(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryContext::CreateContractHostFn(__self_0),
                            ArbitraryContext::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryContext::CreateContractWithCtorHostFn(__self_0),
                            ArbitraryContext::CreateContractWithCtorHostFn(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryContext: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContext {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContext::Contract(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    1u64 => ArbitraryContext::CreateContractHostFn(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    2u64 => ArbitraryContext::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContext::Contract(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    1u64 => ArbitraryContext::CreateContractHostFn(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    2u64 => ArbitraryContext::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
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
                                                    <<ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Context {
                type Prototype = ArbitraryContext;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContext> for Context {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryContext::Contract(field_0) => {
                            Context::Contract(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                        ArbitraryContext::CreateContractHostFn(field_0) => {
                            Context::CreateContractHostFn(soroban_sdk::IntoVal::into_val(
                                field_0, env,
                            ))
                        }
                        ArbitraryContext::CreateContractWithCtorHostFn(field_0) => {
                            Context::CreateContractWithCtorHostFn(soroban_sdk::IntoVal::into_val(
                                field_0, env,
                            ))
                        }
                    })
                }
            }
        };
        pub enum ContractExecutable {
            Wasm(soroban_sdk::BytesN<32>),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ContractExecutable {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ContractExecutable::Wasm(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ContractExecutable {
            #[inline]
            fn clone(&self) -> ContractExecutable {
                match self {
                    ContractExecutable::Wasm(__self_0) => {
                        ContractExecutable::Wasm(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ContractExecutable {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ContractExecutable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ContractExecutable {
            #[inline]
            fn eq(&self, other: &ContractExecutable) -> bool {
                match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ContractExecutable {
            #[inline]
            fn cmp(&self, other: &ContractExecutable) -> ::core::cmp::Ordering {
                match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ContractExecutable {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ContractExecutable,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; 68usize] =
            ContractExecutable::spec_xdr();
        impl ContractExecutable {
            pub const fn spec_xdr() -> [u8; 68usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x12ContractExecutable\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 "
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for ContractExecutable {
            const SPEC_TYPE_ID: [u8; 32] = *b"^\xbe34\xd8\x99\x84\x91\x81\x9fu\x9fu\x05\xb8\xb4\x14\x95\xb7\x9d|\x06$\x04y\xe9\"\xb9\x14\xfc\xf9\x85";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CONTRACTEXECUTABLE: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"^\xbe34\xd8\x99\x84\x91\x81\x9fu\x9fu\x05\xb8\xb4\x14\x95\xb7\x9d|\x06$\x04y\xe9\"\xb9\x14\xfc\xf9\x85",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractExecutable {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &["Wasm"];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Wasm(
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &ContractExecutable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    ContractExecutable::Wasm(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Wasm")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ContractExecutable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&ContractExecutable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutable>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for ContractExecutable {
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
                    "Wasm" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Wasm(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for ContractExecutable {
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
        impl TryFrom<&ContractExecutable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    ContractExecutable::Wasm(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Wasm"
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
        impl TryFrom<ContractExecutable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&ContractExecutable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<ContractExecutable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryContractExecutable {
                Wasm(
                    <soroban_sdk::BytesN<
                        32,
                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryContractExecutable {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryContractExecutable::Wasm(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryContractExecutable {
                #[inline]
                fn clone(&self) -> ArbitraryContractExecutable {
                    match self {
                        ArbitraryContractExecutable::Wasm(__self_0) => {
                            ArbitraryContractExecutable::Wasm(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryContractExecutable {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryContractExecutable {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryContractExecutable {
                #[inline]
                fn eq(&self, other: &ArbitraryContractExecutable) -> bool {
                    match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => __self_0 == __arg1_0,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryContractExecutable {
                #[inline]
                fn cmp(&self, other: &ArbitraryContractExecutable) -> ::core::cmp::Ordering {
                    match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryContractExecutable {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryContractExecutable,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryContractExecutable: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContractExecutable {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 1u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContractExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 1u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContractExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
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
                                                    <<soroban_sdk::BytesN<
                                                        32,
                                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for ContractExecutable {
                type Prototype = ArbitraryContractExecutable;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContractExecutable> for ContractExecutable {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryContractExecutable,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryContractExecutable::Wasm(field_0) => {
                            ContractExecutable::Wasm(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                    })
                }
            }
        };
        pub enum InvokerContractAuthEntry {
            Contract(SubContractInvocation),
            CreateContractHostFn(CreateContractHostFnContext),
            CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InvokerContractAuthEntry {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    InvokerContractAuthEntry::Contract(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
                    }
                    InvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractHostFn",
                            &__self_0,
                        )
                    }
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractWithCtorHostFn",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for InvokerContractAuthEntry {
            #[inline]
            fn clone(&self) -> InvokerContractAuthEntry {
                match self {
                    InvokerContractAuthEntry::Contract(__self_0) => {
                        InvokerContractAuthEntry::Contract(::core::clone::Clone::clone(__self_0))
                    }
                    InvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        InvokerContractAuthEntry::CreateContractHostFn(::core::clone::Clone::clone(
                            __self_0,
                        ))
                    }
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InvokerContractAuthEntry {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<SubContractInvocation>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractHostFnContext>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractWithConstructorHostFnContext>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InvokerContractAuthEntry {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InvokerContractAuthEntry {
            #[inline]
            fn eq(&self, other: &InvokerContractAuthEntry) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            InvokerContractAuthEntry::Contract(__self_0),
                            InvokerContractAuthEntry::Contract(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            InvokerContractAuthEntry::CreateContractHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InvokerContractAuthEntry {
            #[inline]
            fn cmp(&self, other: &InvokerContractAuthEntry) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (
                            InvokerContractAuthEntry::Contract(__self_0),
                            InvokerContractAuthEntry::Contract(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            InvokerContractAuthEntry::CreateContractHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InvokerContractAuthEntry {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InvokerContractAuthEntry,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (
                        InvokerContractAuthEntry::Contract(__self_0),
                        InvokerContractAuthEntry::Contract(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        InvokerContractAuthEntry::CreateContractHostFn(__self_0),
                        InvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; 268usize] =
            InvokerContractAuthEntry::spec_xdr();
        impl InvokerContractAuthEntry {
            pub const fn spec_xdr() -> [u8; 268usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x18InvokerContractAuthEntry\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for InvokerContractAuthEntry {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xf0{\xa6\xe9r\xf3\x10\xf6\x0b)\xdb\x8e\r\xea\xe0\xa0\x89\xca\x1a\x1c\x12\xf8\x8f'K\xda\x9b\x87\xab\xaa\xf8=";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; 138usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            138usize,
            3usize,
        >(
            2,
            *b"\xf0{\xa6\xe9r\xf3\x10\xf6\x0b)\xdb\x8e\r\xea\xe0\xa0\x89\xca\x1a\x1c\x12\xf8\x8f'K\xda\x9b\x87\xab\xaa\xf8=",
            [
                <SubContractInvocation as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractWithConstructorHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for InvokerContractAuthEntry {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &[
                    "Contract",
                    "CreateContractHostFn",
                    "CreateContractWithCtorHostFn",
                ];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Contract(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        1 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractHostFn(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        2 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractWithCtorHostFn(
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
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, InvokerContractAuthEntry> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &InvokerContractAuthEntry,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    InvokerContractAuthEntry::Contract(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Contract")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    InvokerContractAuthEntry::CreateContractHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?
                                .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(
                                env,
                                &"CreateContractWithCtorHostFn",
                            )?
                            .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &InvokerContractAuthEntry> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&InvokerContractAuthEntry,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    InvokerContractAuthEntry,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec>
            for InvokerContractAuthEntry
        {
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
                    "Contract" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Contract(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractWithCtorHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractWithCtorHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal>
            for InvokerContractAuthEntry
        {
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
        impl TryFrom<&InvokerContractAuthEntry> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    InvokerContractAuthEntry::Contract(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    InvokerContractAuthEntry::CreateContractHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractHostFn"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractWithCtorHostFn"
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
        impl TryFrom<InvokerContractAuthEntry> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&InvokerContractAuthEntry> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<InvokerContractAuthEntry> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryInvokerContractAuthEntry {
                Contract(
                    <SubContractInvocation as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractHostFn(
                    <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractWithCtorHostFn(
                    <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryInvokerContractAuthEntry::Contract(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Contract", &__self_0,
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "CreateContractHostFn",
                                &__self_0,
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            __self_0,
                        ) => ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractWithCtorHostFn",
                            &__self_0,
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn clone(&self) -> ArbitraryInvokerContractAuthEntry {
                    match self {
                        ArbitraryInvokerContractAuthEntry::Contract(__self_0) => {
                            ArbitraryInvokerContractAuthEntry::Contract(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                            ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            __self_0,
                        ) => ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryInvokerContractAuthEntry {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <SubContractInvocation as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryInvokerContractAuthEntry {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn eq(&self, other: &ArbitraryInvokerContractAuthEntry) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryInvokerContractAuthEntry::Contract(__self_0),
                                ArbitraryInvokerContractAuthEntry::Contract(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0),
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __self_0,
                                ),
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __arg1_0,
                                ),
                            ) => __self_0 == __arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn cmp(&self, other: &ArbitraryInvokerContractAuthEntry) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryInvokerContractAuthEntry::Contract(__self_0),
                                ArbitraryInvokerContractAuthEntry::Contract(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0),
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __self_0,
                                ),
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __arg1_0,
                                ),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryInvokerContractAuthEntry,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryInvokerContractAuthEntry::Contract(__self_0),
                            ArbitraryInvokerContractAuthEntry::Contract(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0),
                            ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                __self_0,
                            ),
                            ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                __arg1_0,
                            ),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryInvokerContractAuthEntry {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(
                                    <u32 as arbitrary::Arbitrary>::arbitrary(u)?,
                                ) * 3u64) >> 32
                                {
                                    0u64 => {
                                        ArbitraryInvokerContractAuthEntry::Contract(
                                            arbitrary::Arbitrary::arbitrary(u)?,
                                        )
                                    }
                                    1u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                            arbitrary::Arbitrary::arbitrary(u)?,
                                        )
                                    }
                                    2u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                            arbitrary::Arbitrary::arbitrary(u)?,
                                        )
                                    }
                                    _ => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(
                                    <u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?,
                                ) * 3u64) >> 32
                                {
                                    0u64 => {
                                        ArbitraryInvokerContractAuthEntry::Contract(
                                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                        )
                                    }
                                    1u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                        )
                                    }
                                    2u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                        )
                                    }
                                    _ => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
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
                                                    <<SubContractInvocation as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for InvokerContractAuthEntry {
                type Prototype = ArbitraryInvokerContractAuthEntry;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryInvokerContractAuthEntry>
                for InvokerContractAuthEntry
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryInvokerContractAuthEntry,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryInvokerContractAuthEntry::Contract(field_0) => {
                            InvokerContractAuthEntry::Contract(soroban_sdk::IntoVal::into_val(
                                field_0, env,
                            ))
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(field_0) => {
                            InvokerContractAuthEntry::CreateContractHostFn(
                                soroban_sdk::IntoVal::into_val(field_0, env),
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            field_0,
                        ) => InvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            soroban_sdk::IntoVal::into_val(field_0, env),
                        ),
                    })
                }
            }
        };
        pub enum Executable {
            Wasm(soroban_sdk::BytesN<32>),
            StellarAsset,
            Account,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Executable {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Executable::Wasm(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                    }
                    Executable::StellarAsset => {
                        ::core::fmt::Formatter::write_str(f, "StellarAsset")
                    }
                    Executable::Account => ::core::fmt::Formatter::write_str(f, "Account"),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Executable {
            #[inline]
            fn clone(&self) -> Executable {
                match self {
                    Executable::Wasm(__self_0) => {
                        Executable::Wasm(::core::clone::Clone::clone(__self_0))
                    }
                    Executable::StellarAsset => Executable::StellarAsset,
                    Executable::Account => Executable::Account,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Executable {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Executable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Executable {
            #[inline]
            fn eq(&self, other: &Executable) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Executable::Wasm(__self_0), Executable::Wasm(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Executable {
            #[inline]
            fn cmp(&self, other: &Executable) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (Executable::Wasm(__self_0), Executable::Wasm(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Executable {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Executable,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (Executable::Wasm(__self_0), Executable::Wasm(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_EXECUTABLE: [u8; 104usize] = Executable::spec_xdr();
        impl Executable {
            pub const fn spec_xdr() -> [u8; 104usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\nExecutable\0\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\0\0\0\0\0\0\0\0\x0cStellarAsset\0\0\0\0\0\0\0\0\0\0\0\x07Account\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for Executable {
            const SPEC_TYPE_ID: [u8; 32] = *b"L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_EXECUTABLE: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Executable {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &["Wasm", "StellarAsset", "Account"];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Wasm(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        1 => {
                            if iter.len() > 0 {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::StellarAsset
                        }
                        2 => {
                            if iter.len() > 0 {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Account
                        }
                        _ => Err(soroban_sdk::ConversionError {})?,
                    },
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, Executable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &Executable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    Executable::Wasm(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Wasm")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Executable::StellarAsset => {
                        let tup: (soroban_sdk::Val,) =
                            (soroban_sdk::Symbol::try_from_val(env, &"StellarAsset")?.to_val(),);
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Executable::Account => {
                        let tup: (soroban_sdk::Val,) =
                            (soroban_sdk::Symbol::try_from_val(env, &"Account")?.to_val(),);
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Executable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&Executable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Executable>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for Executable {
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
                    "Wasm" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Wasm(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "StellarAsset" => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::StellarAsset
                    }
                    "Account" => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::Account
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Executable {
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
        impl TryFrom<&Executable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    Executable::Wasm(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Wasm"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    Executable::StellarAsset => {
                        let symbol = soroban_sdk::xdr::ScSymbol(
                            "StellarAsset"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        );
                        let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                        (val,)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    }
                    Executable::Account => {
                        let symbol = soroban_sdk::xdr::ScSymbol(
                            "Account"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        );
                        let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                        (val,)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    }
                })
            }
        }
        impl TryFrom<Executable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&Executable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<Executable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryExecutable {
                Wasm(
                    <soroban_sdk::BytesN<
                        32,
                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                StellarAsset,
                Account,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryExecutable {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryExecutable::Wasm(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                        }
                        ArbitraryExecutable::StellarAsset => {
                            ::core::fmt::Formatter::write_str(f, "StellarAsset")
                        }
                        ArbitraryExecutable::Account => {
                            ::core::fmt::Formatter::write_str(f, "Account")
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryExecutable {
                #[inline]
                fn clone(&self) -> ArbitraryExecutable {
                    match self {
                        ArbitraryExecutable::Wasm(__self_0) => {
                            ArbitraryExecutable::Wasm(::core::clone::Clone::clone(__self_0))
                        }
                        ArbitraryExecutable::StellarAsset => ArbitraryExecutable::StellarAsset,
                        ArbitraryExecutable::Account => ArbitraryExecutable::Account,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryExecutable {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryExecutable {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryExecutable {
                #[inline]
                fn eq(&self, other: &ArbitraryExecutable) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryExecutable::Wasm(__self_0),
                                ArbitraryExecutable::Wasm(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryExecutable {
                #[inline]
                fn cmp(&self, other: &ArbitraryExecutable) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryExecutable::Wasm(__self_0),
                                ArbitraryExecutable::Wasm(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => ::core::cmp::Ordering::Equal,
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryExecutable {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryExecutable,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryExecutable::Wasm(__self_0),
                            ArbitraryExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryExecutable: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryExecutable {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    1u64 => ArbitraryExecutable::StellarAsset,
                                    2u64 => ArbitraryExecutable::Account,
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    1u64 => ArbitraryExecutable::StellarAsset,
                                    2u64 => ArbitraryExecutable::Account,
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
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
                                                    <<soroban_sdk::BytesN<
                                                        32,
                                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(&[]),
                                            arbitrary::size_hint::and_all(&[]),
                                        ],
                                    )
                            }),
                        )
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Executable {
                type Prototype = ArbitraryExecutable;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryExecutable> for Executable {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryExecutable,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryExecutable::Wasm(field_0) => {
                            Executable::Wasm(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                        ArbitraryExecutable::StellarAsset => Executable::StellarAsset,
                        ArbitraryExecutable::Account => Executable::Account,
                    })
                }
            }
        };
        pub enum UdtEnum2 {
            A = 10,
            B = 15,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UdtEnum2 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        UdtEnum2::A => "A",
                        UdtEnum2::B => "B",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for UdtEnum2 {}
        #[automatically_derived]
        impl ::core::clone::Clone for UdtEnum2 {
            #[inline]
            fn clone(&self) -> UdtEnum2 {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for UdtEnum2 {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UdtEnum2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UdtEnum2 {
            #[inline]
            fn eq(&self, other: &UdtEnum2) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for UdtEnum2 {
            #[inline]
            fn cmp(&self, other: &UdtEnum2) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for UdtEnum2 {
            #[inline]
            fn partial_cmp(
                &self,
                other: &UdtEnum2,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        pub static __SPEC_XDR_TYPE_UDTENUM2: [u8; 60usize] = UdtEnum2::spec_xdr();
        impl UdtEnum2 {
            pub const fn spec_xdr() -> [u8; 60usize] {
                *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08UdtEnum2\0\0\0\x02\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\n\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x0f"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for UdtEnum2 {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xaf\xf7\x93\xba\x9eM\xde\x9a?'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTENUM2: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"\xaf\xf7\x93\xba\x9eM\xde\x9a?'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UdtEnum2 {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::TryIntoVal;
                let discriminant: u32 = val.try_into_val(env)?;
                Ok(match discriminant {
                    10u32 => Self::A,
                    15u32 => Self::B,
                    _ => Err(soroban_sdk::ConversionError {})?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum2> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &UdtEnum2,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                Ok(match val {
                    UdtEnum2::A => 10u32.into(),
                    UdtEnum2::B => 15u32.into(),
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UdtEnum2> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&UdtEnum2,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UdtEnum2>>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for UdtEnum2 {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
                    Ok(match *discriminant {
                        10u32 => Self::A,
                        15u32 => Self::B,
                        _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                    })
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryInto<soroban_sdk::xdr::ScVal> for &UdtEnum2 {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
                Ok(match self {
                    UdtEnum2::A => 10u32.into(),
                    UdtEnum2::B => 15u32.into(),
                })
            }
        }
        impl TryInto<soroban_sdk::xdr::ScVal> for UdtEnum2 {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
                Ok(match self {
                    UdtEnum2::A => 10u32.into(),
                    UdtEnum2::B => 15u32.into(),
                })
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryUdtEnum2 {
                A,
                B,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryUdtEnum2 {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            ArbitraryUdtEnum2::A => "A",
                            ArbitraryUdtEnum2::B => "B",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryUdtEnum2 {
                #[inline]
                fn clone(&self) -> ArbitraryUdtEnum2 {
                    match self {
                        ArbitraryUdtEnum2::A => ArbitraryUdtEnum2::A,
                        ArbitraryUdtEnum2::B => ArbitraryUdtEnum2::B,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryUdtEnum2 {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryUdtEnum2 {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryUdtEnum2 {
                #[inline]
                fn eq(&self, other: &ArbitraryUdtEnum2) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryUdtEnum2 {
                #[inline]
                fn cmp(&self, other: &ArbitraryUdtEnum2) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryUdtEnum2 {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryUdtEnum2,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryUdtEnum2: ::std::thread::LocalKey<
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
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryUdtEnum2 {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 2u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryUdtEnum2::A,
                                    1u64 => ArbitraryUdtEnum2::B,
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
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
                            RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 2u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryUdtEnum2::A,
                                    1u64 => ArbitraryUdtEnum2::B,
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryUdtEnum2.with(|count| {
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
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for UdtEnum2 {
                type Prototype = ArbitraryUdtEnum2;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryUdtEnum2> for UdtEnum2 {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryUdtEnum2,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryUdtEnum2::A => UdtEnum2::A,
                        ArbitraryUdtEnum2::B => UdtEnum2::B,
                    })
                }
            }
        };
    }
    extern crate test;
    #[rustc_test_marker = "test_with_wasm::test_add"]
    #[doc(hidden)]
    pub const test_add: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_with_wasm::test_add"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 203usize,
            start_col: 8usize,
            end_line: 203usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_add()),
        ),
    };
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);
        let udt = contract::UdtStruct {
            a: 10,
            b: 12,
            c: ::soroban_sdk::Vec::from_array(&e, [1]),
        };
        let z = client.add(&contract::UdtEnum::UdtA, &contract::UdtEnum::UdtB(udt));
        match (&z, &22) {
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
        let udt1 = contract::UdtEnum2::A;
        let udt2 = contract::UdtTuple(1, ::soroban_sdk::Vec::from_array(&e, [2, 3]));
        let z = client.add(
            &contract::UdtEnum::UdtC(udt1),
            &contract::UdtEnum::UdtD(udt2),
        );
        match (&z, &16) {
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
    #[rustc_test_marker = "test_with_wasm::test_recursive"]
    #[doc(hidden)]
    pub const test_recursive: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_with_wasm::test_recursive"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 226usize,
            start_col: 8usize,
            end_line: 226usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_recursive()),
        ),
    };
    fn test_recursive() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);
        let recursive_udt_0 = contract::UdtRecursive {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("0");
                SYMBOL
            },
            b: ::soroban_sdk::Vec::new(&e),
        };
        let recursive_udt_1 = contract::UdtRecursive {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("1");
                SYMBOL
            },
            b: ::soroban_sdk::Vec::from_array(&e, [recursive_udt_0.clone()]),
        };
        let recursive_udt_2 = contract::UdtRecursive {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("2");
                SYMBOL
            },
            b: ::soroban_sdk::Vec::from_array(&e, [recursive_udt_1.clone()]),
        };
        let result_0 = client.recursive(&recursive_udt_2);
        match (&result_0, &Some(recursive_udt_1)) {
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
        let result_1 = client.recursive(&result_0.unwrap());
        match (&result_1, &Some(recursive_udt_0)) {
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
        let result_2 = client.recursive(&result_1.unwrap());
        match (&result_2, &None) {
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
    #[rustc_test_marker = "test_with_wasm::test_recursive_enum"]
    #[doc(hidden)]
    pub const test_recursive_enum: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_with_wasm::test_recursive_enum"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 255usize,
            start_col: 8usize,
            end_line: 255usize,
            end_col: 27usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_recursive_enum()),
        ),
    };
    fn test_recursive_enum() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);
        let entry = contract::RecursiveEnum::Recursive(contract::RecursiveToEnum {
            a: {
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test");
                SYMBOL
            },
            b: Map::from_array(&e, [(42u32, contract::RecursiveEnum::NotRecursive)]),
        });
        let result = client.recursive_enum(&entry, &42);
        match (&result, &Some(contract::RecursiveEnum::NotRecursive)) {
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
        let none_result = client.recursive_enum(&entry, &43);
        match (&none_result, &None) {
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
        &test_add,
        &test_recursive,
        &test_recursive_enum,
        &test_scval_accessibility_from_udt_types,
        &test_serializing,
        &test_add,
        &test_recursive,
        &test_recursive_enum,
    ])
}
