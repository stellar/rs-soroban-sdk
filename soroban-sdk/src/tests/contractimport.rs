use crate as soroban_sdk;
use soroban_sdk::{contractimpl, BytesN, Env};
use stellar_xdr::{
    ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeBytesN, ScSpecTypeDef,
};

mod addcontract {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm"
    );
}

mod subcontract {
    use crate as soroban_sdk;
    pub struct Contract;
    #[soroban_sdk::contractimpl]
    impl Contract {
        pub fn sub(a: u64, b: u64) -> u64 {
            a - b
        }
    }
}

pub struct Contract;

#[contractimpl(crate_path = "crate")]
impl Contract {
    pub fn add_with(env: Env, contract_id: BytesN<32>, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
    pub fn sub_with(env: Env, contract_id: BytesN<32>, x: u64, y: u64) -> u64 {
        subcontract::ContractClient::new(&env, &contract_id).sub(&x, &y)
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
fn test_register_at_id() {
    let e = Env::default();

    let add_contract_id = BytesN::from_array(&e, &[1; 32]);
    e.register_contract_wasm(&add_contract_id, addcontract::WASM);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10u64;
    let y = 12u64;
    let z = client.add_with(&add_contract_id, &x, &y);
    assert!(z == 22);
}

#[test]
fn test_reregister_wasm() {
    let e = Env::default();

    // Register a contract with code that will fail, to ensure this code isn't
    // the code that gets activated when invoked.
    let add_contract_id = e.register_contract_wasm(None, &[]);
    // Reregister the contract with different code replacing the code. This is
    // the contract we expect to be executed.
    e.register_contract_wasm(&add_contract_id, addcontract::WASM);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10u64;
    let y = 12u64;
    let z = client.add_with(&add_contract_id, &x, &y);
    assert!(z == 22);
}

#[test]
fn test_reregister_over_wasm_with_rust_impl() {
    let e = Env::default();

    // Register a contract with wasm.
    let other_contract_id = e.register_contract_wasm(None, addcontract::WASM);
    // Reregister the contract with a rust impl instead that does something
    // different.
    e.register_contract(&other_contract_id, subcontract::Contract);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 12u64;
    let y = 10u64;
    let z = client.sub_with(&other_contract_id, &x, &y);
    assert!(z == 2);
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
