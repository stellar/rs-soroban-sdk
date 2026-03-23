#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_MYSTRUCT: [u8; 60usize] = MyStruct::spec_xdr();
impl MyStruct {
    pub const fn spec_xdr() -> [u8; 60usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x08MyStruct\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x07"
    }
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_MYENUMUNIT: [u8; 64usize] = MyEnumUnit::spec_xdr();
impl MyEnumUnit {
    pub const fn spec_xdr() -> [u8; 64usize] {
        *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\nMyEnumUnit\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01A\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01B\0\0\0\0\0\0\x02"
    }
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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_MYENUMVARIANTS: [u8; 128usize] = MyEnumVariants::spec_xdr();
impl MyEnumVariants {
    pub const fn spec_xdr() -> [u8; 128usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x0eMyEnumVariants\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x04VarA\0\0\0\x01\0\0\0\0\0\0\0\x04VarB\0\0\0\x01\0\0\x07\xd0\0\0\0\x08MyStruct\0\0\0\x01\0\0\0\0\0\0\0\x04VarC\0\0\0\x01\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0"
    }
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
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> AllTypesClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> AllTypesClient<'a> {
    /// Test u32 values.
    /// Returns the input unchanged.
    pub fn test_u32(&self, v: &u32) -> u32 {
        use core::ops::Not;
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
        res
    }
    /// Test i32 values.
    pub fn test_i32(&self, v: &i32) -> i32 {
        use core::ops::Not;
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
        res
    }
    pub fn test_u64(&self, v: &u64) -> u64 {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_u64(
        &self,
        v: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_i64(&self, v: &i64) -> i64 {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_i64(
        &self,
        v: &i64,
    ) -> Result<
        Result<i64, <i64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_u128(&self, v: &u128) -> u128 {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_u128(
        &self,
        v: &u128,
    ) -> Result<
        Result<u128, <u128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_i128(&self, v: &i128) -> i128 {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_i128(
        &self,
        v: &i128,
    ) -> Result<
        Result<i128, <i128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_bool(&self, v: &bool) -> bool {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_bool(
        &self,
        v: &bool,
    ) -> Result<
        Result<bool, <bool as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_address(&self, v: &Address) -> Address {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_bytes(&self, v: &Bytes) -> Bytes {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_bytes_n(&self, v: &BytesN<32>) -> BytesN<32> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_string(&self, v: &String) -> String {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_symbol(&self, v: &Symbol) -> Symbol {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_vec(&self, v: &Vec<u32>) -> Vec<u32> {
        use core::ops::Not;
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
        res
    }
    pub fn test_map(&self, v: &Map<u32, u32>) -> Map<u32, u32> {
        use core::ops::Not;
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
        res
    }
    pub fn test_duration(&self, v: &Duration) -> Duration {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_timepoint(&self, v: &Timepoint) -> Timepoint {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_i256(&self, v: &I256) -> I256 {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_i256(
        &self,
        v: &I256,
    ) -> Result<
        Result<I256, <I256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_u256(&self, v: &U256) -> U256 {
        use core::ops::Not;
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
        res
    }
    pub fn try_test_u256(
        &self,
        v: &U256,
    ) -> Result<
        Result<U256, <U256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
        res
    }
    pub fn test_env_param(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_test_env_param(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn test_struct(&self, v: &MyStruct) -> MyStruct {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_enum_unit(&self, v: &MyEnumUnit) -> MyEnumUnit {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_enum_variants(&self, v: &MyEnumVariants) -> MyEnumVariants {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
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
