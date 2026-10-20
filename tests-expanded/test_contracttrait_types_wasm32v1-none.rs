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
        type Arg = soroban_sdk::client::Borrowed;
        type Ret = Self;
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
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
        type Arg = soroban_sdk::client::Borrowed;
        type Ret = Self;
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
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
        fn spec_shaking_marker() {
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Error::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
        }
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
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> TypesTraitClient<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
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
            x: impl soroban_sdk::client::IntoArg<u32>,
            y: impl soroban_sdk::client::IntoArg<u32>,
        ) -> soroban_sdk::client::Ret<Point> {
            use core::ops::Not;
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
                        soroban_sdk::client::IntoArg::<u32>::into_arg(x, &self.env),
                        soroban_sdk::client::IntoArg::<u32>::into_arg(y, &self.env),
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
            x: impl soroban_sdk::client::IntoArg<u32>,
            y: impl soroban_sdk::client::IntoArg<u32>,
        ) -> soroban_sdk::client::TryRet<Point> {
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
                        soroban_sdk::client::IntoArg::<u32>::into_arg(x, &self.env),
                        soroban_sdk::client::IntoArg::<u32>::into_arg(y, &self.env),
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
            p: impl soroban_sdk::client::IntoArg<Point>,
        ) -> soroban_sdk::client::Ret<Vec<Point>> {
            use core::ops::Not;
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
                    [soroban_sdk::client::IntoArg::<Point>::into_arg(
                        p, &self.env,
                    )],
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
            p: impl soroban_sdk::client::IntoArg<Point>,
        ) -> soroban_sdk::client::TryRet<Vec<Point>> {
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
                    [soroban_sdk::client::IntoArg::<Point>::into_arg(
                        p, &self.env,
                    )],
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
            v: impl soroban_sdk::client::IntoArg<u32>,
        ) -> soroban_sdk::client::Ret<Result<u32, Error>> {
            use core::ops::Not;
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
                    [soroban_sdk::client::IntoArg::<u32>::into_arg(v, &self.env)],
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
            v: impl soroban_sdk::client::IntoArg<u32>,
        ) -> soroban_sdk::client::TryRet<Result<u32, Error>> {
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
                    [soroban_sdk::client::IntoArg::<u32>::into_arg(v, &self.env)],
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
            a: impl soroban_sdk::client::IntoArg<Address>,
            _m: impl soroban_sdk::client::IntoArg<MuxedAddress>,
        ) -> soroban_sdk::client::Ret<Address> {
            use core::ops::Not;
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
                        soroban_sdk::client::IntoArg::<Address>::into_arg(a, &self.env),
                        soroban_sdk::client::IntoArg::<MuxedAddress>::into_arg(_m, &self.env),
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
            a: impl soroban_sdk::client::IntoArg<Address>,
            _m: impl soroban_sdk::client::IntoArg<MuxedAddress>,
        ) -> soroban_sdk::client::TryRet<Address> {
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
                        soroban_sdk::client::IntoArg::<Address>::into_arg(a, &self.env),
                        soroban_sdk::client::IntoArg::<MuxedAddress>::into_arg(_m, &self.env),
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
            b: impl soroban_sdk::client::IntoArg<Bytes>,
        ) -> soroban_sdk::client::Ret<Hash<32>> {
            use core::ops::Not;
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
                    [soroban_sdk::client::IntoArg::<Bytes>::into_arg(
                        b, &self.env,
                    )],
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
            b: impl soroban_sdk::client::IntoArg<Bytes>,
        ) -> soroban_sdk::client::TryRet<Hash<32>> {
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
                    [soroban_sdk::client::IntoArg::<Bytes>::into_arg(
                        b, &self.env,
                    )],
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
#[export_name = "point"]
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
#[export_name = "points"]
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
#[export_name = "even"]
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
#[export_name = "address"]
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
#[export_name = "hash"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0"]
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
        x: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::u32>,
        y: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::Ret<__Contract_TypesTrait_types::Point> {
        use core::ops::Not;
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
                    soroban_sdk::client::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        x, &self.env,
                    ),
                    soroban_sdk::client::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
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
        x: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::u32>,
        y: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::TryRet<__Contract_TypesTrait_types::Point> {
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
                    soroban_sdk::client::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
                        x, &self.env,
                    ),
                    soroban_sdk::client::IntoArg::<__Contract_TypesTrait_types::u32>::into_arg(
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
        p: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::Point>,
    ) -> soroban_sdk::client::Ret<
        __Contract_TypesTrait_types::Vec<__Contract_TypesTrait_types::Point>,
    > {
        use core::ops::Not;
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
                [soroban_sdk::client::IntoArg::<
                    __Contract_TypesTrait_types::Point,
                >::into_arg(p, &self.env)],
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
        p: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::Point>,
    ) -> soroban_sdk::client::TryRet<
        __Contract_TypesTrait_types::Vec<__Contract_TypesTrait_types::Point>,
    > {
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
                [soroban_sdk::client::IntoArg::<
                    __Contract_TypesTrait_types::Point,
                >::into_arg(p, &self.env)],
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
        v: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::Ret<
        __Contract_TypesTrait_types::Result<
            __Contract_TypesTrait_types::u32,
            __Contract_TypesTrait_types::Error,
        >,
    > {
        use core::ops::Not;
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
                [soroban_sdk::client::IntoArg::<
                    __Contract_TypesTrait_types::u32,
                >::into_arg(v, &self.env)],
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
        v: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::u32>,
    ) -> soroban_sdk::client::TryRet<
        __Contract_TypesTrait_types::Result<
            __Contract_TypesTrait_types::u32,
            __Contract_TypesTrait_types::Error,
        >,
    > {
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
                [soroban_sdk::client::IntoArg::<
                    __Contract_TypesTrait_types::u32,
                >::into_arg(v, &self.env)],
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
        a: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::Address>,
        _m: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::MuxedAddress>,
    ) -> soroban_sdk::client::Ret<__Contract_TypesTrait_types::Address> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self
            .env
            .invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short(
                        "address",
                    );
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        soroban_sdk::client::IntoArg::<
                            __Contract_TypesTrait_types::Address,
                        >::into_arg(a, &self.env),
                        soroban_sdk::client::IntoArg::<
                            __Contract_TypesTrait_types::MuxedAddress,
                        >::into_arg(_m, &self.env),
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
        a: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::Address>,
        _m: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::MuxedAddress>,
    ) -> soroban_sdk::client::TryRet<__Contract_TypesTrait_types::Address> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self
            .env
            .try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short(
                        "address",
                    );
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        soroban_sdk::client::IntoArg::<
                            __Contract_TypesTrait_types::Address,
                        >::into_arg(a, &self.env),
                        soroban_sdk::client::IntoArg::<
                            __Contract_TypesTrait_types::MuxedAddress,
                        >::into_arg(_m, &self.env),
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
        b: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::Bytes>,
    ) -> soroban_sdk::client::Ret<__Contract_TypesTrait_types::Hash<32>> {
        use core::ops::Not;
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
                [soroban_sdk::client::IntoArg::<
                    __Contract_TypesTrait_types::Bytes,
                >::into_arg(b, &self.env)],
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
        b: impl soroban_sdk::client::IntoArg<__Contract_TypesTrait_types::Bytes>,
    ) -> soroban_sdk::client::TryRet<__Contract_TypesTrait_types::Hash<32>> {
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
                [soroban_sdk::client::IntoArg::<
                    __Contract_TypesTrait_types::Bytes,
                >::into_arg(b, &self.env)],
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0"]
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
        b: impl soroban_sdk::client::IntoArg<Bytes>,
    ) -> soroban_sdk::client::Ret<Hash<32>> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "hash_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::IntoArg::<Bytes>::into_arg(
                    b, &self.env,
                )],
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
        b: impl soroban_sdk::client::IntoArg<Bytes>,
    ) -> soroban_sdk::client::TryRet<Hash<32>> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "hash_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::IntoArg::<Bytes>::into_arg(
                    b, &self.env,
                )],
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
        m: impl soroban_sdk::client::IntoArg<MuxedAddress>,
    ) -> soroban_sdk::client::Ret<MuxedAddress> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "muxed_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::IntoArg::<MuxedAddress>::into_arg(
                    m, &self.env,
                )],
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
        m: impl soroban_sdk::client::IntoArg<MuxedAddress>,
    ) -> soroban_sdk::client::TryRet<MuxedAddress> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "muxed_direct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::IntoArg::<MuxedAddress>::into_arg(
                    m, &self.env,
                )],
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
#[export_name = "hash_direct"]
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
#[export_name = "muxed_direct"]
pub extern "C" fn __Contract__muxed_direct__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__muxed_direct__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
