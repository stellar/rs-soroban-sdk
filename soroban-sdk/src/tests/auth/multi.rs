use crate as soroban_sdk;

use soroban_sdk::{
    contractimpl,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, BytesN, Env, IntoVal,
};

pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn fna(e: Env, contract: BytesN<32>, a: Address) -> i32 {
        let client = ContractBClient::new(&e, &contract);
        client.fnb(&a)
    }
}

pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn fnb(a: Address) -> i32 {
        a.require_auth();
        1
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_a_id = e.register_contract(None, ContractA);
    let contract_b_id = e.register_contract(None, ContractB);
    let client = ContractAClient::new(&e, &contract_a_id);

    let a = Address::random(&e);

    let c = client
        .mock_auths(&[MockAuth {
            address: &a,
            nonce: 0,
            invoke: &MockAuthInvoke {
                contract: &contract_b_id,
                fn_name: "fnb",
                args: (&a,).into_val(&e),
                sub_invokes: &[],
            },
        }])
        .fna(&contract_b_id, &a);

    assert_eq!(c, 1);

    println!("{:?}", e.auths());
}

#[test]
// This test panics with not authorized because it does not appear to be
// possible to constrain an auth to a specific call tree, unless the top-level
// of that call tree also calls require_auth with the same address.
#[should_panic = "NotAuthorized"]
fn test_auth_tree() {
    let e = Env::default();
    let contract_a_id = e.register_contract(None, ContractA);
    let contract_b_id = e.register_contract(None, ContractB);
    let client = ContractAClient::new(&e, &contract_a_id);

    let a = Address::random(&e);

    let c = client
        .mock_auths(&[MockAuth {
            address: &a,
            nonce: 0,
            invoke: &MockAuthInvoke {
                contract: &contract_a_id,
                fn_name: "fna",
                args: ().into_val(&e), // ???
                sub_invokes: &[MockAuthInvoke {
                    contract: &contract_b_id,
                    fn_name: "fnb",
                    args: (&a,).into_val(&e),
                    sub_invokes: &[],
                }],
            },
        }])
        .fna(&contract_b_id, &a);

    assert_eq!(c, 1);

    println!("{:?}", e.auths());
}
