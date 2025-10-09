#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, contracttype, Vec};
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
    pub fn add(a: UdtEnum, b: &UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => *val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        a + b
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
                self.env.mock_all_auths();
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
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn add<'i>(a: &'i UdtEnum, b: &'i UdtEnum) -> (&'i UdtEnum, &'i UdtEnum) {
        (a, b)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::add(
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).add` instead")]
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
fn __Contract__7e9e5ac30f2216fd0fd6f5faed316f2d5983361a4203c3330cfa46ef65bb4767_ctor() {
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
                    __Contract__7e9e5ac30f2216fd0fd6f5faed316f2d5983361a4203c3330cfa46ef65bb4767_ctor();
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
            &__Contract__add::invoke_raw_slice,
        );
    }
}
mod test {
    use super::*;
    use soroban_sdk::{vec, xdr::ScVal, Bytes, Env, TryFromVal};
    extern crate test;
    #[rustc_test_marker = "test::test_serializing"]
    #[doc(hidden)]
    pub const test_serializing: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_serializing"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/udt/src/lib.rs",
            start_line: 60usize,
            start_col: 8usize,
            end_line: 60usize,
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
            start_line: 80usize,
            start_col: 8usize,
            end_line: 80usize,
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
            start_line: 100usize,
            start_col: 8usize,
            end_line: 100usize,
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_add,
        &test_scval_accessibility_from_udt_types,
        &test_serializing,
    ])
}
