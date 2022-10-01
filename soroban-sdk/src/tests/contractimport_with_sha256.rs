use crate as soroban_sdk;
use soroban_sdk::{contractimpl, BytesN, Env};
use stellar_xdr::{ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef};

const ADD_CONTRACT_ID: [u8; 32] = [0; 32];
mod addcontract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
        sha256 = "de2bdc56dafd6b3b25b7224f7f9f33b4e32a62a1f3cc63b856244de236b690b8",
    );
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &ADD_CONTRACT_ID).add(&x, &y)
    }
}

#[test]
fn test_functional() {
    let e = Env::default();

    let add_contract_id = BytesN::from_array(&e, &ADD_CONTRACT_ID);
    e.register_contract_wasm(&add_contract_id, addcontract::WASM);

    let contract_id = BytesN::from_array(&e, &[1; 32]);
    e.register_contract(&contract_id, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10u64;
    let y = 12u64;
    let z = client.add_with(&x, &y);
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
                type_: ScSpecTypeDef::U64,
            },
            ScSpecFunctionInputV0 {
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
