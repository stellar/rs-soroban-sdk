//! Support for fuzzing Soroban contracts with [`fuzzcheck`].
//!
//! [`fuzzcheck`]: https://github.com/loiclec/fuzzcheck-rs
//!
//! This module provides [`fuzzcheck`] mutators for Soroban contract types, so
//! that contracts can be fuzzed with the structure-aware, coverage-guided
//! fuzzing engine that fuzzcheck implements.
//!
//! This module is only available when the "fuzzcheck" Cargo feature is defined.
//! That feature requires a nightly compiler, as does fuzzcheck itself.
//!
//! ## The `SorobanFuzzcheck` trait
//!
//! Like the [`arbitrary`] module, this module builds on the observation that
//! Soroban types are managed by the host environment, and so must be created
//! from an [`Env`], while fuzzing engines generate values from nothing. The
//! values fuzzcheck generates are therefore _prototypes_ that the fuzz test
//! converts to contract values with [`FromVal`] or [`IntoVal`].
//!
//! [`arbitrary`]: crate::testutils::arbitrary
//! [`Env`]: crate::Env
//! [`FromVal`]: crate::FromVal
//! [`IntoVal`]: crate::IntoVal
//!
//! The prototype types are the same ones the [`arbitrary`] module uses, named
//! by [`SorobanArbitrary::Prototype`], and the mutator that generates and
//! mutates them is named by [`SorobanFuzzcheck::Mutator`]:
//!
//! ```
//! # use soroban_sdk::testutils::arbitrary::SorobanArbitrary;
//! # use soroban_sdk::testutils::fuzzcheck::fuzzcheck::Mutator;
//! pub trait SorobanFuzzcheck: SorobanArbitrary
//! where
//!     Self::Prototype: Clone + 'static,
//! {
//!     type Mutator: Mutator<Self::Prototype>;
//!     fn soroban_mutator() -> Self::Mutator;
//! }
//! ```
//!
//! `SorobanFuzzcheck` is implemented for the same builtin types that
//! [`SorobanArbitrary`] is implemented for, and is derived for all types with
//! the [`contracttype`] attribute when the "fuzzcheck" feature is enabled.
//!
//! [`contracttype`]: crate::contracttype
//!
//! Note that the code fuzzcheck's derive macros generate requires the
//! `coverage_attribute` feature and the std library, and so crates containing
//! `#[contracttype]` types and compiled with the "fuzzcheck" feature must
//! declare both:
//!
//! ```ignore
//! #![no_std]
//! #![cfg_attr(feature = "fuzzcheck", feature(coverage_attribute))]
//!
//! #[cfg(feature = "fuzzcheck")]
//! #[macro_use]
//! extern crate std;
//! ```
//!
//! Recursive contract types, those with a field that contains the type itself,
//! are not supported, because fuzzcheck mutators cannot be derived for them.
//! Opt out of deriving a mutator for such a type with
//! `#[contracttype(fuzzcheck = false)]`.
//!
//! ## Example
//!
//! ```
//! # fn main() {}
//! use soroban_sdk::testutils::arbitrary::SorobanArbitrary;
//! use soroban_sdk::testutils::fuzzcheck::{DebugSerializer, SorobanFuzzcheck};
//! use soroban_sdk::{contract, contractimpl, Env, IntoVal, Vec, U256};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn run(numbers: Vec<U256>) -> u32 {
//!         numbers.len()
//!     }
//! }
//!
//! type Input = <Vec<U256> as SorobanArbitrary>::Prototype;
//!
//! fn test_contract(input: &Input) {
//!     let env = Env::default();
//!     let contract_id = env.register(Contract, ());
//!     let client = ContractClient::new(&env, &contract_id);
//!     let numbers: Vec<U256> = input.into_val(&env);
//!     let _ = client.try_run(&numbers);
//! }
//!
//! // Fuzz tests are compiled only by `cargo fuzzcheck`, which sets the
//! // "fuzzing" cfg, because they require coverage instrumentation.
//! #[cfg(all(fuzzing, test))]
//! #[test]
//! fn fuzz_run() {
//!     let result = fuzzcheck::fuzz_test(test_contract)
//!         .mutator(<Vec<U256> as SorobanFuzzcheck>::soroban_mutator())
//!         .serializer(DebugSerializer::default())
//!         .default_sensor_and_pool()
//!         .arguments_from_cargo_fuzzcheck()
//!         .stop_after_first_test_failure(true)
//!         .launch();
//!     assert!(!result.found_test_failure);
//! }
//! ```
//!
//! ## Limitations
//!
//! Doc tests and other test code that contain `#[contracttype]` types need the
//! same declarations, and so enabling this feature can require changes to test
//! code that has nothing to do with fuzzing.
//!
//! The mutators for [`Vec`] and [`Map`] only generate collections holding
//! values of their declared element types, while the [`arbitrary`] module also
//! occasionally generates collections holding values of the wrong type. The
//! mutator for [`Val`] generates values nested at most two levels deep.
//!
//! [`Vec`]: crate::Vec
//! [`Map`]: crate::Map
//! [`Val`]: crate::Val

/// A reexport of the `fuzzcheck` crate.
///
/// Used by the `contracttype` macro to derive mutators.
pub use ::fuzzcheck;

// Used often enough in fuzz tests to want direct access to them.
pub use ::fuzzcheck::{DefaultMutator, Mutator};

pub use api::*;
pub use serializer::DebugSerializer;

mod api {
    use crate::testutils::arbitrary::SorobanArbitrary;
    use fuzzcheck::Mutator;

    /// A contract value with a prototype that fuzzcheck can generate and
    /// mutate.
    ///
    /// The prototype type is [`SorobanArbitrary::Prototype`], and can be
    /// converted to the contract type with [`FromVal`] or [`IntoVal`].
    ///
    /// [`FromVal`]: crate::FromVal
    /// [`IntoVal`]: crate::IntoVal
    pub trait SorobanFuzzcheck: SorobanArbitrary
    where
        Self::Prototype: Clone + 'static,
    {
        /// A [`Mutator`] that generates and mutates prototypes of this type.
        type Mutator: Mutator<Self::Prototype>;

        /// Create the mutator.
        fn soroban_mutator() -> Self::Mutator;
    }
}

/// Implementations of `SorobanFuzzcheck` for types whose prototype derives
/// fuzzcheck's `DefaultMutator`, and where the derived mutator generates only
/// prototypes that convert to a contract value.
mod default_mutators {
    use super::api::*;
    use crate::crypto::bls12_381::{
        Bls12381Fp, Bls12381Fp2, Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine,
    };
    use crate::crypto::bn254::{Bn254Fp, Bn254Fr, Bn254G1Affine, Bn254G2Affine};
    use crate::testutils::arbitrary::SorobanArbitrary;
    use crate::{Address, Bytes, BytesN, Duration, MuxedAddress, String, Timepoint, I256, U256};
    use fuzzcheck::DefaultMutator;

    macro_rules! impl_default_mutator {
        ($ty:ty) => {
            impl SorobanFuzzcheck for $ty {
                type Mutator = <<$ty as SorobanArbitrary>::Prototype as DefaultMutator>::Mutator;
                fn soroban_mutator() -> Self::Mutator {
                    <<$ty as SorobanArbitrary>::Prototype as DefaultMutator>::default_mutator()
                }
            }
        };
    }

    // Rust scalars, and Soroban types that are their own prototype.
    impl_default_mutator!(());
    impl_default_mutator!(bool);
    impl_default_mutator!(u32);
    impl_default_mutator!(i32);
    impl_default_mutator!(u64);
    impl_default_mutator!(i64);

    // Soroban types with a prototype of plain data.
    impl_default_mutator!(U256);
    impl_default_mutator!(I256);
    impl_default_mutator!(Bytes);
    impl_default_mutator!(String);
    impl_default_mutator!(Address);
    impl_default_mutator!(MuxedAddress);
    impl_default_mutator!(Timepoint);
    impl_default_mutator!(Duration);
    impl_default_mutator!(Bls12381Fp);
    impl_default_mutator!(Bls12381Fp2);
    impl_default_mutator!(Bls12381Fr);
    impl_default_mutator!(Bls12381G1Affine);
    impl_default_mutator!(Bls12381G2Affine);
    impl_default_mutator!(Bn254Fp);
    impl_default_mutator!(Bn254Fr);
    impl_default_mutator!(Bn254G1Affine);
    impl_default_mutator!(Bn254G2Affine);

    impl<const N: usize> SorobanFuzzcheck for BytesN<N> {
        type Mutator = <<BytesN<N> as SorobanArbitrary>::Prototype as DefaultMutator>::Mutator;
        fn soroban_mutator() -> Self::Mutator {
            <<BytesN<N> as SorobanArbitrary>::Prototype as DefaultMutator>::default_mutator()
        }
    }
}

/// Implementations of `SorobanFuzzcheck` for types that need a mutator built by
/// hand, either because their prototype is a type this crate cannot implement
/// `DefaultMutator` for, or because not all values of their prototype convert to
/// a contract value.
mod custom_mutators {
    use super::api::*;
    use crate::testutils::arbitrary::objects::{
        ArbitraryMap, ArbitraryOption, ArbitrarySymbol, ArbitraryVec,
    };
    use crate::testutils::arbitrary::SorobanArbitrary;
    use crate::xdr::{ScError, ScErrorCode, ScErrorType};
    use crate::{Env, Error, Map, Symbol, TryFromVal, Val, Vec};
    use fuzzcheck::mutators::map::MapMutator;
    use fuzzcheck::mutators::option::OptionMutator;
    use fuzzcheck::mutators::tuples::{Tuple2, Tuple2Mutator, TupleMutatorWrapper};
    use fuzzcheck::mutators::vector::VecMutator;
    use fuzzcheck::{DefaultMutator, Mutator};
    use std::vec::Vec as RustVec;

    impl SorobanFuzzcheck for u128 {
        type Mutator = impl Mutator<u128>;
        fn soroban_mutator() -> Self::Mutator {
            MapMutator::new(
                <(u64, u64)>::default_mutator(),
                |v: &u128| Some(((v >> 64) as u64, *v as u64)),
                |&(hi, lo): &(u64, u64)| (u128::from(hi) << 64) | u128::from(lo),
                |_, cplx| cplx,
            )
        }
    }

    impl SorobanFuzzcheck for i128 {
        type Mutator = impl Mutator<i128>;
        fn soroban_mutator() -> Self::Mutator {
            MapMutator::new(
                <(u64, u64)>::default_mutator(),
                |v: &i128| Some((((*v as u128) >> 64) as u64, *v as u64)),
                |&(hi, lo): &(u64, u64)| (((u128::from(hi)) << 64) | u128::from(lo)) as i128,
                |_, cplx| cplx,
            )
        }
    }

    /// The error types that errors are generated for. All of them but
    /// `ScErrorType::Contract` are paired with an `ScErrorCode`.
    const ERROR_TYPES: [ScErrorType; 10] = [
        ScErrorType::Contract,
        ScErrorType::WasmVm,
        ScErrorType::Context,
        ScErrorType::Storage,
        ScErrorType::Object,
        ScErrorType::Crypto,
        ScErrorType::Events,
        ScErrorType::Budget,
        ScErrorType::Value,
        ScErrorType::Auth,
    ];

    /// The error codes that errors of a type other than `ScErrorType::Contract`
    /// are generated for.
    const ERROR_CODES: [ScErrorCode; 10] = [
        ScErrorCode::ArithDomain,
        ScErrorCode::IndexBounds,
        ScErrorCode::InvalidInput,
        ScErrorCode::MissingValue,
        ScErrorCode::ExistingValue,
        ScErrorCode::ExceededLimit,
        ScErrorCode::InvalidAction,
        ScErrorCode::InternalError,
        ScErrorCode::UnexpectedType,
        ScErrorCode::UnexpectedSize,
    ];

    impl SorobanFuzzcheck for Error {
        type Mutator = impl Mutator<Error>;
        fn soroban_mutator() -> Self::Mutator {
            MapMutator::new(
                <(u32, u32)>::default_mutator(),
                |error: &Error| {
                    // Errors that are not one of the types and codes generated
                    // are not representable, and so are rejected.
                    let sc_error = ScError::try_from(*error).ok()?;
                    let type_ = ERROR_TYPES.iter().position(|type_| error.is_type(*type_))? as u32;
                    match sc_error {
                        ScError::Contract(code) => Some((type_, code)),
                        _ => {
                            let code =
                                ERROR_CODES.iter().position(|code| error.is_code(*code))? as u32;
                            Some((type_, code))
                        }
                    }
                },
                |&(type_, code): &(u32, u32)| {
                    let type_ = ERROR_TYPES[type_ as usize % ERROR_TYPES.len()];
                    if let ScErrorType::Contract = type_ {
                        Error::from_contract_error(code)
                    } else {
                        Error::from_type_and_code(
                            type_,
                            ERROR_CODES[code as usize % ERROR_CODES.len()],
                        )
                    }
                },
                |_, cplx| cplx,
            )
        }
    }

    /// The characters a `Symbol` may contain.
    const SYMBOL_CHARS: &[u8; 63] =
        b"_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    /// The maximum number of characters a `Symbol` may contain.
    const SYMBOL_MAX_LEN: usize = 32;

    impl SorobanFuzzcheck for Symbol {
        type Mutator = impl Mutator<ArbitrarySymbol>;
        fn soroban_mutator() -> Self::Mutator {
            // Symbols contain a limited set of characters, so the mutator
            // generates indexes into that set rather than characters.
            MapMutator::new(
                VecMutator::new(u8::default_mutator(), 0..=SYMBOL_MAX_LEN),
                |symbol: &ArbitrarySymbol| {
                    symbol
                        .s
                        .bytes()
                        .map(|char| {
                            SYMBOL_CHARS
                                .iter()
                                .position(|valid| *valid == char)
                                .map(|index| index as u8)
                        })
                        .collect()
                },
                |indexes: &RustVec<u8>| ArbitrarySymbol {
                    s: indexes
                        .iter()
                        .map(|index| char::from(SYMBOL_CHARS[*index as usize % SYMBOL_CHARS.len()]))
                        .collect(),
                },
                |_, cplx| cplx,
            )
        }
    }

    impl<T> SorobanFuzzcheck for Option<T>
    where
        T: SorobanFuzzcheck,
        T::Prototype: Clone + 'static,
        Val: TryFromVal<Env, T>,
    {
        type Mutator = impl Mutator<ArbitraryOption<T::Prototype>>;
        fn soroban_mutator() -> Self::Mutator {
            MapMutator::new(
                OptionMutator::new(T::soroban_mutator()),
                |option: &ArbitraryOption<T::Prototype>| Some(option.0.clone()),
                |option: &Option<T::Prototype>| ArbitraryOption(option.clone()),
                |_, cplx| cplx,
            )
        }
    }

    impl<T> SorobanFuzzcheck for Vec<T>
    where
        T: SorobanFuzzcheck,
        T::Prototype: Clone + 'static,
    {
        type Mutator = impl Mutator<ArbitraryVec<T::Prototype>>;
        fn soroban_mutator() -> Self::Mutator {
            // Only the `Good` variant is generated, so that the values in the
            // vec always have the vec's element type.
            MapMutator::new(
                VecMutator::new(T::soroban_mutator(), 0..=usize::MAX),
                |vec: &ArbitraryVec<T::Prototype>| match vec {
                    ArbitraryVec::Good(vec) => Some(vec.clone()),
                    ArbitraryVec::Wrong(_) => None,
                },
                |vec: &RustVec<T::Prototype>| ArbitraryVec::Good(vec.clone()),
                |_, cplx| cplx,
            )
        }
    }

    impl<K, V> SorobanFuzzcheck for Map<K, V>
    where
        K: SorobanFuzzcheck,
        V: SorobanFuzzcheck,
        K::Prototype: Clone + 'static,
        V::Prototype: Clone + 'static,
    {
        type Mutator = impl Mutator<ArbitraryMap<K::Prototype, V::Prototype>>;
        fn soroban_mutator() -> Self::Mutator {
            // Only the `Good` variant is generated, so that the keys and values
            // in the map always have the map's key and value types.
            MapMutator::new(
                VecMutator::new(
                    TupleMutatorWrapper::<_, Tuple2<K::Prototype, V::Prototype>>::new(
                        Tuple2Mutator::new(K::soroban_mutator(), V::soroban_mutator()),
                    ),
                    0..=usize::MAX,
                ),
                |map: &ArbitraryMap<K::Prototype, V::Prototype>| match map {
                    ArbitraryMap::Good(entries) => Some(entries.clone()),
                    ArbitraryMap::WrongKey(_) | ArbitraryMap::WrongValue(_) => None,
                },
                |entries: &RustVec<(K::Prototype, V::Prototype)>| {
                    ArbitraryMap::Good(entries.clone())
                },
                |_, cplx| cplx,
            )
        }
    }
}

/// Implementations of `SorobanFuzzcheck` for tuples of Soroban types.
mod tuples {
    use super::api::*;
    use crate::testutils::arbitrary::tuples::*;
    use crate::{Env, TryIntoVal, Val};
    use fuzzcheck::mutators::map::MapMutator;
    use fuzzcheck::mutators::tuples::*;
    use fuzzcheck::Mutator;

    macro_rules! impl_tuple {
        ($proto: ident, $tuple_kind: ident, $tuple_mutator: ident, $($ty: ident),+ ) => {
            impl<$($ty,)*> SorobanFuzzcheck for ($($ty,)*)
            where
                $($ty: SorobanFuzzcheck + TryIntoVal<Env, Val>,)*
                $($ty::Prototype: Clone + 'static,)*
            {
                type Mutator = impl Mutator<$proto<$($ty::Prototype,)*>>;
                #[allow(non_snake_case)] // naming bindings T1, etc.
                fn soroban_mutator() -> Self::Mutator {
                    MapMutator::new(
                        TupleMutatorWrapper::<_, $tuple_kind<$($ty::Prototype,)*>>::new(
                            $tuple_mutator::new($($ty::soroban_mutator(),)*),
                        ),
                        |proto: &$proto<$($ty::Prototype,)*>| Some(($(proto.$ty.clone(),)*)),
                        |($($ty,)*): &($($ty::Prototype,)*)| $proto { $($ty: $ty.clone(),)* },
                        |_, cplx| cplx,
                    )
                }
            }
        }
    }

    impl_tuple!(ArbitraryTuple1, Tuple1, Tuple1Mutator, T1);
    impl_tuple!(ArbitraryTuple2, Tuple2, Tuple2Mutator, T1, T2);
    impl_tuple!(ArbitraryTuple3, Tuple3, Tuple3Mutator, T1, T2, T3);
    impl_tuple!(ArbitraryTuple4, Tuple4, Tuple4Mutator, T1, T2, T3, T4);
    impl_tuple!(ArbitraryTuple5, Tuple5, Tuple5Mutator, T1, T2, T3, T4, T5);
    impl_tuple!(
        ArbitraryTuple6,
        Tuple6,
        Tuple6Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6
    );
    impl_tuple!(
        ArbitraryTuple7,
        Tuple7,
        Tuple7Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7
    );
    impl_tuple!(
        ArbitraryTuple8,
        Tuple8,
        Tuple8Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8
    );
    impl_tuple!(
        ArbitraryTuple9,
        Tuple9,
        Tuple9Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9
    );
    impl_tuple!(
        ArbitraryTuple10,
        Tuple10,
        Tuple10Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10
    );
    impl_tuple!(
        ArbitraryTuple11,
        Tuple11,
        Tuple11Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11
    );
    impl_tuple!(
        ArbitraryTuple12,
        Tuple12,
        Tuple12Mutator,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12
    );
}

/// The implementation of `SorobanFuzzcheck` for `Val`.
///
/// The prototype of `Val` is recursive: a `Val` may be a vec or map of `Val`s.
/// Mutators are not able to be recursive without being defined by hand, so the
/// mutator here generates `Val`s that are nested at most two levels deep, by
/// mapping from the non-recursive types defined in this module.
mod val {
    use super::api::*;
    use crate::testutils::arbitrary::composite::{ArbitraryVal, ArbitraryValMap, ArbitraryValVec};
    use crate::testutils::arbitrary::objects::{ArbitraryMap, ArbitraryVec};
    use crate::testutils::arbitrary::SorobanArbitrary;
    use crate::{Address, Bytes, Duration, Error, String, Symbol, Timepoint, Val, I256, U256};
    use fuzzcheck::mutators::map::MapMutator;
    use fuzzcheck::{DefaultMutator, Mutator};
    use std::vec::Vec as RustVec;

    /// A `Val` that contains no other `Val`s.
    #[derive(Clone, Debug, DefaultMutator)]
    pub enum ValScalarProto {
        Void,
        Bool(bool),
        Error(
            #[field_mutator(<Error as SorobanFuzzcheck>::Mutator = { <Error as SorobanFuzzcheck>::soroban_mutator() })]
             <Error as SorobanArbitrary>::Prototype,
        ),
        U32(u32),
        I32(i32),
        U64(u64),
        I64(i64),
        U128(
            #[field_mutator(<u128 as SorobanFuzzcheck>::Mutator = { <u128 as SorobanFuzzcheck>::soroban_mutator() })]
             <u128 as SorobanArbitrary>::Prototype,
        ),
        I128(
            #[field_mutator(<i128 as SorobanFuzzcheck>::Mutator = { <i128 as SorobanFuzzcheck>::soroban_mutator() })]
             <i128 as SorobanArbitrary>::Prototype,
        ),
        U256(
            #[field_mutator(<U256 as SorobanFuzzcheck>::Mutator = { <U256 as SorobanFuzzcheck>::soroban_mutator() })]
             <U256 as SorobanArbitrary>::Prototype,
        ),
        I256(
            #[field_mutator(<I256 as SorobanFuzzcheck>::Mutator = { <I256 as SorobanFuzzcheck>::soroban_mutator() })]
             <I256 as SorobanArbitrary>::Prototype,
        ),
        Bytes(
            #[field_mutator(<Bytes as SorobanFuzzcheck>::Mutator = { <Bytes as SorobanFuzzcheck>::soroban_mutator() })]
             <Bytes as SorobanArbitrary>::Prototype,
        ),
        String(
            #[field_mutator(<String as SorobanFuzzcheck>::Mutator = { <String as SorobanFuzzcheck>::soroban_mutator() })]
             <String as SorobanArbitrary>::Prototype,
        ),
        Symbol(
            #[field_mutator(<Symbol as SorobanFuzzcheck>::Mutator = { <Symbol as SorobanFuzzcheck>::soroban_mutator() })]
             <Symbol as SorobanArbitrary>::Prototype,
        ),
        Address(
            #[field_mutator(<Address as SorobanFuzzcheck>::Mutator = { <Address as SorobanFuzzcheck>::soroban_mutator() })]
             <Address as SorobanArbitrary>::Prototype,
        ),
        Timepoint(
            #[field_mutator(<Timepoint as SorobanFuzzcheck>::Mutator = { <Timepoint as SorobanFuzzcheck>::soroban_mutator() })]
             <Timepoint as SorobanArbitrary>::Prototype,
        ),
        Duration(
            #[field_mutator(<Duration as SorobanFuzzcheck>::Mutator = { <Duration as SorobanFuzzcheck>::soroban_mutator() })]
             <Duration as SorobanArbitrary>::Prototype,
        ),
    }

    /// A `Val`, containing at most one level of `Val`s.
    #[derive(Clone, Debug, DefaultMutator)]
    pub enum ValProto {
        Scalar(ValScalarProto),
        Vec(RustVec<ValScalarProto>),
        Map(RustVec<(ValScalarProto, ValScalarProto)>),
    }

    impl From<&ValScalarProto> for ArbitraryVal {
        fn from(v: &ValScalarProto) -> ArbitraryVal {
            match v {
                ValScalarProto::Void => ArbitraryVal::Void,
                ValScalarProto::Bool(v) => ArbitraryVal::Bool(*v),
                ValScalarProto::Error(v) => ArbitraryVal::Error(*v),
                ValScalarProto::U32(v) => ArbitraryVal::U32(*v),
                ValScalarProto::I32(v) => ArbitraryVal::I32(*v),
                ValScalarProto::U64(v) => ArbitraryVal::U64(*v),
                ValScalarProto::I64(v) => ArbitraryVal::I64(*v),
                ValScalarProto::U128(v) => ArbitraryVal::U128(*v),
                ValScalarProto::I128(v) => ArbitraryVal::I128(*v),
                ValScalarProto::U256(v) => ArbitraryVal::U256(v.clone()),
                ValScalarProto::I256(v) => ArbitraryVal::I256(v.clone()),
                ValScalarProto::Bytes(v) => ArbitraryVal::Bytes(v.clone()),
                ValScalarProto::String(v) => ArbitraryVal::String(v.clone()),
                ValScalarProto::Symbol(v) => ArbitraryVal::Symbol(v.clone()),
                ValScalarProto::Address(v) => ArbitraryVal::Address(v.clone()),
                ValScalarProto::Timepoint(v) => ArbitraryVal::Timepoint(v.clone()),
                ValScalarProto::Duration(v) => ArbitraryVal::Duration(v.clone()),
            }
        }
    }

    impl From<&ValProto> for ArbitraryVal {
        fn from(v: &ValProto) -> ArbitraryVal {
            match v {
                ValProto::Scalar(v) => v.into(),
                ValProto::Vec(vals) => ArbitraryVal::Vec(ArbitraryValVec::Val(ArbitraryVec::Good(
                    vals.iter().map(ArbitraryVal::from).collect(),
                ))),
                ValProto::Map(entries) => {
                    ArbitraryVal::Map(ArbitraryValMap::ValToVal(ArbitraryMap::Good(
                        entries
                            .iter()
                            .map(|(k, v)| (ArbitraryVal::from(k), ArbitraryVal::from(v)))
                            .collect(),
                    )))
                }
            }
        }
    }

    impl SorobanFuzzcheck for Val {
        type Mutator = impl Mutator<ArbitraryVal>;
        fn soroban_mutator() -> Self::Mutator {
            MapMutator::new(
                ValProto::default_mutator(),
                // Prototypes that the mutator did not generate itself, such as
                // those read from a corpus on the file system, are rejected,
                // because the mapping from `ValProto` is not reversed.
                |_: &ArbitraryVal| -> Option<ValProto> { None },
                |proto: &ValProto| ArbitraryVal::from(proto),
                |_, cplx| cplx,
            )
        }
    }
}

mod serializer {
    use fuzzcheck::Serializer;
    use std::fmt::Debug;
    use std::marker::PhantomData;

    /// A [`Serializer`] that writes values with their [`Debug`] implementation.
    ///
    /// Soroban prototypes are not deserializable, so values written by this
    /// serializer, such as the failing test cases fuzzcheck saves as artifacts,
    /// can be read by people but not by fuzzcheck.
    pub struct DebugSerializer<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> Default for DebugSerializer<T> {
        fn default() -> Self {
            Self {
                _phantom: PhantomData,
            }
        }
    }

    impl<T> Serializer for DebugSerializer<T>
    where
        T: Debug,
    {
        type Value = T;

        fn extension(&self) -> &str {
            "txt"
        }

        fn from_data(&self, _data: &[u8]) -> Option<Self::Value> {
            None
        }

        fn to_data(&self, value: &Self::Value) -> std::vec::Vec<u8> {
            std::format!("{value:#?}").into_bytes()
        }
    }
}
