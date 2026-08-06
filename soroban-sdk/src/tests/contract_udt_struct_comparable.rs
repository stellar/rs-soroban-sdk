use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractimpl, contracttype, Comparable, Env, IntoVal, TryFromVal, Val,
};
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
    ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
};

// The derives here are the same derives that contractimport adds to the types
// it generates, and that a Val alone would not support.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype]
pub struct Udt {
    pub a: u32,
    pub v: Comparable<Val>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn echo(u: Udt) -> Udt {
        u
    }

    pub fn eq(a: Comparable<Val>, b: Comparable<Val>) -> bool {
        a == b
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let u = Udt {
        a: 1,
        v: Comparable::new(&env, 2u32.into_val(&env)),
    };
    assert_eq!(client.echo(&u), u);

    assert!(client.eq(
        &Comparable::new(&env, 2u32.into_val(&env)),
        &Comparable::new(&env, 2u32.into_val(&env)),
    ));
    assert!(!client.eq(
        &Comparable::new(&env, 2u32.into_val(&env)),
        &Comparable::new(&env, 3u32.into_val(&env)),
    ));
}

#[test]
fn test_to_and_from_val() {
    let env = Env::default();

    let u = Udt {
        a: 1,
        v: Comparable::new(&env, 2u32.into_val(&env)),
    };
    let val: Val = (&u).into_val(&env);
    let rt = Udt::try_from_val(&env, &val).unwrap();

    assert_eq!(u, rt);
}

#[test]
fn test_udt_spec() {
    let entry = ScSpecEntry::from_xdr(Udt::spec_xdr(), Limits::none()).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        name: "Udt".try_into().unwrap(),
        fields: vec![
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            },
            // The Comparable<Val> field is represented in the spec by the type
            // it wraps.
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "v".try_into().unwrap(),
                type_: ScSpecTypeDef::Val,
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entry, expect);
}

#[test]
fn test_fn_spec() {
    let entry = ScSpecEntry::from_xdr(Contract::spec_xdr_eq(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "eq".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Val,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::Val,
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::Bool].try_into().unwrap(),
    });
    assert_eq!(entry, expect);
}
