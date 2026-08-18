//! Support for property testing Soroban contracts with [`proptest`].
//!
//! This module implements [`proptest`]'s [`Arbitrary`] trait for the "prototype"
//! types of [`SorobanArbitrary`], so that Soroban contract types can be property
//! tested directly, with real shrinking.
//!
//! [`Arbitrary`]: ::proptest::arbitrary::Arbitrary
//! [`SorobanArbitrary`]: crate::testutils::arbitrary::SorobanArbitrary
//!
//! This module is available when the "testutils" Cargo feature is defined, and
//! under this crate's own tests. It is not available for wasm targets.
//!
//! ## Relationship to the `arbitrary` support
//!
//! The [`arbitrary`] module implements the trait `cargo-fuzz` relies on, which
//! turns raw bytes into values. Prototypes built that way have no structure for
//! `proptest` to shrink, so driving `proptest` from them with the
//! `proptest-arbitrary-interop` crate produces failing cases that barely shrink.
//! This module builds the same prototypes out of `proptest` strategies instead.
//! Use the `arbitrary` module for `cargo-fuzz`, and this one for `proptest`.
//!
//! Note one behavioural difference: the `arbitrary` impls occasionally generate
//! ill-typed collections — a `Vec<Address>` holding non-addresses — which is
//! useful over a fuzzer's millions of iterations. The strategies here always
//! generate well-typed collections.
//!
//! [`arbitrary`]: crate::testutils::arbitrary
//!
//! ## Example
//!
//! [`arb`] takes a _contract_ type and produces a strategy of prototypes for it,
//! which convert into the contract type once an [`Env`] exists.
//!
//! [`Env`]: crate::Env
//!
//! ```
//! use proptest::prelude::*;
//! use soroban_sdk::testutils::proptest::arb;
//! use soroban_sdk::{Address, Env, IntoVal, Vec};
//!
//! proptest! {
//!     #[test]
//!     fn test(input in arb::<Vec<Address>>()) {
//!         let env = Env::default();
//!         let addresses: Vec<Address> = input.into_val(&env);
//!         // test the contract with the generated addresses
//!     }
//! }
//! ```
//!
//! Types with the [`contracttype`] attribute get a prototype implementing
//! `proptest`'s `Arbitrary`, so [`any`] works on them directly:
//! `any::<<TimeBound as SorobanArbitrary>::Prototype>()`.
//!
//! [`contracttype`]: crate::contracttype
//! [`any`]: ::proptest::arbitrary::any
//!
//! ## The `Error` type
//!
//! [`Error`] is not defined in this crate and is its own prototype, so coherence
//! forbids implementing `proptest`'s `Arbitrary` for it. Every prototype that
//! *contains* an `Error` still works, including a `#[contracttype]` with an
//! `Error` field, because the generic impls here are bounded on [`ProtoStrategy`]
//! rather than `Arbitrary`. Only `any::<Error>()` itself is impossible; use
//! [`arb`] or [`arb_error`].
//!
//! ## How large generated values get
//!
//! A `#[contracttype]` may be recursive — `struct Node { children: Vec<Node> }` —
//! so strategies for them cannot be built by naive recursion. Construction is
//! bounded by a budget of collection elements, [`NODE_BUDGET`], divided up as the
//! strategy is built: a container gives its elements what is left after its own
//! length is paid for, and the fields of a struct, tuple or enum variant share
//! what their prototype was given. A container with nothing left generates an
//! empty collection without building an element strategy, which is what
//! terminates the recursion.
//!
//! So a generated value holds at most `NODE_BUDGET` collection elements, whatever
//! a type's field count, tuple arity or cycle shape. Two things it does *not*
//! bound:
//!
//! - Host objects. Every `#[contracttype]` struct or enum is itself a host map or
//!   vec, so the host-side object count and container depth of a converted value
//!   are larger than the element count — a recursive UDT reaches roughly twice
//!   the container depth of the prototype.
//! - Recursion coverage past the budget. Deeply nested positions get the smallest
//!   value of their type rather than none: a fifth level of nested `Vec` still
//!   generates, but only a couple of elements, so a property about a collection
//!   that deep is weak rather than vacuous.
//!
//! Two consequences worth knowing. The bound is shared out from the entry point,
//! so `arb::<Vec<Node>>()` generates shallower `Node`s than `arb::<Node>()` — a
//! recursive type is best property tested directly. And a `Val` prototype charges
//! its own worst case against the budget up front, because its containers are
//! built while values are generated rather than while the strategy is: a `Val`
//! with little budget left generates scalars.

// The examples above are `proptest!` blocks, which expand to `#[test]`
// functions. Doctests are disabled for this crate, so they are never run.
#![allow(clippy::test_attr_in_doctest)]

/// A reexport of the `proptest` crate.
///
/// Used by the `contracttype` macro, so that contract crates need no `proptest`
/// dependency of their own. Note that this makes `proptest` part of this crate's
/// public API, and that it is reexported with `default-features = false` plus
/// "std": the `fork`, `timeout` and `bit-set` features are off, so
/// `ProptestConfig`'s `fork` and `timeout` fields and `proptest::bits::bitset`
/// are absent. A crate that wants them should depend on `proptest` itself.
pub use ::proptest;

use core::cell::Cell;
use core::fmt::Debug;
use std::string::String as RustString;

use ::proptest::arbitrary::{any, Arbitrary};
use ::proptest::collection;
use ::proptest::option;
use ::proptest::prop_oneof;
use ::proptest::sample::select;
use ::proptest::strategy::{BoxedStrategy, Just, LazyJust, Strategy, Union};

use super::arbitrary::composite::{
    ArbitraryVal, ArbitraryValMap, ArbitraryValOption, ArbitraryValVec,
};
use super::arbitrary::objects::*;
use super::arbitrary::tuples::*;
use super::arbitrary::{SorobanArbitrary, SYMBOL_CHARS};
use crate::crypto::bls12_381::{
    FP2_SERIALIZED_SIZE, FP_SERIALIZED_SIZE, G1_SERIALIZED_SIZE, G2_SERIALIZED_SIZE,
};
use crate::crypto::bn254::{
    BN254_FP_SERIALIZED_SIZE, BN254_G1_SERIALIZED_SIZE, BN254_G2_SERIALIZED_SIZE,
};
use crate::xdr::{ScErrorCode, ScErrorType};
use crate::Error;

/// The most elements a `Vec` or `Map` prototype generates.
pub const LEN_MAX: usize = 7;

/// The most collection elements a generated value holds.
///
/// Large enough that ordinary shapes are generated in full: four levels of
/// nested `Vec` at `LEN_MAX` need `7 + 7² + 7³ + 7⁴`, and a fifth level is
/// generated too, though only a couple of elements wide.
pub const NODE_BUDGET: usize = 8192;

/// The most container strategies one construction builds.
///
/// A backstop on construction *work*, which the element budget does not bound:
/// an enum's variants are all built but only one is generated, so each keeps the
/// whole element budget, and a recursive enum with many recursive variants would
/// otherwise build a strategy for every path through the cycle. Generous enough
/// that no shape in this crate's tests reaches it.
const STRATEGY_CAP: usize = 100_000;

/// The most levels of `Val` prototypes that nest inside each other.
const VAL_DEPTH: u32 = 2;

/// The length of generated `Bytes` values, in bytes.
const BYTES_LEN: core::ops::Range<usize> = 0..32;

/// The length of generated `String` values, in `char`s, so up to four times as
/// many bytes.
const STRING_LEN: core::ops::Range<usize> = 0..32;

/// The length of generated `Symbol` values. Symbols hold at most 32 characters.
const SYMBOL_LEN: core::ops::Range<usize> = 0..33;

thread_local! {
    /// Collection elements still available to the strategy being built.
    ///
    /// Strategy construction is eager and single-threaded, so this is the share
    /// belonging to the prototype under construction. Nothing may read it while
    /// values are generated: the deferred closures in this module — the
    /// `prop_recursive` in `arb_val_proto` and the `LazyJust` base cases — build
    /// no strategy that consults it. Breaking that would degrade generated
    /// values rather than corrupt anything, so it is a comment and a
    /// `debug_assert!`, not a type.
    static BUDGET: Cell<usize> = const { Cell::new(NODE_BUDGET) };
    /// Container strategies built so far in this construction; see STRATEGY_CAP.
    static STRATEGIES: Cell<usize> = const { Cell::new(0) };
    /// Nesting depth of live `BudgetGuard`s. Zero means no construction is in
    /// progress, so the next guard is the outermost one.
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// Restores the enclosing prototype's budget when it goes out of scope,
/// including when construction panics.
struct BudgetGuard {
    budget: usize,
}

impl BudgetGuard {
    fn enter(budget: usize) -> Self {
        let previous = BUDGET.replace(budget);
        if DEPTH.replace(DEPTH.get() + 1) == 0 {
            // The outermost guard of a construction: start counting afresh.
            STRATEGIES.set(0);
        }
        BudgetGuard { budget: previous }
    }
}

impl Drop for BudgetGuard {
    fn drop(&mut self) {
        BUDGET.set(self.budget);
        DEPTH.set(DEPTH.get() - 1);
    }
}

/// Divides the budget between the fields of a struct, tuple or enum variant.
///
/// Called by the `contracttype` macro and by the tuple prototypes, wrapped
/// around the eager construction of the fields' strategies.
#[doc(hidden)]
pub fn with_fields<R>(fields: usize, f: impl FnOnce() -> R) -> R {
    let _guard = BudgetGuard::enter(BUDGET.get() / fields.max(1));
    f()
}

/// How a container divides its budget between its length and its elements.
struct Apportion {
    /// The most elements the collection generates.
    len: usize,
    /// The budget each element slot is built with.
    each: usize,
}

impl Apportion {
    /// `slots` is how many budgeted values one element holds: one for a `Vec`
    /// element, two for a `Map` entry.
    ///
    /// `None` means generate an empty collection, without building an element
    /// strategy — the base case that terminates recursion.
    fn new(slots: usize) -> Option<Apportion> {
        debug_assert!(slots > 0, "a container element holds at least one value");
        let built = STRATEGIES.get();
        if built >= STRATEGY_CAP {
            return None;
        }
        STRATEGIES.set(built + 1);
        let budget = BUDGET.get();
        let len = (budget / slots).min(LEN_MAX);
        if len == 0 {
            return None;
        }
        let spent = len * slots;
        Some(Apportion {
            len,
            each: (budget - spent) / spent,
        })
    }

    /// The range of lengths to generate, at most `len`.
    fn range(&self) -> core::ops::Range<usize> {
        0..self.len + 1
    }

    /// Builds an element strategy with this container's per-element share.
    fn element<S>(&self, f: impl FnOnce() -> S) -> S {
        let _guard = BudgetGuard::enter(self.each);
        f()
    }
}

/// A `proptest` strategy for a [`SorobanArbitrary::Prototype`] type.
///
/// A stand-in for `proptest`'s [`Arbitrary`] that is also implemented for
/// [`Error`], which cannot implement `Arbitrary` because neither it nor that
/// trait is defined in this crate. Every prototype implements this trait,
/// including those generated by the [`contracttype`] macro, and every prototype
/// except `Error` itself also implements `Arbitrary` by delegating to it.
///
/// [`contracttype`]: crate::contracttype
///
/// Implementations must build their strategy eagerly, when `proto_strategy` is
/// called, and must not assume anything about the budget they are given: it
/// depends on where the prototype sits in the strategy being built.
pub trait ProtoStrategy: Debug + Sized + 'static {
    /// A strategy generating values of this prototype type.
    fn proto_strategy() -> BoxedStrategy<Self>;
}

/// A strategy generating prototypes of the given Soroban contract type.
///
/// Unlike `proptest_arbitrary_interop::arb`, which takes the prototype type,
/// this takes the contract type and yields its prototype, which converts into
/// the contract type with [`IntoVal`] once an [`Env`] exists.
///
/// [`IntoVal`]: crate::IntoVal
/// [`Env`]: crate::Env
pub fn arb<T>() -> BoxedStrategy<T::Prototype>
where
    T: SorobanArbitrary,
    T::Prototype: ProtoStrategy,
{
    T::Prototype::proto_strategy()
}

/// A strategy generating prototypes of the given prototype type.
fn proto<T: ProtoStrategy>() -> BoxedStrategy<T> {
    T::proto_strategy()
}

/// A strategy generating [`Error`] values with valid type and code
/// combinations, including contract errors.
///
/// `Error` cannot implement `proptest`'s [`Arbitrary`], so use this, or
/// `arb::<Error>()`, instead of `any::<Error>()`.
pub fn arb_error() -> BoxedStrategy<Error> {
    prop_oneof![
        select(ScErrorType::VARIANTS.to_vec())
            .prop_flat_map(|type_| {
                select(ScErrorCode::VARIANTS.to_vec()).prop_map(move |code| (type_, code))
            })
            .prop_map(|(type_, code)| Error::from_type_and_code(type_, code)),
        // Contract errors carry an arbitrary u32 rather than an `ScErrorCode`.
        any::<u32>().prop_map(Error::from_contract_error),
    ]
    .boxed()
}

/// Implements [`ProtoStrategy`] and, delegating to it, `proptest`'s
/// [`Arbitrary`].
///
/// Both impls come from one macro so that a prototype cannot end up with one
/// and not the other.
macro_rules! impl_proto_strategy {
    ([$($param:tt)*] $ty:ty => $strategy:expr) => {
        impl<$($param)*> ProtoStrategy for $ty {
            fn proto_strategy() -> BoxedStrategy<Self> {
                $strategy
            }
        }

        impl<$($param)*> Arbitrary for $ty {
            type Parameters = ();
            type Strategy = BoxedStrategy<Self>;
            fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
                <Self as ProtoStrategy>::proto_strategy()
            }
        }
    };
}

/// Implements [`ProtoStrategy`] for types that are their own prototype and
/// already implement `proptest`'s [`Arbitrary`].
macro_rules! impl_proto_strategy_for_self {
    ($($ty:ty,)*) => {
        $(
            impl ProtoStrategy for $ty {
                fn proto_strategy() -> BoxedStrategy<Self> {
                    any::<Self>().boxed()
                }
            }
        )*
    };
}

impl_proto_strategy_for_self!((), bool, u32, i32, u64, i64, u128, i128,);

// `Error` is its own prototype but cannot implement `proptest`'s `Arbitrary`.
impl ProtoStrategy for Error {
    fn proto_strategy() -> BoxedStrategy<Self> {
        arb_error()
    }
}

//////////////////////////////////
// Prototypes that hold no collections, and so spend no budget.

fn arb_bytes_proto() -> BoxedStrategy<ArbitraryBytes> {
    collection::vec(any::<u8>(), BYTES_LEN)
        .prop_map(|vec| ArbitraryBytes { vec })
        .boxed()
}

fn arb_string_proto() -> BoxedStrategy<ArbitraryString> {
    collection::vec(any::<char>(), STRING_LEN)
        .prop_map(|chars| ArbitraryString {
            inner: chars.into_iter().collect(),
        })
        .boxed()
}

fn arb_symbol_proto() -> BoxedStrategy<ArbitrarySymbol> {
    collection::vec(
        select(SYMBOL_CHARS.chars().collect::<std::vec::Vec<char>>()),
        SYMBOL_LEN,
    )
    .prop_map(|chars| ArbitrarySymbol {
        s: chars.into_iter().collect::<RustString>(),
    })
    .boxed()
}

impl_proto_strategy!([] ArbitraryU256 => any::<(u64, u64, u64, u64)>()
    .prop_map(|parts| ArbitraryU256 { parts })
    .boxed());
impl_proto_strategy!([] ArbitraryI256 => any::<(i64, u64, u64, u64)>()
    .prop_map(|parts| ArbitraryI256 { parts })
    .boxed());
impl_proto_strategy!([] ArbitraryBytes => arb_bytes_proto());
impl_proto_strategy!([] ArbitraryString => arb_string_proto());
impl_proto_strategy!([] ArbitrarySymbol => arb_symbol_proto());
impl_proto_strategy!([] ArbitraryAddress => any::<[u8; 32]>()
    .prop_map(|inner| ArbitraryAddress { inner })
    .boxed());
impl_proto_strategy!([] ArbitraryMuxedAddress => prop_oneof![
    proto::<ArbitraryAddress>().prop_map(ArbitraryMuxedAddress::Address),
    (any::<[u8; 32]>(), any::<u64>())
        .prop_map(|(ed25519, id)| ArbitraryMuxedAddress::Muxed { ed25519, id }),
]
.boxed());
impl_proto_strategy!([] ArbitraryTimepoint => any::<u64>()
    .prop_map(|inner| ArbitraryTimepoint { inner })
    .boxed());
impl_proto_strategy!([] ArbitraryDuration => any::<u64>()
    .prop_map(|inner| ArbitraryDuration { inner })
    .boxed());
impl_proto_strategy!([] ArbitraryBls12381Fp => any::<[u8; FP_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBls12381Fp { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBls12381Fp2 => any::<[u8; FP2_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBls12381Fp2 { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBls12381G1Affine => any::<[u8; G1_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBls12381G1Affine { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBls12381G2Affine => any::<[u8; G2_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBls12381G2Affine { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBls12381Fr => any::<[u8; 32]>()
    .prop_map(|bytes| ArbitraryBls12381Fr { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBn254G1Affine => any::<[u8; BN254_G1_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBn254G1Affine { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBn254G2Affine => any::<[u8; BN254_G2_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBn254G2Affine { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBn254Fp => any::<[u8; BN254_FP_SERIALIZED_SIZE]>()
    .prop_map(|bytes| ArbitraryBn254Fp { bytes })
    .boxed());
impl_proto_strategy!([] ArbitraryBn254Fr => any::<[u8; 32]>()
    .prop_map(|bytes| ArbitraryBn254Fr { bytes })
    .boxed());
impl_proto_strategy!([const N: usize] ArbitraryBytesN<N> => any::<[u8; N]>()
    .prop_map(|array| ArbitraryBytesN { array })
    .boxed());

//////////////////////////////////
// Container prototypes, which spend the budget.
//
// Element strategies come from `ProtoStrategy` rather than `Arbitrary` so that
// they work for `Error` too.

impl_proto_strategy!([T: ProtoStrategy] ArbitraryOption<T> => {
    // An `Option` holds at most one element.
    match Apportion::new(1) {
        None => LazyJust::new(|| ArbitraryOption(None)).boxed(),
        Some(apportion) => option::of(apportion.element(proto::<T>))
            .prop_map(ArbitraryOption)
            .boxed(),
    }
});

impl_proto_strategy!([T: ProtoStrategy] ArbitraryVec<T> => {
    match Apportion::new(1) {
        None => LazyJust::new(|| ArbitraryVec::Good(std::vec::Vec::new())).boxed(),
        Some(apportion) => {
            collection::vec(apportion.element(proto::<T>), apportion.range())
                .prop_map(ArbitraryVec::Good)
                .boxed()
        }
    }
});

impl_proto_strategy!([K: ProtoStrategy, V: ProtoStrategy] ArbitraryMap<K, V> => {
    // A map entry holds a key and a value.
    match Apportion::new(2) {
        None => LazyJust::new(|| ArbitraryMap::Good(std::vec::Vec::new())).boxed(),
        Some(apportion) => {
            let entry = apportion.element(|| (proto::<K>(), proto::<V>()));
            collection::vec(entry, apportion.range())
                .prop_map(ArbitraryMap::Good)
                .boxed()
        }
    }
});

//////////////////////////////////
// Tuple prototypes. Their fields share the budget, like a struct's.

macro_rules! impl_tuple {
    ($name:ident, $count:expr, $($ty:ident),+) => {
        impl_proto_strategy!([$($ty: ProtoStrategy),+] $name<$($ty,)+> => {
            #[allow(non_snake_case)] // bindings named after the fields T1, etc.
            with_fields($count, || {
                ($(proto::<$ty>(),)+)
                    .prop_map(|($($ty,)+)| $name { $($ty,)+ })
                    .boxed()
            })
        });
    };
}

// `proptest` implements `Strategy` for tuples of up to ten elements, so larger
// prototypes are built from nested tuples.
macro_rules! impl_tuple_nested {
    ($name:ident, $count:expr, ($($a:ident),+), ($($b:ident),+)) => {
        impl_proto_strategy!([$($a: ProtoStrategy,)+ $($b: ProtoStrategy,)+] $name<$($a,)+ $($b,)+> => {
            #[allow(non_snake_case)] // bindings named after the fields T1, etc.
            with_fields($count, || {
                (($(proto::<$a>(),)+), ($(proto::<$b>(),)+))
                    .prop_map(|(($($a,)+), ($($b,)+))| $name { $($a,)+ $($b,)+ })
                    .boxed()
            })
        });
    };
}

impl_tuple!(ArbitraryTuple1, 1, T1);
impl_tuple!(ArbitraryTuple2, 2, T1, T2);
impl_tuple!(ArbitraryTuple3, 3, T1, T2, T3);
impl_tuple!(ArbitraryTuple4, 4, T1, T2, T3, T4);
impl_tuple!(ArbitraryTuple5, 5, T1, T2, T3, T4, T5);
impl_tuple!(ArbitraryTuple6, 6, T1, T2, T3, T4, T5, T6);
impl_tuple!(ArbitraryTuple7, 7, T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(ArbitraryTuple8, 8, T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(ArbitraryTuple9, 9, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(
    ArbitraryTuple10,
    10,
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
impl_tuple_nested!(
    ArbitraryTuple11,
    11,
    (T1, T2, T3, T4, T5, T6),
    (T7, T8, T9, T10, T11)
);
impl_tuple_nested!(
    ArbitraryTuple12,
    12,
    (T1, T2, T3, T4, T5, T6),
    (T7, T8, T9, T10, T11, T12)
);

//////////////////////////////////
// `Val` prototypes.
//
// `ArbitraryVal` is mutually recursive with `ArbitraryValVec`,
// `ArbitraryValMap` and `ArbitraryValOption`, and unlike a contract type it
// cannot be reached from one, so its size is bounded on its own with
// `Strategy::prop_recursive` rather than out of the element budget. The
// containers below therefore take fixed lengths, which makes the worst case a
// function of the depth alone — `val_max_elements` — and `ArbitraryVal` charges
// that against the budget before choosing its depth.
//
// The closure `prop_recursive` calls runs while values are generated, so
// nothing reachable from it may consult the budget: the helpers here take their
// element strategies as arguments and never call `Apportion::new`.

/// The most collection elements a `Val` prototype of this depth can hold.
const fn val_max_elements(depth: u32) -> usize {
    // A full `Map` holds `LEN_MAX` entries of a key and a value.
    let entry_slots = LEN_MAX * 2;
    match depth {
        0 => 0,
        d => {
            // The widest monomorphic arm is a map of maps; the widest arm
            // overall is a map of the next level down.
            let nested = val_max_elements(d - 1);
            let of_vals = entry_slots * (1 + nested);
            let of_maps = entry_slots + entry_slots * entry_slots;
            if of_vals > of_maps {
                of_vals
            } else {
                of_maps
            }
        }
    }
}

/// The deepest `Val` prototype that fits in the given budget.
fn val_depth(budget: usize) -> u32 {
    let mut depth = VAL_DEPTH;
    while depth > 0 && val_max_elements(depth) > budget {
        depth -= 1;
    }
    depth
}

fn fixed_vec<T: Debug + 'static>(elem: BoxedStrategy<T>) -> BoxedStrategy<ArbitraryVec<T>> {
    collection::vec(elem, 0..LEN_MAX + 1)
        .prop_map(ArbitraryVec::Good)
        .boxed()
}

fn fixed_map<K: Debug + 'static, V: Debug + 'static>(
    key: BoxedStrategy<K>,
    value: BoxedStrategy<V>,
) -> BoxedStrategy<ArbitraryMap<K, V>> {
    collection::vec((key, value), 0..LEN_MAX + 1)
        .prop_map(ArbitraryMap::Good)
        .boxed()
}

fn fixed_option<T: Debug + 'static>(elem: BoxedStrategy<T>) -> BoxedStrategy<ArbitraryOption<T>> {
    option::of(elem).prop_map(ArbitraryOption).boxed()
}

/// The `Val` prototype variants that contain no other `Val` prototypes.
fn arb_val_leaf() -> BoxedStrategy<ArbitraryVal> {
    // More than ten variants, so `Union` rather than `prop_oneof!`.
    Union::new([
        Just(ArbitraryVal::Void).boxed(),
        any::<bool>().prop_map(ArbitraryVal::Bool).boxed(),
        arb_error().prop_map(ArbitraryVal::Error).boxed(),
        any::<u32>().prop_map(ArbitraryVal::U32).boxed(),
        any::<i32>().prop_map(ArbitraryVal::I32).boxed(),
        any::<u64>().prop_map(ArbitraryVal::U64).boxed(),
        any::<i64>().prop_map(ArbitraryVal::I64).boxed(),
        any::<u128>().prop_map(ArbitraryVal::U128).boxed(),
        any::<i128>().prop_map(ArbitraryVal::I128).boxed(),
        proto::<ArbitraryU256>()
            .prop_map(ArbitraryVal::U256)
            .boxed(),
        proto::<ArbitraryI256>()
            .prop_map(ArbitraryVal::I256)
            .boxed(),
        arb_bytes_proto().prop_map(ArbitraryVal::Bytes).boxed(),
        arb_string_proto().prop_map(ArbitraryVal::String).boxed(),
        arb_symbol_proto().prop_map(ArbitraryVal::Symbol).boxed(),
        proto::<ArbitraryAddress>()
            .prop_map(ArbitraryVal::Address)
            .boxed(),
        proto::<ArbitraryTimepoint>()
            .prop_map(ArbitraryVal::Timepoint)
            .boxed(),
        proto::<ArbitraryDuration>()
            .prop_map(ArbitraryVal::Duration)
            .boxed(),
    ])
    .boxed()
}

/// The element strategies of the monomorphic `Val` collection arms, which hold
/// no `Val`s themselves.
fn unit() -> BoxedStrategy<()> {
    Just(()).boxed()
}
fn bytes_n() -> BoxedStrategy<ArbitraryBytesN<32>> {
    any::<[u8; 32]>()
        .prop_map(|array| ArbitraryBytesN { array })
        .boxed()
}

fn arb_val_vec_proto(inner: BoxedStrategy<ArbitraryVal>) -> BoxedStrategy<ArbitraryValVec> {
    Union::new([
        fixed_vec(unit()).prop_map(ArbitraryValVec::Void).boxed(),
        fixed_vec(any::<bool>().boxed())
            .prop_map(ArbitraryValVec::Bool)
            .boxed(),
        fixed_vec(arb_error())
            .prop_map(ArbitraryValVec::Error)
            .boxed(),
        fixed_vec(any::<u32>().boxed())
            .prop_map(ArbitraryValVec::U32)
            .boxed(),
        fixed_vec(any::<i32>().boxed())
            .prop_map(ArbitraryValVec::I32)
            .boxed(),
        fixed_vec(any::<u64>().boxed())
            .prop_map(ArbitraryValVec::U64)
            .boxed(),
        fixed_vec(any::<i64>().boxed())
            .prop_map(ArbitraryValVec::I64)
            .boxed(),
        fixed_vec(any::<u128>().boxed())
            .prop_map(ArbitraryValVec::U128)
            .boxed(),
        fixed_vec(any::<i128>().boxed())
            .prop_map(ArbitraryValVec::I128)
            .boxed(),
        fixed_vec(proto::<ArbitraryU256>())
            .prop_map(ArbitraryValVec::U256)
            .boxed(),
        fixed_vec(proto::<ArbitraryI256>())
            .prop_map(ArbitraryValVec::I256)
            .boxed(),
        fixed_vec(arb_bytes_proto())
            .prop_map(ArbitraryValVec::Bytes)
            .boxed(),
        fixed_vec(bytes_n())
            .prop_map(ArbitraryValVec::BytesN)
            .boxed(),
        fixed_vec(arb_string_proto())
            .prop_map(ArbitraryValVec::String)
            .boxed(),
        fixed_vec(arb_symbol_proto())
            .prop_map(ArbitraryValVec::Symbol)
            .boxed(),
        fixed_vec(fixed_vec(any::<u32>().boxed()))
            .prop_map(ArbitraryValVec::Vec)
            .boxed(),
        fixed_vec(fixed_map(any::<u32>().boxed(), any::<u32>().boxed()))
            .prop_map(ArbitraryValVec::Map)
            .boxed(),
        fixed_vec(proto::<ArbitraryAddress>())
            .prop_map(ArbitraryValVec::Address)
            .boxed(),
        fixed_vec(proto::<ArbitraryTimepoint>())
            .prop_map(ArbitraryValVec::Timepoint)
            .boxed(),
        fixed_vec(proto::<ArbitraryDuration>())
            .prop_map(ArbitraryValVec::Duration)
            .boxed(),
        fixed_vec(inner).prop_map(ArbitraryValVec::Val).boxed(),
    ])
    .boxed()
}

fn arb_val_map_proto(inner: BoxedStrategy<ArbitraryVal>) -> BoxedStrategy<ArbitraryValMap> {
    Union::new([
        fixed_map(unit(), unit())
            .prop_map(ArbitraryValMap::VoidToVoid)
            .boxed(),
        fixed_map(any::<bool>().boxed(), any::<bool>().boxed())
            .prop_map(ArbitraryValMap::BoolToBool)
            .boxed(),
        fixed_map(arb_error(), arb_error())
            .prop_map(ArbitraryValMap::ErrorToError)
            .boxed(),
        fixed_map(any::<u32>().boxed(), any::<u32>().boxed())
            .prop_map(ArbitraryValMap::U32ToU32)
            .boxed(),
        fixed_map(any::<i32>().boxed(), any::<i32>().boxed())
            .prop_map(ArbitraryValMap::I32ToI32)
            .boxed(),
        fixed_map(any::<u64>().boxed(), any::<u64>().boxed())
            .prop_map(ArbitraryValMap::U64ToU64)
            .boxed(),
        fixed_map(any::<i64>().boxed(), any::<i64>().boxed())
            .prop_map(ArbitraryValMap::I64ToI64)
            .boxed(),
        fixed_map(any::<u128>().boxed(), any::<u128>().boxed())
            .prop_map(ArbitraryValMap::U128ToU128)
            .boxed(),
        fixed_map(any::<i128>().boxed(), any::<i128>().boxed())
            .prop_map(ArbitraryValMap::I128ToI128)
            .boxed(),
        fixed_map(proto::<ArbitraryU256>(), proto::<ArbitraryU256>())
            .prop_map(ArbitraryValMap::U256ToU256)
            .boxed(),
        fixed_map(proto::<ArbitraryI256>(), proto::<ArbitraryI256>())
            .prop_map(ArbitraryValMap::I256ToI256)
            .boxed(),
        fixed_map(arb_bytes_proto(), arb_bytes_proto())
            .prop_map(ArbitraryValMap::BytesToBytes)
            .boxed(),
        fixed_map(bytes_n(), bytes_n())
            .prop_map(ArbitraryValMap::BytesNToBytesN)
            .boxed(),
        fixed_map(arb_string_proto(), arb_string_proto())
            .prop_map(ArbitraryValMap::StringToString)
            .boxed(),
        fixed_map(arb_symbol_proto(), arb_symbol_proto())
            .prop_map(ArbitraryValMap::SymbolToSymbol)
            .boxed(),
        fixed_map(
            fixed_vec(any::<u32>().boxed()),
            fixed_vec(any::<u32>().boxed()),
        )
        .prop_map(ArbitraryValMap::VecToVec)
        .boxed(),
        fixed_map(
            fixed_map(any::<u32>().boxed(), any::<u32>().boxed()),
            fixed_map(any::<u32>().boxed(), any::<u32>().boxed()),
        )
        .prop_map(ArbitraryValMap::MapToMap)
        .boxed(),
        fixed_map(proto::<ArbitraryAddress>(), proto::<ArbitraryAddress>())
            .prop_map(ArbitraryValMap::AddressToAddress)
            .boxed(),
        fixed_map(proto::<ArbitraryTimepoint>(), proto::<ArbitraryTimepoint>())
            .prop_map(ArbitraryValMap::TimepointToTimepoint)
            .boxed(),
        fixed_map(proto::<ArbitraryDuration>(), proto::<ArbitraryDuration>())
            .prop_map(ArbitraryValMap::DurationToDuration)
            .boxed(),
        fixed_map(inner.clone(), inner)
            .prop_map(ArbitraryValMap::ValToVal)
            .boxed(),
        fixed_map(
            fixed_option(any::<u32>().boxed()),
            fixed_option(any::<u32>().boxed()),
        )
        .prop_map(ArbitraryValMap::OptionToOption)
        .boxed(),
    ])
    .boxed()
}

fn arb_val_option_proto(inner: BoxedStrategy<ArbitraryVal>) -> BoxedStrategy<ArbitraryValOption> {
    Union::new([
        fixed_option(unit())
            .prop_map(ArbitraryValOption::Void)
            .boxed(),
        fixed_option(any::<bool>().boxed())
            .prop_map(ArbitraryValOption::Bool)
            .boxed(),
        fixed_option(arb_error())
            .prop_map(ArbitraryValOption::Error)
            .boxed(),
        fixed_option(any::<u32>().boxed())
            .prop_map(ArbitraryValOption::U32)
            .boxed(),
        fixed_option(any::<i32>().boxed())
            .prop_map(ArbitraryValOption::I32)
            .boxed(),
        fixed_option(any::<u64>().boxed())
            .prop_map(ArbitraryValOption::U64)
            .boxed(),
        fixed_option(any::<i64>().boxed())
            .prop_map(ArbitraryValOption::I64)
            .boxed(),
        fixed_option(any::<u128>().boxed())
            .prop_map(ArbitraryValOption::U128)
            .boxed(),
        fixed_option(any::<i128>().boxed())
            .prop_map(ArbitraryValOption::I128)
            .boxed(),
        fixed_option(proto::<ArbitraryU256>())
            .prop_map(ArbitraryValOption::U256)
            .boxed(),
        fixed_option(proto::<ArbitraryI256>())
            .prop_map(ArbitraryValOption::I256)
            .boxed(),
        fixed_option(arb_bytes_proto())
            .prop_map(ArbitraryValOption::Bytes)
            .boxed(),
        fixed_option(bytes_n())
            .prop_map(ArbitraryValOption::BytesN)
            .boxed(),
        fixed_option(arb_string_proto())
            .prop_map(ArbitraryValOption::String)
            .boxed(),
        fixed_option(arb_symbol_proto())
            .prop_map(ArbitraryValOption::Symbol)
            .boxed(),
        fixed_option(fixed_vec(any::<u32>().boxed()))
            .prop_map(ArbitraryValOption::Vec)
            .boxed(),
        fixed_option(fixed_map(any::<u32>().boxed(), any::<u32>().boxed()))
            .prop_map(ArbitraryValOption::Map)
            .boxed(),
        fixed_option(proto::<ArbitraryAddress>())
            .prop_map(ArbitraryValOption::Address)
            .boxed(),
        fixed_option(proto::<ArbitraryTimepoint>())
            .prop_map(ArbitraryValOption::Timepoint)
            .boxed(),
        fixed_option(proto::<ArbitraryDuration>())
            .prop_map(ArbitraryValOption::Duration)
            .boxed(),
        fixed_option(inner)
            .prop_map(|option| ArbitraryValOption::Val(std::boxed::Box::new(option)))
            .boxed(),
    ])
    .boxed()
}

impl_proto_strategy!([] ArbitraryVal => {
    let depth = val_depth(BUDGET.get());
    if depth == 0 {
        return arb_val_leaf();
    }
    // Small, so that scalar `Val`s are the common case and nesting is
    // occasional: `prop_recursive` takes a container branch with probability
    // `desired_size / (2 * branch_size) ^ (level + 1)`, clamped to 0.9.
    const DESIRED_SIZE: u32 = 1;
    const BRANCH_SIZE: u32 = 2;
    arb_val_leaf()
        .prop_recursive(depth, DESIRED_SIZE, BRANCH_SIZE, |inner| {
            // Values are being generated, so no construction is in progress and
            // nothing here may consult the budget.
            debug_assert_eq!(
                DEPTH.get(),
                0,
                "a `Val` strategy is built while values are generated"
            );
            Union::new([
                arb_val_vec_proto(inner.clone())
                    .prop_map(ArbitraryVal::Vec)
                    .boxed(),
                arb_val_map_proto(inner.clone())
                    .prop_map(ArbitraryVal::Map)
                    .boxed(),
                arb_val_option_proto(inner)
                    .prop_map(ArbitraryVal::Option)
                    .boxed(),
            ])
        })
        .boxed()
});
