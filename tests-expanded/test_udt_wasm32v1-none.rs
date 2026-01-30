#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTENUM2: [u8; 60usize] = UdtEnum2::spec_xdr();
impl UdtEnum2 {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08UdtEnum2\0\0\0\x02\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\n\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x0f"
    }
}
impl soroban_sdk::IncludeSpecMarker for UdtEnum2 {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xaf\xf7\x93\xba\x9eM\xde\x9a";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTENUM: [u8; 156usize] = UdtEnum::spec_xdr();
impl UdtEnum {
    pub const fn spec_xdr() -> [u8; 156usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07UdtEnum\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x04UdtA\0\0\0\x01\0\0\0\0\0\0\0\x04UdtB\0\0\0\x01\0\0\x07\xd0\0\0\0\tUdtStruct\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04UdtC\0\0\0\x01\0\0\x07\xd0\0\0\0\x08UdtEnum2\0\0\0\x01\0\0\0\0\0\0\0\x04UdtD\0\0\0\x01\0\0\x07\xd0\0\0\0\x08UdtTuple"
    }
}
impl soroban_sdk::IncludeSpecMarker for UdtEnum {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <UdtStruct as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <UdtEnum2 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <UdtTuple as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xf3\xb0\xab@i\rH\xb4";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTTUPLE: [u8; 64usize] = UdtTuple::spec_xdr();
impl UdtTuple {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x08UdtTuple\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x011\0\0\0\0\0\x03\xea\0\0\0\x07"
    }
}
impl soroban_sdk::IncludeSpecMarker for UdtTuple {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <Vec<i64> as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\xeb\x9f\x12&\x9av(*";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UDTSTRUCT: [u8; 84usize] = UdtStruct::spec_xdr();
impl UdtStruct {
    pub const fn spec_xdr() -> [u8; 84usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\tUdtStruct\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01c\0\0\0\0\0\x03\xea\0\0\0\x07"
    }
}
impl soroban_sdk::IncludeSpecMarker for UdtStruct {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <Vec<i64> as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\x16'd8\xff\xc9\xb1\xf8";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
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
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
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
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
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
#[export_name = "add"]
pub extern "C" fn __Contract__add__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__add__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
