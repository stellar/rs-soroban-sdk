use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, Env};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: i32, b: i32) -> i32 {
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
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::from_xdr(__SPEC_XDR_FN_ADD, Limits::none()).unwrap();
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
