//! Property tests of the direct `proptest` support in
//! [`crate::testutils::proptest`].

use crate::testutils::arbitrary::SorobanArbitrary;
use crate::testutils::proptest::{arb, arb_error, NODE_BUDGET};
use crate::{
    contracttype, Address, Bytes, BytesN, Duration, Env, Error, IntoVal, Map, MuxedAddress, String,
    Symbol, Timepoint, TryFromVal, Val, Vec, I256, U256,
};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{Config, TestError, TestRunner};

/// Asserts a prototype converts into its contract type.
macro_rules! check {
    ($env:expr, $ty:ty, $proto:expr) => {{
        let value: $ty = $proto.into_val($env);
        // Round-trip through `Val` to check the conversion produced something
        // the host accepts.
        let val: Val = value.into_val($env);
        prop_assert!(<$ty>::try_from_val($env, &val).is_ok());
    }};
}

proptest! {
    #[test]
    fn test_scalars(
        u in arb::<u32>(),
        i in arb::<i32>(),
        u_64 in arb::<u64>(),
        i_64 in arb::<i64>(),
        u_128 in arb::<u128>(),
        i_128 in arb::<i128>(),
        b in arb::<bool>(),
    ) {
        let env = &Env::default();
        check!(env, u32, u);
        check!(env, i32, i);
        check!(env, u64, u_64);
        check!(env, i64, i_64);
        check!(env, u128, u_128);
        check!(env, i128, i_128);
        check!(env, bool, b);
    }

    #[test]
    fn test_objects(
        bytes in arb::<Bytes>(),
        bytes_n in arb::<BytesN<32>>(),
        string in arb::<String>(),
        symbol in arb::<Symbol>(),
        address in arb::<Address>(),
        muxed in arb::<MuxedAddress>(),
        timepoint in arb::<Timepoint>(),
        duration in arb::<Duration>(),
        u256 in arb::<U256>(),
        i256 in arb::<I256>(),
    ) {
        let env = &Env::default();
        check!(env, Bytes, bytes);
        check!(env, BytesN<32>, bytes_n);
        check!(env, String, string);
        check!(env, Symbol, symbol);
        check!(env, Address, address);
        check!(env, MuxedAddress, muxed);
        check!(env, Timepoint, timepoint);
        check!(env, Duration, duration);
        check!(env, U256, u256);
        check!(env, I256, i256);
    }

    #[test]
    fn test_collections(
        addresses in arb::<Vec<Address>>(),
        map in arb::<Map<Symbol, i128>>(),
        option in arb::<Option<u32>>(),
        nested in arb::<Vec<Map<Symbol, Vec<u32>>>>(),
        tuple in arb::<(u32, Address, Vec<i128>)>(),
    ) {
        let env = &Env::default();
        check!(env, Vec<Address>, addresses);
        check!(env, Map<Symbol, i128>, map);
        check!(env, Option<u32>, option);
        check!(env, Vec<Map<Symbol, Vec<u32>>>, nested);
        check!(env, (u32, Address, Vec<i128>), tuple);
    }

    #[test]
    fn test_error(error in arb_error(), errors in arb::<Vec<Error>>(), also in arb::<Error>()) {
        let env = &Env::default();
        // `Error` is its own prototype, so it needs no conversion, but the
        // prototypes that contain one do.
        prop_assert!(error.get_code() == error.get_code());
        prop_assert!(also.get_code() == also.get_code());
        check!(env, Vec<Error>, errors);
    }

    #[test]
    fn test_val(val in arb::<Val>(), vals in arb::<Vec<Val>>(), map in arb::<Map<Val, Val>>()) {
        let env = &Env::default();
        check!(env, Val, val);
        check!(env, Vec<Val>, vals);
        check!(env, Map<Val, Val>, map);
    }
}

//////////////////////////////////
// Contract types.

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdtStruct {
    pub a: i128,
    pub b: Vec<Address>,
    pub c: Symbol,
}

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdtTuple(pub u32, pub Bytes);

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UdtEnum {
    A,
    B(u32),
    C(Symbol, Bytes),
}

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UdtEnumInt {
    First = 1,
    Second = 2,
}

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdtWithError {
    pub error: Error,
    pub value: u32,
}

/// A struct with more than ten fields, which the macro builds from nested
/// tuples because `proptest` implements `Strategy` for tuples only up to ten.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdtWideStruct {
    pub f0: u32,
    pub f1: u32,
    pub f2: u32,
    pub f3: u32,
    pub f4: u32,
    pub f5: u32,
    pub f6: u32,
    pub f7: u32,
    pub f8: u32,
    pub f9: u32,
    pub f10: u32,
    pub f11: Symbol,
}

proptest! {
    #[test]
    fn test_udts(
        s in arb::<UdtStruct>(),
        t in arb::<UdtTuple>(),
        e in arb::<UdtEnum>(),
        i in arb::<UdtEnumInt>(),
        w in arb::<UdtWideStruct>(),
        // The macro's `Arbitrary` impl is emitted for every prototype, including
        // ones with an `Error` field, so `any` works on them even though
        // `any::<Error>()` cannot exist.
        err in any::<<UdtWithError as SorobanArbitrary>::Prototype>(),
    ) {
        let env = &Env::default();
        check!(env, UdtStruct, s);
        check!(env, UdtTuple, t);
        check!(env, UdtEnum, e);
        check!(env, UdtEnumInt, i);
        check!(env, UdtWideStruct, w);
        check!(env, UdtWithError, err);
    }
}

//////////////////////////////////
// Recursive contract types.
//
// Each of these defeated an earlier version of the bound in
// `crate::testutils::proptest`: naive recursion overflowed the stack building a
// strategy for any of them, a per-container depth cap left the deeper ones
// generating only empty collections, and a cap that counted depth alone let the
// wide ones generate values so large that converting them exhausted the host
// budget.

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdtRecursive {
    pub a: Symbol,
    pub b: Vec<UdtRecursive>,
}

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UdtRecursiveOption {
    pub a: Vec<Option<UdtRecursiveOption>>,
}

/// Mutual recursion that alternates container types, and puts the recursive type
/// in a `Map` key.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutualA {
    pub a: Vec<MutualB>,
}

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutualB {
    pub b: Map<MutualA, u32>,
}

/// Four recursive fields of distinct container types. Depth alone does not bound
/// this: the fan-out is the field count.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WideRecursive {
    pub a: Vec<WideRecursive>,
    pub b: Map<u32, WideRecursive>,
    pub c: Map<Symbol, WideRecursive>,
    pub d: Vec<Option<WideRecursive>>,
}

/// Recursion through a tuple, which multiplies the fan-out by the tuple arity.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TupleRecursive {
    pub a: Vec<(TupleRecursive, TupleRecursive, TupleRecursive)>,
}

/// Nine recursive fields, of nearly every container shape. The widest recursive
/// struct that has come up.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WiderRecursive {
    pub a: Vec<WiderRecursive>,
    pub b: Map<u32, WiderRecursive>,
    pub c: Map<i32, WiderRecursive>,
    pub d: Map<u64, WiderRecursive>,
    pub e: Map<i64, WiderRecursive>,
    pub f: Map<Symbol, WiderRecursive>,
    pub g: Vec<Option<WiderRecursive>>,
    pub h: Vec<(WiderRecursive, WiderRecursive)>,
    pub i: Map<u128, WiderRecursive>,
}

/// Recursion through a four-element tuple in four container fields, which
/// multiplies the fan-out by the tuple arity. Converting a value of this shape
/// exhausted the host budget under an earlier bound.
pub type Quad = (TupleWide, TupleWide, TupleWide, TupleWide);

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TupleWide {
    pub a: Vec<Quad>,
    pub b: Map<u32, Quad>,
    pub c: Map<i32, Quad>,
    pub d: Vec<Option<Quad>>,
}

/// A recursive enum with many recursive variants. All of the variants are built
/// even though only one is generated, so this is the shape that bounds
/// construction work rather than value size.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WideRecursiveEnum {
    Leaf(u32),
    V1(Vec<WideRecursiveEnum>),
    V2(Vec<WideRecursiveEnum>),
    V3(Vec<WideRecursiveEnum>),
    V4(Vec<WideRecursiveEnum>),
    V5(Vec<WideRecursiveEnum>),
    V6(Vec<WideRecursiveEnum>),
    V7(Vec<WideRecursiveEnum>),
    V8(Vec<WideRecursiveEnum>),
    V9(Map<u32, WideRecursiveEnum>),
    V10(Map<u32, WideRecursiveEnum>),
    V11(Map<u32, WideRecursiveEnum>),
    V12(Map<u32, WideRecursiveEnum>),
    V13(Map<u32, WideRecursiveEnum>),
    V14(Map<u32, WideRecursiveEnum>),
    V15(Vec<Option<WideRecursiveEnum>>),
    V16(Vec<Option<WideRecursiveEnum>>),
}

/// Recursive shapes generate values large enough to be worth converting, so
/// fewer cases than the default keep the suite quick. Honours `PROPTEST_CASES`
/// when it is set, so a stress run can ask for more.
fn recursive_config() -> Config {
    let default = Config::default();
    // `Config::default` reads PROPTEST_CASES, so only override it when the
    // variable is unset.
    let cases = match std::env::var("PROPTEST_CASES") {
        Ok(_) => default.cases,
        Err(_) => 8,
    };
    Config {
        cases,
        failure_persistence: None,
        ..default
    }
}

proptest! {
    #![proptest_config(recursive_config())]

    /// Converting these is what exhausted the host budget when the bound
    /// counted depth alone.
    #[test]
    fn test_recursive_udts(
        r in arb::<UdtRecursive>(),
        rs in arb::<Vec<UdtRecursive>>(),
        o in arb::<UdtRecursiveOption>(),
        m in arb::<MutualA>(),
        w in arb::<WideRecursive>(),
        t in arb::<TupleRecursive>(),
        e in arb::<WideRecursiveEnum>(),
        wider in arb::<WiderRecursive>(),
        tw in arb::<TupleWide>(),
    ) {
        let env = &Env::default();
        check!(env, UdtRecursive, r);
        check!(env, Vec<UdtRecursive>, rs);
        check!(env, UdtRecursiveOption, o);
        check!(env, MutualA, m);
        check!(env, WideRecursive, w);
        check!(env, TupleRecursive, t);
        check!(env, WideRecursiveEnum, e);
        check!(env, WiderRecursive, wider);
        check!(env, TupleWide, tw);
    }
}

/// Building a strategy for a recursive enum with many recursive variants used to
/// take tens of seconds and gigabytes of memory, because every variant is built
/// at every level.
#[test]
fn test_a_wide_recursive_enum_builds_quickly() {
    let start = std::time::Instant::now();
    let _strategy = arb::<WideRecursiveEnum>();
    let elapsed = start.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(1),
        "building the strategy took {elapsed:?}"
    );
}

//////////////////////////////////
// Coverage of positions the bound could silently empty out.

/// Runs `property` over `cases` generated values.
fn sample<S: Strategy>(strategy: S, cases: u32, mut property: impl FnMut(S::Value)) {
    let mut runner = TestRunner::deterministic();
    for _ in 0..cases {
        let tree = strategy.new_tree(&mut runner).expect("new tree");
        property(tree.current());
    }
}

/// Nesting that is not recursive must be generated in full: an earlier version
/// of the bound truncated the fourth container level to empty, which makes a
/// property about it vacuous rather than weak.
#[test]
fn test_deep_nesting_is_not_vacuous() {
    let env = &Env::default();
    let mut somes = 0usize;
    sample(arb::<Vec<Vec<Vec<Option<u32>>>>>(), 64, |proto| {
        let value: Vec<Vec<Vec<Option<u32>>>> = proto.into_val(env);
        for outer in value.iter() {
            for middle in outer.iter() {
                somes += middle.iter().flatten().count();
            }
        }
    });
    assert!(somes > 0, "the innermost level generated nothing");
}

/// A wide shape that is *not* recursive must not be truncated either: budgets
/// that divided by the field count collapsed every collection in a type whose
/// fields multiplied out past the budget.
#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WideLeaf {
    pub a: Vec<u32>,
    pub b: Vec<u32>,
    pub c: Vec<u32>,
    pub d: Vec<u32>,
    pub e: Vec<u32>,
    pub f: Vec<u32>,
    pub g: Vec<u32>,
    pub h: Vec<u32>,
    pub i: Vec<u32>,
    pub j: Vec<u32>,
    pub k: Vec<u32>,
}

#[contracttype(crate_path = "crate")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WideMiddle {
    pub a: Vec<WideLeaf>,
    pub b: Vec<WideLeaf>,
    pub c: Vec<WideLeaf>,
    pub d: Vec<WideLeaf>,
    pub e: Vec<WideLeaf>,
    pub f: Vec<WideLeaf>,
    pub g: Vec<WideLeaf>,
    pub h: Vec<WideLeaf>,
    pub i: Vec<WideLeaf>,
    pub j: Vec<WideLeaf>,
    pub k: Vec<WideLeaf>,
}

#[test]
fn test_wide_non_recursive_nesting_is_not_vacuous() {
    let env = &Env::default();
    let mut elements = 0usize;
    sample(arb::<Vec<WideMiddle>>(), 16, |proto| {
        let value: Vec<WideMiddle> = proto.into_val(env);
        for middle in value.iter() {
            for leaf in middle.a.iter() {
                elements += leaf.a.len() as usize;
            }
        }
    });
    assert!(elements > 0, "a wide non-recursive shape generated nothing");
}

/// `Val` prototypes must reach their container variants, not only scalars: the
/// budget charges a `Val` its worst case, and getting that wrong would leave
/// every `Val` a scalar.
#[test]
fn test_a_container_val_is_generated() {
    let mut containers = 0usize;
    let cases = 1024;
    sample(arb::<Val>(), cases, |proto| {
        let debug = std::format!("{proto:?}");
        if debug.starts_with("Vec") || debug.starts_with("Map") || debug.starts_with("Option") {
            containers += 1;
        }
    });
    assert!(
        containers > cases as usize / 100,
        "only {containers} of {cases} `Val`s were containers"
    );
}

//////////////////////////////////
// Shrinking, which is the point of generating prototypes with `proptest`
// strategies rather than from bytes.

/// Runs a deliberately failing property and returns the shrunk counterexample.
fn shrink_failure<S: Strategy>(strategy: S, property: impl Fn(&S::Value) -> bool) -> S::Value {
    let config = Config {
        cases: 1024,
        failure_persistence: None,
        ..Config::default()
    };
    let mut runner = TestRunner::new(config);
    let result = runner.run(&strategy, |value| {
        prop_assert!(property(&value));
        Ok(())
    });
    match result {
        Err(TestError::Fail(_, value)) => value,
        other => panic!("expected a failing case, got {other:?}"),
    }
}

#[test]
fn test_shrinking_bytes() {
    // Fails for any `Bytes` of three or more bytes, so the minimal
    // counterexample is three zero bytes. `collection::vec` shrinks its length
    // by binary search, which can settle one element above the minimum.
    let proto = shrink_failure(arb::<Bytes>(), |proto| {
        std::format!("{proto:?}").len() < 200 && proto_len(proto) < 3
    });
    let len = proto_len(&proto);
    assert!(len <= 4, "shrank to {len} bytes");
}

fn proto_len(proto: &<Bytes as SorobanArbitrary>::Prototype) -> usize {
    // The prototype's fields are private outside the SDK, but `Debug` counts.
    std::format!("{proto:?}").matches(',').count() + 1
}

#[test]
fn test_shrinking_vec() {
    // Fails for a `Vec<u32>` of three or more elements.
    let env = &Env::default();
    let proto = shrink_failure(arb::<Vec<u32>>(), |proto| {
        let value: Vec<u32> = proto.into_val(env);
        value.len() < 3
    });
    let value: Vec<u32> = proto.into_val(env);
    assert!(value.len() <= 4, "shrank to {} elements", value.len());
    assert!(
        value.iter().all(|element| element == 0),
        "elements did not shrink to zero: {value:?}"
    );
}

#[test]
fn test_shrinking_map() {
    let env = &Env::default();
    let proto = shrink_failure(arb::<Map<u32, u32>>(), |proto| {
        let value: Map<u32, u32> = proto.into_val(env);
        value.len() < 2
    });
    let value: Map<u32, u32> = proto.into_val(env);
    assert!(value.len() <= 3, "shrank to {} entries", value.len());
}

#[test]
fn test_shrinking_udt_struct() {
    let env = &Env::default();
    let proto = shrink_failure(arb::<UdtStruct>(), |proto| {
        let value: UdtStruct = proto.into_val(env);
        value.a < 1
    });
    let value: UdtStruct = proto.into_val(env);
    assert!(value.a <= 2, "shrank to {}", value.a);
    assert_eq!(value.b.len(), 0, "the other fields did not shrink");
}

#[test]
fn test_shrinking_udt_enum() {
    let env = &Env::default();
    // Fails only for the `B` variant, which must survive shrinking with a
    // minimal payload.
    let proto = shrink_failure(arb::<UdtEnum>(), |proto| {
        let value: UdtEnum = proto.into_val(env);
        !matches!(value, UdtEnum::B(n) if n >= 1)
    });
    let value: UdtEnum = proto.into_val(env);
    match value {
        UdtEnum::B(n) => assert!(n <= 2, "payload shrank to {n}"),
        other => panic!("the failing variant was lost: {other:?}"),
    }
}

#[test]
fn test_shrinking_wide_struct() {
    let env = &Env::default();
    // The failing field is in the second chunk of the nested tuple the macro
    // builds for a type with more than ten fields.
    let proto = shrink_failure(arb::<UdtWideStruct>(), |proto| {
        let value: UdtWideStruct = proto.into_val(env);
        value.f10 < 1
    });
    let value: UdtWideStruct = proto.into_val(env);
    assert!(value.f10 <= 2, "shrank to {}", value.f10);
    assert_eq!(value.f0, 0, "the other fields did not shrink");
}

#[test]
fn test_shrinking_val() {
    let env = &Env::default();
    // Fails for any `Val` holding a u32 of three or more.
    let proto = shrink_failure(arb::<Val>(), |proto| {
        let val: Val = proto.into_val(env);
        match u32::try_from_val(env, &val) {
            Ok(value) => value < 3,
            Err(_) => true,
        }
    });
    let val: Val = proto.into_val(env);
    let value = u32::try_from_val(env, &val).expect("a u32 val");
    assert!(value <= 4, "shrank to {value}");
}

#[test]
fn test_shrinking_recursive() {
    let env = &Env::default();
    // Fails for a `UdtRecursive` with any children, so the minimal
    // counterexample has one child, itself minimal.
    let proto = shrink_failure(arb::<UdtRecursive>(), |proto| {
        let value: UdtRecursive = proto.into_val(env);
        value.b.is_empty()
    });
    let value: UdtRecursive = proto.into_val(env);
    assert!(value.b.len() <= 2, "shrank to {} children", value.b.len());
    for child in value.b.iter() {
        assert_eq!(child.b.len(), 0, "the child did not shrink");
    }
}

//////////////////////////////////
// Size of generated values.

/// Every prototype the SDK provides must implement `ProtoStrategy`, or `arb`
/// does not work for it. Extend this when a contract type gains
/// `SorobanArbitrary`.
#[test]
fn test_every_prototype_implements_proto_strategy() {
    fn assert_proto<T: SorobanArbitrary>()
    where
        T::Prototype: crate::testutils::proptest::ProtoStrategy,
    {
    }

    assert_proto::<()>();
    assert_proto::<bool>();
    assert_proto::<u32>();
    assert_proto::<i32>();
    assert_proto::<u64>();
    assert_proto::<i64>();
    assert_proto::<u128>();
    assert_proto::<i128>();
    assert_proto::<Error>();
    assert_proto::<U256>();
    assert_proto::<I256>();
    assert_proto::<Bytes>();
    assert_proto::<BytesN<32>>();
    assert_proto::<String>();
    assert_proto::<Symbol>();
    assert_proto::<Address>();
    assert_proto::<MuxedAddress>();
    assert_proto::<Timepoint>();
    assert_proto::<Duration>();
    assert_proto::<Val>();
    assert_proto::<Option<u32>>();
    assert_proto::<Vec<u32>>();
    assert_proto::<Map<u32, u32>>();
    assert_proto::<(u32,)>();
    assert_proto::<(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)>();
    assert_proto::<UdtStruct>();
    assert_proto::<UdtEnum>();
    assert_proto::<UdtEnumInt>();
    assert_proto::<UdtWithError>();
}

/// A rough size for a generated prototype: the separators in its `Debug`, which
/// is the only view of a prototype's insides available from outside the module.
///
/// This over-counts the collection elements [`NODE_BUDGET`] bounds, because the
/// fields of every struct and enum variant on the way down are separated too, so
/// it is a bound on the size of the whole prototype rather than on its
/// collection elements alone. Both directions matter here: too large means the
/// budget stopped bounding recursion, and too small means it emptied the
/// collections out instead of shrinking them.
fn debug_size<T: core::fmt::Debug>(proto: &T) -> usize {
    std::format!("{proto:?}").matches(',').count()
}

#[test]
fn test_recursive_values_stay_small() {
    // Each node of the widest of these types contributes its own field
    // separators, so allow a generous multiple of the element budget.
    const LIMIT: usize = NODE_BUDGET * 4;
    for (name, size) in [
        ("WideRecursive", sample_max(arb::<WideRecursive>())),
        ("TupleRecursive", sample_max(arb::<TupleRecursive>())),
        ("WideRecursiveEnum", sample_max(arb::<WideRecursiveEnum>())),
        ("WiderRecursive", sample_max(arb::<WiderRecursive>())),
        ("TupleWide", sample_max(arb::<TupleWide>())),
    ] {
        assert!(size <= LIMIT, "{name} generated a value of size {size}");
        assert!(size > 0, "{name} generated nothing at all");
    }
}

fn sample_max<S: Strategy>(strategy: S) -> usize
where
    S::Value: core::fmt::Debug,
{
    let mut largest = 0usize;
    sample(strategy, 16, |value| {
        largest = largest.max(debug_size(&value));
    });
    largest
}
