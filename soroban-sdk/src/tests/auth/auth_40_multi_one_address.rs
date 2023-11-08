use crate as soroban_sdk;

use soroban_sdk::{
    contract, contractimpl,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn fna(e: Env, contract: Address, a: Address) -> i32 {
        a.require_auth_for_args((&a,).into_val(&e));
        let client = ContractBClient::new(&e, &contract);
        client.fnb(&a)
    }
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn fnb(a: Address) -> i32 {
        a.require_auth();
        1
    }
}

#[test]
fn test_auth_not_allowed_with_separated_tree() {
    let e = Env::default();
    let contract_a_id = e.register_contract(None, ContractA);
    let contract_b_id = e.register_contract(None, ContractB);
    let client = ContractAClient::new(&e, &contract_a_id);

    let a = Address::generate(&e);

    assert!(client
        .mock_auths(&[
            MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_a_id,
                    fn_name: "fna",
                    args: (&a,).into_val(&e),
                    sub_invokes: &[],
                },
            },
            MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_b_id,
                    fn_name: "fnb",
                    args: (&a,).into_val(&e),
                    sub_invokes: &[],
                },
            },
        ])
        .try_fna(&contract_b_id, &a)
        .is_err());
}

#[test]
fn test_auth_as_tree() {
    let e = Env::default();
    let contract_a_id = e.register_contract(None, ContractA);
    let contract_b_id = e.register_contract(None, ContractB);
    let client = ContractAClient::new(&e, &contract_a_id);

    let a = Address::generate(&e);

    let c = client
        .mock_auths(&[MockAuth {
            address: &a,
            invoke: &MockAuthInvoke {
                contract: &contract_a_id,
                fn_name: "fna",
                args: (&a,).into_val(&e),
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
