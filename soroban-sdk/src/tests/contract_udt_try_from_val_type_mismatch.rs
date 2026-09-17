//! Tests what happens when converting a [Val] into a contract type when the
//! [Val] holds a value of an incompatible type.
//!
//! Every kind of type that the contract type macros support is tested against a
//! [Val] of every type the host supports, in the same way that the SDK types
//! are tested in the [super::try_from_val_type_mismatch] module. Each test also
//! converts a [Val] that does match, so that the type being tested is known to
//! be convertible at all.
//!
//! Converting is an error in every case except two: a struct converting from a
//! map that has keys that are not symbols, and a tuple struct converting from a
//! vec of a different length, both of which trap in the host and so panic.

use crate::{self as soroban_sdk};
use soroban_sdk::{
    contracterror, contracttype, map, symbol_short, vec, Env, IntoVal, TryFromVal, Val,
};

use super::try_from_val_type_mismatch::{assert_compatible_with, assert_compatible_with_skipping};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStruct {
    pub a: i32,
    pub b: i32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStructTuple(pub i32, pub i32);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnum {
    Unit,
    Tuple(i32),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnumInt {
    A = 0,
    B = 1,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum UdtError {
    AnError = 1,
}

#[test]
fn test_struct() {
    let env = Env::default();

    // A struct converts from a map keyed by the field names.
    let map = map![&env, (symbol_short!("a"), 1i32), (symbol_short!("b"), 2i32)].to_val();
    assert_eq!(
        UdtStruct::try_from_val(&env, &map),
        Ok(UdtStruct { a: 1, b: 2 })
    );

    // No val of another type converts. The map val is skipped because it is
    // keyed by integers, which traps, and is tested in
    // test_struct_from_map_with_non_symbol_keys_panics.
    assert_compatible_with_skipping::<UdtStruct>(&env, &[], &["map"]);
}

#[test]
#[should_panic(expected = "UnexpectedType")]
fn test_struct_from_map_with_non_symbol_keys_panics() {
    let env = Env::default();

    // The host traps when unpacking a map that has keys that are not symbols,
    // and so the conversion panics rather than returning an error.
    let map = map![&env, (1i32, 2i32)].to_val();
    let _ = UdtStruct::try_from_val(&env, &map);
}

#[test]
fn test_struct_tuple() {
    let env = Env::default();

    // A tuple struct converts from a vec with an element per field.
    let vec = vec![&env, 1i32, 2i32].to_val();
    assert_eq!(
        UdtStructTuple::try_from_val(&env, &vec),
        Ok(UdtStructTuple(1, 2))
    );

    // No val of another type converts. The vec val is skipped because it has
    // one element and not two, which traps, and is tested in
    // test_struct_tuple_from_vec_of_other_len_panics.
    assert_compatible_with_skipping::<UdtStructTuple>(&env, &[], &["vec"]);
}

#[test]
#[should_panic(expected = "UnexpectedSize")]
fn test_struct_tuple_from_vec_of_other_len_panics() {
    let env = Env::default();

    // The host traps when unpacking a vec into a slice of a different length,
    // and so the conversion panics rather than returning an error.
    let vec = vec![&env, 1i32].to_val();
    let _ = UdtStructTuple::try_from_val(&env, &vec);
}

#[test]
fn test_enum() {
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

#[test]
fn test_enum_int() {
    let env = Env::default();

    // An enum with integer values converts from a u32 val, and so the u32 val
    // in the list of vals converts because it holds the value of a variant.
    let a: Val = <_ as IntoVal<Env, Val>>::into_val(&0u32, &env);
    assert_eq!(UdtEnumInt::try_from_val(&env, &a), Ok(UdtEnumInt::A));
    let unknown: Val = <_ as IntoVal<Env, Val>>::into_val(&2u32, &env);
    assert!(UdtEnumInt::try_from_val(&env, &unknown).is_err());

    assert_compatible_with::<UdtEnumInt>(&env, &["u32"]);
}

#[test]
fn test_error_enum() {
    let env = Env::default();

    // An error enum converts from an error val, and so the error val in the
    // list of vals converts because it holds the value of a variant.
    let e: Val = soroban_sdk::Error::from_contract_error(1).into_val(&env);
    assert_eq!(UdtError::try_from_val(&env, &e), Ok(UdtError::AnError));

    assert_compatible_with::<UdtError>(&env, &["error"]);
}
