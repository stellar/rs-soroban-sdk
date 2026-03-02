#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Env, Map, Symbol, Vec,
};
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
pub struct UsedParamStruct {
    pub a: u32,
    pub nested: UsedNestedInStruct,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedParamStruct {
    #[inline]
    fn clone(&self) -> UsedParamStruct {
        UsedParamStruct {
            a: ::core::clone::Clone::clone(&self.a),
            nested: ::core::clone::Clone::clone(&self.nested),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedParamStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedParamStruct",
            "a",
            &self.a,
            "nested",
            &&self.nested,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedParamStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<UsedNestedInStruct>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedParamStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedParamStruct {
    #[inline]
    fn eq(&self, other: &UsedParamStruct) -> bool {
        self.a == other.a && self.nested == other.nested
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDPARAMSTRUCT: [u8; 96usize] = UsedParamStruct::spec_xdr();
impl UsedParamStruct {
    pub const fn spec_xdr() -> [u8; 96usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fUsedParamStruct\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x06nested\0\0\0\0\x07\xd0\0\0\0\x12UsedNestedInStruct\0\0"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedParamStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "nested"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            a: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            nested: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedParamStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedParamStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["a", "nested"];
        let vals: [Val; 2usize] = [
            (&val.a).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.nested)
                .try_into_val(env)
                .map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedParamStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedParamStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedParamStruct>>::try_from_val(env, *val)
    }
}
pub enum UsedReturnEnum {
    A(u32),
    B(i64),
}
#[automatically_derived]
impl ::core::clone::Clone for UsedReturnEnum {
    #[inline]
    fn clone(&self) -> UsedReturnEnum {
        match self {
            UsedReturnEnum::A(__self_0) => UsedReturnEnum::A(::core::clone::Clone::clone(__self_0)),
            UsedReturnEnum::B(__self_0) => UsedReturnEnum::B(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedReturnEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            UsedReturnEnum::A(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A", &__self_0)
            }
            UsedReturnEnum::B(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedReturnEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedReturnEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedReturnEnum {
    #[inline]
    fn eq(&self, other: &UsedReturnEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (UsedReturnEnum::A(__self_0), UsedReturnEnum::A(__arg1_0)) => __self_0 == __arg1_0,
                (UsedReturnEnum::B(__self_0), UsedReturnEnum::B(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDRETURNENUM: [u8; 84usize] = UsedReturnEnum::spec_xdr();
impl UsedReturnEnum {
    pub const fn spec_xdr() -> [u8; 84usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x0eUsedReturnEnum\0\0\0\0\0\x02\0\0\0\x01\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x01\0\0\0\x07"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedReturnEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["A", "B"];
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
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::A(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::B(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedReturnEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedReturnEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            UsedReturnEnum::A(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"A")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            UsedReturnEnum::B(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"B")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedReturnEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedReturnEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedReturnEnum>>::try_from_val(env, *val)
    }
}
pub enum UsedParamIntEnum {
    X = 1,
    Y = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for UsedParamIntEnum {}
#[automatically_derived]
impl ::core::clone::Clone for UsedParamIntEnum {
    #[inline]
    fn clone(&self) -> UsedParamIntEnum {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedParamIntEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UsedParamIntEnum::X => "X",
                UsedParamIntEnum::Y => "Y",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedParamIntEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedParamIntEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedParamIntEnum {
    #[inline]
    fn eq(&self, other: &UsedParamIntEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDPARAMINTENUM: [u8; 68usize] = UsedParamIntEnum::spec_xdr();
impl UsedParamIntEnum {
    pub const fn spec_xdr() -> [u8; 68usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x10UsedParamIntEnum\0\0\0\x02\0\0\0\0\0\0\0\x01X\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01Y\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedParamIntEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::X,
            2u32 => Self::Y,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedParamIntEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedParamIntEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UsedParamIntEnum::X => 1u32.into(),
            UsedParamIntEnum::Y => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedParamIntEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedParamIntEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedParamIntEnum>>::try_from_val(env, *val)
    }
}
pub enum UsedErrorEnum {
    NotFound = 1,
    Invalid = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for UsedErrorEnum {}
#[automatically_derived]
impl ::core::clone::Clone for UsedErrorEnum {
    #[inline]
    fn clone(&self) -> UsedErrorEnum {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedErrorEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UsedErrorEnum::NotFound => "NotFound",
                UsedErrorEnum::Invalid => "Invalid",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedErrorEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedErrorEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedErrorEnum {
    #[inline]
    fn eq(&self, other: &UsedErrorEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDERRORENUM: [u8; 76usize] = UsedErrorEnum::spec_xdr();
impl UsedErrorEnum {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\rUsedErrorEnum\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x08NotFound\0\0\0\x01\0\0\0\0\0\0\0\x07Invalid\0\0\0\0\x02"
    }
}
impl TryFrom<soroban_sdk::Error> for UsedErrorEnum {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::NotFound,
                2u32 => Self::Invalid,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for UsedErrorEnum {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<UsedErrorEnum> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: UsedErrorEnum) -> soroban_sdk::Error {
        <_ as From<&UsedErrorEnum>>::from(&val)
    }
}
impl From<&UsedErrorEnum> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &UsedErrorEnum) -> soroban_sdk::Error {
        match val {
            UsedErrorEnum::NotFound => soroban_sdk::Error::from_contract_error(1u32),
            UsedErrorEnum::Invalid => soroban_sdk::Error::from_contract_error(2u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for UsedErrorEnum {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                1u32 => Self::NotFound,
                2u32 => Self::Invalid,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for UsedErrorEnum {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<UsedErrorEnum> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: UsedErrorEnum) -> soroban_sdk::InvokeError {
        <_ as From<&UsedErrorEnum>>::from(&val)
    }
}
impl From<&UsedErrorEnum> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &UsedErrorEnum) -> soroban_sdk::InvokeError {
        match val {
            UsedErrorEnum::NotFound => soroban_sdk::InvokeError::Contract(1u32),
            UsedErrorEnum::Invalid => soroban_sdk::InvokeError::Contract(2u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedErrorEnum {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedErrorEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedErrorEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedErrorEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedErrorEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedErrorEnum>>::try_from_val(env, *val)
    }
}
pub struct UsedNestedInStruct {
    pub val: i64,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedNestedInStruct {
    #[inline]
    fn clone(&self) -> UsedNestedInStruct {
        UsedNestedInStruct {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedNestedInStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedNestedInStruct",
            "val",
            &&self.val,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedNestedInStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedNestedInStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedNestedInStruct {
    #[inline]
    fn eq(&self, other: &UsedNestedInStruct) -> bool {
        self.val == other.val
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDNESTEDINSTRUCT: [u8; 56usize] = UsedNestedInStruct::spec_xdr();
impl UsedNestedInStruct {
    pub const fn spec_xdr() -> [u8; 56usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x12UsedNestedInStruct\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03val\0\0\0\0\x07"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedNestedInStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedNestedInStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedNestedInStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedNestedInStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedNestedInStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedNestedInStruct>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UsedVecElement {
    pub data: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedVecElement {
    #[inline]
    fn clone(&self) -> UsedVecElement {
        UsedVecElement {
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedVecElement {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsedVecElement", "data", &&self.data)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedVecElement {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedVecElement {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedVecElement {
    #[inline]
    fn eq(&self, other: &UsedVecElement) -> bool {
        self.data == other.data
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDVECELEMENT: [u8; 52usize] = UsedVecElement::spec_xdr();
impl UsedVecElement {
    pub const fn spec_xdr() -> [u8; 52usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0eUsedVecElement\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04data\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedVecElement {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["data"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            data: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedVecElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedVecElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["data"];
        let vals: [Val; 1usize] = [(&val.data).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedVecElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedVecElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedVecElement>>::try_from_val(env, *val)
    }
}
pub enum UsedMapKey {
    K1 = 1,
    K2 = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for UsedMapKey {}
#[automatically_derived]
impl ::core::clone::Clone for UsedMapKey {
    #[inline]
    fn clone(&self) -> UsedMapKey {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedMapKey {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UsedMapKey::K1 => "K1",
                UsedMapKey::K2 => "K2",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedMapKey {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedMapKey {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedMapKey {
    #[inline]
    fn eq(&self, other: &UsedMapKey) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDMAPKEY: [u8; 64usize] = UsedMapKey::spec_xdr();
impl UsedMapKey {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\nUsedMapKey\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02K1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02K2\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedMapKey {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::K1,
            2u32 => Self::K2,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedMapKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedMapKey,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UsedMapKey::K1 => 1u32.into(),
            UsedMapKey::K2 => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedMapKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedMapKey,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedMapKey>>::try_from_val(env, *val)
    }
}
pub struct UsedMapVal {
    pub v: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedMapVal {
    #[inline]
    fn clone(&self) -> UsedMapVal {
        UsedMapVal {
            v: ::core::clone::Clone::clone(&self.v),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedMapVal {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsedMapVal", "v", &&self.v)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedMapVal {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedMapVal {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedMapVal {
    #[inline]
    fn eq(&self, other: &UsedMapVal) -> bool {
        self.v == other.v
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDMAPVAL: [u8; 48usize] = UsedMapVal::spec_xdr();
impl UsedMapVal {
    pub const fn spec_xdr() -> [u8; 48usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\nUsedMapVal\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedMapVal {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["v"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            v: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedMapVal> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedMapVal,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["v"];
        let vals: [Val; 1usize] = [(&val.v).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedMapVal> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedMapVal,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedMapVal>>::try_from_val(env, *val)
    }
}
pub struct UsedOptionElement {
    pub data: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedOptionElement {
    #[inline]
    fn clone(&self) -> UsedOptionElement {
        UsedOptionElement {
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedOptionElement {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedOptionElement",
            "data",
            &&self.data,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedOptionElement {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedOptionElement {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedOptionElement {
    #[inline]
    fn eq(&self, other: &UsedOptionElement) -> bool {
        self.data == other.data
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDOPTIONELEMENT: [u8; 56usize] = UsedOptionElement::spec_xdr();
impl UsedOptionElement {
    pub const fn spec_xdr() -> [u8; 56usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x11UsedOptionElement\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04data\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedOptionElement {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["data"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            data: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedOptionElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedOptionElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["data"];
        let vals: [Val; 1usize] = [(&val.data).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedOptionElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedOptionElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedOptionElement>>::try_from_val(env, *val)
    }
}
pub struct UsedResultOk {
    pub data: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedResultOk {
    #[inline]
    fn clone(&self) -> UsedResultOk {
        UsedResultOk {
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedResultOk {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsedResultOk", "data", &&self.data)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedResultOk {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedResultOk {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedResultOk {
    #[inline]
    fn eq(&self, other: &UsedResultOk) -> bool {
        self.data == other.data
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDRESULTOK: [u8; 48usize] = UsedResultOk::spec_xdr();
impl UsedResultOk {
    pub const fn spec_xdr() -> [u8; 48usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cUsedResultOk\0\0\0\x01\0\0\0\0\0\0\0\x04data\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedResultOk {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["data"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            data: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedResultOk> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedResultOk,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["data"];
        let vals: [Val; 1usize] = [(&val.data).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedResultOk> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedResultOk,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedResultOk>>::try_from_val(env, *val)
    }
}
pub struct UsedEventSimple {
    pub kind: Symbol,
    pub amount: i128,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventSimple {
    #[inline]
    fn clone(&self) -> UsedEventSimple {
        UsedEventSimple {
            kind: ::core::clone::Clone::clone(&self.kind),
            amount: ::core::clone::Clone::clone(&self.amount),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventSimple {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventSimple",
            "kind",
            &self.kind,
            "amount",
            &&self.amount,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventSimple {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<i128>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventSimple {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventSimple {
    #[inline]
    fn eq(&self, other: &UsedEventSimple) -> bool {
        self.kind == other.kind && self.amount == other.amount
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_USEDEVENTSIMPLE: [u8; 112usize] = UsedEventSimple::spec_xdr();
impl UsedEventSimple {
    pub const fn spec_xdr() -> [u8; 112usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x0fUsedEventSimple\0\0\0\0\x01\0\0\0\x11used_event_simple\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x04kind\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::Event for UsedEventSimple {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "used_event_simple") }, {
            let v: soroban_sdk::Val = self.kind.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["amount"];
        let vals: [soroban_sdk::Val; 1usize] = [self.amount.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl UsedEventSimple {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub enum UsedEventTopicType {
    Transfer = 1,
    Mint = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for UsedEventTopicType {}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventTopicType {
    #[inline]
    fn clone(&self) -> UsedEventTopicType {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventTopicType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UsedEventTopicType::Transfer => "Transfer",
                UsedEventTopicType::Mint => "Mint",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventTopicType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventTopicType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventTopicType {
    #[inline]
    fn eq(&self, other: &UsedEventTopicType) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDEVENTTOPICTYPE: [u8; 76usize] = UsedEventTopicType::spec_xdr();
impl UsedEventTopicType {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x12UsedEventTopicType\0\0\0\0\0\x02\0\0\0\0\0\0\0\x08Transfer\0\0\0\x01\0\0\0\0\0\0\0\x04Mint\0\0\0\x02"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedEventTopicType {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::Transfer,
            2u32 => Self::Mint,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventTopicType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedEventTopicType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UsedEventTopicType::Transfer => 1u32.into(),
            UsedEventTopicType::Mint => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedEventTopicType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedEventTopicType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventTopicType>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UsedEventWithTopicType {
    pub kind: UsedEventTopicType,
    pub amount: i128,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventWithTopicType {
    #[inline]
    fn clone(&self) -> UsedEventWithTopicType {
        UsedEventWithTopicType {
            kind: ::core::clone::Clone::clone(&self.kind),
            amount: ::core::clone::Clone::clone(&self.amount),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventWithTopicType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventWithTopicType",
            "kind",
            &self.kind,
            "amount",
            &&self.amount,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventWithTopicType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UsedEventTopicType>;
        let _: ::core::cmp::AssertParamIsEq<i128>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventWithTopicType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventWithTopicType {
    #[inline]
    fn eq(&self, other: &UsedEventWithTopicType) -> bool {
        self.kind == other.kind && self.amount == other.amount
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_USEDEVENTWITHTOPICTYPE: [u8; 152usize] =
    UsedEventWithTopicType::spec_xdr();
impl UsedEventWithTopicType {
    pub const fn spec_xdr() -> [u8; 152usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x16UsedEventWithTopicType\0\0\0\0\0\x01\0\0\0\x1aused_event_with_topic_type\0\0\0\0\0\x02\0\0\0\0\0\0\0\x04kind\0\0\x07\xd0\0\0\0\x12UsedEventTopicType\0\0\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::Event for UsedEventWithTopicType {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{ soroban_sdk::Symbol::new(env, "used_event_with_topic_type") },
            {
                let v: soroban_sdk::Val = self.kind.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["amount"];
        let vals: [soroban_sdk::Val; 1usize] = [self.amount.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl UsedEventWithTopicType {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct UsedEventDataType {
    pub x: u32,
    pub y: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventDataType {
    #[inline]
    fn clone(&self) -> UsedEventDataType {
        UsedEventDataType {
            x: ::core::clone::Clone::clone(&self.x),
            y: ::core::clone::Clone::clone(&self.y),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventDataType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventDataType",
            "x",
            &self.x,
            "y",
            &&self.y,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventDataType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventDataType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventDataType {
    #[inline]
    fn eq(&self, other: &UsedEventDataType) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDEVENTDATATYPE: [u8; 72usize] = UsedEventDataType::spec_xdr();
impl UsedEventDataType {
    pub const fn spec_xdr() -> [u8; 72usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x11UsedEventDataType\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedEventDataType {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["x", "y"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventDataType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedEventDataType,
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedEventDataType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedEventDataType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventDataType>>::try_from_val(env, *val)
    }
}
pub struct UsedEventWithDataType {
    pub kind: Symbol,
    pub payload: UsedEventDataType,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventWithDataType {
    #[inline]
    fn clone(&self) -> UsedEventWithDataType {
        UsedEventWithDataType {
            kind: ::core::clone::Clone::clone(&self.kind),
            payload: ::core::clone::Clone::clone(&self.payload),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventWithDataType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventWithDataType",
            "kind",
            &self.kind,
            "payload",
            &&self.payload,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventWithDataType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<UsedEventDataType>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventWithDataType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventWithDataType {
    #[inline]
    fn eq(&self, other: &UsedEventWithDataType) -> bool {
        self.kind == other.kind && self.payload == other.payload
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_USEDEVENTWITHDATATYPE: [u8; 152usize] =
    UsedEventWithDataType::spec_xdr();
impl UsedEventWithDataType {
    pub const fn spec_xdr() -> [u8; 152usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x15UsedEventWithDataType\0\0\0\0\0\0\x01\0\0\0\x19used_event_with_data_type\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x04kind\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x07payload\0\0\0\x07\xd0\0\0\0\x11UsedEventDataType\0\0\0\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::Event for UsedEventWithDataType {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{ soroban_sdk::Symbol::new(env, "used_event_with_data_type") },
            {
                let v: soroban_sdk::Val = self.kind.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["payload"];
        let vals: [soroban_sdk::Val; 1usize] = [self.payload.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl UsedEventWithDataType {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct UsedEventTopicOuter {
    pub inner: UsedEventTopicInner,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventTopicOuter {
    #[inline]
    fn clone(&self) -> UsedEventTopicOuter {
        UsedEventTopicOuter {
            inner: ::core::clone::Clone::clone(&self.inner),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventTopicOuter {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedEventTopicOuter",
            "inner",
            &&self.inner,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventTopicOuter {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UsedEventTopicInner>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventTopicOuter {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventTopicOuter {
    #[inline]
    fn eq(&self, other: &UsedEventTopicOuter) -> bool {
        self.inner == other.inner
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDEVENTTOPICOUTER: [u8; 84usize] = UsedEventTopicOuter::spec_xdr();
impl UsedEventTopicOuter {
    pub const fn spec_xdr() -> [u8; 84usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x13UsedEventTopicOuter\0\0\0\0\x01\0\0\0\0\0\0\0\x05inner\0\0\0\0\0\x07\xd0\0\0\0\x13UsedEventTopicInner\0"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedEventTopicOuter {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["inner"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            inner: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventTopicOuter> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedEventTopicOuter,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["inner"];
        let vals: [Val; 1usize] = [(&val.inner)
            .try_into_val(env)
            .map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedEventTopicOuter> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedEventTopicOuter,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventTopicOuter>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UsedEventTopicInner {
    pub val: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventTopicInner {
    #[inline]
    fn clone(&self) -> UsedEventTopicInner {
        UsedEventTopicInner {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventTopicInner {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedEventTopicInner",
            "val",
            &&self.val,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventTopicInner {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventTopicInner {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventTopicInner {
    #[inline]
    fn eq(&self, other: &UsedEventTopicInner) -> bool {
        self.val == other.val
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDEVENTTOPICINNER: [u8; 56usize] = UsedEventTopicInner::spec_xdr();
impl UsedEventTopicInner {
    pub const fn spec_xdr() -> [u8; 56usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x13UsedEventTopicInner\0\0\0\0\x01\0\0\0\0\0\0\0\x03val\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedEventTopicInner {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventTopicInner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedEventTopicInner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedEventTopicInner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedEventTopicInner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventTopicInner>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UsedEventWithNestedTopic {
    pub info: UsedEventTopicOuter,
    pub amount: i128,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventWithNestedTopic {
    #[inline]
    fn clone(&self) -> UsedEventWithNestedTopic {
        UsedEventWithNestedTopic {
            info: ::core::clone::Clone::clone(&self.info),
            amount: ::core::clone::Clone::clone(&self.amount),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventWithNestedTopic {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventWithNestedTopic",
            "info",
            &self.info,
            "amount",
            &&self.amount,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventWithNestedTopic {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UsedEventTopicOuter>;
        let _: ::core::cmp::AssertParamIsEq<i128>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventWithNestedTopic {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventWithNestedTopic {
    #[inline]
    fn eq(&self, other: &UsedEventWithNestedTopic) -> bool {
        self.info == other.info && self.amount == other.amount
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_USEDEVENTWITHNESTEDTOPIC: [u8; 152usize] =
    UsedEventWithNestedTopic::spec_xdr();
impl UsedEventWithNestedTopic {
    pub const fn spec_xdr() -> [u8; 152usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x18UsedEventWithNestedTopic\0\0\0\x01\0\0\0\x1cused_event_with_nested_topic\0\0\0\x02\0\0\0\0\0\0\0\x04info\0\0\x07\xd0\0\0\0\x13UsedEventTopicOuter\0\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::Event for UsedEventWithNestedTopic {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{ soroban_sdk::Symbol::new(env, "used_event_with_nested_topic") },
            {
                let v: soroban_sdk::Val = self.info.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["amount"];
        let vals: [soroban_sdk::Val; 1usize] = [self.amount.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl UsedEventWithNestedTopic {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct UsedEventDataOuter {
    pub inner: UsedEventDataInner,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventDataOuter {
    #[inline]
    fn clone(&self) -> UsedEventDataOuter {
        UsedEventDataOuter {
            inner: ::core::clone::Clone::clone(&self.inner),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventDataOuter {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedEventDataOuter",
            "inner",
            &&self.inner,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventDataOuter {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UsedEventDataInner>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventDataOuter {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventDataOuter {
    #[inline]
    fn eq(&self, other: &UsedEventDataOuter) -> bool {
        self.inner == other.inner
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDEVENTDATAOUTER: [u8; 84usize] = UsedEventDataOuter::spec_xdr();
impl UsedEventDataOuter {
    pub const fn spec_xdr() -> [u8; 84usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x12UsedEventDataOuter\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05inner\0\0\0\0\0\x07\xd0\0\0\0\x12UsedEventDataInner\0\0"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedEventDataOuter {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["inner"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            inner: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventDataOuter> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedEventDataOuter,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["inner"];
        let vals: [Val; 1usize] = [(&val.inner)
            .try_into_val(env)
            .map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedEventDataOuter> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedEventDataOuter,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventDataOuter>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UsedEventDataInner {
    pub val: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventDataInner {
    #[inline]
    fn clone(&self) -> UsedEventDataInner {
        UsedEventDataInner {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventDataInner {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedEventDataInner",
            "val",
            &&self.val,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventDataInner {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventDataInner {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventDataInner {
    #[inline]
    fn eq(&self, other: &UsedEventDataInner) -> bool {
        self.val == other.val
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDEVENTDATAINNER: [u8; 56usize] = UsedEventDataInner::spec_xdr();
impl UsedEventDataInner {
    pub const fn spec_xdr() -> [u8; 56usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x12UsedEventDataInner\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03val\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedEventDataInner {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventDataInner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedEventDataInner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedEventDataInner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedEventDataInner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedEventDataInner>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UsedEventWithNestedData {
    pub kind: Symbol,
    pub payload: UsedEventDataOuter,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedEventWithNestedData {
    #[inline]
    fn clone(&self) -> UsedEventWithNestedData {
        UsedEventWithNestedData {
            kind: ::core::clone::Clone::clone(&self.kind),
            payload: ::core::clone::Clone::clone(&self.payload),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedEventWithNestedData {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventWithNestedData",
            "kind",
            &self.kind,
            "payload",
            &&self.payload,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedEventWithNestedData {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<UsedEventDataOuter>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedEventWithNestedData {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedEventWithNestedData {
    #[inline]
    fn eq(&self, other: &UsedEventWithNestedData) -> bool {
        self.kind == other.kind && self.payload == other.payload
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_USEDEVENTWITHNESTEDDATA: [u8; 152usize] =
    UsedEventWithNestedData::spec_xdr();
impl UsedEventWithNestedData {
    pub const fn spec_xdr() -> [u8; 152usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x17UsedEventWithNestedData\0\0\0\0\x01\0\0\0\x1bused_event_with_nested_data\0\0\0\0\x02\0\0\0\0\0\0\0\x04kind\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x07payload\0\0\0\x07\xd0\0\0\0\x12UsedEventDataOuter\0\0\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::Event for UsedEventWithNestedData {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{ soroban_sdk::Symbol::new(env, "used_event_with_nested_data") },
            {
                let v: soroban_sdk::Val = self.kind.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["payload"];
        let vals: [soroban_sdk::Val; 1usize] = [self.payload.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl UsedEventWithNestedData {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub enum UsedRefTopicType {
    Send = 1,
    Recv = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for UsedRefTopicType {}
#[automatically_derived]
impl ::core::clone::Clone for UsedRefTopicType {
    #[inline]
    fn clone(&self) -> UsedRefTopicType {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedRefTopicType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UsedRefTopicType::Send => "Send",
                UsedRefTopicType::Recv => "Recv",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedRefTopicType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedRefTopicType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedRefTopicType {
    #[inline]
    fn eq(&self, other: &UsedRefTopicType) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDREFTOPICTYPE: [u8; 68usize] = UsedRefTopicType::spec_xdr();
impl UsedRefTopicType {
    pub const fn spec_xdr() -> [u8; 68usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x10UsedRefTopicType\0\0\0\x02\0\0\0\0\0\0\0\x04Send\0\0\0\x01\0\0\0\0\0\0\0\x04Recv\0\0\0\x02"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedRefTopicType {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::Send,
            2u32 => Self::Recv,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedRefTopicType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedRefTopicType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UsedRefTopicType::Send => 1u32.into(),
            UsedRefTopicType::Recv => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedRefTopicType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedRefTopicType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedRefTopicType>>::try_from_val(env, *val)
    }
}
pub struct UsedRefDataType {
    pub nested: UsedRefDataInner,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedRefDataType {
    #[inline]
    fn clone(&self) -> UsedRefDataType {
        UsedRefDataType {
            nested: ::core::clone::Clone::clone(&self.nested),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedRefDataType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedRefDataType",
            "nested",
            &&self.nested,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedRefDataType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<UsedRefDataInner>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedRefDataType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedRefDataType {
    #[inline]
    fn eq(&self, other: &UsedRefDataType) -> bool {
        self.nested == other.nested
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDREFDATATYPE: [u8; 76usize] = UsedRefDataType::spec_xdr();
impl UsedRefDataType {
    pub const fn spec_xdr() -> [u8; 76usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fUsedRefDataType\0\0\0\0\x01\0\0\0\0\0\0\0\x06nested\0\0\0\0\x07\xd0\0\0\0\x10UsedRefDataInner"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedRefDataType {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["nested"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            nested: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedRefDataType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedRefDataType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["nested"];
        let vals: [Val; 1usize] = [(&val.nested)
            .try_into_val(env)
            .map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedRefDataType> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedRefDataType,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedRefDataType>>::try_from_val(env, *val)
    }
}
pub struct UsedRefDataInner {
    pub val: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedRefDataInner {
    #[inline]
    fn clone(&self) -> UsedRefDataInner {
        UsedRefDataInner {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedRefDataInner {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsedRefDataInner", "val", &&self.val)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedRefDataInner {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedRefDataInner {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedRefDataInner {
    #[inline]
    fn eq(&self, other: &UsedRefDataInner) -> bool {
        self.val == other.val
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDREFDATAINNER: [u8; 52usize] = UsedRefDataInner::spec_xdr();
impl UsedRefDataInner {
    pub const fn spec_xdr() -> [u8; 52usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x10UsedRefDataInner\0\0\0\x01\0\0\0\0\0\0\0\x03val\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedRefDataInner {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedRefDataInner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedRefDataInner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedRefDataInner> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedRefDataInner,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedRefDataInner>>::try_from_val(env, *val)
    }
}
pub struct UsedEventWithRefs<'a> {
    pub kind: &'a UsedRefTopicType,
    pub payload: &'a UsedRefDataType,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for UsedEventWithRefs<'a> {
    #[inline]
    fn clone(&self) -> UsedEventWithRefs<'a> {
        UsedEventWithRefs {
            kind: ::core::clone::Clone::clone(&self.kind),
            payload: ::core::clone::Clone::clone(&self.payload),
        }
    }
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for UsedEventWithRefs<'a> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UsedEventWithRefs",
            "kind",
            &self.kind,
            "payload",
            &&self.payload,
        )
    }
}
#[automatically_derived]
impl<'a> ::core::cmp::Eq for UsedEventWithRefs<'a> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<&'a UsedRefTopicType>;
        let _: ::core::cmp::AssertParamIsEq<&'a UsedRefDataType>;
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for UsedEventWithRefs<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for UsedEventWithRefs<'a> {
    #[inline]
    fn eq(&self, other: &UsedEventWithRefs<'a>) -> bool {
        self.kind == other.kind && self.payload == other.payload
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_USEDEVENTWITHREFS: [u8; 156usize] = UsedEventWithRefs::spec_xdr();
impl<'a> UsedEventWithRefs<'a> {
    pub const fn spec_xdr() -> [u8; 156usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x11UsedEventWithRefs\0\0\0\0\0\0\x01\0\0\0\x14used_event_with_refs\0\0\0\x02\0\0\0\0\0\0\0\x04kind\0\0\x07\xd0\0\0\0\x10UsedRefTopicType\0\0\0\x01\0\0\0\0\0\0\0\x07payload\0\0\0\x07\xd0\0\0\0\x0fUsedRefDataType\0\0\0\0\0\0\0\0\x02"
    }
}
impl<'a> soroban_sdk::Event for UsedEventWithRefs<'a> {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{ soroban_sdk::Symbol::new(env, "used_event_with_refs") },
            {
                let v: soroban_sdk::Val = self.kind.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["payload"];
        let vals: [soroban_sdk::Val; 1usize] = [self.payload.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl<'a> UsedEventWithRefs<'a> {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct UsedTupleElement {
    pub val: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedTupleElement {
    #[inline]
    fn clone(&self) -> UsedTupleElement {
        UsedTupleElement {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedTupleElement {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsedTupleElement", "val", &&self.val)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedTupleElement {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedTupleElement {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedTupleElement {
    #[inline]
    fn eq(&self, other: &UsedTupleElement) -> bool {
        self.val == other.val
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDTUPLEELEMENT: [u8; 52usize] = UsedTupleElement::spec_xdr();
impl UsedTupleElement {
    pub const fn spec_xdr() -> [u8; 52usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x10UsedTupleElement\0\0\0\x01\0\0\0\0\0\0\0\x03val\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedTupleElement {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedTupleElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedTupleElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedTupleElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedTupleElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedTupleElement>>::try_from_val(env, *val)
    }
}
pub struct UsedTupleReturnElement {
    pub val: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedTupleReturnElement {
    #[inline]
    fn clone(&self) -> UsedTupleReturnElement {
        UsedTupleReturnElement {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedTupleReturnElement {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UsedTupleReturnElement",
            "val",
            &&self.val,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedTupleReturnElement {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedTupleReturnElement {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedTupleReturnElement {
    #[inline]
    fn eq(&self, other: &UsedTupleReturnElement) -> bool {
        self.val == other.val
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_USEDTUPLERETURNELEMENT: [u8; 60usize] =
    UsedTupleReturnElement::spec_xdr();
impl UsedTupleReturnElement {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x16UsedTupleReturnElement\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03val\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedTupleReturnElement {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedTupleReturnElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedTupleReturnElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedTupleReturnElement> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedTupleReturnElement,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedTupleReturnElement>>::try_from_val(
            env, *val,
        )
    }
}
struct UsedNonPubStruct {
    pub val: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UsedNonPubStruct {
    #[inline]
    fn clone(&self) -> UsedNonPubStruct {
        UsedNonPubStruct {
            val: ::core::clone::Clone::clone(&self.val),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedNonPubStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsedNonPubStruct", "val", &&self.val)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedNonPubStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedNonPubStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedNonPubStruct {
    #[inline]
    fn eq(&self, other: &UsedNonPubStruct) -> bool {
        self.val == other.val
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedNonPubStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            val: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedNonPubStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedNonPubStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["val"];
        let vals: [Val; 1usize] = [(&val.val).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedNonPubStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedNonPubStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedNonPubStruct>>::try_from_val(env, *val)
    }
}
enum UsedNonPubError {
    Fail = 1,
}
#[automatically_derived]
impl ::core::marker::Copy for UsedNonPubError {}
#[automatically_derived]
impl ::core::clone::Clone for UsedNonPubError {
    #[inline]
    fn clone(&self) -> UsedNonPubError {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UsedNonPubError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Fail")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UsedNonPubError {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UsedNonPubError {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UsedNonPubError {
    #[inline]
    fn eq(&self, other: &UsedNonPubError) -> bool {
        true
    }
}
impl TryFrom<soroban_sdk::Error> for UsedNonPubError {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::Fail,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for UsedNonPubError {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<UsedNonPubError> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: UsedNonPubError) -> soroban_sdk::Error {
        <_ as From<&UsedNonPubError>>::from(&val)
    }
}
impl From<&UsedNonPubError> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &UsedNonPubError) -> soroban_sdk::Error {
        match val {
            UsedNonPubError::Fail => soroban_sdk::Error::from_contract_error(1u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for UsedNonPubError {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                1u32 => Self::Fail,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for UsedNonPubError {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<UsedNonPubError> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: UsedNonPubError) -> soroban_sdk::InvokeError {
        <_ as From<&UsedNonPubError>>::from(&val)
    }
}
impl From<&UsedNonPubError> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &UsedNonPubError) -> soroban_sdk::InvokeError {
        match val {
            UsedNonPubError::Fail => soroban_sdk::InvokeError::Contract(1u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UsedNonPubError {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UsedNonPubError> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UsedNonPubError,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UsedNonPubError> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UsedNonPubError,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UsedNonPubError>>::try_from_val(env, *val)
    }
}
mod imported {
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x019\n`\x01~\x01~`\x02~~\x01~`\x03~~~\x01~`\x02\x7f~\x00`\x03\x7f~~\x00`\x04\x7f\x7f\x7f\x7f\x01~`\x02\x7f\x7f\x01~`\x00\x01~`\x01\x7f\x00`\x01\x7f\x01~\x02I\x0c\x01i\x015\x00\x00\x01i\x014\x00\x00\x01i\x013\x00\x01\x01x\x011\x00\x01\x01i\x018\x00\x00\x01i\x017\x00\x00\x01i\x016\x00\x01\x01v\x01g\x00\x01\x01b\x01j\x00\x01\x01m\x019\x00\x02\x01i\x012\x00\x00\x01i\x011\x00\x00\x03\x16\x15\x03\x04\x01\x05\x01\x03\x03\x01\x06\x01\x07\x08\x00\x07\x07\x00\x00\x00\x01\t\x02\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x82\x80\xc0\x00\x0b\x7f\x00A\xbc\x80\xc0\x00\x0b\x7f\x00A\xc0\x80\xc0\x00\x0b\x07\xf5\x01\x11\x06memory\x02\x00\x0fcreate_struct_a\x00\x0e\x0fcreate_struct_b\x00\x10\x15create_struct_tuple_a\x00\x13\x15create_struct_tuple_b\x00\x15\nget_enum_a\x00\x16\nget_enum_b\x00\x18\x0eget_enum_int_a\x00\x19\x0eget_enum_int_b\x00\x1a\x07check_a\x00\x1b\x07check_b\x00\x1c\x07check_c\x00\x1d\x0cemit_event_a\x00\x1e\x0cemit_event_b\x00 \x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xa9\x12\x15{\x02\x01\x7f\x01~\x02@\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc4\x00F\r\x00 \x02A\nG\r\x01B\x00!\x03 \x00B\x007\x03\x10 \x00 \x01B\x08\x887\x03\x08\x0c\x02\x0b \x01\x10\x80\x80\x80\x80\x00!\x03 \x01\x10\x81\x80\x80\x80\x00!\x01 \x00 \x037\x03\x10 \x00 \x017\x03\x08B\x00!\x03\x0c\x01\x0b \x00B\x83\x90\x80\x80\x80\x017\x03\x08B\x01!\x03\x0b \x00 \x037\x03\x00\x0bF\x00\x02@\x02@ \x01B\xff\xff\xff\xff\xff\xff\xff\xff\x00V \x02B\x00R \x02P\x1b\r\x00 \x01B\x08\x86B\n\x84!\x02\x0c\x01\x0b \x02 \x01\x10\x82\x80\x80\x80\x00!\x02\x0b \x00B\x007\x03\x00 \x00 \x027\x03\x08\x0bz\x01\x02\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\x04R\r\x00A\x01 \x01\xa7A\xff\x01q\"\x03A\x00GA\x01t \x03A\x01F\x1b\"\x03A\x02F\r\x00 \x02 \x03\xad7\x03\x08 \x02 \x00B\x84\x80\x80\x80p\x837\x03\x00A\x88\x80\xc0\x80\x00A\x02 \x02A\x02\x10\x8f\x80\x80\x80\x00!\x00 \x02A\x10j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b.\x00\x02@ \x01 \x03F\r\x00\x00\x0b \x00\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x89\x80\x80\x80\x00\x0b\x8d\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00 \x02A\x10j \x00\x10\x91\x80\x80\x80\x00\x02@ \x02(\x02\x10A\x01F\r\x00 \x01B\xff\x01\x83B\xc9\x00R\r\x00 \x02A\x10j \x02)\x03\x18\x10\x92\x80\x80\x80\x00 \x02(\x02\x10A\x01F\r\x00 \x02)\x03\x18!\x00 \x02 \x017\x03\x08 \x02 \x007\x03\x00A\x88\x80\xc0\x80\x00A\x02 \x02A\x02\x10\x8f\x80\x80\x80\x00!\x01 \x02A j$\x80\x80\x80\x80\x00 \x01\x0f\x0b\x00\x0b]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc1\x00F\r\x00\x02@ \x02A\x07F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x87!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x8a\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0bF\x00\x02@\x02@ \x01B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x01B\x08\x86B\x07\x84!\x01\x0c\x01\x0b \x01\x10\x8b\x80\x80\x80\x00!\x01\x0b \x00B\x007\x03\x00 \x00 \x017\x03\x08\x0b\xb2\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00 \x02A\x10j \x00\x10\x91\x80\x80\x80\x00\x02@ \x02(\x02\x10A\x01F\r\x00 \x02)\x03\x18!\x00 \x02A\x10j \x01\x10\x91\x80\x80\x80\x00 \x02(\x02\x10A\x01F\r\x00 \x02)\x03\x18!\x01 \x02A\x10j \x00\x10\x92\x80\x80\x80\x00 \x02(\x02\x10\r\x00 \x02)\x03\x18!\x00 \x02A\x10j \x01\x10\x92\x80\x80\x80\x00 \x02(\x02\x10A\x01F\r\x00 \x02 \x02)\x03\x187\x03\x08 \x02 \x007\x03\x00 \x02A\x02\x10\x94\x80\x80\x80\x00!\x00 \x02A j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x87\x80\x80\x80\x00\x0b\xce\x01\x02\x02\x7f\x02~#\x80\x80\x80\x80\x00A0k\"\x02$\x80\x80\x80\x80\x00 \x02A\x08j \x00\x10\x8c\x80\x80\x80\x00\x02@ \x02(\x02\x08A\x01F\r\x00 \x02A\x18j\"\x03)\x03\x00!\x00 \x02)\x03\x10!\x04 \x02A\x08j \x01\x10\x8c\x80\x80\x80\x00 \x02(\x02\x08A\x01F\r\x00 \x03)\x03\x00!\x01 \x02)\x03\x10!\x05 \x02A\x08j \x04 \x00\x10\x8d\x80\x80\x80\x00 \x02(\x02\x08\r\x00 \x02)\x03\x10!\x00 \x02A\x08j \x05 \x01\x10\x8d\x80\x80\x80\x00 \x02(\x02\x08A\x01F\r\x00 \x02 \x02)\x03\x107\x03( \x02 \x007\x03  \x02A jA\x02\x10\x94\x80\x80\x80\x00!\x00 \x02A0j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0bP\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x00$\x80\x80\x80\x80\x00 \x00\x10\x97\x80\x80\x80\x00\x02@ \x00(\x02\x00A\x01G\r\x00\x00\x0b \x00 \x00)\x03\x087\x03\x00 \x00A\x01\x10\x94\x80\x80\x80\x00!\x01 \x00A\x10j$\x80\x80\x80\x80\x00 \x01\x0b\x83\x02\x03\x01\x7f\x01~\x03\x7f#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00B\x00!\x02A~!\x03\x02@\x02@\x02@\x03@ \x03E\r\x01A\x01!\x04\x02@ \x03A\x82\x80\xc0\x80\x00j-\x00\x00\"\x05A\xdf\x00F\r\x00\x02@ \x05APjA\xff\x01qA\nI\r\x00\x02@ \x05A\xbf\x7fjA\xff\x01qA\x1aI\r\x00 \x05A\x9f\x7fjA\xff\x01qA\x19K\r\x05 \x05AEj!\x04\x0c\x02\x0b \x05AKj!\x04\x0c\x01\x0b \x05ARj!\x04\x0b \x02B\x06\x86 \x04\xadB\xff\x01\x83\x84!\x02 \x03A\x01j!\x03\x0c\x00\x0b\x0b \x01 \x02B\x08\x86B\x0e\x84\"\x027\x02\x04\x0c\x01\x0b \x01 \x05\xadB\x08\x86B\x01\x847\x03\x00A\x80\x80\xc0\x80\x00\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x88\x80\x80\x80\x00!\x02\x0b \x00B\x007\x03\x00 \x00 \x027\x03\x08 \x01A\x10j$\x80\x80\x80\x80\x00\x0b\x8b\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00 \x01 \x00\x10\x91\x80\x80\x80\x00\x02@ \x01(\x02\x00A\x01F\r\x00 \x01)\x03\x08!\x00 \x01\x10\x97\x80\x80\x80\x00 \x01(\x02\x00\r\x00 \x01)\x03\x08!\x02 \x01 \x00\x10\x92\x80\x80\x80\x00 \x01(\x02\x00A\x01F\r\x00 \x01 \x01)\x03\x087\x03\x08 \x01 \x027\x03\x00 \x01A\x02\x10\x94\x80\x80\x80\x00!\x00 \x01A\x10j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\x08\x00B\x84\x80\x80\x800\x0b\t\x00B\x84\x80\x80\x80\xc0\x02\x0b*\x00\x02@ \x00B\xff\x01\x83B\x04Q\r\x00\x00\x0bB\x83\x80\x80\x80  \x00B\x84\x80\x80\x80p\x83 \x00B\x80\x80\x80\x80\x10T\x1b\x0b/\x00\x02@ \x00B\xff\x01\x83B\x04Q\r\x00\x00\x0bB\x83\x80\x80\x80\xc0\x01 \x00B\x84\x80\x80\x80\xf0\xff\x00\x83 \x00B\xff\xff\xff\xff\x8f\xfd\x00V\x1b\x0b,\x00\x02@ \x00B\xff\x01\x83B\x04Q\r\x00\x00\x0bB\x83\x80\x80\x80\xc0\x0c \x00B\x84\x80\x80\x80p\x83 \x00B\x80\x80\x80\x80\xa0\x01T\x1b\x0b\xe7\x01\x03\x01\x7f\x01~\x01\x7f#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\xc9\x00R\r\x00A\x98\x80\xc0\x80\x00\x10\x9f\x80\x80\x80\x00!\x03 \x02 \x007\x03\x08 \x02 \x037\x03\x00A\x00!\x04\x03@\x02@ \x04A\x10G\r\x00A\x00!\x04\x02@\x03@ \x04A\x10F\r\x01 \x02A\x10j \x04j \x02 \x04j)\x03\x007\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b \x02A\x10jA\x02\x10\x94\x80\x80\x80\x00!\x00 \x02 \x017\x03\x10 \x00A\xa0\x80\xc0\x80\x00A\x01 \x02A\x10jA\x01\x10\x8f\x80\x80\x80\x00\x10\x83\x80\x80\x80\x00\x1a \x02A j$\x80\x80\x80\x80\x00B\x02\x0f\x0b \x02A\x10j \x04jB\x027\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b\x00\x0b\x07\x00 \x00)\x03\x00\x0b\xf7\x02\x02\x02\x7f\x02~#\x80\x80\x80\x80\x00A0k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\xcd\x00R\r\x00\x02@\x02@ \x02\xa7A\xff\x01q\"\x04A\xc5\x00F\r\x00 \x04A\x0bG\r\x02 \x02B?\x87!\x05 \x02B\x08\x87!\x02\x0c\x01\x0b \x02\x10\x84\x80\x80\x80\x00!\x05 \x02\x10\x85\x80\x80\x80\x00!\x02\x0bA\xa8\x80\xc0\x80\x00\x10\x9f\x80\x80\x80\x00!\x06 \x03 \x017\x03\x10 \x03 \x007\x03\x08 \x03 \x067\x03\x00A\x00!\x04\x03@\x02@ \x04A\x18G\r\x00A\x00!\x04\x02@\x03@ \x04A\x18F\r\x01 \x03A\x18j \x04j \x03 \x04j)\x03\x007\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b \x03A\x18jA\x03\x10\x94\x80\x80\x80\x00!\x00\x02@\x02@ \x02B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x02 \x02\x85 \x05 \x02B?\x87\x85\x84B\x00R\r\x00 \x02B\x08\x86B\x0b\x84!\x02\x0c\x01\x0b \x05 \x02\x10\x86\x80\x80\x80\x00!\x02\x0b \x03 \x027\x03\x18 \x00A\xb4\x80\xc0\x80\x00A\x01 \x03A\x18jA\x01\x10\x8f\x80\x80\x80\x00\x10\x83\x80\x80\x80\x00\x1a \x03A0j$\x80\x80\x80\x80\x00B\x02\x0f\x0b \x03A\x18j \x04jB\x027\x03\x00 \x04A\x08j!\x04\x0c\x00\x0b\x0b\x00\x0b\x0bE\x01\x00A\x80\x80\xc0\x00\x0b<V2f1f2\x00\x00\x02\x00\x10\x00\x02\x00\x00\x00\x04\x00\x10\x00\x02\x00\x00\x00\x0ef\x90\xcf\xea\xae\x02\x00\x04\x00\x10\x00\x02\x00\x00\x00\x0eg\x90\xcf\xea\xae\x02\x00f3\x00\x000\x00\x10\x00\x02\x00\x00\x00\x00\xcf\x12\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fcreate_struct_a\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x07StructA\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fcreate_struct_b\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x07StructB\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x15create_struct_tuple_a\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0cStructTupleA\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x15create_struct_tuple_b\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0cStructTupleB\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nget_enum_a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x05EnumA\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nget_enum_b\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05value\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x05EnumB\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0eget_enum_int_a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x08EnumIntA\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0eget_enum_int_b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x08EnumIntB\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07check_a\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05input\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x04\x00\x00\x07\xd0\x00\x00\x00\x06ErrorA\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07check_b\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05input\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x04\x00\x00\x07\xd0\x00\x00\x00\x06ErrorB\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07check_c\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05input\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x04\x00\x00\x07\xd0\x00\x00\x00\x06ErrorC\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cemit_event_a\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cemit_event_b\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02f3\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07StructA\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07StructB\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07StructC\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x03\xea\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStructTupleA\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x011\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStructTupleB\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x011\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStructTupleC\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x010\x00\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x011\x00\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05EnumA\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02V1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02V2\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02V3\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05EnumB\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02V1\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02V2\x00\x00\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02V3\x00\x00\x00\x00\x00\x02\x00\x00\x00\x07\x00\x00\x00\x07\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05EnumC\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02V1\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02V2\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x07StructA\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02V3\x00\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0cStructTupleA\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08EnumIntA\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02V1\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02V2\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02V3\x00\x00\x00\x00\x00\x03\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08EnumIntB\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02V1\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x02V2\x00\x00\x00\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00\x02V3\x00\x00\x00\x00\x00\x1e\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08EnumIntC\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02V1\x00\x00\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00\x02V2\x00\x00\x00\x00\x00\xc8\x00\x00\x00\x00\x00\x00\x00\x02V3\x00\x00\x00\x00\x01,\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06ErrorA\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02E1\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02E2\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02E3\x00\x00\x00\x00\x00\x03\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06ErrorB\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02E1\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x02E2\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x02E3\x00\x00\x00\x00\x00\x0c\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06ErrorC\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02E1\x00\x00\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00\x02E2\x00\x00\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00\x02E3\x00\x00\x00\x00\x00f\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06EventA\x00\x00\x00\x00\x00\x01\x00\x00\x00\x07event_a\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06EventB\x00\x00\x00\x00\x00\x01\x00\x00\x00\x07event_b\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02f3\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06EventC\x00\x00\x00\x00\x00\x01\x00\x00\x00\x07event_c\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x02f1\x00\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02f2\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02f3\x00\x00\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x02\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x19\x00\x00\x00\x00\x00+\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.84.0\x00\x00";
    pub trait Contract {
        fn create_struct_a(env: soroban_sdk::Env, f1: u32, f2: bool) -> StructA;
        fn create_struct_b(env: soroban_sdk::Env, f1: i64, f2: soroban_sdk::String) -> StructB;
        fn create_struct_tuple_a(env: soroban_sdk::Env, f1: i64, f2: i64) -> StructTupleA;
        fn create_struct_tuple_b(env: soroban_sdk::Env, f1: u128, f2: u128) -> StructTupleB;
        fn get_enum_a(env: soroban_sdk::Env) -> EnumA;
        fn get_enum_b(env: soroban_sdk::Env, value: i64) -> EnumB;
        fn get_enum_int_a(env: soroban_sdk::Env) -> EnumIntA;
        fn get_enum_int_b(env: soroban_sdk::Env) -> EnumIntB;
        fn check_a(env: soroban_sdk::Env, input: u32) -> Result<u32, ErrorA>;
        fn check_b(env: soroban_sdk::Env, input: u32) -> Result<u32, ErrorB>;
        fn check_c(env: soroban_sdk::Env, input: u32) -> Result<u32, ErrorC>;
        fn emit_event_a(env: soroban_sdk::Env, f1: soroban_sdk::Address, f2: soroban_sdk::String);
        fn emit_event_b(
            env: soroban_sdk::Env,
            f1: soroban_sdk::Address,
            f2: soroban_sdk::Address,
            f3: i128,
        );
    }
    ///Client is a client for calling the contract defined in "Contract".
    pub struct Client<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> Client<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> Client<'a> {
        pub fn create_struct_a(&self, f1: &u32, f2: &bool) -> StructA {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn try_create_struct_a(
            &self,
            f1: &u32,
            f2: &bool,
        ) -> Result<
            Result<
                StructA,
                <StructA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn create_struct_b(&self, f1: &i64, f2: &soroban_sdk::String) -> StructB {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn try_create_struct_b(
            &self,
            f1: &i64,
            f2: &soroban_sdk::String,
        ) -> Result<
            Result<
                StructB,
                <StructB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn create_struct_tuple_a(&self, f1: &i64, f2: &i64) -> StructTupleA {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn try_create_struct_tuple_a(
            &self,
            f1: &i64,
            f2: &i64,
        ) -> Result<
            Result<
                StructTupleA,
                <StructTupleA as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        >{
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn create_struct_tuple_b(&self, f1: &u128, f2: &u128) -> StructTupleB {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn try_create_struct_tuple_b(
            &self,
            f1: &u128,
            f2: &u128,
        ) -> Result<
            Result<
                StructTupleB,
                <StructTupleB as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        >{
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn get_enum_a(&self) -> EnumA {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_get_enum_a(
            &self,
        ) -> Result<
            Result<
                EnumA,
                <EnumA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn get_enum_b(&self, value: &i64) -> EnumB {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
                ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
            );
            res
        }
        pub fn try_get_enum_b(
            &self,
            value: &i64,
        ) -> Result<
            Result<
                EnumB,
                <EnumB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
                ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
            );
            res
        }
        pub fn get_enum_int_a(&self) -> EnumIntA {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_get_enum_int_a(
            &self,
        ) -> Result<
            Result<
                EnumIntA,
                <EnumIntA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn get_enum_int_b(&self) -> EnumIntB {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_get_enum_int_b(
            &self,
        ) -> Result<
            Result<
                EnumIntB,
                <EnumIntB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn check_a(&self, input: &u32) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            res
        }
        pub fn try_check_a(
            &self,
            input: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<ErrorA, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            res
        }
        pub fn check_b(&self, input: &u32) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            res
        }
        pub fn try_check_b(
            &self,
            input: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<ErrorB, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            res
        }
        pub fn check_c(&self, input: &u32) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            res
        }
        pub fn try_check_c(
            &self,
            input: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<ErrorC, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            res
        }
        pub fn emit_event_a(&self, f1: &soroban_sdk::Address, f2: &soroban_sdk::String) -> () {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn try_emit_event_a(
            &self,
            f1: &soroban_sdk::Address,
            f2: &soroban_sdk::String,
        ) -> Result<
            Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn emit_event_b(
            &self,
            f1: &soroban_sdk::Address,
            f2: &soroban_sdk::Address,
            f3: &i128,
        ) -> () {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        f1.into_val(&self.env),
                        f2.into_val(&self.env),
                        f3.into_val(&self.env),
                    ],
                ),
            );
            res
        }
        pub fn try_emit_event_b(
            &self,
            f1: &soroban_sdk::Address,
            f2: &soroban_sdk::Address,
            f3: &i128,
        ) -> Result<
            Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        f1.into_val(&self.env),
                        f2.into_val(&self.env),
                        f3.into_val(&self.env),
                    ],
                ),
            );
            res
        }
    }
    ///Args is a type for building arg lists for functions defined in "Contract".
    pub struct Args;
    impl Args {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_a<'i>(f1: &'i u32, f2: &'i bool) -> (&'i u32, &'i bool) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_b<'i>(
            f1: &'i i64,
            f2: &'i soroban_sdk::String,
        ) -> (&'i i64, &'i soroban_sdk::String) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_tuple_a<'i>(f1: &'i i64, f2: &'i i64) -> (&'i i64, &'i i64) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_tuple_b<'i>(f1: &'i u128, f2: &'i u128) -> (&'i u128, &'i u128) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_a<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_b<'i>(value: &'i i64) -> (&'i i64,) {
            (value,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_int_a<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_int_b<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn check_a<'i>(input: &'i u32) -> (&'i u32,) {
            (input,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn check_b<'i>(input: &'i u32) -> (&'i u32,) {
            (input,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn check_c<'i>(input: &'i u32) -> (&'i u32,) {
            (input,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn emit_event_a<'i>(
            f1: &'i soroban_sdk::Address,
            f2: &'i soroban_sdk::String,
        ) -> (&'i soroban_sdk::Address, &'i soroban_sdk::String) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn emit_event_b<'i>(
            f1: &'i soroban_sdk::Address,
            f2: &'i soroban_sdk::Address,
            f3: &'i i128,
        ) -> (&'i soroban_sdk::Address, &'i soroban_sdk::Address, &'i i128) {
            (f1, f2, f3)
        }
    }
    pub struct StructA {
        pub f1: u32,
        pub f2: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StructA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "StructA", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructA {
        #[inline]
        fn clone(&self) -> StructA {
            StructA {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructA {
        #[inline]
        fn eq(&self, other: &StructA) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructA {
        #[inline]
        fn cmp(&self, other: &StructA) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructA {
        #[inline]
        fn partial_cmp(&self, other: &StructA) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructA {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                f1: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                f2: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let vals: [Val; 2usize] = [
                (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructA>>::try_from_val(env, *val)
        }
    }
    pub struct StructB {
        pub f1: i64,
        pub f2: soroban_sdk::String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StructB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "StructB", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructB {
        #[inline]
        fn clone(&self) -> StructB {
            StructB {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructB {
        #[inline]
        fn eq(&self, other: &StructB) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructB {
        #[inline]
        fn cmp(&self, other: &StructB) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructB {
        #[inline]
        fn partial_cmp(&self, other: &StructB) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructB {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                f1: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                f2: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let vals: [Val; 2usize] = [
                (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructB>>::try_from_val(env, *val)
        }
    }
    pub struct StructC {
        pub f1: soroban_sdk::Vec<u32>,
        pub f2: soroban_sdk::Address,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StructC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "StructC", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructC {
        #[inline]
        fn clone(&self) -> StructC {
            StructC {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<u32>>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructC {
        #[inline]
        fn eq(&self, other: &StructC) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructC {
        #[inline]
        fn cmp(&self, other: &StructC) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructC {
        #[inline]
        fn partial_cmp(&self, other: &StructC) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructC {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                f1: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                f2: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let vals: [Val; 2usize] = [
                (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructC>>::try_from_val(env, *val)
        }
    }
    pub struct StructTupleA(pub i64, pub i64);
    #[automatically_derived]
    impl ::core::fmt::Debug for StructTupleA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleA", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructTupleA {
        #[inline]
        fn clone(&self) -> StructTupleA {
            StructTupleA(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructTupleA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructTupleA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructTupleA {
        #[inline]
        fn eq(&self, other: &StructTupleA) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructTupleA {
        #[inline]
        fn cmp(&self, other: &StructTupleA) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructTupleA {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StructTupleA,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleA {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructTupleA,
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructTupleA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleA>>::try_from_val(env, *val)
        }
    }
    pub struct StructTupleB(pub u128, pub u128);
    #[automatically_derived]
    impl ::core::fmt::Debug for StructTupleB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleB", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructTupleB {
        #[inline]
        fn clone(&self) -> StructTupleB {
            StructTupleB(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructTupleB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructTupleB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructTupleB {
        #[inline]
        fn eq(&self, other: &StructTupleB) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructTupleB {
        #[inline]
        fn cmp(&self, other: &StructTupleB) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructTupleB {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StructTupleB,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleB {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructTupleB,
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructTupleB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleB>>::try_from_val(env, *val)
        }
    }
    pub struct StructTupleC(pub soroban_sdk::Address, pub i128);
    #[automatically_derived]
    impl ::core::fmt::Debug for StructTupleC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleC", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructTupleC {
        #[inline]
        fn clone(&self) -> StructTupleC {
            StructTupleC(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructTupleC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<i128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructTupleC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructTupleC {
        #[inline]
        fn eq(&self, other: &StructTupleC) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructTupleC {
        #[inline]
        fn cmp(&self, other: &StructTupleC) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructTupleC {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StructTupleC,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleC {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructTupleC,
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructTupleC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleC>>::try_from_val(env, *val)
        }
    }
    pub enum EnumA {
        V1,
        V2,
        V3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumA::V1 => "V1",
                    EnumA::V2 => "V2",
                    EnumA::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumA {
        #[inline]
        fn clone(&self) -> EnumA {
            match self {
                EnumA::V1 => EnumA::V1,
                EnumA::V2 => EnumA::V2,
                EnumA::V3 => EnumA::V3,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumA {
        #[inline]
        fn eq(&self, other: &EnumA) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumA {
        #[inline]
        fn cmp(&self, other: &EnumA) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumA {
        #[inline]
        fn partial_cmp(&self, other: &EnumA) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumA {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
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
                        Self::V1
                    }
                    1 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V2
                    }
                    2 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V3
                    }
                    _ => Err(soroban_sdk::ConversionError {})?,
                },
            )
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{TryFromVal, TryIntoVal};
            match val {
                EnumA::V1 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumA::V2 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumA::V3 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumA>>::try_from_val(env, *val)
        }
    }
    pub enum EnumB {
        V1,
        V2(i64),
        V3(i64, i64),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EnumB::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
                EnumB::V2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                }
                EnumB::V3(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "V3", __self_0, &__self_1)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumB {
        #[inline]
        fn clone(&self) -> EnumB {
            match self {
                EnumB::V1 => EnumB::V1,
                EnumB::V2(__self_0) => EnumB::V2(::core::clone::Clone::clone(__self_0)),
                EnumB::V3(__self_0, __self_1) => EnumB::V3(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumB {
        #[inline]
        fn eq(&self, other: &EnumB) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => __self_0 == __arg1_0,
                    (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                        __self_0 == __arg1_0 && __self_1 == __arg1_1
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumB {
        #[inline]
        fn cmp(&self, other: &EnumB) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                            }
                            cmp => cmp,
                        }
                    }
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumB {
        #[inline]
        fn partial_cmp(&self, other: &EnumB) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                    match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                        }
                        cmp => cmp,
                    }
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumB {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
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
                        Self::V1
                    }
                    1 => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V2(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                        )
                    }
                    2 => {
                        if iter.len() > 2usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V3(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{TryFromVal, TryIntoVal};
            match val {
                EnumB::V1 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumB::V2(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumB::V3(ref value0, ref value1) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),
                        value0.try_into_val(env)?,
                        value1.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumB>>::try_from_val(env, *val)
        }
    }
    pub enum EnumC {
        V1,
        V2(StructA),
        V3(StructTupleA),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EnumC::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
                EnumC::V2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                }
                EnumC::V3(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V3", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumC {
        #[inline]
        fn clone(&self) -> EnumC {
            match self {
                EnumC::V1 => EnumC::V1,
                EnumC::V2(__self_0) => EnumC::V2(::core::clone::Clone::clone(__self_0)),
                EnumC::V3(__self_0) => EnumC::V3(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<StructA>;
            let _: ::core::cmp::AssertParamIsEq<StructTupleA>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumC {
        #[inline]
        fn eq(&self, other: &EnumC) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => __self_0 == __arg1_0,
                    (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumC {
        #[inline]
        fn cmp(&self, other: &EnumC) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumC {
        #[inline]
        fn partial_cmp(&self, other: &EnumC) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumC {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
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
                        Self::V1
                    }
                    1 => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V2(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                        )
                    }
                    2 => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V3(
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{TryFromVal, TryIntoVal};
            match val {
                EnumC::V1 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumC::V2(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumC::V3(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumC>>::try_from_val(env, *val)
        }
    }
    pub enum EnumIntA {
        V1 = 1,
        V2 = 2,
        V3 = 3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumIntA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumIntA::V1 => "V1",
                    EnumIntA::V2 => "V2",
                    EnumIntA::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EnumIntA {}
    #[automatically_derived]
    impl ::core::clone::Clone for EnumIntA {
        #[inline]
        fn clone(&self) -> EnumIntA {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumIntA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumIntA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumIntA {
        #[inline]
        fn eq(&self, other: &EnumIntA) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumIntA {
        #[inline]
        fn cmp(&self, other: &EnumIntA) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumIntA {
        #[inline]
        fn partial_cmp(&self, other: &EnumIntA) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntA {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let discriminant: u32 = val.try_into_val(env)?;
            Ok(match discriminant {
                1u32 => Self::V1,
                2u32 => Self::V2,
                3u32 => Self::V3,
                _ => Err(soroban_sdk::ConversionError {})?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumIntA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            Ok(match val {
                EnumIntA::V1 => 1u32.into(),
                EnumIntA::V2 => 2u32.into(),
                EnumIntA::V3 => 3u32.into(),
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumIntA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntA>>::try_from_val(env, *val)
        }
    }
    pub enum EnumIntB {
        V1 = 10,
        V2 = 20,
        V3 = 30,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumIntB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumIntB::V1 => "V1",
                    EnumIntB::V2 => "V2",
                    EnumIntB::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EnumIntB {}
    #[automatically_derived]
    impl ::core::clone::Clone for EnumIntB {
        #[inline]
        fn clone(&self) -> EnumIntB {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumIntB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumIntB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumIntB {
        #[inline]
        fn eq(&self, other: &EnumIntB) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumIntB {
        #[inline]
        fn cmp(&self, other: &EnumIntB) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumIntB {
        #[inline]
        fn partial_cmp(&self, other: &EnumIntB) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntB {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let discriminant: u32 = val.try_into_val(env)?;
            Ok(match discriminant {
                10u32 => Self::V1,
                20u32 => Self::V2,
                30u32 => Self::V3,
                _ => Err(soroban_sdk::ConversionError {})?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumIntB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            Ok(match val {
                EnumIntB::V1 => 10u32.into(),
                EnumIntB::V2 => 20u32.into(),
                EnumIntB::V3 => 30u32.into(),
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumIntB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntB>>::try_from_val(env, *val)
        }
    }
    pub enum EnumIntC {
        V1 = 100,
        V2 = 200,
        V3 = 300,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumIntC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumIntC::V1 => "V1",
                    EnumIntC::V2 => "V2",
                    EnumIntC::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EnumIntC {}
    #[automatically_derived]
    impl ::core::clone::Clone for EnumIntC {
        #[inline]
        fn clone(&self) -> EnumIntC {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumIntC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumIntC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumIntC {
        #[inline]
        fn eq(&self, other: &EnumIntC) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumIntC {
        #[inline]
        fn cmp(&self, other: &EnumIntC) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumIntC {
        #[inline]
        fn partial_cmp(&self, other: &EnumIntC) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntC {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let discriminant: u32 = val.try_into_val(env)?;
            Ok(match discriminant {
                100u32 => Self::V1,
                200u32 => Self::V2,
                300u32 => Self::V3,
                _ => Err(soroban_sdk::ConversionError {})?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumIntC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            Ok(match val {
                EnumIntC::V1 => 100u32.into(),
                EnumIntC::V2 => 200u32.into(),
                EnumIntC::V3 => 300u32.into(),
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumIntC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntC>>::try_from_val(env, *val)
        }
    }
    pub enum ErrorA {
        E1 = 1,
        E2 = 2,
        E3 = 3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorA::E1 => "E1",
                    ErrorA::E2 => "E2",
                    ErrorA::E3 => "E3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorA {}
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorA {
        #[inline]
        fn clone(&self) -> ErrorA {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorA {
        #[inline]
        fn eq(&self, other: &ErrorA) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ErrorA {
        #[inline]
        fn cmp(&self, other: &ErrorA) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ErrorA {
        #[inline]
        fn partial_cmp(&self, other: &ErrorA) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl TryFrom<soroban_sdk::Error> for ErrorA {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    1u32 => Self::E1,
                    2u32 => Self::E2,
                    3u32 => Self::E3,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for ErrorA {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<ErrorA> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: ErrorA) -> soroban_sdk::Error {
            <_ as From<&ErrorA>>::from(&val)
        }
    }
    impl From<&ErrorA> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &ErrorA) -> soroban_sdk::Error {
            match val {
                ErrorA::E1 => soroban_sdk::Error::from_contract_error(1u32),
                ErrorA::E2 => soroban_sdk::Error::from_contract_error(2u32),
                ErrorA::E3 => soroban_sdk::Error::from_contract_error(3u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for ErrorA {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    1u32 => Self::E1,
                    2u32 => Self::E2,
                    3u32 => Self::E3,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for ErrorA {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<ErrorA> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: ErrorA) -> soroban_sdk::InvokeError {
            <_ as From<&ErrorA>>::from(&val)
        }
    }
    impl From<&ErrorA> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &ErrorA) -> soroban_sdk::InvokeError {
            match val {
                ErrorA::E1 => soroban_sdk::InvokeError::Contract(1u32),
                ErrorA::E2 => soroban_sdk::InvokeError::Contract(2u32),
                ErrorA::E3 => soroban_sdk::InvokeError::Contract(3u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorA {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ErrorA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ErrorA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorA>>::try_from_val(env, *val)
        }
    }
    pub enum ErrorB {
        E1 = 10,
        E2 = 11,
        E3 = 12,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorB::E1 => "E1",
                    ErrorB::E2 => "E2",
                    ErrorB::E3 => "E3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorB {}
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorB {
        #[inline]
        fn clone(&self) -> ErrorB {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorB {
        #[inline]
        fn eq(&self, other: &ErrorB) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ErrorB {
        #[inline]
        fn cmp(&self, other: &ErrorB) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ErrorB {
        #[inline]
        fn partial_cmp(&self, other: &ErrorB) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl TryFrom<soroban_sdk::Error> for ErrorB {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    10u32 => Self::E1,
                    11u32 => Self::E2,
                    12u32 => Self::E3,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for ErrorB {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<ErrorB> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: ErrorB) -> soroban_sdk::Error {
            <_ as From<&ErrorB>>::from(&val)
        }
    }
    impl From<&ErrorB> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &ErrorB) -> soroban_sdk::Error {
            match val {
                ErrorB::E1 => soroban_sdk::Error::from_contract_error(10u32),
                ErrorB::E2 => soroban_sdk::Error::from_contract_error(11u32),
                ErrorB::E3 => soroban_sdk::Error::from_contract_error(12u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for ErrorB {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    10u32 => Self::E1,
                    11u32 => Self::E2,
                    12u32 => Self::E3,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for ErrorB {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<ErrorB> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: ErrorB) -> soroban_sdk::InvokeError {
            <_ as From<&ErrorB>>::from(&val)
        }
    }
    impl From<&ErrorB> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &ErrorB) -> soroban_sdk::InvokeError {
            match val {
                ErrorB::E1 => soroban_sdk::InvokeError::Contract(10u32),
                ErrorB::E2 => soroban_sdk::InvokeError::Contract(11u32),
                ErrorB::E3 => soroban_sdk::InvokeError::Contract(12u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorB {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ErrorB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ErrorB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorB>>::try_from_val(env, *val)
        }
    }
    pub enum ErrorC {
        E1 = 100,
        E2 = 101,
        E3 = 102,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorC::E1 => "E1",
                    ErrorC::E2 => "E2",
                    ErrorC::E3 => "E3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorC {}
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorC {
        #[inline]
        fn clone(&self) -> ErrorC {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorC {
        #[inline]
        fn eq(&self, other: &ErrorC) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ErrorC {
        #[inline]
        fn cmp(&self, other: &ErrorC) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ErrorC {
        #[inline]
        fn partial_cmp(&self, other: &ErrorC) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl TryFrom<soroban_sdk::Error> for ErrorC {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    100u32 => Self::E1,
                    101u32 => Self::E2,
                    102u32 => Self::E3,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for ErrorC {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<ErrorC> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: ErrorC) -> soroban_sdk::Error {
            <_ as From<&ErrorC>>::from(&val)
        }
    }
    impl From<&ErrorC> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &ErrorC) -> soroban_sdk::Error {
            match val {
                ErrorC::E1 => soroban_sdk::Error::from_contract_error(100u32),
                ErrorC::E2 => soroban_sdk::Error::from_contract_error(101u32),
                ErrorC::E3 => soroban_sdk::Error::from_contract_error(102u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for ErrorC {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    100u32 => Self::E1,
                    101u32 => Self::E2,
                    102u32 => Self::E3,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for ErrorC {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<ErrorC> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: ErrorC) -> soroban_sdk::InvokeError {
            <_ as From<&ErrorC>>::from(&val)
        }
    }
    impl From<&ErrorC> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &ErrorC) -> soroban_sdk::InvokeError {
            match val {
                ErrorC::E1 => soroban_sdk::InvokeError::Contract(100u32),
                ErrorC::E2 => soroban_sdk::InvokeError::Contract(101u32),
                ErrorC::E3 => soroban_sdk::InvokeError::Contract(102u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorC {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ErrorC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ErrorC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorC>>::try_from_val(env, *val)
        }
    }
    pub struct EventA {
        pub f1: soroban_sdk::Address,
        pub f2: soroban_sdk::String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "EventA", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventA {
        #[inline]
        fn clone(&self) -> EventA {
            EventA {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventA {
        #[inline]
        fn eq(&self, other: &EventA) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EventA {
        #[inline]
        fn cmp(&self, other: &EventA) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EventA {
        #[inline]
        fn partial_cmp(&self, other: &EventA) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_EVENT_EVENTA: [u8; 88usize] = EventA::spec_xdr();
    impl EventA {
        pub const fn spec_xdr() -> [u8; 88usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventA\0\0\0\0\0\x01\0\0\0\x07event_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::Event for EventA {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_a");
                    SYMBOL
                },
                {
                    let v: soroban_sdk::Val = self.f1.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 1usize] = ["f2"];
            let vals: [soroban_sdk::Val; 1usize] = [self.f2.into_val(env)];
            env.map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl EventA {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct EventB {
        pub f1: soroban_sdk::Address,
        pub f2: soroban_sdk::Address,
        pub f3: i128,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f, "EventB", "f1", &self.f1, "f2", &self.f2, "f3", &&self.f3,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventB {
        #[inline]
        fn clone(&self) -> EventB {
            EventB {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
                f3: ::core::clone::Clone::clone(&self.f3),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<i128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventB {
        #[inline]
        fn eq(&self, other: &EventB) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2 && self.f3 == other.f3
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EventB {
        #[inline]
        fn cmp(&self, other: &EventB) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.f2, &other.f2) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f3, &other.f3),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EventB {
        #[inline]
        fn partial_cmp(&self, other: &EventB) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.f3, &other.f3)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_EVENT_EVENTB: [u8; 108usize] = EventB::spec_xdr();
    impl EventB {
        pub const fn spec_xdr() -> [u8; 108usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventB\0\0\0\0\0\x01\0\0\0\x07event_b\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::Event for EventB {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_b");
                    SYMBOL
                },
                {
                    let v: soroban_sdk::Val = self.f1.into_val(env);
                    v
                },
                {
                    let v: soroban_sdk::Val = self.f2.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 1usize] = ["f3"];
            let vals: [soroban_sdk::Val; 1usize] = [self.f3.into_val(env)];
            env.map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl EventB {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct EventC {
        pub f1: soroban_sdk::Symbol,
        pub f2: i64,
        pub f3: i64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f, "EventC", "f1", &self.f1, "f2", &self.f2, "f3", &&self.f3,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventC {
        #[inline]
        fn clone(&self) -> EventC {
            EventC {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
                f3: ::core::clone::Clone::clone(&self.f3),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventC {
        #[inline]
        fn eq(&self, other: &EventC) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2 && self.f3 == other.f3
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EventC {
        #[inline]
        fn cmp(&self, other: &EventC) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.f2, &other.f2) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f3, &other.f3),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EventC {
        #[inline]
        fn partial_cmp(&self, other: &EventC) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.f3, &other.f3)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_EVENT_EVENTC: [u8; 108usize] = EventC::spec_xdr();
    impl EventC {
        pub const fn spec_xdr() -> [u8; 108usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventC\0\0\0\0\0\x01\0\0\0\x07event_c\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::Event for EventC {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_c");
                    SYMBOL
                },
                {
                    let v: soroban_sdk::Val = self.f1.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 2usize] = ["f2", "f3"];
            let vals: [soroban_sdk::Val; 2usize] = [self.f2.into_val(env), self.f3.into_val(env)];
            env.map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl EventC {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
}
pub struct UnusedStruct {
    pub x: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UnusedStruct {
    #[inline]
    fn clone(&self) -> UnusedStruct {
        UnusedStruct {
            x: ::core::clone::Clone::clone(&self.x),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UnusedStruct", "x", &&self.x)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedStruct {
    #[inline]
    fn eq(&self, other: &UnusedStruct) -> bool {
        self.x == other.x
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UNUSEDSTRUCT: [u8; 48usize] = UnusedStruct::spec_xdr();
impl UnusedStruct {
    pub const fn spec_xdr() -> [u8; 48usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cUnusedStruct\0\0\0\x01\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            x: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let vals: [Val; 1usize] = [(&val.x).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedStruct>>::try_from_val(env, *val)
    }
}
pub enum UnusedEnum {
    A,
    B(i64),
}
#[automatically_derived]
impl ::core::clone::Clone for UnusedEnum {
    #[inline]
    fn clone(&self) -> UnusedEnum {
        match self {
            UnusedEnum::A => UnusedEnum::A,
            UnusedEnum::B(__self_0) => UnusedEnum::B(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            UnusedEnum::A => ::core::fmt::Formatter::write_str(f, "A"),
            UnusedEnum::B(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedEnum {
    #[inline]
    fn eq(&self, other: &UnusedEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (UnusedEnum::B(__self_0), UnusedEnum::B(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UNUSEDENUM: [u8; 72usize] = UnusedEnum::spec_xdr();
impl UnusedEnum {
    pub const fn spec_xdr() -> [u8; 72usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\nUnusedEnum\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x01\0\0\0\x07"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["A", "B"];
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
                    Self::A
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::B(
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            UnusedEnum::A => {
                let tup: (soroban_sdk::Val,) =
                    (soroban_sdk::Symbol::try_from_val(env, &"A")?.to_val(),);
                tup.try_into_val(env).map_err(Into::into)
            }
            UnusedEnum::B(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"B")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedEnum>>::try_from_val(env, *val)
    }
}
pub enum UnusedIntEnum {
    U1 = 1,
    U2 = 2,
}
#[automatically_derived]
impl ::core::marker::Copy for UnusedIntEnum {}
#[automatically_derived]
impl ::core::clone::Clone for UnusedIntEnum {
    #[inline]
    fn clone(&self) -> UnusedIntEnum {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedIntEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                UnusedIntEnum::U1 => "U1",
                UnusedIntEnum::U2 => "U2",
            },
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedIntEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedIntEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedIntEnum {
    #[inline]
    fn eq(&self, other: &UnusedIntEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UNUSEDINTENUM: [u8; 68usize] = UnusedIntEnum::spec_xdr();
impl UnusedIntEnum {
    pub const fn spec_xdr() -> [u8; 68usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\rUnusedIntEnum\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02U1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02U2\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedIntEnum {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::TryIntoVal;
        let discriminant: u32 = val.try_into_val(env)?;
        Ok(match discriminant {
            1u32 => Self::U1,
            2u32 => Self::U2,
            _ => Err(soroban_sdk::ConversionError {})?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedIntEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedIntEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        Ok(match val {
            UnusedIntEnum::U1 => 1u32.into(),
            UnusedIntEnum::U2 => 2u32.into(),
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedIntEnum> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedIntEnum,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedIntEnum>>::try_from_val(env, *val)
    }
}
pub struct UnusedEvent {
    pub kind: Symbol,
    pub data: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UnusedEvent {
    #[inline]
    fn clone(&self) -> UnusedEvent {
        UnusedEvent {
            kind: ::core::clone::Clone::clone(&self.kind),
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "UnusedEvent",
            "kind",
            &self.kind,
            "data",
            &&self.data,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedEvent {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Symbol>;
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedEvent {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedEvent {
    #[inline]
    fn eq(&self, other: &UnusedEvent) -> bool {
        self.kind == other.kind && self.data == other.data
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_UNUSEDEVENT: [u8; 96usize] = UnusedEvent::spec_xdr();
impl UnusedEvent {
    pub const fn spec_xdr() -> [u8; 96usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x0bUnusedEvent\0\0\0\0\x01\0\0\0\x0cunused_event\0\0\0\x02\0\0\0\0\0\0\0\x04kind\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x04data\0\0\0\x04\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::Event for UnusedEvent {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (&{ soroban_sdk::Symbol::new(env, "unused_event") }, {
            let v: soroban_sdk::Val = self.kind.into_val(env);
            v
        })
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["data"];
        let vals: [soroban_sdk::Val; 1usize] = [self.data.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl UnusedEvent {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
pub struct UnusedNonContractFnParam {
    pub x: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UnusedNonContractFnParam {
    #[inline]
    fn clone(&self) -> UnusedNonContractFnParam {
        UnusedNonContractFnParam {
            x: ::core::clone::Clone::clone(&self.x),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedNonContractFnParam {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UnusedNonContractFnParam",
            "x",
            &&self.x,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedNonContractFnParam {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedNonContractFnParam {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedNonContractFnParam {
    #[inline]
    fn eq(&self, other: &UnusedNonContractFnParam) -> bool {
        self.x == other.x
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UNUSEDNONCONTRACTFNPARAM: [u8; 60usize] =
    UnusedNonContractFnParam::spec_xdr();
impl UnusedNonContractFnParam {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x18UnusedNonContractFnParam\0\0\0\x01\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedNonContractFnParam {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            x: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonContractFnParam> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedNonContractFnParam,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let vals: [Val; 1usize] = [(&val.x).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedNonContractFnParam> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedNonContractFnParam,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonContractFnParam>>::try_from_val(
            env, *val,
        )
    }
}
pub struct UnusedNonContractFnReturn {
    pub x: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UnusedNonContractFnReturn {
    #[inline]
    fn clone(&self) -> UnusedNonContractFnReturn {
        UnusedNonContractFnReturn {
            x: ::core::clone::Clone::clone(&self.x),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedNonContractFnReturn {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UnusedNonContractFnReturn",
            "x",
            &&self.x,
        )
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedNonContractFnReturn {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedNonContractFnReturn {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedNonContractFnReturn {
    #[inline]
    fn eq(&self, other: &UnusedNonContractFnReturn) -> bool {
        self.x == other.x
    }
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_UNUSEDNONCONTRACTFNRETURN: [u8; 64usize] =
    UnusedNonContractFnReturn::spec_xdr();
impl UnusedNonContractFnReturn {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x19UnusedNonContractFnReturn\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x04"
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedNonContractFnReturn {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            x: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonContractFnReturn> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedNonContractFnReturn,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let vals: [Val; 1usize] = [(&val.x).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedNonContractFnReturn> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedNonContractFnReturn,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonContractFnReturn>>::try_from_val(
            env, *val,
        )
    }
}
struct UnusedNonPubStruct {
    pub x: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UnusedNonPubStruct {
    #[inline]
    fn clone(&self) -> UnusedNonPubStruct {
        UnusedNonPubStruct {
            x: ::core::clone::Clone::clone(&self.x),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedNonPubStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UnusedNonPubStruct", "x", &&self.x)
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedNonPubStruct {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedNonPubStruct {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedNonPubStruct {
    #[inline]
    fn eq(&self, other: &UnusedNonPubStruct) -> bool {
        self.x == other.x
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedNonPubStruct {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let mut vals: [Val; 1usize] = [Val::VOID.to_val(); 1usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            x: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonPubStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedNonPubStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 1usize] = ["x"];
        let vals: [Val; 1usize] = [(&val.x).try_into_val(env).map_err(|_| ConversionError)?];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedNonPubStruct> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedNonPubStruct,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonPubStruct>>::try_from_val(
            env, *val,
        )
    }
}
enum UnusedNonPubError {
    Bad = 1,
}
#[automatically_derived]
impl ::core::marker::Copy for UnusedNonPubError {}
#[automatically_derived]
impl ::core::clone::Clone for UnusedNonPubError {
    #[inline]
    fn clone(&self) -> UnusedNonPubError {
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UnusedNonPubError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Bad")
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UnusedNonPubError {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UnusedNonPubError {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UnusedNonPubError {
    #[inline]
    fn eq(&self, other: &UnusedNonPubError) -> bool {
        true
    }
}
impl TryFrom<soroban_sdk::Error> for UnusedNonPubError {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::Bad,
                _ => return Err(error),
            })
        } else {
            Err(error)
        }
    }
}
impl TryFrom<&soroban_sdk::Error> for UnusedNonPubError {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
    }
}
impl From<UnusedNonPubError> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: UnusedNonPubError) -> soroban_sdk::Error {
        <_ as From<&UnusedNonPubError>>::from(&val)
    }
}
impl From<&UnusedNonPubError> for soroban_sdk::Error {
    #[inline(always)]
    fn from(val: &UnusedNonPubError) -> soroban_sdk::Error {
        match val {
            UnusedNonPubError::Bad => soroban_sdk::Error::from_contract_error(1u32),
        }
    }
}
impl TryFrom<soroban_sdk::InvokeError> for UnusedNonPubError {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        match error {
            soroban_sdk::InvokeError::Abort => Err(error),
            soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                1u32 => Self::Bad,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for UnusedNonPubError {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
        <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
    }
}
impl From<UnusedNonPubError> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: UnusedNonPubError) -> soroban_sdk::InvokeError {
        <_ as From<&UnusedNonPubError>>::from(&val)
    }
}
impl From<&UnusedNonPubError> for soroban_sdk::InvokeError {
    #[inline(always)]
    fn from(val: &UnusedNonPubError) -> soroban_sdk::InvokeError {
        match val {
            UnusedNonPubError::Bad => soroban_sdk::InvokeError::Contract(1u32),
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for UnusedNonPubError {
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
impl soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonPubError> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &UnusedNonPubError,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        let error: soroban_sdk::Error = val.into();
        Ok(error.into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &UnusedNonPubError> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&UnusedNonPubError,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, UnusedNonPubError>>::try_from_val(env, *val)
    }
}
#[allow(private_interfaces)]
impl Contract {
    pub fn with_param(_env: Env, _s: UsedParamStruct, _ie: UsedParamIntEnum) {}
    pub fn with_return(_env: Env) -> UsedReturnEnum {
        UsedReturnEnum::A(1)
    }
    pub fn with_error(_env: Env) -> Result<u32, UsedErrorEnum> {
        Ok(42)
    }
    pub fn with_vec(_env: Env, _v: Vec<UsedVecElement>) {}
    pub fn with_map(_env: Env, _m: Map<UsedMapKey, UsedMapVal>) {}
    pub fn with_option(_env: Env, _o: Option<UsedOptionElement>) {}
    pub fn with_result(_env: Env) -> Result<UsedResultOk, UsedErrorEnum> {
        Ok(UsedResultOk { data: 1 })
    }
    pub fn publish_simple(env: Env) {
        UsedEventSimple {
            kind: Symbol::new(&env, "transfer"),
            amount: 100,
        }
        .publish(&env);
    }
    pub fn publish_topic_type(env: Env) {
        UsedEventWithTopicType {
            kind: UsedEventTopicType::Transfer,
            amount: 100,
        }
        .publish(&env);
    }
    pub fn publish_data_type(env: Env) {
        UsedEventWithDataType {
            kind: Symbol::new(&env, "coords"),
            payload: UsedEventDataType { x: 1, y: 2 },
        }
        .publish(&env);
    }
    pub fn publish_nested_topic(env: Env) {
        UsedEventWithNestedTopic {
            info: UsedEventTopicOuter {
                inner: UsedEventTopicInner { val: 42 },
            },
            amount: 100,
        }
        .publish(&env);
    }
    pub fn publish_nested_data(env: Env) {
        UsedEventWithNestedData {
            kind: Symbol::new(&env, "nested"),
            payload: UsedEventDataOuter {
                inner: UsedEventDataInner { val: 42 },
            },
        }
        .publish(&env);
    }
    pub fn with_imported(_env: Env, _s: imported::StructA) {}
    pub fn with_non_pub(_env: Env, _s: UsedNonPubStruct) {}
    pub fn with_non_pub_error(_env: Env) -> Result<u32, UsedNonPubError> {
        Ok(1)
    }
    pub fn with_tuple(_env: Env, _t: (UsedTupleElement, u32)) {}
    pub fn with_tuple_return(_env: Env) -> (UsedTupleReturnElement, u32) {
        (UsedTupleReturnElement { val: 1 }, 2)
    }
    pub fn publish_ref_event(env: Env) {
        let kind = UsedRefTopicType::Send;
        let payload = UsedRefDataType {
            nested: UsedRefDataInner { val: 99 },
        };
        UsedEventWithRefs {
            kind: &kind,
            payload: &payload,
        }
        .publish(&env);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_param__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_PARAM: [u8; 104usize] = super::Contract::spec_xdr_with_param();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_param() -> [u8; 104usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nwith_param\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01s\0\0\0\0\0\x07\xd0\0\0\0\x0fUsedParamStruct\0\0\0\0\0\0\0\0\x02ie\0\0\0\0\x07\xd0\0\0\0\x10UsedParamIntEnum\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_return__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_RETURN: [u8; 56usize] = super::Contract::spec_xdr_with_return();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_return() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0bwith_return\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0eUsedReturnEnum\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_error__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_ERROR: [u8; 64usize] = super::Contract::spec_xdr_with_error();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_error() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nwith_error\0\0\0\0\0\0\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\rUsedErrorEnum\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_vec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_VEC: [u8; 68usize] = super::Contract::spec_xdr_with_vec();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_vec() -> [u8; 68usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08with_vec\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x0eUsedVecElement\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_map__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_MAP: [u8; 84usize] = super::Contract::spec_xdr_with_map();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_map() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08with_map\0\0\0\x01\0\0\0\0\0\0\0\x01m\0\0\0\0\0\x03\xec\0\0\x07\xd0\0\0\0\nUsedMapKey\0\0\0\0\x07\xd0\0\0\0\nUsedMapVal\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_option__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_OPTION: [u8; 76usize] = super::Contract::spec_xdr_with_option();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_option() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0bwith_option\0\0\0\0\x01\0\0\0\0\0\0\0\x01o\0\0\0\0\0\x03\xe8\0\0\x07\xd0\0\0\0\x11UsedOptionElement\0\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_result__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_RESULT: [u8; 80usize] = super::Contract::spec_xdr_with_result();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_result() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0bwith_result\0\0\0\0\0\0\0\0\x01\0\0\x03\xe9\0\0\x07\xd0\0\0\0\x0cUsedResultOk\0\0\x07\xd0\0\0\0\rUsedErrorEnum\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish_simple__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH_SIMPLE: [u8; 36usize] =
        super::Contract::spec_xdr_publish_simple();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish_simple() -> [u8; 36usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0epublish_simple\0\0\0\0\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish_topic_type__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH_TOPIC_TYPE: [u8; 40usize] =
        super::Contract::spec_xdr_publish_topic_type();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish_topic_type() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x12publish_topic_type\0\0\0\0\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish_data_type__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH_DATA_TYPE: [u8; 40usize] =
        super::Contract::spec_xdr_publish_data_type();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish_data_type() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11publish_data_type\0\0\0\0\0\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish_nested_topic__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH_NESTED_TOPIC: [u8; 40usize] =
        super::Contract::spec_xdr_publish_nested_topic();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish_nested_topic() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x14publish_nested_topic\0\0\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish_nested_data__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH_NESTED_DATA: [u8; 40usize] =
        super::Contract::spec_xdr_publish_nested_data();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish_nested_data() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x13publish_nested_data\0\0\0\0\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_imported__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_IMPORTED: [u8; 64usize] =
        super::Contract::spec_xdr_with_imported();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_imported() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rwith_imported\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01s\0\0\0\0\0\x07\xd0\0\0\0\x07StructA\0\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_non_pub__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_NON_PUB: [u8; 68usize] = super::Contract::spec_xdr_with_non_pub();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_non_pub() -> [u8; 68usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cwith_non_pub\0\0\0\x01\0\0\0\0\0\0\0\x01s\0\0\0\0\0\x07\xd0\0\0\0\x10UsedNonPubStruct\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_non_pub_error__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_NON_PUB_ERROR: [u8; 72usize] =
        super::Contract::spec_xdr_with_non_pub_error();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_non_pub_error() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x12with_non_pub_error\0\0\0\0\0\0\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x0fUsedNonPubError\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_tuple__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_TUPLE: [u8; 80usize] = super::Contract::spec_xdr_with_tuple();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_tuple() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nwith_tuple\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01t\0\0\0\0\0\x03\xed\0\0\0\x02\0\0\x07\xd0\0\0\0\x10UsedTupleElement\0\0\0\x04\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__with_tuple_return__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WITH_TUPLE_RETURN: [u8; 84usize] =
        super::Contract::spec_xdr_with_tuple_return();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_with_tuple_return() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11with_tuple_return\0\0\0\0\0\0\0\0\0\0\x01\0\0\x03\xed\0\0\0\x02\0\0\x07\xd0\0\0\0\x16UsedTupleReturnElement\0\0\0\0\0\x04"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__publish_ref_event__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUBLISH_REF_EVENT: [u8; 40usize] =
        super::Contract::spec_xdr_publish_ref_event();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_publish_ref_event() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11publish_ref_event\0\0\0\0\0\0\0\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn with_param(&self, _s: &UsedParamStruct, _ie: &UsedParamIntEnum) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_param") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [_s.into_val(&self.env), _ie.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_with_param(
        &self,
        _s: &UsedParamStruct,
        _ie: &UsedParamIntEnum,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_param") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [_s.into_val(&self.env), _ie.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn with_return(&self) -> UsedReturnEnum {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_return") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_with_return(
        &self,
    ) -> Result<
        Result<
            UsedReturnEnum,
            <UsedReturnEnum as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_return") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn with_error(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_error") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_with_error(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<UsedErrorEnum, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_error") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn with_vec(&self, _v: &Vec<UsedVecElement>) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("with_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_with_vec(
        &self,
        _v: &Vec<UsedVecElement>,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("with_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_v.into_val(&self.env)]),
        );
        res
    }
    pub fn with_map(&self, _m: &Map<UsedMapKey, UsedMapVal>) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("with_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_m.into_val(&self.env)]),
        );
        res
    }
    pub fn try_with_map(
        &self,
        _m: &Map<UsedMapKey, UsedMapVal>,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("with_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_m.into_val(&self.env)]),
        );
        res
    }
    pub fn with_option(&self, _o: &Option<UsedOptionElement>) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_option") },
            ::soroban_sdk::Vec::from_array(&self.env, [_o.into_val(&self.env)]),
        );
        res
    }
    pub fn try_with_option(
        &self,
        _o: &Option<UsedOptionElement>,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_option") },
            ::soroban_sdk::Vec::from_array(&self.env, [_o.into_val(&self.env)]),
        );
        res
    }
    pub fn with_result(&self) -> UsedResultOk {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_result") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_with_result(
        &self,
    ) -> Result<
        Result<
            UsedResultOk,
            <UsedResultOk as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<UsedErrorEnum, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_result") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn publish_simple(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_simple") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_publish_simple(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_simple") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn publish_topic_type(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_topic_type") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_publish_topic_type(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_topic_type") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn publish_data_type(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_data_type") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_publish_data_type(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_data_type") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn publish_nested_topic(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_nested_topic") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_publish_nested_topic(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_nested_topic") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn publish_nested_data(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_nested_data") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_publish_nested_data(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_nested_data") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn with_imported(&self, _s: &imported::StructA) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_imported") },
            ::soroban_sdk::Vec::from_array(&self.env, [_s.into_val(&self.env)]),
        );
        res
    }
    pub fn try_with_imported(
        &self,
        _s: &imported::StructA,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_imported") },
            ::soroban_sdk::Vec::from_array(&self.env, [_s.into_val(&self.env)]),
        );
        res
    }
    pub fn with_non_pub(&self, _s: &UsedNonPubStruct) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_non_pub") },
            ::soroban_sdk::Vec::from_array(&self.env, [_s.into_val(&self.env)]),
        );
        res
    }
    pub fn try_with_non_pub(
        &self,
        _s: &UsedNonPubStruct,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_non_pub") },
            ::soroban_sdk::Vec::from_array(&self.env, [_s.into_val(&self.env)]),
        );
        res
    }
    pub fn with_non_pub_error(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_non_pub_error") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_with_non_pub_error(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<UsedNonPubError, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_non_pub_error") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn with_tuple(&self, _t: &(UsedTupleElement, u32)) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_tuple") },
            ::soroban_sdk::Vec::from_array(&self.env, [_t.into_val(&self.env)]),
        );
        res
    }
    pub fn try_with_tuple(
        &self,
        _t: &(UsedTupleElement, u32),
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_tuple") },
            ::soroban_sdk::Vec::from_array(&self.env, [_t.into_val(&self.env)]),
        );
        res
    }
    pub fn with_tuple_return(&self) -> (UsedTupleReturnElement, u32) {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_tuple_return") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_with_tuple_return(
        &self,
    ) -> Result<
        Result<
            (UsedTupleReturnElement, u32),
            <(UsedTupleReturnElement, u32) as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "with_tuple_return") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn publish_ref_event(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_ref_event") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_publish_ref_event(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "publish_ref_event") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_param<'i>(
        _s: &'i UsedParamStruct,
        _ie: &'i UsedParamIntEnum,
    ) -> (&'i UsedParamStruct, &'i UsedParamIntEnum) {
        (_s, _ie)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_return<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_error<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_vec<'i>(_v: &'i Vec<UsedVecElement>) -> (&'i Vec<UsedVecElement>,) {
        (_v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_map<'i>(_m: &'i Map<UsedMapKey, UsedMapVal>) -> (&'i Map<UsedMapKey, UsedMapVal>,) {
        (_m,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_option<'i>(_o: &'i Option<UsedOptionElement>) -> (&'i Option<UsedOptionElement>,) {
        (_o,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_result<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish_simple<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish_topic_type<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish_data_type<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish_nested_topic<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish_nested_data<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_imported<'i>(_s: &'i imported::StructA) -> (&'i imported::StructA,) {
        (_s,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_non_pub<'i>(_s: &'i UsedNonPubStruct) -> (&'i UsedNonPubStruct,) {
        (_s,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_non_pub_error<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_tuple<'i>(_t: &'i (UsedTupleElement, u32)) -> (&'i (UsedTupleElement, u32),) {
        (_t,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn with_tuple_return<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn publish_ref_event<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_param` instead")]
#[allow(deprecated)]
pub fn __Contract__with_param__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_param(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_param` instead")]
#[export_name = "with_param"]
pub extern "C" fn __Contract__with_param__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_param__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_return` instead")]
#[allow(deprecated)]
pub fn __Contract__with_return__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_return(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_return` instead")]
#[export_name = "with_return"]
pub extern "C" fn __Contract__with_return__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_return__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_error` instead")]
#[allow(deprecated)]
pub fn __Contract__with_error__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_error(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_error` instead")]
#[export_name = "with_error"]
pub extern "C" fn __Contract__with_error__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_error__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_vec` instead")]
#[allow(deprecated)]
pub fn __Contract__with_vec__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_vec(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_vec` instead")]
#[export_name = "with_vec"]
pub extern "C" fn __Contract__with_vec__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_vec__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_map` instead")]
#[allow(deprecated)]
pub fn __Contract__with_map__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_map(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_map` instead")]
#[export_name = "with_map"]
pub extern "C" fn __Contract__with_map__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_map__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_option` instead")]
#[allow(deprecated)]
pub fn __Contract__with_option__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_option(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_option` instead")]
#[export_name = "with_option"]
pub extern "C" fn __Contract__with_option__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_option__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_result` instead")]
#[allow(deprecated)]
pub fn __Contract__with_result__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_result(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_result` instead")]
#[export_name = "with_result"]
pub extern "C" fn __Contract__with_result__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_result__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_simple` instead")]
#[allow(deprecated)]
pub fn __Contract__publish_simple__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish_simple(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_simple` instead")]
#[export_name = "publish_simple"]
pub extern "C" fn __Contract__publish_simple__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish_simple__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_topic_type` instead")]
#[allow(deprecated)]
pub fn __Contract__publish_topic_type__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish_topic_type(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_topic_type` instead")]
#[export_name = "publish_topic_type"]
pub extern "C" fn __Contract__publish_topic_type__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish_topic_type__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_data_type` instead")]
#[allow(deprecated)]
pub fn __Contract__publish_data_type__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish_data_type(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_data_type` instead")]
#[export_name = "publish_data_type"]
pub extern "C" fn __Contract__publish_data_type__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish_data_type__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_nested_topic` instead")]
#[allow(deprecated)]
pub fn __Contract__publish_nested_topic__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish_nested_topic(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_nested_topic` instead")]
#[export_name = "publish_nested_topic"]
pub extern "C" fn __Contract__publish_nested_topic__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish_nested_topic__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_nested_data` instead")]
#[allow(deprecated)]
pub fn __Contract__publish_nested_data__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish_nested_data(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_nested_data` instead")]
#[export_name = "publish_nested_data"]
pub extern "C" fn __Contract__publish_nested_data__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish_nested_data__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_imported` instead")]
#[allow(deprecated)]
pub fn __Contract__with_imported__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_imported(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_imported` instead")]
#[export_name = "with_imported"]
pub extern "C" fn __Contract__with_imported__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_imported__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_non_pub` instead")]
#[allow(deprecated)]
pub fn __Contract__with_non_pub__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_non_pub(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_non_pub` instead")]
#[export_name = "with_non_pub"]
pub extern "C" fn __Contract__with_non_pub__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_non_pub__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_non_pub_error` instead")]
#[allow(deprecated)]
pub fn __Contract__with_non_pub_error__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_non_pub_error(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_non_pub_error` instead")]
#[export_name = "with_non_pub_error"]
pub extern "C" fn __Contract__with_non_pub_error__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_non_pub_error__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_tuple` instead")]
#[allow(deprecated)]
pub fn __Contract__with_tuple__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_tuple(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_tuple` instead")]
#[export_name = "with_tuple"]
pub extern "C" fn __Contract__with_tuple__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_tuple__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_tuple_return` instead")]
#[allow(deprecated)]
pub fn __Contract__with_tuple_return__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::with_tuple_return(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).with_tuple_return` instead")]
#[export_name = "with_tuple_return"]
pub extern "C" fn __Contract__with_tuple_return__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__with_tuple_return__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_ref_event` instead")]
#[allow(deprecated)]
pub fn __Contract__publish_ref_event__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::publish_ref_event(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).publish_ref_event` instead")]
#[export_name = "publish_ref_event"]
pub extern "C" fn __Contract__publish_ref_event__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__publish_ref_event__invoke_raw(soroban_sdk::Env::default())
}
#[allow(dead_code)]
fn non_contract_fn(_s: UnusedNonContractFnParam) -> UnusedNonContractFnReturn {
    UnusedNonContractFnReturn { x: 1 }
}
