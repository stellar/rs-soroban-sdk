use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contractimpl, xdr, xdr::ReadXdr as _, Env, IntoVal, Timepoint};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn exec(_t: Timepoint) {}
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let t: Timepoint = xdr::ScVal::Timepoint(xdr::TimePoint(0)).into_val(&env);
    client.exec(&t);
}

#[test]
fn test_spec() {
    let entries = xdr::ScSpecEntry::from_xdr(__SPEC_XDR_FN_EXEC, xdr::Limits::none()).unwrap();
    let expect = xdr::ScSpecEntry::FunctionV0(xdr::ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "exec".try_into().unwrap(),
        inputs: [xdr::ScSpecFunctionInputV0 {
            doc: "".try_into().unwrap(),
            name: "_t".try_into().unwrap(),
            type_: xdr::ScSpecTypeDef::Timepoint,
        }]
        .try_into()
        .unwrap(),
        outputs: [].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}
