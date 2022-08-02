#![cfg(feature = "testutils")]

use std::io::Cursor;

use soroban_sdk::{contractimpl, Env, FixedBinary};
use stellar_xdr::{ReadXdr, ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef};

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
    let contract_id = FixedBinary::from_array(&e, [0; 32]);
    e.register_contract(&contract_id, Contract);

    let a = 10i32;
    let b = 12i32;
    let c = add::invoke(&e, &contract_id, &a, &b);
    assert_eq!(c, 22);
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::read_xdr(&mut Cursor::new(&__SPEC_XDR_ADD)).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        name: "add".try_into().unwrap(),
        input_types: vec![ScSpecTypeDef::I32, ScSpecTypeDef::I32]
            .try_into()
            .unwrap(),
        output_types: vec![ScSpecTypeDef::I32].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}
