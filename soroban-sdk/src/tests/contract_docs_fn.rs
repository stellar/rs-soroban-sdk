use crate as soroban_sdk;
use soroban_sdk::{contractimpl, Env};
use stellar_xdr::{ReadXdr, ScSpecEntry, ScSpecFunctionV0};

pub struct Contract;

#[contractimpl]
impl Contract {
    /// Add adds
    // TODO: Implement this.
    /// things together.
    pub fn add() {}
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    client.add();
}

#[test]
fn test_spec() {
    let entry = ScSpecEntry::from_xdr(__SPEC_XDR_FN_ADD).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "Add adds\nthings together.".try_into().unwrap(),
        name: "add".try_into().unwrap(),
        inputs: vec![].try_into().unwrap(),
        outputs: vec![].try_into().unwrap(),
    });
    assert_eq!(entry, expect);
}
