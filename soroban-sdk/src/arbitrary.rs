//! Support for fuzzing Soroban contracts with [`cargo-fuzz`].
//!
//! This module provides a pattern for generating Soroban contract types for the
//! purpose of fuzzing Soroban contracts. It is focused on implementing the
//! [`Arbitrary`] trait that `cargo-fuzz` relies on to feed fuzzers with
//! generated Rust values.
//!
//! [`cargo-fuzz`]: https://github.com/rust-fuzz/cargo-fuzz/
//! [`Arbitrary`]: ::arbitrary::Arbitrary
//!
//! This module
//!
//! - defines the [`SorobanArbitrary`] trait,
//! - defines the [`fuzz_catch_panic`] helper,
//! - reexports the [`arbitrary`] crate and the [`Arbitrary`] type.
//!
//! This module is only available when the "testutils" Cargo feature is defined.
//!
//!
//! ## About `cargo-fuzz` and `Arbitrary`
//!
//! In its basic operation `cargo-fuzz` fuzz generates raw bytes and feeds them
//! to a program-dependent fuzzer designed to exercise a program, in our case a
//! Soroban contract.
//!
//! `cargo-fuzz` programs declare their entry points with a macro:
//!
//! ```
//! # macro_rules! fuzz_target {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! fuzz_target!(|input: &[u8]| {
//!     // feed bytes to the program
//! });
//! ```
//!
//! More sophisticated fuzzers accept not bytes but Rust types:
//!
//! ```
//! # use arbitrary::Arbitrary;
//! # macro_rules! fuzz_target {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! #[derive(Arbitrary, Debug)]
//! struct FuzzDeposit {
//!     deposit_amount: i128,
//! }
//!
//! fuzz_target!(|input: FuzzDeposit| {
//!     // fuzz the program based on the input
//! });
//! ```
//!
//! Types accepted as input to `fuzz_target` must implement the `Arbitrary` trait,
//! which transforms bytes to Rust types.
//!
//!
//! ## The `SorobanArbitrary` trait
//!
//! Soroban types are managed by the host environment, and so must be created
//! from an [`Env`] value; `Arbitrary` values though must be created from
//! nothing but bytes. The [`SorobanArbitrary`] trait, implemented for all
//! Soroban contract types, exists to bridge this gap: it defines a _prototype_
//! pattern whereby the `fuzz_target` macro creates prototype values that the
//! fuzz program can convert to contract values with the standard soroban
//! conversion traits, [`FromVal`] or [`IntoVal`].
//!
//! [`Env`]: crate::Env
//! [`FromVal`]: crate::FromVal
//! [`IntoVal`]: crate::IntoVal
//!
//! The types of prototypes are identified by the associated type,
//! [`SorobanArbitrary::Prototype`]:
//!
//! ```
//! # use soroban_sdk::arbitrary::Arbitrary;
//! # use soroban_sdk::{TryFromVal, IntoVal, RawVal, Env};
//! pub trait SorobanArbitrary:
//!     TryFromVal<Env, Self::Prototype> + IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>
//! {
//!     type Prototype: for <'a> Arbitrary<'a>;
//! }
//! ```
//!
//! Types that implement `SorobanArbitrary` include:
//!
//! - `i32`, `u32`, `i64`, `u64`, `i128`, `u128`
//! - `Static`, `Status`,
//! - `Bytes`, `BytesN`, `Vec`, `Map`, `Address`, `RawVal`
//!
//! All user-defined contract types, those with the [`contracttype`] attribute,
//! automatically derive `SorobanArbitrary`. Note that `SorobanArbitrary` is
//! only derived when the "testutils" Cargo feature is active. This implies
//! that, in general, to make a Soroban contract fuzzable, the contract crate
//! must define a "testutils" Cargo feature, that feature should turn on the
//! "soroban-sdk/testutils" feature, and the fuzz test, which is its own crate,
//! must turn that feature on.
//!
//! [`contracttype`]: crate::contracttype
//!
//!
//! ## Example: take a Soroban `Vec` of `Address` as fuzzer input
//!
//! ```
//! # macro_rules! fuzz_target {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! use soroban_sdk::{Address, Env, Vec};
//! use soroban_sdk::arbitrary::SorobanArbitrary;
//!
//! fuzz_target!(|input: <Vec<Address> as SorobanArbitrary>::Prototype| {
//!     let env = Env::default();
//!     let addresses: Vec<Address> = input.into_val(&env);
//!     // fuzz the program based on the input
//! });
//! ```
//!
//!
//! ## Example: take a custom contract type as fuzzer input
//!
//! ```
//! # macro_rules! fuzz_target {
//! #     (|$data:ident: $dty: ty| $body:block) => { };
//! # }
//! use soroban_sdk::{Address, Env, Vec};
//! use soroban_sdk::contracttype;
//! use soroban_sdk::arbitrary::{Arbitrary, SorobanArbitrary};
//! use std::vec::Vec as RustVec;
//!
//! #[derive(Arbitrary, Debug)]
//! struct TestInput {
//!     deposit_amount: i128,
//!     claim_address: <Address as SorobanArbitrary>::Prototype,
//!     time_bound: <TimeBound as SorobanArbitrary>::Prototype,
//! }
//!
//! #[contracttype]
//! pub struct TimeBound {
//!     pub kind: TimeBoundKind,
//!     pub timestamp: u64,
//! }
//!
//! #[contracttype]
//! pub enum TimeBoundKind {
//!     Before,
//!     After,
//! }
//!
//! fuzz_target!(|input: TestInput| {
//!     let env = Env::default();
//!     let claim_address: Address = input.claim_address.into_val(&env);
//!     let time_bound: TimeBound = input.time_bound.into_val(&env);
//!     // fuzz the program based on the input
//! });
//! ```

#![cfg(feature = "testutils")]

/// A reexport of the `arbitrary` crate.
///
/// Used by the `contracttype` macro to derive `Arbitrary`.
pub use arbitrary;

// Used often enough in fuzz tests to want direct access to it.
pub use arbitrary::Arbitrary;

// Used by `contracttype`
#[doc(hidden)]
pub use std;

pub use api::*;
pub use fuzz_test_helpers::*;

/// The traits that must be implemented on Soroban types to support fuzzing.
///
/// These allow for ergonomic conversion from a randomly-generated "prototype"
/// that implements `Arbitrary` into `Env`-"hosted" values that are paired with an
/// `Env`.
///
/// These traits are intended to be easy to automatically derive.
mod api {
    use crate::Env;
    use crate::RawVal;
    use crate::{IntoVal, TryFromVal};
    use arbitrary::Arbitrary;

    /// An `Env`-hosted contract value that can be randomly generated.
    ///
    /// Types that implement `SorabanArbitrary` have an associated "prototype"
    /// type that implements [`Arbitrary`].
    ///
    /// This exists partly that the prototype can be named like
    ///
    /// ```ignore
    /// fuzz_target!(|input: <Bytes as SorobanArbitrary>::Arbitrary| {
    ///   ...
    /// });
    /// ```
    // This also makes derivation of `SorobanArbitrary` for custom types easier
    // since we depend on all fields also implementing `SorobanArbitrary`.
    //
    // The IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> bounds are to satisfy
    // the bounds of Vec and Map, so that collections of prototypes can be
    // converted to contract types.
    pub trait SorobanArbitrary:
        TryFromVal<Env, Self::Prototype> + IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>
    {
        /// A type that implements [`Arbitrary`] and can be converted to this
        /// [`SorobanArbitrary`] type.
        // NB: The `Arbitrary` bound here is not necessary for the correct use of
        // `SorobanArbitrary`, but it makes the purpose clear.
        type Prototype: for<'a> Arbitrary<'a>;
    }
}

/// Implementations of `soroban_sdk::arbitrary::api` for Rust scalar types.
///
/// These types
///
/// - do not have a distinct `Arbitrary` prototype,
///   i.e. they use themselves as the `SorobanArbitrary::Prototype` type,
/// - implement `Arbitrary` in the `arbitrary` crate,
/// - trivially implement `TryFromVal<Env, SorobanArbitrary::Prototype>`,
///
/// Examples:
///
/// - `u32`
mod scalars {
    use crate::arbitrary::api::*;

    impl SorobanArbitrary for u32 {
        type Prototype = u32;
    }

    impl SorobanArbitrary for i32 {
        type Prototype = i32;
    }

    impl SorobanArbitrary for u64 {
        type Prototype = u64;
    }

    impl SorobanArbitrary for i64 {
        type Prototype = i64;
    }

    impl SorobanArbitrary for u128 {
        type Prototype = u128;
    }

    impl SorobanArbitrary for i128 {
        type Prototype = i128;
    }
}

/// Implementations of `soroban_sdk::arbitrary::api` for Soroban types that do not
/// need access to the Soroban host environment.
///
/// These types
///
/// - do not have a distinct `Arbitrary` prototype,
///   i.e. they use themselves as the `SorobanArbitrary::Prototype` type,
/// - implement `Arbitrary` in the `soroban-env-common` crate,
/// - trivially implement `TryFromVal<Env, SorobanArbitrary::Prototype>`,
///
/// Examples:
///
/// - `Status`
mod simple {
    use crate::arbitrary::api::*;
    pub use crate::Status;

    impl SorobanArbitrary for Status {
        type Prototype = Status;
    }
}

/// Implementations of `soroban_sdk::arbitrary::api` for Soroban types that do
/// need access to the Soroban host environment.
///
/// These types
///
/// - have a distinct `Arbitrary` prototype that derives `Arbitrary`,
/// - require an `Env` to be converted to their actual contract type.
///
/// Examples:
///
/// - `Vec`
mod objects {
    use arbitrary::Arbitrary;

    use crate::arbitrary::api::*;
    use crate::ConversionError;
    use crate::{Env, IntoVal, TryFromVal};

    use crate::{Address, Bytes, BytesN, Map, Symbol, Vec};

    use std::vec::Vec as RustVec;

    //////////////////////////////////

    #[derive(Arbitrary, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ArbitraryBytes {
        vec: RustVec<u8>,
    }

    impl SorobanArbitrary for Bytes {
        type Prototype = ArbitraryBytes;
    }

    impl TryFromVal<Env, ArbitraryBytes> for Bytes {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryBytes) -> Result<Self, Self::Error> {
            Self::try_from_val(env, &v.vec.as_slice())
        }
    }

    //////////////////////////////////

    #[derive(Arbitrary, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ArbitraryBytesN<const N: usize> {
        array: [u8; N],
    }

    impl<const N: usize> SorobanArbitrary for BytesN<N> {
        type Prototype = ArbitraryBytesN<N>;
    }

    impl<const N: usize> TryFromVal<Env, ArbitraryBytesN<N>> for BytesN<N> {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryBytesN<N>) -> Result<Self, Self::Error> {
            Self::try_from_val(env, &v.array)
        }
    }

    //////////////////////////////////

    #[derive(Arbitrary, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ArbitrarySymbol {
        s: String,
    }

    impl SorobanArbitrary for Symbol {
        type Prototype = ArbitrarySymbol;
    }

    impl TryFromVal<Env, ArbitrarySymbol> for Symbol {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitrarySymbol) -> Result<Self, Self::Error> {
            Self::try_from_val(env, &&v.s[..])
        }
    }

    //////////////////////////////////

    #[derive(Arbitrary, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ArbitraryVec<T> {
        vec: RustVec<T>,
    }

    impl<T> SorobanArbitrary for Vec<T>
    where
        T: SorobanArbitrary,
    {
        type Prototype = ArbitraryVec<T::Prototype>;
    }

    impl<T> TryFromVal<Env, ArbitraryVec<T::Prototype>> for Vec<T>
    where
        T: SorobanArbitrary,
    {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryVec<T::Prototype>) -> Result<Self, Self::Error> {
            let mut buf: Vec<T> = Vec::new(env);
            for item in v.vec.iter() {
                buf.push_back(item.into_val(env));
            }
            Ok(buf)
        }
    }

    //////////////////////////////////

    #[derive(Arbitrary, Debug, Clone)]
    // todo eq and ord?
    pub struct ArbitraryMap<K, V> {
        map: RustVec<(K, V)>,
    }

    impl<K, V> SorobanArbitrary for Map<K, V>
    where
        K: SorobanArbitrary,
        V: SorobanArbitrary,
    {
        type Prototype = ArbitraryMap<K::Prototype, V::Prototype>;
    }

    impl<K, V> TryFromVal<Env, ArbitraryMap<K::Prototype, V::Prototype>> for Map<K, V>
    where
        K: SorobanArbitrary,
        V: SorobanArbitrary,
    {
        type Error = ConversionError;
        fn try_from_val(
            env: &Env,
            v: &ArbitraryMap<K::Prototype, V::Prototype>,
        ) -> Result<Self, Self::Error> {
            let mut map: Map<K, V> = Map::new(env);
            for (k, v) in v.map.iter() {
                map.set(k.into_val(env), v.into_val(env));
            }
            Ok(map)
        }
    }

    //////////////////////////////////

    #[derive(Arbitrary, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ArbitraryAddress {
        inner: [u8; 32],
    }

    impl SorobanArbitrary for Address {
        type Prototype = ArbitraryAddress;
    }

    impl TryFromVal<Env, ArbitraryAddress> for Address {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryAddress) -> Result<Self, Self::Error> {
            use crate::env::xdr::{Hash, ScAddress, ScVal};

            let sc_addr = ScVal::Address(ScAddress::Contract(Hash(v.inner)));
            Ok(sc_addr.into_val(env))
        }
    }
}

/// Implementations of `soroban_sdk::arbitrary::api` for `RawVal`.
mod composite {
    use arbitrary::Arbitrary;

    use crate::arbitrary::api::*;
    use crate::ConversionError;
    use crate::{Env, IntoVal, TryFromVal};

    use super::objects::*;
    use super::simple::*;
    use crate::{Address, Bytes, Map, RawVal, Vec};

    #[derive(Arbitrary, Debug, Clone)]
    // todo eq and ord?
    pub enum ArbitraryRawVal {
        U32(u32),
        I32(i32),
        U64(u64),
        I64(i64),
        U128(u128),
        I128(i128),
        //Symbol(Symbol), // todo
        Status(Status),
        Bytes(ArbitraryBytes),
        Address(<Address as SorobanArbitrary>::Prototype),
        Vec(ArbitraryRawValVec),
        Map(ArbitraryRawValMap),
    }

    impl SorobanArbitrary for RawVal {
        type Prototype = ArbitraryRawVal;
    }

    impl TryFromVal<Env, ArbitraryRawVal> for RawVal {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryRawVal) -> Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryRawVal::U32(v) => v.into_val(env),
                ArbitraryRawVal::I32(v) => v.into_val(env),
                ArbitraryRawVal::U64(v) => v.into_val(env),
                ArbitraryRawVal::I64(v) => v.into_val(env),
                ArbitraryRawVal::U128(v) => v.into_val(env),
                ArbitraryRawVal::I128(v) => v.into_val(env),
                ArbitraryRawVal::Status(v) => v.into_val(env),
                ArbitraryRawVal::Bytes(v) => {
                    let v: Bytes = v.into_val(env);
                    v.into_val(env)
                }
                ArbitraryRawVal::Address(v) => {
                    let v: Address = v.into_val(env);
                    v.into_val(env)
                }
                ArbitraryRawVal::Vec(v) => v.into_val(env),
                ArbitraryRawVal::Map(v) => v.into_val(env),
            })
        }
    }

    #[derive(Arbitrary, Debug, Clone)]
    pub enum ArbitraryRawValVec {
        U32(<Vec<u32> as SorobanArbitrary>::Prototype),
        // todo
    }

    impl TryFromVal<Env, ArbitraryRawValVec> for RawVal {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryRawValVec) -> Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryRawValVec::U32(v) => {
                    let v: Vec<u32> = v.into_val(env);
                    v.into_val(env)
                }
            })
        }
    }

    #[derive(Arbitrary, Debug, Clone)]
    pub enum ArbitraryRawValMap {
        U32ToU32(<Map<u32, u32> as SorobanArbitrary>::Prototype),
        // todo
    }

    impl TryFromVal<Env, ArbitraryRawValMap> for RawVal {
        type Error = ConversionError;
        fn try_from_val(env: &Env, v: &ArbitraryRawValMap) -> Result<Self, Self::Error> {
            Ok(match v {
                ArbitraryRawValMap::U32ToU32(v) => {
                    let v: Map<u32, u32> = v.into_val(env);
                    v.into_val(env)
                }
            })
        }
    }
}

/// Additional tools for writing fuzz tests.
mod fuzz_test_helpers {
    use soroban_env_host::call_with_suppressed_panic_hook;

    /// Catch panics within a fuzz test.
    ///
    /// The `cargo-fuzz` test harness turns panics into test failures,
    /// immediately aborting the process.
    ///
    /// This function catches panics while temporarily disabling the
    /// `cargo-fuzz` panic handler.
    ///
    /// # Example
    ///
    /// ```
    /// # macro_rules! fuzz_target {
    /// #     (|$data:ident: $dty: ty| $body:block) => { };
    /// # }
    /// # struct ExampleContract;
    /// # impl ExampleContract {
    /// #   fn new(e: &soroban_sdk::Env, b: &soroban_sdk::BytesN<32>) { }
    /// #   fn deposit(&self, a: soroban_sdk::Address, n: i128) { }
    /// # }
    /// use soroban_sdk::{Address, Env};
    /// use soroban_sdk::arbitrary::{Arbitrary, SorobanArbitrary};
    ///
    /// #[derive(Arbitrary, Debug)]
    /// struct FuzzDeposit {
    ///     deposit_amount: i128,
    ///     deposit_address: <Address as SorobanArbitrary>::Prototype,
    /// }
    ///
    /// fuzz_target!(|input: FuzzDeposit| {
    ///     let env = Env::default();
    ///
    ///     let contract = ExampleContract::new(env, &env.register_contract(None, ExampleContract {}));
    ///
    ///     let addresses: Address = input.deposit_address.into_val(&env);
    ///     let r = fuzz_catch_panic(|| {
    ///         contract.deposit(deposit_address, input.deposit_amount);
    ///     });
    /// });
    /// ```
    pub fn fuzz_catch_panic<F, R>(f: F) -> std::thread::Result<R>
    where
        F: FnOnce() -> R,
    {
        call_with_suppressed_panic_hook(std::panic::AssertUnwindSafe(f))
    }
}

#[cfg(test)]
mod tests {
    use crate::arbitrary::*;
    use crate::{Bytes, BytesN, Map, RawVal, Vec};
    use crate::{Env, IntoVal};
    use arbitrary::{Arbitrary, Unstructured};
    use rand::RngCore;

    fn run_test<T>()
    where
        T: SorobanArbitrary,
        T::Prototype: for<'a> Arbitrary<'a>,
    {
        let env = Env::default();
        let mut rng = rand::thread_rng();
        let mut rng_data = [0u8; 64];

        for _ in 0..100 {
            rng.fill_bytes(&mut rng_data);
            let mut unstructured = Unstructured::new(&rng_data);
            let input = T::Prototype::arbitrary(&mut unstructured).expect("SorobanArbitrary");
            let _val: T = input.into_val(&env);
        }
    }

    #[test]
    fn test_u32() {
        run_test::<u32>()
    }

    #[test]
    fn test_i32() {
        run_test::<i32>()
    }

    #[test]
    fn test_u64() {
        run_test::<u64>()
    }

    #[test]
    fn test_i64() {
        run_test::<i64>()
    }

    #[test]
    fn test_u128() {
        run_test::<u128>()
    }

    #[test]
    fn test_i128() {
        run_test::<i128>()
    }

    #[test]
    fn test_bytes() {
        run_test::<Bytes>()
    }

    #[test]
    fn test_bytes_n() {
        run_test::<BytesN<64>>()
    }

    #[test]
    fn test_vec_u32() {
        run_test::<Vec<u32>>()
    }

    #[test]
    fn test_vec_i32() {
        run_test::<Vec<i32>>()
    }

    #[test]
    fn test_vec_bytes() {
        run_test::<Vec<Bytes>>()
    }

    #[test]
    fn test_vec_bytes_n() {
        run_test::<Vec<BytesN<32>>>()
    }

    #[test]
    fn test_vec_vec_bytes() {
        run_test::<Vec<Vec<Bytes>>>()
    }

    #[test]
    fn test_map_u32() {
        run_test::<Map<u32, Vec<u32>>>()
    }

    #[test]
    fn test_map_i32() {
        run_test::<Map<i32, Vec<i32>>>()
    }

    #[test]
    fn test_map_bytes() {
        run_test::<Map<Bytes, Bytes>>()
    }

    #[test]
    fn test_map_bytes_n() {
        run_test::<Map<BytesN<32>, Bytes>>()
    }

    #[test]
    fn test_map_vec() {
        run_test::<Map<Vec<Bytes>, Vec<Bytes>>>()
    }

    #[test]
    fn test_raw_val() {
        run_test::<RawVal>()
    }

    mod user_defined_types {
        use crate as soroban_sdk;
        use crate::arbitrary::tests::run_test;
        use crate::{Bytes, BytesN, Map, Vec};
        use soroban_sdk::contracttype;

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct PrivStruct {
            count_u: u32,
            count_i: i32,
            bytes_n: BytesN<32>,
            vec: Vec<Bytes>,
            map: Map<Bytes, Vec<i32>>,
        }

        #[test]
        fn test_user_defined_priv_struct() {
            run_test::<PrivStruct>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct PrivStructPubFields {
            pub count_u: u32,
            pub count_i: i32,
            pub bytes_n: BytesN<32>,
            pub vec: Vec<Bytes>,
            pub map: Map<Bytes, Vec<i32>>,
        }

        #[test]
        fn test_user_defined_priv_struct_pub_fields() {
            run_test::<PrivStructPubFields>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct PubStruct {
            count_u: u32,
            count_i: i32,
            bytes_n: BytesN<32>,
            vec: Vec<Bytes>,
            map: Map<Bytes, Vec<i32>>,
        }

        #[test]
        fn test_user_defined_pub_struct() {
            run_test::<PubStruct>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct PubStructPubFields {
            pub count_u: u32,
            pub count_i: i32,
            pub bytes_n: BytesN<32>,
            pub vec: Vec<Bytes>,
            pub map: Map<Bytes, Vec<i32>>,
        }

        #[test]
        fn test_user_defined_pubstruct_pub_fields() {
            run_test::<PubStructPubFields>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct PrivTupleStruct(u32, i32, BytesN<32>, Vec<Bytes>, Map<Bytes, Vec<i32>>);

        #[test]
        fn test_user_defined_priv_tuple_struct() {
            run_test::<PrivTupleStruct>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct PrivTupleStructPubFields(
            pub u32,
            pub i32,
            pub BytesN<32>,
            pub Vec<Bytes>,
            pub Map<Bytes, Vec<i32>>,
        );

        #[test]
        fn test_user_defined_priv_tuple_struct_pub_fields() {
            run_test::<PrivTupleStructPubFields>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct PubTupleStruct(u32, i32, BytesN<32>, Vec<Bytes>, Map<Bytes, Vec<i32>>);

        #[test]
        fn test_user_defined_pub_tuple_struct() {
            run_test::<PubTupleStruct>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct PubTupleStructPubFields(
            pub u32,
            pub i32,
            pub BytesN<32>,
            pub Vec<Bytes>,
            pub Map<Bytes, Vec<i32>>,
        );

        #[test]
        fn test_user_defined_pub_tuple_struct_pub_fields() {
            run_test::<PubTupleStructPubFields>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub(crate) struct PubCrateStruct(u32);

        #[test]
        fn test_user_defined_pub_crate_struct() {
            run_test::<PubCrateStruct>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        enum PrivEnum {
            A(u32),
            Aa(u32, u32),
            C,
            D,
        }

        #[test]
        fn test_user_defined_priv_enum() {
            run_test::<PrivEnum>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum PubEnum {
            A(u32),
            C,
            D,
        }

        #[test]
        fn test_user_defined_pub_enum() {
            run_test::<PubEnum>();
        }

        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub(crate) enum PubCrateEnum {
            A(u32),
            C,
            D,
        }

        #[test]
        fn test_user_defined_pub_crate_enum() {
            run_test::<PubCrateEnum>();
        }

        #[contracttype]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum PrivEnumInt {
            A = 1,
            C = 2,
            D = 3,
        }

        #[test]
        fn test_user_defined_priv_enum_int() {
            run_test::<PrivEnumInt>();
        }

        #[contracttype]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum PubEnumInt {
            A = 1,
            C = 2,
            D = 3,
        }

        #[test]
        fn test_user_defined_pub_enum_int() {
            run_test::<PubEnumInt>();
        }

        #[test]
        fn test_declared_inside_a_fn() {
            #[contracttype]
            struct Foo {
                a: u32,
            }

            #[contracttype]
            enum Bar {
                Baz,
                Qux,
            }

            run_test::<Foo>();
            run_test::<Bar>();
        }
    }
}
