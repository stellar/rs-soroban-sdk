use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl, contracttype, vec, ConversionError, Env, IntoVal, TryFromVal,
    TryIntoVal, Val, Vec,
};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeTuple,
    ScSpecTypeUdt,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt(pub i32, pub i32);

#[contract]
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
    let r: Val = a.into_val(&env);
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

// TODO: at present UDT try_from_vals actually trap rather than returning
// catchable errors. This is intentional to minimize code size. Can revisit.
#[test]
#[should_panic]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a vec will decode to a Udt.
    let vec = vec![&env, 5, 7].to_val();
    let udt = Udt::try_from_val(&env, &vec);
    assert_eq!(udt, Ok(Udt(5, 7)));

    // If a struct has 2 fields, and a vec is decoded into it where the vec has
    // 1 element, it is an error. It is an error because all fields must be
    // assigned values.
    let vec = vec![&env, 5].to_val();
    let udt = Udt::try_from_val(&env, &vec);
    assert_eq!(udt, Err(ConversionError));

    // If a struct has 2 fields, and a vec is decoded into it where the vec has
    // 3 elements, it is an error. It is an error because decoding and encoding
    // will not round trip the data, and therefore partial decoding is
    // relatively difficult to use safely.
    let vec = vec![&env, 5, 7, 9].to_val();
    let udt = Udt::try_from_val(&env, &vec);
    assert_eq!(udt, Err(ConversionError));
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::from_xdr(__SPEC_XDR_FN_ADD).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add".try_into().unwrap(),
        inputs: std::vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
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
