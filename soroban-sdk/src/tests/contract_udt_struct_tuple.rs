use crate as soroban_sdk;
use soroban_sdk::{
    contractimpl, contracttype, vec, ConversionError, Env, IntoVal, RawVal, TryFromVal, TryIntoVal,
    Vec,
};
use stellar_xdr::{
    ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeTuple,
    ScSpecTypeUdt,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt(pub i32, pub i32);

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }
}

#[test]
fn test_conversion() {
    let env = Env::default();
    let a = Udt(5, 7);
    let r: RawVal = a.into_val(&env);
    let v: Vec<i32> = r.try_into_val(&env).unwrap();
    assert_eq!(v, vec![&env, 5, 7]);
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let a = Udt(5, 7);
    let b = Udt(10, 14);
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a vec will decode to a Udt.
    let map = vec![&env, 5, 7].to_raw();
    let udt = Udt::try_from_val(&env, map);
    assert_eq!(udt, Ok(Udt(5, 7)));

    // If a struct has 2 fields, and a vec is decoded into it where the vec has
    // 2 elements, it is an error. It is an error because all fields must be
    // assigned values.
    let map = vec![&env, 5, 7, 9].to_raw();
    let udt = Udt::try_from_val(&env, map);
    assert_eq!(udt, Err(ConversionError));

    // If a struct has 2 fields, and a vec is decoded into it where the vec has
    // 3 elements, it is an error. It is an error because decoding and encoding
    // will not round trip the data, and therefore partial decoding is
    // relatively difficult to use safely.
    let map = vec![&env, 5, 7, 9].to_raw();
    let udt = Udt::try_from_val(&env, map);
    assert_eq!(udt, Err(ConversionError));
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::from_xdr(__SPEC_XDR_ADD).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        name: "add".try_into().unwrap(),
        inputs: std::vec![
            ScSpecFunctionInputV0 {
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
            ScSpecFunctionInputV0 {
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
        outputs: std::vec![ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: std::vec![
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        }))]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);
}
