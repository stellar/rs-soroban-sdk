use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, Env};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
    ScSpecTypeTuple,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(_e: &Env, a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn add_with_unused_arg(_e: &Env, a: i32, _b: i32) -> i32 {
        a + 2
    }

    pub fn add_with_mut_arg(_e: &Env, a: i32, mut b: i32) -> i32 {
        b *= 1;
        a + b
    }

    pub fn add_with_ref_arg(_e: &Env, a: i32, b: &i32) -> i32 {
        a + b
    }

    pub fn void_fn(_e: &Env, _void_arg: ()) -> () {}

    pub fn tuple_single_fn(_e: &Env, arg: (u32,)) -> (u32,) {
        arg
    }

    pub fn tuple_two_fn(_e: &Env, arg: (u32, i64)) -> (u32, i64) {
        arg
    }
}

#[contract]
pub struct Contract2;

#[contractimpl]
impl Contract2 {
    pub fn add(_e: &Env, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[test]
fn test_functional() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());

    let a = 10i32;
    let b = 12i32;
    let c = ContractClient::new(&e, &contract_id).add(&a, &b);
    assert_eq!(c, 22);

    let c = ContractClient::new(&e, &contract_id).add_with_mut_arg(&a, &b);
    assert_eq!(c, 22);

    let c = ContractClient::new(&e, &contract_id).add_with_ref_arg(&a, &b);
    assert_eq!(c, 22);
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::from_xdr(Contract::spec_xdr_add(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::I32].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}

#[test]
fn test_spec_with_unused_arg() {
    let entries =
        ScSpecEntry::from_xdr(Contract::spec_xdr_add_with_unused_arg(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add_with_unused_arg".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::I32].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}

#[test]
fn test_spec_void_types() {
    let entries = ScSpecEntry::from_xdr(Contract::spec_xdr_void_fn(), Limits::none()).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "void_fn".try_into().unwrap(),
        inputs: vec![ScSpecFunctionInputV0 {
            doc: "".try_into().unwrap(),
            name: "void_arg".try_into().unwrap(),
            type_: ScSpecTypeDef::Void,
        }]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::Void].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}

#[test]
fn test_spec_tuple_single() {
    let entries =
        ScSpecEntry::from_xdr(Contract::spec_xdr_tuple_single_fn(), Limits::none()).unwrap();
    let tuple_type = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
        value_types: vec![ScSpecTypeDef::U32].try_into().unwrap(),
    }));
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "tuple_single_fn".try_into().unwrap(),
        inputs: vec![ScSpecFunctionInputV0 {
            doc: "".try_into().unwrap(),
            name: "arg".try_into().unwrap(),
            type_: tuple_type.clone(),
        }]
        .try_into()
        .unwrap(),
        outputs: vec![tuple_type].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}

#[test]
fn test_spec_tuple_two() {
    let entries = ScSpecEntry::from_xdr(Contract::spec_xdr_tuple_two_fn(), Limits::none()).unwrap();
    let tuple_type = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
        value_types: vec![ScSpecTypeDef::U32, ScSpecTypeDef::I64]
            .try_into()
            .unwrap(),
    }));
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "tuple_two_fn".try_into().unwrap(),
        inputs: vec![ScSpecFunctionInputV0 {
            doc: "".try_into().unwrap(),
            name: "arg".try_into().unwrap(),
            type_: tuple_type.clone(),
        }]
        .try_into()
        .unwrap(),
        outputs: vec![tuple_type].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}
