use crate as soroban_sdk;
use soroban_sdk::{contractimpl, Address, Env};
use stellar_xdr::{ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef};

mod addcontract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
        sha256 = "2a83b4b0370b8806b08c56f5c1cf8430c7eb029accb6cdbc403d5b82479b1316",
    );
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, contract_id: Address, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
}

#[test]
fn test_functional() {
    let e = Env::default();

    let add_contract_id = e.register_contract_wasm(None, addcontract::WASM);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10u64;
    let y = 12u64;
    let z = client.add_with(&add_contract_id, &x, &y);
    assert!(z == 22);
}

#[test]
fn test_spec() {
    let entries = soroban_spec::read::parse_raw(&Contract::spec_xdr_add_with()).unwrap();
    let expect = vec![ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add_with".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "contract_id".try_into().unwrap(),
                type_: ScSpecTypeDef::Address,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "x".try_into().unwrap(),
                type_: ScSpecTypeDef::U64,
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "y".try_into().unwrap(),
                type_: ScSpecTypeDef::U64,
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::U64].try_into().unwrap(),
    })];
    assert_eq!(entries, expect);
}
