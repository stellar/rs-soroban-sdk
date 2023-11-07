//! Demonstrates how an address can be required auth multiple times in a single
//! invocation.
//!
//! The authorizations cannot be grouped into a single authorization.

use crate as soroban_sdk;

use soroban_sdk::{
    contract, contractimpl,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Address, x: i32, y: i32) -> i32 {
        a.require_auth();
        a.require_auth();
        x + y
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let a = Address::generate(&e);

    let c = client
        .mock_auths(&[
            MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_id,
                    fn_name: "add",
                    args: (&a, 10, 12).into_val(&e),
                    sub_invokes: &[],
                },
            },
            MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_id,
                    fn_name: "add",
                    args: (&a, 10, 12).into_val(&e),
                    sub_invokes: &[],
                },
            },
        ])
        .add(&a, &10, &12);

    assert_eq!(c, 22);

    println!("{:?}", e.auths());
}
