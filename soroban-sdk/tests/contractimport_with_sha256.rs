#![cfg(feature = "testutils")]

use soroban_sdk::{contractimpl, BytesN, Env};
use stellar_xdr::{ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef};

const ADD_CONTRACT_ID: [u8; 32] = [0; 32];
mod addcontract {
    soroban_sdk::contractimport!(
        file = "target/wasm32-unknown-unknown/release/example_add_i32.wasm",
        sha256 = "6c5a4fe67b30474ff71fe489ac7321d850b91cc2c7db3f36f377fad3d3087d6f",
    );
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: i32, y: i32) -> i32 {
        addcontract::Client::add(&env, &BytesN::from_array(&env, &ADD_CONTRACT_ID), x, y)
    }
}

#[test]
fn test_functional() {
    let e = Env::default();

    let add_contract_id = BytesN::from_array(&e, &ADD_CONTRACT_ID);
    e.register_contract_wasm(&add_contract_id, addcontract::WASM);

    let contract_id = BytesN::from_array(&e, &[1; 32]);
    e.register_contract(&contract_id, Contract);

    let x = 10i32;
    let y = 12i32;
    let z = add_with::invoke(&e, &contract_id, &x, &y);
    assert!(z == 22);
}

#[test]
fn test_spec() {
    let entries = soroban_spec::read::parse_raw(&Contract::spec_xdr_add_with()).unwrap();
    let expect = vec![ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        name: "add_with".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                name: "x".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
            ScSpecFunctionInputV0 {
                name: "y".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::I32].try_into().unwrap(),
    })];
    assert_eq!(entries, expect);
}
