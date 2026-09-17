//! Tests that converting a [Val] into an SDK type errors, and does not panic,
//! when the [Val] holds a value of an incompatible type.
//!
//! Every type is tested against a [Val] of every type the host supports, and
//! the types named as compatible are the only ones that may convert. Anything
//! else must be a conversion error, because contracts receive [Val]s from
//! callers that are free to send a value of any type.

use crate::{self as soroban_sdk};
use soroban_sdk::{
    map, symbol_short,
    testutils::{Address as _, MuxedAddress as _},
    vec, Address, Bytes, BytesN, Duration, Env, Error, IntoVal, Map, MuxedAddress, String, Symbol,
    Timepoint, TryFromVal, Val, Vec, I256, U256,
};

#[test]
fn test_void() {
    let env = Env::default();
    assert_compatible_with::<()>(&env, &["void"]);
}

#[test]
fn test_bool() {
    let env = Env::default();
    assert_compatible_with::<bool>(&env, &["bool"]);
}

#[test]
fn test_u32() {
    let env = Env::default();
    assert_compatible_with::<u32>(&env, &["u32"]);
}

#[test]
fn test_i32() {
    let env = Env::default();
    assert_compatible_with::<i32>(&env, &["i32"]);
}

#[test]
fn test_u64() {
    let env = Env::default();
    assert_compatible_with::<u64>(&env, &["u64"]);
}

#[test]
fn test_i64() {
    let env = Env::default();
    assert_compatible_with::<i64>(&env, &["i64"]);
}

#[test]
fn test_u128() {
    let env = Env::default();
    assert_compatible_with::<u128>(&env, &["u128"]);
}

#[test]
fn test_i128() {
    let env = Env::default();
    assert_compatible_with::<i128>(&env, &["i128"]);
}

#[test]
fn test_u256() {
    let env = Env::default();
    assert_compatible_with::<U256>(&env, &["u256"]);
}

#[test]
fn test_i256() {
    let env = Env::default();
    assert_compatible_with::<I256>(&env, &["i256"]);
}

#[test]
fn test_timepoint() {
    let env = Env::default();
    assert_compatible_with::<Timepoint>(&env, &["timepoint"]);
}

#[test]
fn test_duration() {
    let env = Env::default();
    assert_compatible_with::<Duration>(&env, &["duration"]);
}

#[test]
fn test_symbol() {
    let env = Env::default();
    assert_compatible_with::<Symbol>(&env, &["symbol"]);
}

#[test]
fn test_string() {
    let env = Env::default();
    assert_compatible_with::<String>(&env, &["string"]);
}

#[test]
fn test_bytes() {
    let env = Env::default();
    assert_compatible_with::<Bytes>(&env, &["bytes"]);
}

#[test]
fn test_bytes_n() {
    let env = Env::default();
    // The bytes val is 32 bytes long, so it converts into a BytesN of that
    // length, and not into one of any other length.
    assert_compatible_with::<BytesN<32>>(&env, &["bytes"]);
    assert_compatible_with::<BytesN<1>>(&env, &[]);
}

#[test]
fn test_vec() {
    let env = Env::default();
    assert_compatible_with::<Vec<i32>>(&env, &["vec"]);
}

#[test]
fn test_map() {
    let env = Env::default();
    assert_compatible_with::<Map<i32, i32>>(&env, &["map"]);
}

#[test]
fn test_address() {
    let env = Env::default();
    assert_compatible_with::<Address>(&env, &["address"]);
}

#[test]
fn test_muxed_address() {
    let env = Env::default();
    assert_compatible_with::<MuxedAddress>(&env, &["address", "muxed_address"]);
}

#[test]
fn test_error() {
    let env = Env::default();
    assert_compatible_with::<Error>(&env, &["error"]);
}

#[test]
fn test_option() {
    let env = Env::default();
    // A void val is the absent option, and so both void and the option's own
    // type convert.
    assert_compatible_with::<Option<u32>>(&env, &["void", "u32"]);
}

#[test]
fn test_val() {
    let env = Env::default();
    // Val holds a value of any type, and so every val converts.
    for (name, val) in vals(&env) {
        assert!(
            Val::try_from_val(&env, &val).is_ok(),
            "converting a {} val into Val errored but should succeed",
            name,
        );
    }
}

/// A [Val] of every type the host supports, each labelled with a name used by
/// the tests to say which types are compatible with the type being converted
/// into.
pub(crate) fn vals(env: &Env) -> [(&'static str, Val); 20] {
    [
        ("void", ().into_val(env)),
        ("bool", true.into_val(env)),
        ("u32", 1u32.into_val(env)),
        ("i32", 1i32.into_val(env)),
        ("u64", 1u64.into_val(env)),
        ("i64", 1i64.into_val(env)),
        ("timepoint", Timepoint::from_unix(env, 1).into_val(env)),
        ("duration", Duration::from_seconds(env, 1).into_val(env)),
        ("u128", 1u128.into_val(env)),
        ("i128", 1i128.into_val(env)),
        ("u256", U256::from_u32(env, 1).into_val(env)),
        ("i256", I256::from_i32(env, 1).into_val(env)),
        ("symbol", symbol_short!("a").into_val(env)),
        ("bytes", Bytes::from_array(env, &[0u8; 32]).into_val(env)),
        ("string", String::from_str(env, "a").into_val(env)),
        ("vec", vec![env, 1i32].into_val(env)),
        ("map", map![env, (1i32, 2i32)].into_val(env)),
        ("address", Address::generate(env).into_val(env)),
        (
            "muxed_address",
            MuxedAddress::new(MuxedAddress::generate(env), 1).into_val(env),
        ),
        ("error", Error::from_contract_error(1).into_val(env)),
    ]
}

/// Asserts that `T` converts from the [Val]s named in `compatible`, and that
/// converting from a [Val] of any other type is an error rather than a panic.
pub(crate) fn assert_compatible_with<T>(env: &Env, compatible: &[&str])
where
    T: TryFromVal<Env, Val>,
{
    assert_compatible_with_skipping::<T>(env, compatible, &[]);
}

/// Same as [assert_compatible_with], but not converting from the [Val]s named
/// in `skip`, for types where converting from those [Val]s traps in the host
/// instead of erroring, which the caller tests separately.
pub(crate) fn assert_compatible_with_skipping<T>(env: &Env, compatible: &[&str], skip: &[&str])
where
    T: TryFromVal<Env, Val>,
{
    for (name, val) in vals(env) {
        if skip.contains(&name) {
            continue;
        }
        let converted = T::try_from_val(env, &val).is_ok();
        assert_eq!(
            converted,
            compatible.contains(&name),
            "converting a {} val into {} {} but should {}",
            name,
            core::any::type_name::<T>(),
            if converted { "succeeded" } else { "errored" },
            if converted { "error" } else { "succeed" },
        );
    }
}
