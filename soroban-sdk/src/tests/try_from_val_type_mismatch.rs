//! Tests what happens when converting a [Val] into an SDK type or a contract
//! type when the [Val] holds a value of an incompatible type.
//!
//! Every type is tested against a [Val] of every type the host supports, and
//! the types named as compatible are the only ones that may convert. Anything
//! else must be a conversion error, because contracts receive [Val]s from
//! callers that are free to send a value of any type.
//!
//! Converting is an error in every case except two: a contract type struct
//! converting from a map that has keys that are not symbols, and a contract
//! type tuple struct converting from a vec of a different length, both of which
//! trap in the host and so panic.

use crate::{self as soroban_sdk};
use soroban_sdk::{
    contracterror, contracttype, map, symbol_short,
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
    // Bytes converts from a bytes val of any length.
    assert_compatible_with::<Bytes>(&env, &["bytes32", "bytes64"]);
}

#[test]
fn test_bytes_n() {
    let env = Env::default();
    // A BytesN converts only from a bytes val of its own length.
    assert_compatible_with::<BytesN<32>>(&env, &["bytes32"]);
    assert_compatible_with::<BytesN<64>>(&env, &["bytes64"]);
}

#[test]
fn test_vec() {
    let env = Env::default();
    assert_compatible_with::<Vec<i32>>(&env, &["vec_i32"]);
}

#[test]
fn test_map() {
    let env = Env::default();
    assert_compatible_with::<Map<i32, i32>>(&env, &["map_i32_i32"]);
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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStruct {
    pub a: i32,
    pub b: i32,
}

#[test]
fn test_udt_struct() {
    let env = Env::default();

    // A struct converts from a map keyed by the field names.
    let map = map![&env, (symbol_short!("a"), 1i32), (symbol_short!("b"), 2i32)].to_val();
    assert_eq!(
        UdtStruct::try_from_val(&env, &map),
        Ok(UdtStruct { a: 1, b: 2 })
    );

    // No val of another type converts. The map val is skipped because it is
    // keyed by integers, which traps, and is tested in
    // test_udt_struct_from_map_with_non_symbol_keys_panics.
    assert_compatible_with_skipping::<UdtStruct>(&env, &[], &["map_i32_i32"]);
}

#[test]
#[should_panic(expected = "UnexpectedType")]
fn test_udt_struct_from_map_with_non_symbol_keys_panics() {
    let env = Env::default();

    // The host traps when unpacking a map that has keys that are not symbols,
    // and so the conversion panics rather than returning an error.
    let map = map![&env, (1i32, 2i32)].to_val();
    let _ = UdtStruct::try_from_val(&env, &map);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStructTuple(pub i32, pub i32);

#[test]
fn test_udt_struct_tuple() {
    let env = Env::default();

    // A tuple struct converts from a vec with an element per field.
    let vec = vec![&env, 1i32, 2i32].to_val();
    assert_eq!(
        UdtStructTuple::try_from_val(&env, &vec),
        Ok(UdtStructTuple(1, 2))
    );

    // No val of another type converts. The vec val is skipped because it has
    // one element and not two, which traps, and is tested in
    // test_udt_struct_tuple_from_vec_of_other_len_panics.
    assert_compatible_with_skipping::<UdtStructTuple>(&env, &[], &["vec_i32"]);
}

#[test]
#[should_panic(expected = "UnexpectedSize")]
fn test_udt_struct_tuple_from_vec_of_other_len_panics() {
    let env = Env::default();

    // The host traps when unpacking a vec into a slice of a different length,
    // and so the conversion panics rather than returning an error.
    let vec = vec![&env, 1i32].to_val();
    let _ = UdtStructTuple::try_from_val(&env, &vec);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnum {
    Unit,
    Tuple(i32),
}

#[test]
fn test_udt_enum() {
    let env = Env::default();

    // An enum converts from a vec with the variant name as its first element.
    let unit: Val = vec![&env, symbol_short!("Unit").to_val()].to_val();
    assert_eq!(UdtEnum::try_from_val(&env, &unit), Ok(UdtEnum::Unit));
    let tuple: Val = vec![
        &env,
        symbol_short!("Tuple").to_val(),
        <_ as IntoVal<Env, Val>>::into_val(&1i32, &env),
    ]
    .to_val();
    assert_eq!(UdtEnum::try_from_val(&env, &tuple), Ok(UdtEnum::Tuple(1)));

    // No val of another type converts. Unlike the struct types above, an enum
    // errors rather than traps on a vec of the wrong shape, because it checks
    // the variant name is a symbol before unpacking the rest of the vec.
    assert_compatible_with::<UdtEnum>(&env, &[]);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnumInt {
    A = 0,
    B = 1,
}

#[test]
fn test_udt_enum_int() {
    let env = Env::default();

    // An enum with integer values converts from a u32 val, and so the u32 val
    // in the list of vals converts because it holds the value of a variant.
    let a: Val = <_ as IntoVal<Env, Val>>::into_val(&0u32, &env);
    assert_eq!(UdtEnumInt::try_from_val(&env, &a), Ok(UdtEnumInt::A));
    let unknown: Val = <_ as IntoVal<Env, Val>>::into_val(&2u32, &env);
    assert!(UdtEnumInt::try_from_val(&env, &unknown).is_err());

    assert_compatible_with::<UdtEnumInt>(&env, &["u32"]);
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum UdtError {
    AnError = 1,
}

#[test]
fn test_udt_error_enum() {
    let env = Env::default();

    // An error enum converts from an error val, and so the error val in the
    // list of vals converts because it holds the value of a variant.
    let e: Val = Error::from_contract_error(1).into_val(&env);
    assert_eq!(UdtError::try_from_val(&env, &e), Ok(UdtError::AnError));

    assert_compatible_with::<UdtError>(&env, &["error"]);
}

/// A [Val] of every type the host supports, each labelled with a name used by
/// the tests to say which types are compatible with the type being converted
/// into.
fn vals(env: &Env) -> [(&'static str, Val); 21] {
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
        ("bytes32", Bytes::from_array(env, &[0u8; 32]).into_val(env)),
        ("bytes64", Bytes::from_array(env, &[0u8; 64]).into_val(env)),
        ("string", String::from_str(env, "a").into_val(env)),
        ("vec_i32", vec![env, 1i32].into_val(env)),
        ("map_i32_i32", map![env, (1i32, 2i32)].into_val(env)),
        ("address", Address::generate(env).into_val(env)),
        ("muxed_address", MuxedAddress::generate(env).into_val(env)),
        ("error", Error::from_contract_error(1).into_val(env)),
    ]
}

/// Asserts that `T` converts from the [Val]s named in `compatible`, and that
/// converting from a [Val] of any other type is an error rather than a panic.
fn assert_compatible_with<T>(env: &Env, compatible: &[&str])
where
    T: TryFromVal<Env, Val>,
{
    assert_compatible_with_skipping::<T>(env, compatible, &[]);
}

/// Same as [assert_compatible_with], but not converting from the [Val]s named
/// in `skip`, for types where converting from those [Val]s traps in the host
/// instead of erroring, which the caller tests separately.
fn assert_compatible_with_skipping<T>(env: &Env, compatible: &[&str], skip: &[&str])
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
