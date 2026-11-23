use crate::{self as soroban_sdk};
use soroban_sdk::xdr::{ScMap, ScMapEntry, ScSymbol, ScVal, ScVec};
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, ConversionError, Env, IntoVal, Symbol,
    TryFromVal, TryIntoVal, Val, Vec,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Udt {
    Aaa,
    Bbb(i32),
    MaxFields(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
    Nested(Udt2, Udt2),
    ThisIsAVeryLongName1234567890,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt2 {
    a: u32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let a = Udt::Aaa;
    let b = Udt::Bbb(3);
    let c = client.add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a vec will decode to a Udt if the first element is the
    // variant name as a Symbol, and following elements are tuple-like values
    // for the variant.
    let vec: Vec<Val> = vec![&env, symbol_short!("Aaa").into_val(&env)];
    let udt = Udt::try_from_val(&env, &vec.to_val());
    assert_eq!(udt, Ok(Udt::Aaa));
    let vec: Vec<Val> = vec![&env, symbol_short!("Bbb").into_val(&env), 8.into()];
    let udt = Udt::try_from_val(&env, &vec.to_val());
    assert_eq!(udt, Ok(Udt::Bbb(8)));

    // If an enum has a tuple like variant with one value, but the vec has
    // multiple values, it is an error. It is an error because decoding and
    // encoding will not round trip the data, and therefore partial decoding is
    // relatively difficult to use safely.
    let vec: Vec<Val> = vec![&env, symbol_short!("Aaa").into_val(&env), 8.into()];
    let udt = Udt::try_from_val(&env, &vec.to_val());
    assert_eq!(udt, Err(ConversionError));
    let vec: Vec<Val> = vec![
        &env,
        symbol_short!("Bbb").into_val(&env),
        8.into(),
        9.into(),
    ];
    let udt = Udt::try_from_val(&env, &vec.to_val());
    assert_eq!(udt, Err(ConversionError));
}

#[test]
fn round_trips() {
    let env = Env::default();

    let before = Udt::Nested(Udt2 { a: 1 }, Udt2 { a: 2 });
    let val: Val = before.try_into_val(&env).unwrap();
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);
    let scvec: ScVec = before.try_into().unwrap();
    let after: Udt = scvec.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let before = Udt::MaxFields(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    let val: Val = before.try_into_val(&env).unwrap();
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);
    let scvec: ScVec = before.try_into().unwrap();
    let after: Udt = scvec.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let before = Udt::ThisIsAVeryLongName1234567890;
    let val: Val = before.try_into_val(&env).unwrap();
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);
    let scvec: ScVec = before.try_into().unwrap();
    let after: Udt = scvec.try_into_val(&env).unwrap();
    assert_eq!(before, after);
}

#[test]
fn test_unit_variant_round_trips_as_vec_of_symbol() {
    let env = Env::default();

    let before = Udt::Aaa;

    let val: Val = before.try_into_val(&env).unwrap();
    let vec: Vec<Val> = val.try_into_val(&env).unwrap();
    assert_eq!(vec, vec![&env, symbol_short!("Aaa").into_val(&env)]);
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let scval: ScVal = (&before).try_into().unwrap();
    assert_eq!(
        scval,
        ScVal::Vec(Some(ScVec(
            [ScVal::Symbol(ScSymbol("Aaa".try_into().unwrap()))]
                .try_into()
                .unwrap()
        )))
    );
    let after = Udt::try_from_val(&env, &scval).unwrap();
    assert_eq!(before, after);
}

#[test]
fn test_unit_variant_with_long_name_round_trips_as_vec_of_symbol() {
    let env = Env::default();

    let before = Udt::ThisIsAVeryLongName1234567890;

    let val: Val = before.try_into_val(&env).unwrap();
    let vec: Vec<Val> = val.try_into_val(&env).unwrap();
    assert_eq!(
        vec,
        vec![
            &env,
            Symbol::new(&env, "ThisIsAVeryLongName1234567890").into_val(&env)
        ]
    );
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let scval: ScVal = (&before).try_into().unwrap();
    assert_eq!(
        scval,
        ScVal::Vec(Some(ScVec(
            [ScVal::Symbol(ScSymbol(
                "ThisIsAVeryLongName1234567890".try_into().unwrap()
            ))]
            .try_into()
            .unwrap()
        )))
    );
    let after = Udt::try_from_val(&env, &scval).unwrap();
    assert_eq!(before, after);
}

#[test]
fn test_tuple_variant_round_trips_as_vec_of_symbol_and_field() {
    let env = Env::default();

    let before = Udt::Bbb(3);

    let val: Val = before.try_into_val(&env).unwrap();
    let vec: Vec<Val> = val.try_into_val(&env).unwrap();
    assert_eq!(
        vec,
        vec![
            &env,
            symbol_short!("Bbb").into_val(&env),
            3i32.into_val(&env)
        ]
    );
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let scval: ScVal = (&before).try_into().unwrap();
    assert_eq!(
        scval,
        ScVal::Vec(Some(ScVec(
            [
                ScVal::Symbol(ScSymbol("Bbb".try_into().unwrap())),
                ScVal::I32(3),
            ]
            .try_into()
            .unwrap()
        )))
    );
    let after = Udt::try_from_val(&env, &scval).unwrap();
    assert_eq!(before, after);
}

#[test]
fn test_tuple_variant_with_many_fields_round_trips_as_vec_of_symbol_and_fields() {
    let env = Env::default();

    let before = Udt::MaxFields(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

    let val: Val = before.try_into_val(&env).unwrap();
    let vec: Vec<Val> = val.try_into_val(&env).unwrap();
    let mut expected: Vec<Val> = vec![&env, symbol_short!("MaxFields").into_val(&env)];
    for i in 1u32..=12 {
        expected.push_back(i.into_val(&env));
    }
    assert_eq!(vec, expected);
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let scval: ScVal = (&before).try_into().unwrap();
    let mut expected = std::vec![ScVal::Symbol(ScSymbol("MaxFields".try_into().unwrap()))];
    expected.extend((1u32..=12).map(ScVal::U32));
    assert_eq!(scval, ScVal::Vec(Some(ScVec(expected.try_into().unwrap()))));
    let after = Udt::try_from_val(&env, &scval).unwrap();
    assert_eq!(before, after);
}

#[test]
fn test_tuple_variant_with_udt_fields_round_trips_as_vec_of_symbol_and_fields() {
    let env = Env::default();

    let before = Udt::Nested(Udt2 { a: 1 }, Udt2 { a: 2 });

    let val: Val = before.try_into_val(&env).unwrap();
    let vec: Vec<Val> = val.try_into_val(&env).unwrap();
    assert_eq!(
        vec,
        vec![
            &env,
            symbol_short!("Nested").into_val(&env),
            Udt2 { a: 1 }.into_val(&env),
            Udt2 { a: 2 }.into_val(&env),
        ]
    );
    let after: Udt = val.try_into_val(&env).unwrap();
    assert_eq!(before, after);

    let udt2 = |a: u32| {
        ScVal::Map(Some(ScMap(
            [ScMapEntry {
                key: ScVal::Symbol(ScSymbol("a".try_into().unwrap())),
                val: ScVal::U32(a),
            }]
            .try_into()
            .unwrap(),
        )))
    };
    let scval: ScVal = (&before).try_into().unwrap();
    assert_eq!(
        scval,
        ScVal::Vec(Some(ScVec(
            [
                ScVal::Symbol(ScSymbol("Nested".try_into().unwrap())),
                udt2(1),
                udt2(2),
            ]
            .try_into()
            .unwrap()
        )))
    );
    let after = Udt::try_from_val(&env, &scval).unwrap();
    assert_eq!(before, after);
}

#[test]
fn test_owned_to_val() {
    let env = Env::default();

    let u = Udt::Aaa;
    let val: Val = u.clone().into_val(&env);
    let rt: Udt = val.into_val(&env);

    assert_eq!(u, rt);
}

#[test]
fn test_ref_to_val() {
    let env = Env::default();

    let u = Udt::Aaa;
    let val: Val = (&u).into_val(&env);
    let rt: Udt = val.into_val(&env);

    assert_eq!(u, rt);
}

#[test]
fn test_double_ref_to_val() {
    let env = Env::default();

    let u = Udt::Aaa;
    let val: Val = (&&u).into_val(&env);
    let rt: Udt = val.into_val(&env);

    assert_eq!(u, rt);
}
