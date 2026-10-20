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
impl soroban_sdk::SpecName for MyStruct {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_contracttrait_trait::MyStruct";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `MyStruct` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_contracttrait_trait::MyStruct`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for MyStruct {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
impl soroban_sdk::client::Type for MyStruct {
    type Ret = Self;
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_MYSTRUCT: [u8; MyStruct::spec_xdr().len()] = MyStruct::spec_xdr();
impl MyStruct {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
            soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <MyStruct as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                        type_: <i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                        type_: <i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr() -> [u8; MyStruct::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { MyStruct::__SPEC_XDR_ENTRY.const_to_xdr() }
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
        env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
impl soroban_sdk::SpecName for MyEnumUnit {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_contracttrait_trait::MyEnumUnit";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `MyEnumUnit` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_contracttrait_trait::MyEnumUnit`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for MyEnumUnit {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
impl soroban_sdk::client::Type for MyEnumUnit {
    type Ret = Self;
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_MYENUMUNIT: [u8; MyEnumUnit::spec_xdr().len()] = MyEnumUnit::spec_xdr();
impl MyEnumUnit {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtEnumV0(
            soroban_sdk::xdr::r#const::ScSpecUdtEnumV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <MyEnumUnit as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"A"),
                        value: 1u32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"B"),
                        value: 2u32,
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
    pub const fn spec_xdr() -> [u8; MyEnumUnit::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { MyEnumUnit::__SPEC_XDR_ENTRY.const_to_xdr() }
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
impl soroban_sdk::SpecName for MyEnumVariants {
    const SPEC_NAME: &'static str = {
        const NAME: &str = "::test_contracttrait_trait::MyEnumVariants";
        const CHECKED_NAME: &str = {
            if !(NAME.len() <= soroban_sdk::xdr::SC_SPEC_TYPE_NAME_LIMIT as usize) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "type `MyEnumVariants` full name including its module path is longer than the contract spec\'s type name limit, shorten its module path or name: `::test_contracttrait_trait::MyEnumVariants`",
                        ),
                    );
                }
            }
            NAME
        };
        CHECKED_NAME
    };
}
impl soroban_sdk::SpecTypeDef for MyEnumVariants {
    const SPEC_TYPE_DEF: soroban_sdk::xdr::r#const::ScSpecTypeDef =
        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                <Self as soroban_sdk::SpecName>::SPEC_NAME,
            ),
        });
}
impl soroban_sdk::client::Type for MyEnumVariants {
    type Ret = Self;
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_MYENUMVARIANTS: [u8; MyEnumVariants::spec_xdr().len()] =
    MyEnumVariants::spec_xdr();
impl MyEnumVariants {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(
            soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                    <MyEnumVariants as soroban_sdk::SpecName>::SPEC_NAME,
                ),
                cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::VoidV0(
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseVoidV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"VarA",
                            ),
                        },
                    ),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"VarB",
                            ),
                            type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                                <MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                            ]),
                        },
                    ),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"VarC",
                            ),
                            type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                                <MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                            ]),
                        },
                    ),
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
    pub const fn spec_xdr() -> [u8; MyEnumVariants::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { MyEnumVariants::__SPEC_XDR_ENTRY.const_to_xdr() }
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
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_u32 {
    type __AllTypes_u32;
}
impl<__T: ?Sized> __AllTypes_u32 for __T {
    type __AllTypes_u32 = u32;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_i32 {
    type __AllTypes_i32;
}
impl<__T: ?Sized> __AllTypes_i32 for __T {
    type __AllTypes_i32 = i32;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_u64 {
    type __AllTypes_u64;
}
impl<__T: ?Sized> __AllTypes_u64 for __T {
    type __AllTypes_u64 = u64;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_i64 {
    type __AllTypes_i64;
}
impl<__T: ?Sized> __AllTypes_i64 for __T {
    type __AllTypes_i64 = i64;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_u128 {
    type __AllTypes_u128;
}
impl<__T: ?Sized> __AllTypes_u128 for __T {
    type __AllTypes_u128 = u128;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_i128 {
    type __AllTypes_i128;
}
impl<__T: ?Sized> __AllTypes_i128 for __T {
    type __AllTypes_i128 = i128;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_bool {
    type __AllTypes_bool;
}
impl<__T: ?Sized> __AllTypes_bool for __T {
    type __AllTypes_bool = bool;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Address {
    type __AllTypes_Address;
}
impl<__T: ?Sized> __AllTypes_Address for __T {
    type __AllTypes_Address = Address;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Bytes {
    type __AllTypes_Bytes;
}
impl<__T: ?Sized> __AllTypes_Bytes for __T {
    type __AllTypes_Bytes = Bytes;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_BytesN {
    type __AllTypes_BytesN<const N0: usize>;
}
impl<__T: ?Sized> __AllTypes_BytesN for __T {
    type __AllTypes_BytesN<const N0: usize> = BytesN<N0>;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_String {
    type __AllTypes_String;
}
impl<__T: ?Sized> __AllTypes_String for __T {
    type __AllTypes_String = String;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Symbol {
    type __AllTypes_Symbol;
}
impl<__T: ?Sized> __AllTypes_Symbol for __T {
    type __AllTypes_Symbol = Symbol;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Vec {
    type __AllTypes_Vec<T0>;
}
impl<__T: ?Sized> __AllTypes_Vec for __T {
    type __AllTypes_Vec<T0> = Vec<T0>;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Map {
    type __AllTypes_Map<T0, T1>;
}
impl<__T: ?Sized> __AllTypes_Map for __T {
    type __AllTypes_Map<T0, T1> = Map<T0, T1>;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Duration {
    type __AllTypes_Duration;
}
impl<__T: ?Sized> __AllTypes_Duration for __T {
    type __AllTypes_Duration = Duration;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Timepoint {
    type __AllTypes_Timepoint;
}
impl<__T: ?Sized> __AllTypes_Timepoint for __T {
    type __AllTypes_Timepoint = Timepoint;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_I256 {
    type __AllTypes_I256;
}
impl<__T: ?Sized> __AllTypes_I256 for __T {
    type __AllTypes_I256 = I256;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_U256 {
    type __AllTypes_U256;
}
impl<__T: ?Sized> __AllTypes_U256 for __T {
    type __AllTypes_U256 = U256;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_Env {
    type __AllTypes_Env;
}
impl<__T: ?Sized> __AllTypes_Env for __T {
    type __AllTypes_Env = Env;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_MyStruct {
    type __AllTypes_MyStruct;
}
impl<__T: ?Sized> __AllTypes_MyStruct for __T {
    type __AllTypes_MyStruct = MyStruct;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_MyEnumUnit {
    type __AllTypes_MyEnumUnit;
}
impl<__T: ?Sized> __AllTypes_MyEnumUnit for __T {
    type __AllTypes_MyEnumUnit = MyEnumUnit;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __AllTypes_MyEnumVariants {
    type __AllTypes_MyEnumVariants;
}
impl<__T: ?Sized> __AllTypes_MyEnumVariants for __T {
    type __AllTypes_MyEnumVariants = MyEnumVariants;
}
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_all_types as AllTypes;
pub trait AllTypes:
    __AllTypes_u32
    + __AllTypes_i32
    + __AllTypes_u64
    + __AllTypes_i64
    + __AllTypes_u128
    + __AllTypes_i128
    + __AllTypes_bool
    + __AllTypes_Address
    + __AllTypes_Bytes
    + __AllTypes_BytesN
    + __AllTypes_String
    + __AllTypes_Symbol
    + __AllTypes_Vec
    + __AllTypes_Map
    + __AllTypes_Duration
    + __AllTypes_Timepoint
    + __AllTypes_I256
    + __AllTypes_U256
    + __AllTypes_Env
    + __AllTypes_MyStruct
    + __AllTypes_MyEnumUnit
    + __AllTypes_MyEnumVariants
{
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
    /// Call `test_u32` function on the contract.
    ///
    /// Test u32 values.
    /// Returns the input unchanged.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u32`] to receive errors instead of panicking.
    pub fn test_u32(&self, v: impl soroban_sdk::IntoArg<u32>) -> soroban_sdk::client::Ret<u32> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<u32>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u32` function on the contract.
    ///
    /// Test u32 values.
    /// Returns the input unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u32(
        &self,
        v: impl soroban_sdk::IntoArg<u32>,
    ) -> soroban_sdk::client::TryRet<u32> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<u32>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i32` function on the contract.
    ///
    /// Test i32 values.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i32`] to receive errors instead of panicking.
    pub fn test_i32(&self, v: impl soroban_sdk::IntoArg<i32>) -> soroban_sdk::client::Ret<i32> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<i32>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i32` function on the contract.
    ///
    /// Test i32 values.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i32(
        &self,
        v: impl soroban_sdk::IntoArg<i32>,
    ) -> soroban_sdk::client::TryRet<i32> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<i32>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u64` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u64`] to receive errors instead of panicking.
    pub fn test_u64(&self, v: impl soroban_sdk::IntoArg<u64>) -> soroban_sdk::client::Ret<u64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<u64>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u64` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u64(
        &self,
        v: impl soroban_sdk::IntoArg<u64>,
    ) -> soroban_sdk::client::TryRet<u64> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<u64>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i64` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i64`] to receive errors instead of panicking.
    pub fn test_i64(&self, v: impl soroban_sdk::IntoArg<i64>) -> soroban_sdk::client::Ret<i64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<i64>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i64` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i64(
        &self,
        v: impl soroban_sdk::IntoArg<i64>,
    ) -> soroban_sdk::client::TryRet<i64> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<i64>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u128` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u128`] to receive errors instead of panicking.
    pub fn test_u128(&self, v: impl soroban_sdk::IntoArg<u128>) -> soroban_sdk::client::Ret<u128> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<u128>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u128` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u128(
        &self,
        v: impl soroban_sdk::IntoArg<u128>,
    ) -> soroban_sdk::client::TryRet<u128> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<u128>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i128` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i128`] to receive errors instead of panicking.
    pub fn test_i128(&self, v: impl soroban_sdk::IntoArg<i128>) -> soroban_sdk::client::Ret<i128> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<i128>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i128` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i128(
        &self,
        v: impl soroban_sdk::IntoArg<i128>,
    ) -> soroban_sdk::client::TryRet<i128> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<i128>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bool` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_bool`] to receive errors instead of panicking.
    pub fn test_bool(&self, v: impl soroban_sdk::IntoArg<bool>) -> soroban_sdk::client::Ret<bool> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<bool>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bool` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_bool(
        &self,
        v: impl soroban_sdk::IntoArg<bool>,
    ) -> soroban_sdk::client::TryRet<bool> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<bool>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_address` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_address`] to receive errors instead of panicking.
    pub fn test_address(
        &self,
        v: impl soroban_sdk::IntoArg<Address>,
    ) -> soroban_sdk::client::Ret<Address> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Address>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_address` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_address(
        &self,
        v: impl soroban_sdk::IntoArg<Address>,
    ) -> soroban_sdk::client::TryRet<Address> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Address>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_bytes`] to receive errors instead of panicking.
    pub fn test_bytes(
        &self,
        v: impl soroban_sdk::IntoArg<Bytes>,
    ) -> soroban_sdk::client::Ret<Bytes> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Bytes>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_bytes(
        &self,
        v: impl soroban_sdk::IntoArg<Bytes>,
    ) -> soroban_sdk::client::TryRet<Bytes> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Bytes>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes_n` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_bytes_n`] to receive errors instead of panicking.
    pub fn test_bytes_n(
        &self,
        v: impl soroban_sdk::IntoArg<BytesN<32>>,
    ) -> soroban_sdk::client::Ret<BytesN<32>> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<BytesN<32>>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes_n` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_bytes_n(
        &self,
        v: impl soroban_sdk::IntoArg<BytesN<32>>,
    ) -> soroban_sdk::client::TryRet<BytesN<32>> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<BytesN<32>>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_string` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_string`] to receive errors instead of panicking.
    pub fn test_string(
        &self,
        v: impl soroban_sdk::IntoArg<String>,
    ) -> soroban_sdk::client::Ret<String> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<String>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_string` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_string(
        &self,
        v: impl soroban_sdk::IntoArg<String>,
    ) -> soroban_sdk::client::TryRet<String> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<String>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_symbol` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_symbol`] to receive errors instead of panicking.
    pub fn test_symbol(
        &self,
        v: impl soroban_sdk::IntoArg<Symbol>,
    ) -> soroban_sdk::client::Ret<Symbol> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Symbol>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_symbol` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_symbol(
        &self,
        v: impl soroban_sdk::IntoArg<Symbol>,
    ) -> soroban_sdk::client::TryRet<Symbol> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Symbol>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_vec` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_vec`] to receive errors instead of panicking.
    pub fn test_vec(
        &self,
        v: impl soroban_sdk::IntoArg<Vec<u32>>,
    ) -> soroban_sdk::client::Ret<Vec<u32>> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Vec<u32>>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_vec` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_vec(
        &self,
        v: impl soroban_sdk::IntoArg<Vec<u32>>,
    ) -> soroban_sdk::client::TryRet<Vec<u32>> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Vec<u32>>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_map` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_map`] to receive errors instead of panicking.
    pub fn test_map(
        &self,
        v: impl soroban_sdk::IntoArg<Map<u32, u32>>,
    ) -> soroban_sdk::client::Ret<Map<u32, u32>> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Map<u32, u32>>::into_arg(
                    v, &self.env,
                )],
            ),
        );
        res
    }
    /// Call `test_map` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_map(
        &self,
        v: impl soroban_sdk::IntoArg<Map<u32, u32>>,
    ) -> soroban_sdk::client::TryRet<Map<u32, u32>> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Map<u32, u32>>::into_arg(
                    v, &self.env,
                )],
            ),
        );
        res
    }
    /// Call `test_duration` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_duration`] to receive errors instead of panicking.
    pub fn test_duration(
        &self,
        v: impl soroban_sdk::IntoArg<Duration>,
    ) -> soroban_sdk::client::Ret<Duration> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Duration>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_duration` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_duration(
        &self,
        v: impl soroban_sdk::IntoArg<Duration>,
    ) -> soroban_sdk::client::TryRet<Duration> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Duration>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_timepoint` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_timepoint`] to receive errors instead of panicking.
    pub fn test_timepoint(
        &self,
        v: impl soroban_sdk::IntoArg<Timepoint>,
    ) -> soroban_sdk::client::Ret<Timepoint> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Timepoint>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_timepoint` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_timepoint(
        &self,
        v: impl soroban_sdk::IntoArg<Timepoint>,
    ) -> soroban_sdk::client::TryRet<Timepoint> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<Timepoint>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i256` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i256`] to receive errors instead of panicking.
    pub fn test_i256(&self, v: impl soroban_sdk::IntoArg<I256>) -> soroban_sdk::client::Ret<I256> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<I256>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i256` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i256(
        &self,
        v: impl soroban_sdk::IntoArg<I256>,
    ) -> soroban_sdk::client::TryRet<I256> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<I256>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u256` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u256`] to receive errors instead of panicking.
    pub fn test_u256(&self, v: impl soroban_sdk::IntoArg<U256>) -> soroban_sdk::client::Ret<U256> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<U256>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u256` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u256(
        &self,
        v: impl soroban_sdk::IntoArg<U256>,
    ) -> soroban_sdk::client::TryRet<U256> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<U256>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_env_param` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_env_param`] to receive errors instead of panicking.
    pub fn test_env_param(&self) -> soroban_sdk::client::Ret<u32> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `test_env_param` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_env_param(&self) -> soroban_sdk::client::TryRet<u32> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `test_struct` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_struct`] to receive errors instead of panicking.
    pub fn test_struct(
        &self,
        v: impl soroban_sdk::IntoArg<MyStruct>,
    ) -> soroban_sdk::client::Ret<MyStruct> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MyStruct>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_struct` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_struct(
        &self,
        v: impl soroban_sdk::IntoArg<MyStruct>,
    ) -> soroban_sdk::client::TryRet<MyStruct> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MyStruct>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_unit` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_enum_unit`] to receive errors instead of panicking.
    pub fn test_enum_unit(
        &self,
        v: impl soroban_sdk::IntoArg<MyEnumUnit>,
    ) -> soroban_sdk::client::Ret<MyEnumUnit> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MyEnumUnit>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_unit` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_enum_unit(
        &self,
        v: impl soroban_sdk::IntoArg<MyEnumUnit>,
    ) -> soroban_sdk::client::TryRet<MyEnumUnit> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MyEnumUnit>::into_arg(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_variants` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_enum_variants`] to receive errors instead of panicking.
    pub fn test_enum_variants(
        &self,
        v: impl soroban_sdk::IntoArg<MyEnumVariants>,
    ) -> soroban_sdk::client::Ret<MyEnumVariants> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MyEnumVariants>::into_arg(
                    v, &self.env,
                )],
            ),
        );
        res
    }
    /// Call `test_enum_variants` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_enum_variants(
        &self,
        v: impl soroban_sdk::IntoArg<MyEnumVariants>,
    ) -> soroban_sdk::client::TryRet<MyEnumVariants> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::IntoArg::<MyEnumVariants>::into_arg(
                    v, &self.env,
                )],
            ),
        );
        res
    }
}
///AllTypesArgs is a type for building arg lists for functions defined in "AllTypes".
pub struct AllTypesArgs;
impl AllTypesArgs {
    /// Test u32 values.
    /// Returns the input unchanged.
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u32<'i>(v: &'i u32) -> (&'i u32,) {
        (v,)
    }
    /// Test i32 values.
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
    #[allow(non_upper_case_globals)]
    /// Test u32 values.
    /// Returns the input unchanged.
    const __SPEC_XDR_ENTRY_test_u32: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                    b"Test u32 values.\nReturns the input unchanged.",
                ),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u32"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    /// Test u32 values.
    /// Returns the input unchanged.
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_u32() -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_u32.const_xdr_len()]
    {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_u32.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    /// Test i32 values.
    const __SPEC_XDR_ENTRY_test_i32: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                    b"Test i32 values.",
                ),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i32"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
    #[allow(non_snake_case)]
    /// Test i32 values.
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_i32() -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_i32.const_xdr_len()]
    {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_i32.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_u64: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u64"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u64() -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_u64.const_xdr_len()]
    {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_u64.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_i64: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i64"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i64() -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_i64.const_xdr_len()]
    {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_i64.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_u128: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u128"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u128(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_u128.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_u128.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_i128: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i128"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i128(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_i128.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_i128.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_bool: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_bool"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_bool(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_bool.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_bool.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_address: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_address"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_address(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_address.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_address.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_bytes: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_bytes"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_bytes(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_bytes.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_bytes.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_bytes_n: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_bytes_n"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <BytesN<32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <BytesN<32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_bytes_n(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_bytes_n.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_bytes_n.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_string: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_string"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_string(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_string.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_string.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_symbol: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_symbol"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_symbol(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_symbol.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_symbol.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_vec: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_vec"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Vec<u32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Vec<u32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_vec() -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_vec.const_xdr_len()]
    {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_vec.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_map: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_map"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Map<u32, u32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Map<u32, u32> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_map() -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_map.const_xdr_len()]
    {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_map.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_duration: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_duration"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_duration(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_duration.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_duration.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_timepoint: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_timepoint"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_timepoint(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_timepoint.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_timepoint.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_i256: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i256"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i256(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_i256.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_i256.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_u256: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u256"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u256(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_u256.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_u256.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_env_param: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_env_param"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_env_param(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_env_param.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_env_param.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_struct: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_struct"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_struct(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_struct.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_struct.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_enum_unit: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_enum_unit"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_enum_unit(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_enum_unit.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_enum_unit.const_to_xdr() }
    }
}
impl AllTypesSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_enum_variants: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"test_enum_variants",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_enum_variants(
    ) -> [u8; AllTypesSpec::__SPEC_XDR_ENTRY_test_enum_variants.const_xdr_len()] {
        const { AllTypesSpec::__SPEC_XDR_ENTRY_test_enum_variants.const_to_xdr() }
    }
}
