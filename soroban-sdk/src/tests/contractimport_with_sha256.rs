use crate as soroban_sdk;
use soroban_sdk::{contractimpl, BytesN, Env};
use stellar_xdr::{
    ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeBytesN, ScSpecTypeDef,
};

mod addcontract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
        sha256 = "9cdd4b74295e226c9d6f26d40106a59fe0b69ba1d2b5f7a9cc3e5e94f5508ad5",
    );
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, contract_id: BytesN<32>, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
}

#[test]
fn test_functional() {
    let e = Env::default();

    let add_contract_id = e.register_contract_wasm(addcontract::WASM);

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
        name: "add_with".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                name: "contract_id".try_into().unwrap(),
                type_: ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n: 32 }),
            },
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
