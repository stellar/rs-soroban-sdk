//! Tests what happens when converting a [Val] into an SDK type or a contract
//! type when the [Val] holds a value of an incompatible type.
//!
//! Every type is tested against a [Val] of every type the host supports, and
//! the types named as compatible are the only ones that may convert. Anything
//! else must be a conversion error, because contracts receive [Val]s from
//! callers that are free to send a value of any type.
//!
//! Converting is an error in every case except four, which trap in the host
//! and so panic: a contract type struct converting from a map that has keys
//! that are not symbols, a tuple or a contract type tuple struct converting
//! from a vec of a different length, and a contract type enum converting from
//! a vec whose first element is a symbol that is not one of the variant names.

use crate::{
    self as soroban_sdk,
    crypto::{
        bls12_381::{Bls12381Fp, Bls12381Fp2, Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine},
        bn254::{Bn254Fp, Bn254Fr, Bn254G1Affine, Bn254G2Affine},
        Hash,
    },
};
use soroban_sdk::{
    contracterror, contracttype, map, symbol_short,
    testutils::{Address as _, MuxedAddress as _},
    vec, Address, Bytes, BytesN, ConversionError, Duration, Env, Error, IntoVal, Map, MuxedAddress,
    String, Symbol, Timepoint, TryFromVal, Val, Vec, I256, U256,
};
// TryFromValForContractFn is deprecated for use outside the SDK, but it is the
// conversion a contract function performs, and so is what these tests convert
// with where a type has no TryFromVal.
#[allow(deprecated)]
use soroban_sdk::TryFromValForContractFn;

#[test]
fn test_void() {
    let env = Env::default();
    assert_with::<()>(&env, &["void"]);
}

#[test]
fn test_bool() {
    let env = Env::default();
    assert_with::<bool>(&env, &["bool"]);
}

#[test]
fn test_u32() {
    let env = Env::default();
    assert_with::<u32>(&env, &["u32"]);
}

#[test]
fn test_i32() {
    let env = Env::default();
    assert_with::<i32>(&env, &["i32"]);
}

#[test]
fn test_u64() {
    let env = Env::default();
    assert_with::<u64>(&env, &["u64", "u64_large"]);
}

#[test]
fn test_i64() {
    let env = Env::default();
    assert_with::<i64>(&env, &["i64", "i64_large"]);
}

#[test]
fn test_u128() {
    let env = Env::default();
    assert_with::<u128>(&env, &["u128", "u128_large"]);
}

#[test]
fn test_i128() {
    let env = Env::default();
    assert_with::<i128>(&env, &["i128", "i128_large"]);
}

#[test]
fn test_u256() {
    let env = Env::default();
    assert_with::<U256>(&env, &["u256", "u256_large"]);
}

#[test]
fn test_i256() {
    let env = Env::default();
    assert_with::<I256>(&env, &["i256", "i256_large"]);
}

#[test]
fn test_timepoint() {
    let env = Env::default();
    assert_with::<Timepoint>(&env, &["timepoint", "timepoint_large"]);
}

#[test]
fn test_duration() {
    let env = Env::default();
    assert_with::<Duration>(&env, &["duration", "duration_large"]);
}

#[test]
fn test_symbol() {
    let env = Env::default();
    assert_with::<Symbol>(&env, &["symbol", "symbol_long"]);
}

#[test]
fn test_string() {
    let env = Env::default();
    assert_with::<String>(&env, &["string"]);
}

#[test]
fn test_bytes() {
    let env = Env::default();
    // Bytes converts from a bytes val of any length.
    assert_with::<Bytes>(
        &env,
        &[
            "bytes32", "bytes48", "bytes64", "bytes96", "bytes128", "bytes192",
        ],
    );
}

#[test]
fn test_bytes_n() {
    let env = Env::default();
    // A BytesN converts only from a bytes val of its own length.
    assert_with::<BytesN<32>>(&env, &["bytes32"]);
    assert_with::<BytesN<64>>(&env, &["bytes64"]);
}

#[test]
fn test_tuple() {
    let env = Env::default();

    // A tuple converts from a vec with an element per field.
    let vec = vec![&env, 1i32, 2i32].to_val();
    assert_eq!(<(i32, i32)>::try_from_val(&env, &vec), Ok((1, 2)));

    // No val of another type converts. The vec vals are skipped because they
    // have one element and not two, which traps, and is tested in
    // test_tuple_from_vec_of_other_len_panics.
    let vecs = &["vec_i32", "vec_string"];
    assert_with_skipping::<(i32, i32)>(&env, &[], vecs);
}

#[test]
#[should_panic(expected = "UnexpectedSize")]
fn test_tuple_from_vec_of_other_len_panics() {
    let env = Env::default();

    let vec = vec![&env, 1i32].to_val();

    // A tuple unpacks the same way a contract type tuple struct does, and so
    // the host traps on a vec of a different length.
    let _ = <(i32, i32)>::try_from_val(&env, &vec);
}

#[test]
#[allow(deprecated)]
fn test_crypto_hash() {
    let env = Env::default();

    // Hash has no public TryFromVal, but a contract function converts the vals
    // its caller supplies into it through TryFromValForContractFn, so that
    // conversion is tested in the same way.
    for (name, val) in vals(&env) {
        let converted =
            <Hash<32> as TryFromValForContractFn<Env, Val>>::try_from_val_for_contract_fn(
                &env, &val,
            )
            .is_ok();
        assert_eq!(
            converted,
            name == "bytes32",
            "converting a {} val into Hash<32> {} but should {}",
            name,
            if converted { "succeeded" } else { "errored" },
            if converted { "error" } else { "succeed" },
        );
    }
}

#[test]
fn test_crypto_bls12_381() {
    let env = Env::default();

    // The BLS12-381 types wrap a BytesN of their own size, or a U256, and so
    // convert only from a bytes val of that size, or from a u256 val.
    assert_with::<Bls12381Fp>(&env, &["bytes48"]);
    assert_with::<Bls12381Fp2>(&env, &["bytes96"]);
    assert_with::<Bls12381G1Affine>(&env, &["bytes96"]);
    assert_with::<Bls12381G2Affine>(&env, &["bytes192"]);
    assert_with::<Bls12381Fr>(&env, &["u256", "u256_large"]);
}

#[test]
fn test_crypto_bn254() {
    let env = Env::default();

    // The BN254 types wrap a BytesN of their own size, or a U256, in the same
    // way as the BLS12-381 types.
    assert_with::<Bn254Fp>(&env, &["bytes32"]);
    assert_with::<Bn254G1Affine>(&env, &["bytes64"]);
    assert_with::<Bn254G2Affine>(&env, &["bytes128"]);
    assert_with::<Bn254Fr>(&env, &["u256", "u256_large"]);
}

#[test]
fn test_vec() {
    let env = Env::default();
    // The element type of a vec is not checked when converting, and so a vec
    // of any element type converts into a Vec of any element type. Conversion
    // of the elements happens when they are accessed.
    assert_with::<Vec<i32>>(&env, &["vec_i32", "vec_string"]);
    assert_with::<Vec<String>>(&env, &["vec_i32", "vec_string"]);
}

#[test]
fn test_map() {
    let env = Env::default();
    // The key and value types of a map are not checked when converting, and so
    // a map of any key and value types converts into a Map of any key and value
    // types. Conversion of the keys and values happens when they are accessed.
    let maps = &["map_i32_i32", "map_string_string"];
    assert_with::<Map<i32, i32>>(&env, maps);
    assert_with::<Map<String, String>>(&env, maps);
}

#[test]
fn test_address() {
    let env = Env::default();
    assert_with::<Address>(&env, &["address"]);
}

#[test]
fn test_muxed_address() {
    let env = Env::default();
    assert_with::<MuxedAddress>(&env, &["address", "muxed_address"]);
}

#[test]
fn test_error() {
    let env = Env::default();
    assert_with::<Error>(&env, &["error"]);
}

#[test]
fn test_option() {
    let env = Env::default();
    // A void val is the absent option, and so both void and the option's own
    // type convert.
    assert_with::<Option<u32>>(&env, &["void", "u32"]);
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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStructOption {
    pub a: i32,
    pub b: Option<i32>,
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
    assert_eq!(
        UdtStructOption::try_from_val(&env, &map),
        Ok(UdtStructOption { a: 1, b: Some(2) })
    );

    // A map that is missing a field does not partially convert, because a
    // missing field decodes as void.
    let partial = map![&env, (symbol_short!("a"), 1i32)].to_val();
    assert_eq!(
        UdtStruct::try_from_val(&env, &partial),
        Err(ConversionError)
    );

    // Unless the missing field is an Option, which converts from that void as
    // None.
    assert_eq!(
        UdtStructOption::try_from_val(&env, &partial),
        Ok(UdtStructOption { a: 1, b: None })
    );

    // No val of another type converts. The map vals are skipped because their
    // keys are not symbols, which traps, and is tested in
    // test_udt_struct_from_map_with_non_string_keys_panics.
    let maps = &["map_i32_i32", "map_string_string"];
    assert_with_skipping::<UdtStruct>(&env, &[], maps);
    assert_with_skipping::<UdtStructOption>(&env, &[], maps);
}

#[test]
#[should_panic(expected = "UnexpectedType")]
fn test_udt_struct_from_map_with_non_string_keys_panics() {
    let env = Env::default();

    let map = map![&env, (1i32, 2i32)].to_val();

    // The host traps when unpacking a map that has keys that are not symbols,
    // and so the conversion panics rather than returning an error.
    let _ = UdtStruct::try_from_val(&env, &map);
}

#[test]
#[should_panic(expected = "UnexpectedType")]
fn test_udt_struct_from_map_with_string_keys_panics() {
    let env = Env::default();

    let map = map![
        &env,
        (String::from_str(&env, "a"), 1i32),
        (String::from_str(&env, "b"), 2i32)
    ]
    .to_val();

    // Strings are not symbols either, even though they hold the same field
    // names, and so a map keyed by them traps in the same way.
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

    // No val of another type converts. The vec vals are skipped because they
    // have one element and not two, which traps, and is tested in
    // test_udt_struct_tuple_from_vec_of_other_len_panics.
    let vecs = &["vec_i32", "vec_string"];
    assert_with_skipping::<UdtStructTuple>(&env, &[], vecs);
}

#[test]
#[should_panic(expected = "UnexpectedSize")]
fn test_udt_struct_tuple_from_vec_of_other_len_panics() {
    let env = Env::default();

    let vec = vec![&env, 1i32].to_val();

    // The host traps when unpacking a vec into a slice of a different length,
    // and so the conversion panics rather than returning an error.
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

    let wrong_payload: Val = vec![
        &env,
        symbol_short!("Tuple").to_val(),
        String::from_str(&env, "not an i32").to_val(),
    ]
    .to_val();
    assert_eq!(
        UdtEnum::try_from_val(&env, &wrong_payload),
        Err(ConversionError)
    );

    // No val of another type converts. Unlike the struct types above, an enum
    // errors rather than traps on a vec of the wrong shape, because it checks
    // the variant name is a symbol before unpacking the rest of the vec.
    assert_with::<UdtEnum>(&env, &[]);
}

#[test]
#[should_panic(expected = "InvalidInput")]
fn test_udt_enum_from_vec_with_unknown_variant_name_panics() {
    let env = Env::default();

    let vec: Val = vec![&env, symbol_short!("Nope").to_val()].to_val();

    // The host traps when looking up a symbol that is not one of the variant
    // names, and so the conversion panics rather than returning an error. A vec
    // whose first element is not a symbol at all errors, as does a vec with a
    // known variant name and the wrong payload.
    let _ = UdtEnum::try_from_val(&env, &vec);
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

    assert_with::<UdtEnumInt>(&env, &["u32"]);
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

    assert_with::<UdtError>(&env, &["error"]);
}

/// A [Val] of every type the host supports, each labelled with a name used by
/// the tests to say which types are compatible with the type being converted
/// into.
///
/// The types that the host stores as a small value when they fit and as an
/// object when they do not have a val of each form, because the conversions
/// take different paths for the two.
fn vals(env: &Env) -> [(&'static str, Val); 36] {
    [
        ("void", ().into_val(env)),
        ("bool", true.into_val(env)),
        ("u32", 1u32.into_val(env)),
        ("i32", 1i32.into_val(env)),
        ("u64", 1u64.into_val(env)),
        ("u64_large", u64::MAX.into_val(env)),
        ("i64", 1i64.into_val(env)),
        ("i64_large", i64::MIN.into_val(env)),
        ("timepoint", Timepoint::from_unix(env, 1).into_val(env)),
        (
            "timepoint_large",
            Timepoint::from_unix(env, u64::MAX).into_val(env),
        ),
        ("duration", Duration::from_seconds(env, 1).into_val(env)),
        (
            "duration_large",
            Duration::from_seconds(env, u64::MAX).into_val(env),
        ),
        ("u128", 1u128.into_val(env)),
        ("u128_large", u128::MAX.into_val(env)),
        ("i128", 1i128.into_val(env)),
        ("i128_large", i128::MIN.into_val(env)),
        ("u256", U256::from_u32(env, 1).into_val(env)),
        ("u256_large", U256::max_value(env).into_val(env)),
        ("i256", I256::from_i32(env, 1).into_val(env)),
        ("i256_large", I256::min_value(env).into_val(env)),
        ("symbol", symbol_short!("a").into_val(env)),
        (
            "symbol_long",
            Symbol::new(env, "a_symbol_too_long_to_be_small").into_val(env),
        ),
        ("bytes32", Bytes::from_array(env, &[0u8; 32]).into_val(env)),
        ("bytes48", Bytes::from_array(env, &[0u8; 48]).into_val(env)),
        ("bytes64", Bytes::from_array(env, &[0u8; 64]).into_val(env)),
        ("bytes96", Bytes::from_array(env, &[0u8; 96]).into_val(env)),
        (
            "bytes128",
            Bytes::from_array(env, &[0u8; 128]).into_val(env),
        ),
        (
            "bytes192",
            Bytes::from_array(env, &[0u8; 192]).into_val(env),
        ),
        ("string", String::from_str(env, "a").into_val(env)),
        ("vec_i32", vec![env, 1i32].into_val(env)),
        (
            "vec_string",
            vec![env, String::from_str(env, "a")].into_val(env),
        ),
        ("map_i32_i32", map![env, (1i32, 2i32)].into_val(env)),
        (
            "map_string_string",
            map![
                env,
                (String::from_str(env, "a"), String::from_str(env, "b"))
            ]
            .into_val(env),
        ),
        ("address", Address::generate(env).into_val(env)),
        ("muxed_address", MuxedAddress::generate(env).into_val(env)),
        ("error", Error::from_contract_error(1).into_val(env)),
    ]
}

/// Asserts that `T` converts from the [Val]s named in `compatible`, and that
/// converting from a [Val] of any other type is an error rather than a panic.
fn assert_with<T>(env: &Env, compatible: &[&str])
where
    T: TryFromVal<Env, Val>,
{
    assert_with_skipping::<T>(env, compatible, &[]);
}

/// Same as [assert_with], but not converting from the [Val]s named
/// in `skip`, for types where converting from those [Val]s traps in the host
/// instead of erroring, which the caller tests separately.
fn assert_with_skipping<T>(env: &Env, compatible: &[&str], skip: &[&str])
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
