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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTENUM2: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    42usize,
    0usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTENUM: [u8; 138usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    138usize,
    3usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTTUPLE: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    42usize,
    0usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTSTRUCT: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    42usize,
    0usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_UDTRECURSIVE: [u8; 74usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    74usize,
    1usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_RECURSIVETOENUM: [u8; 74usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    74usize,
    1usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_RECURSIVEENUM: [u8; 74usize] =
    soroban_sdk::spec_shaking::spec_graph_record::<74usize, 1usize>(
        soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_ADD: [u8; 106usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    106usize,
    2usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_FUNCTION,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_RECURSIVE: [u8; 106usize] =
    soroban_sdk::spec_shaking::spec_graph_record::<106usize, 2usize>(
        soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_FUNCTION,
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
#[used]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_RECURSIVE_ENUM: [u8; 106usize] = soroban_sdk::spec_shaking::spec_graph_record::<
    106usize,
    2usize,
>(
    soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_FUNCTION,
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
        pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01N\r`\x01~\x01~`\x03~~~\x01~`\x02~~\x01~`\x04~~~~\x01~`\x02\x7f~\x00`\x02\x7f\x7f\x00`\x03~\x7f\x7f\x01~`\x02\x7f\x7f\x01\x7f`\x05~\x7f\x7f\x7f\x7f\x00`\x00\x00`\x01\x7f\x01~`\x03\x7f\x7f\x7f\x00`\x02\x7f\x7f\x01~\x02O\r\x01v\x013\x00\x00\x01v\x01h\x00\x01\x01i\x012\x00\x00\x01v\x011\x00\x02\x01i\x011\x00\x00\x01v\x018\x00\x00\x01m\x014\x00\x02\x01m\x011\x00\x02\x01v\x01g\x00\x02\x01b\x01j\x00\x02\x01m\x019\x00\x01\x01m\x01a\x00\x03\x01b\x01m\x00\x01\x03\x12\x11\x04\x05\x06\x07\x08\x04\x04\x04\x05\x02\t\x00\n\x02\x0b\x0c\t\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\xa8\x87\xc0\x00\x0b\x7f\x00A\xa8\x87\xc0\x00\x0b\x7f\x00A\xb0\x87\xc0\x00\x0b\x07L\x07\x06memory\x02\x00\x03add\x00\x16\trecursive\x00\x18\x0erecursive_enum\x00\x1a\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xaa\x1a\x11\xb6\x06\x04\x01\x7f\x01~\x02\x7f\x01~#\x80\x80\x80\x80\x00A\xc0\x00k\"\x02$\x80\x80\x80\x80\x00\x02@\x02@ \x01B\xff\x01\x83B\xcb\x00Q\r\x00 \x00A\x04:\x00\x00\x0c\x01\x0b \x01\x10\x80\x80\x80\x80\x00!\x03 \x02A\x006\x02\x10 \x02 \x017\x03\x08 \x02 \x03B \x88>\x02\x14 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00\x02@\x02@\x02@\x02@ \x02)\x03\x18\"\x01B\x02Q\r\x00 \x01\xa7A\x01q\r\x00\x02@ \x02)\x03 \"\x01\xa7A\xff\x01q\"\x04A\xca\x00F\r\x00 \x04A\x0eG\r\x01\x0b\x02@\x02@\x02@\x02@ \x01A\xb4\x86\xc0\x80\x00A\x04\x10\x8f\x80\x80\x80\x00B \x88\xa7\x0e\x04\x00\x01\x02\x03\x05\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00\r\x04A\x00!\x05\x0c\x05\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x03 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x01B\x02Q\r\x03 \x01\xa7A\x01q\r\x03 \x02)\x03 !\x01A\x00!\x04\x02@\x03@ \x04A\x18F\r\x01 \x02A\x18j \x04jB\x027\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b \x01B\xff\x01\x83B\xcc\x00R\r\x03 \x01A\xd8\x86\xc0\x80\x00A\x03 \x02A\x18jA\x03\x10\x91\x80\x80\x80\x00 \x02A0j \x02)\x03\x18\x10\x92\x80\x80\x80\x00 \x02(\x020\r\x03 \x02)\x038!\x03 \x02A0j \x02)\x03 \x10\x92\x80\x80\x80\x00 \x02(\x020\r\x03 \x02)\x03(\"\x06B\xff\x01\x83B\xcb\x00R\r\x03 \x02)\x038!\x01A\x01!\x05\x0c\x04\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x02 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x01B\x02Q\r\x02 \x01\xa7A\x01q\r\x02 \x02)\x03 \"\x01B\xff\x01\x83B\x04R\r\x02A\nA\x0fA\t \x01B \x88\xa7\"\x04A\x0fF\x1b \x04A\nF\x1b\"\x04A\tF\r\x02A\x02!\x05\x0c\x04\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x01 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x01B\x02Q\r\x01 \x01\xa7A\x01q\r\x01 \x02)\x03 \"\x01B\xff\x01\x83B\xcb\x00R\r\x01A\x00!\x04\x02@\x03@ \x04A\x10F\r\x01 \x02A0j \x04jB\x027\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b \x01 \x02A0j\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x81\x80\x80\x80\x00\x1a \x02A\x18j \x02)\x030\x10\x92\x80\x80\x80\x00 \x02(\x02\x18A\x01F\r\x01 \x02)\x038\"\x01B\xff\x01\x83B\xcb\x00R\r\x01 \x02)\x03 !\x03A\x03!\x05\x0c\x02\x0b \x00A\x04:\x00\x00\x0c\x03\x0b \x00A\x04:\x00\x00\x0c\x02\x0b\x0b \x00 \x067\x03\x18 \x00 \x017\x03\x10 \x00 \x037\x03\x08 \x00 \x04:\x00\x01 \x00 \x05:\x00\x00\x0b \x02A\xc0\x00j$\x80\x80\x80\x80\x00\x0bJ\x02\x01~\x01\x7fB\x02!\x02\x02@ \x01(\x02\x08\"\x03 \x01(\x02\x0cO\r\x00 \x00 \x01)\x03\x00 \x03\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x007\x03\x08 \x01 \x03A\x01j6\x02\x08B\x00!\x02\x0b \x00 \x027\x03\x00\x0b\x1c\x00 \x00 \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x8c\x80\x80\x80\x00\x0b\x19\x00\x02@ \x01 \x00I\r\x00 \x01 \x00k\x0f\x0b\x10\x97\x80\x80\x80\x00\x00\x0b1\x00\x02@ \x02 \x04F\r\x00\x00\x0b \x00 \x01\xadB \x86B\x04\x84 \x03\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x8b\x80\x80\x80\x00\x1a\x0b]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc1\x00F\r\x00\x02@ \x02A\x07F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x87!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x82\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0b\xb5\x01\x02\x02\x7f\x02~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00A\x00!\x03\x02@\x03@ \x03A\x10F\r\x01 \x02 \x03jB\x027\x03\x00 \x03A\x08j!\x03\x0c\x00\x0b\x0bB\x01!\x04\x02@ \x01B\xff\x01\x83B\xcc\x00R\r\x00 \x01A\xf0\x86\xc0\x80\x00A\x02 \x02A\x02\x10\x91\x80\x80\x80\x00\x02@ \x02)\x03\x00\"\x01\xa7A\xff\x01q\"\x03A\xca\x00F\r\x00 \x03A\x0eG\r\x01\x0b \x02)\x03\x08\"\x05B\xff\x01\x83B\xcb\x00R\r\x00 \x00 \x057\x03\x10 \x00 \x017\x03\x08B\x00!\x04\x0b \x00 \x047\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0b\xb5\x01\x02\x02\x7f\x02~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00A\x00!\x03\x02@\x03@ \x03A\x10F\r\x01 \x02 \x03jB\x027\x03\x00 \x03A\x08j!\x03\x0c\x00\x0b\x0bB\x01!\x04\x02@ \x01B\xff\x01\x83B\xcc\x00R\r\x00 \x01A\xf0\x86\xc0\x80\x00A\x02 \x02A\x02\x10\x91\x80\x80\x80\x00\x02@ \x02)\x03\x00\"\x01\xa7A\xff\x01q\"\x03A\xca\x00F\r\x00 \x03A\x0eG\r\x01\x0b \x02)\x03\x08\"\x05B\xff\x01\x83B\xcc\x00R\r\x00 \x00 \x057\x03\x10 \x00 \x017\x03\x08B\x00!\x04\x0b \x00 \x047\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0bx\x03\x01\x7f\x01~\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00B\x02!\x03\x02@ \x01(\x02\x08\"\x04 \x01(\x02\x0cO\r\x00 \x02 \x01)\x03\x00 \x04\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x00\x10\x92\x80\x80\x80\x00 \x02)\x03\x00!\x03 \x00 \x02)\x03\x087\x03\x08 \x01 \x04A\x01j6\x02\x08\x0b \x00 \x037\x03\x00 \x02A\x10j$\x80\x80\x80\x80\x00\x0b\xb7\x04\x04\x02\x7f\x02~\x01\x7f\x05~#\x80\x80\x80\x80\x00A0k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x8d\x80\x80\x80\x00\x02@\x02@\x02@ \x02-\x00\x00\"\x03A\x04F\r\x00 \x02)\x03\x10!\x00 \x02)\x03\x08!\x04 \x021\x00\x01!\x05 \x02 \x01\x10\x8d\x80\x80\x80\x00 \x02-\x00\x00\"\x06A\x04F\r\x00 \x02)\x03\x10!\x07 \x02)\x03\x08!\x08 \x021\x00\x01!\tB\x00!\nB\x00!\x0b\x02@\x02@\x02@\x02@ \x03\x0e\x04\x05\x02\x01\x00\x05\x0b \x00\x10\x80\x80\x80\x80\x00!\x01 \x02A\x006\x02( \x02 \x007\x03  \x02 \x01B \x88>\x02,B\x00!\x00\x03@ \x02 \x02A j\x10\x95\x80\x80\x80\x00 \x02)\x03\x00\"\x01B\x02Q\r\x03 \x01\xa7A\x01q\r\x06 \x02)\x03\x08\"\x01B\x00S \x00 \x01|\"\x01 \x00SG\r\x06 \x01!\x00\x0c\x00\x0b\x0b \x05!\x0b\x0c\x03\x0b \x00B\x00S \x04 \x00|\"\x0b \x04SsE\r\x02\x0c\x03\x0b \x00B\x00S \x04 \x00|\"\x0b \x04SsE\r\x01\x0c\x02\x0b\x00\x0b\x02@\x02@\x02@\x02@\x02@ \x06\x0e\x04\x04\x02\x01\x00\x04\x0b \x07\x10\x80\x80\x80\x80\x00!\x00 \x02A\x006\x02( \x02 \x077\x03  \x02 \x00B \x88>\x02,B\x00!\x00\x03@ \x02 \x02A j\x10\x95\x80\x80\x80\x00 \x02)\x03\x00\"\x01B\x02Q\r\x03 \x01\xa7A\x01q\r\x05 \x02)\x03\x08\"\x01B\x00S \x00 \x01|\"\x01 \x00SG\r\x05 \x01!\x00\x0c\x00\x0b\x0b \t!\n\x0c\x02\x0b \x07B\x00S \x08 \x07|\"\n \x08Ss\r\x02\x0c\x01\x0b \x00B\x00S \x08 \x00|\"\n \x08Ss\r\x01\x0b \nB\x00S \x0b \n|\"\x00 \x0bSs\r\x00\x02@\x02@ \x00B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00B\x08\x86B\x07\x84!\x00\x0c\x01\x0b \x00\x10\x84\x80\x80\x80\x00!\x00\x0b \x02A0j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x10\x97\x80\x80\x80\x00\x00\x0b\t\x00\x10\x9d\x80\x80\x80\x00\x00\x0b\x9f\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x01$\x80\x80\x80\x80\x00 \x01A\x08j \x00\x10\x93\x80\x80\x80\x00\x02@ \x01(\x02\x08A\x01F\r\x00B\x02!\x00\x02@ \x01)\x03\x18\"\x02\x10\x80\x80\x80\x80\x00B\x80\x80\x80\x80\x10T\r\x00 \x01A\x08j \x02\x10\x85\x80\x80\x80\x00\x10\x93\x80\x80\x80\x00 \x01(\x02\x08A\x01F\r\x01 \x01)\x03\x10!\x00 \x01 \x01)\x03\x187\x03\x10 \x01 \x007\x03\x08 \x01A\x08j\x10\x99\x80\x80\x80\x00!\x00\x0b \x01A j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b$\x00A\xf0\x86\xc0\x80\x00\xadB \x86B\x04\x84 \x00\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x8a\x80\x80\x80\x00\x0b\xf6\x05\x03\x01\x7f\x01~\x01\x7f#\x80\x80\x80\x80\x00A\xc0\x00k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00 \x00\x10\x80\x80\x80\x80\x00!\x03 \x02A\x006\x02\x10 \x02 \x007\x03\x08 \x02 \x03B \x88>\x02\x14 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x00B\x02Q\r\x00 \x00\xa7A\x01q\r\x00\x02@ \x02)\x03 \"\x00\xa7A\xff\x01q\"\x04A\xca\x00F\r\x00 \x04A\x0eG\r\x01\x0b\x02@\x02@\x02@\x02@\x02@\x02@ \x00A\x98\x87\xc0\x80\x00A\x02\x10\x8f\x80\x80\x80\x00B \x88\xa7\x0e\x02\x01\x00\x06\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x05 \x02A0j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x030\"\x00B\x02Q\r\x05 \x00\xa7A\x01q\r\x05 \x02A\x18j \x02)\x038\x10\x94\x80\x80\x80\x00 \x02(\x02\x18A\x01F\r\x05 \x01B\xff\x01\x83B\x04R\r\x05 \x02)\x03(\"\x00 \x01B\x84\x80\x80\x80p\x83\"\x01\x10\x86\x80\x80\x80\x00B\x01R\r\x01 \x00 \x01\x10\x87\x80\x80\x80\x00\"\x00B\xff\x01\x83B\xcb\x00R\r\x05 \x00\x10\x80\x80\x80\x80\x00!\x01 \x02A\x006\x02\x10 \x02 \x007\x03\x08 \x02 \x01B \x88>\x02\x14 \x02A\x18j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x03\x18\"\x00B\x02Q\r\x05 \x00\xa7A\x01q\r\x05\x02@ \x02)\x03 \"\x00\xa7A\xff\x01q\"\x04A\xca\x00F\r\x00 \x04A\x0eG\r\x06\x0b \x00A\x98\x87\xc0\x80\x00A\x02\x10\x8f\x80\x80\x80\x00B \x88\xa7\x0e\x02\x03\x02\x05\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00\r\x04 \x01B\xff\x01\x83B\x04R\r\x04\x0bB\x02!\x00\x0c\x02\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00A\x01K\r\x02 \x02A0j \x02A\x08j\x10\x8e\x80\x80\x80\x00 \x02)\x030\"\x00B\x02Q\r\x02 \x00\xa7A\x01q\r\x02 \x02A\x18j \x02)\x038\x10\x94\x80\x80\x80\x00 \x02(\x02\x18A\x01F\r\x02 \x02)\x03(!\x00 \x02)\x03 !\x01 \x02A\x18jA\x8c\x87\xc0\x80\x00A\t\x10\x9b\x80\x80\x80\x00 \x02(\x02\x18\r\x02 \x02)\x03 !\x03 \x02 \x007\x03  \x02 \x017\x03\x18 \x02 \x02A\x18j\x10\x99\x80\x80\x80\x007\x03  \x02 \x037\x03\x18 \x02A\x18jA\x02\x10\x9c\x80\x80\x80\x00!\x00\x0c\x01\x0b \x02(\x02\x10 \x02(\x02\x14\x10\x90\x80\x80\x80\x00\r\x01 \x02A\x18jA\x80\x87\xc0\x80\x00A\x0c\x10\x9b\x80\x80\x80\x00 \x02(\x02\x18\r\x01 \x02 \x02)\x03 7\x03\x18 \x02A\x18jA\x01\x10\x9c\x80\x80\x80\x00!\x00\x0b \x02A\xc0\x00j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\xd6\x01\x02\x01~\x03\x7f\x02@\x02@ \x02A\tK\r\x00B\x00!\x03A\x00!\x04\x03@\x02@ \x04A\tG\r\x00 \x03B\x08\x86B\x0e\x84!\x03\x0c\x03\x0bA\x01!\x05\x02@ \x01 \x04j-\x00\x00\"\x06A\xdf\x00F\r\x00\x02@\x02@ \x06APjA\xff\x01qA\nI\r\x00 \x06A\xbf\x7fjA\xff\x01qA\x1aI\r\x01 \x06A\x9f\x7fjA\xff\x01qA\x1aO\r\x04 \x06AEj!\x05\x0c\x02\x0b \x06ARj!\x05\x0c\x01\x0b \x06AKj!\x05\x0b \x03B\x06\x86 \x05\xadB\xff\x01\x83\x84!\x03 \x04A\x01j!\x04\x0c\x00\x0b\x0b \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x89\x80\x80\x80\x00!\x03\x0b \x00B\x007\x03\x00 \x00 \x037\x03\x08\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x88\x80\x80\x80\x00\x0b\x03\x00\x00\x0b\x0b\xb2\x07\x01\x00A\x80\x80\xc0\x00\x0b\xa8\x07SpGrV\x01\x00\x02\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2\x00\x03\x16\'d8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87\xaf\xf7\x93\xba\x9eM\xde\x9a?\'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xebSpGrV\x01\x00\x02\xaf\xf7\x93\xba\x9eM\xde\x9a?\'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e\x00\x00SpGrV\x01\x00\x02\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb\x00\x00SpGrV\x01\x00\x02\x16\'d8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87\x00\x00SpGrV\x01\x00\x00\xeb\xb9m\xe34\x1d[[\xe4K\xe7\xe3\xf4.\x99\x9b\xf2\x1a\xe15\xa1D+\xa8\x1b\x1cV\n\xed\xc1\xa4\x89\x00\x02\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2SpGrV\x01\x00\x02\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05\x00\x01\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05SpGrV\x01\x00\x02\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\x00\x01\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97CiSpGrV\x01\x00\x02\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci\x00\x01\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00MSpGrV\x01\x00\x00(`\x83Z;\x970\xd8\xdaZp\xcf\x9e\xbf\x82\x86|0\xb6\x90\x10Mf\x13\xcf\xd76\x0cDn\xdb\xb2\x00\x02\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05SpGrV\x01\x00\x00\x84H!\x0e\xfc\xdbM6\x02\xaaN\xe4\xee\x99J\x08\x94\x08\xa9\xc0D\x88Ci\xc9\x07~\xb9\xa6\xc5\xec\xaa\x00\x02\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00MUdtAUdtBUdtCUdtD$\x03\x10\x00\x04\x00\x00\x00(\x03\x10\x00\x04\x00\x00\x00,\x03\x10\x00\x04\x00\x00\x000\x03\x10\x00\x04\x00\x00\x00abc\x00T\x03\x10\x00\x01\x00\x00\x00U\x03\x10\x00\x01\x00\x00\x00V\x03\x10\x00\x01\x00\x00\x00T\x03\x10\x00\x01\x00\x00\x00U\x03\x10\x00\x01\x00\x00\x00NotRecursiveRecursive\x00\x00\x00\x80\x03\x10\x00\x0c\x00\x00\x00\x8c\x03\x10\x00\t\x00\x00\x00\x00\xc7\x07\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03add\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x07UdtEnum\x00\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x07UdtEnum\x00\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07UdtEnum\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04UdtA\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04UdtB\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\tUdtStruct\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04UdtC\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x08UdtEnum2\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04UdtD\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x08UdtTuple\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08UdtEnum2\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01A\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x01B\x00\x00\x00\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08UdtTuple\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x011\x00\x00\x00\x00\x00\x03\xea\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\tUdtStruct\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x01c\x00\x00\x00\x00\x00\x03\xea\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\trecursive\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x01\x00\x00\x03\xe8\x00\x00\x07\xd0\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x0cUdtRecursive\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cNotRecursive\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\tRecursive\x00\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fRecursiveToEnum\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fRecursiveToEnum\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x03\xec\x00\x00\x00\x04\x00\x00\x07\xd0\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0erecursive_enum\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x07\xd0\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03key\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x03\xe8\x00\x00\x07\xd0\x00\x00\x00\rRecursiveEnum\x00\x00\x00\x00\x00\x00\x03\x00\xc1\x06\x1ccontractspecv0.rssdk.graphv0SpGrV\x01\x00\x02\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2\x00\x03\x16\'d8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87\xaf\xf7\x93\xba\x9eM\xde\x9a?\'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xebSpGrV\x01\x00\x02\xaf\xf7\x93\xba\x9eM\xde\x9a?\'\xa8\xb9\xcb\x94\x9f\x88\xf6L\xc0\xb9\x18\xd1\xc9\x1a5\xf8\x99\xa59\xc2\xcf\x9e\x00\x00SpGrV\x01\x00\x02\xeb\x9f\x12&\x9av(*\x7f17.3\x91\xccc\xfc\xec\xee3\x0f\x96\xf2P\x1b9\xe8\xc6\x8f\xf0\xe0\xeb\x00\x00SpGrV\x01\x00\x02\x16\'d8\xff\xc9\xb1\xf8\x1cf\xb0\x84\xb1\xb8\xfay\x84`\xe4\xddp;\xc5*\x0e\xbaH:\x94\xbb\xb2\x87\x00\x00SpGrV\x01\x00\x00\xeb\xb9m\xe34\x1d[[\xe4K\xe7\xe3\xf4.\x99\x9b\xf2\x1a\xe15\xa1D+\xa8\x1b\x1cV\n\xed\xc1\xa4\x89\x00\x02\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2\xf3\xb0\xab@i\rH\xb4\x81\x9c\x94|?A\xef\xcf\xf3%Q\xd5\x8b\x90\xb2B\x18\xfb\x8c>\xaa\x8c^2SpGrV\x01\x00\x02\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05\x00\x01\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05SpGrV\x01\x00\x02\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\x00\x01\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97CiSpGrV\x01\x00\x02\xe1oU\xdb\xd47\x98\x14z\xb2+\xbb\xdf\xdbn\x14$\x92\xbb\xf1M\xf2\x10&P\x0c\xd1\x13J\x97Ci\x00\x01\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00MSpGrV\x01\x00\x00(`\x83Z;\x970\xd8\xdaZp\xcf\x9e\xbf\x82\x86|0\xb6\x90\x10Mf\x13\xcf\xd76\x0cDn\xdb\xb2\x00\x02\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05\xc8\x12\x91\xfe\xd7\x13\xf5\x9c\xe4\xc7\x03\xdc@#$F\r\x04\xe2j_\xb0\xacC\xfd\x0b\xc0J~<\xfc\x05SpGrV\x01\x00\x00\x84H!\x0e\xfc\xdbM6\x02\xaaN\xe4\xee\x99J\x08\x94\x08\xa9\xc0D\x88Ci\xc9\x07~\xb9\xa6\xc5\xec\xaa\x00\x02\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\xff{V \xab\r\xdcd\xe7~\x19\x83<\xc27t\xc6\x9d=\x9f\x8f\x12\x0e\x18>%\x08\x89.&\x00M\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTTUPLE: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            42usize,
            0usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTSTRUCT: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            42usize,
            0usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTRECURSIVE: [u8; 74usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            74usize,
            1usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_RECURSIVETOENUM: [u8; 74usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            74usize,
            1usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTENUM: [u8; 138usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            138usize,
            3usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_RECURSIVEENUM: [u8; 74usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            74usize,
            1usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
        #[used]
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_UDTENUM2: [u8; 42usize] = soroban_sdk::spec_shaking::spec_graph_record::<
            42usize,
            0usize,
        >(
            soroban_sdk::spec_shaking::GRAPH_RECORD_KIND_UDT,
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
