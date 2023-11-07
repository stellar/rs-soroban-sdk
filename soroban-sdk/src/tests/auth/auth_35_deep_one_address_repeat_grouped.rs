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

    let a = Address::generate(&e);

    let c = client
        .mock_auths(&[
            MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_b_id,
                    fn_name: "fnb",
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
        .fna(&contract_b_id, &a);

    assert_eq!(c, 1);

    println!("{:?}", e.auths());
}

#[test]
// This test panics with not authorized because it is not possible to constrain
// an auth to a specific call tree, unless the top-level of that call tree also
// calls require_auth with the same address.
//
// It also isn't possible to group auths that occur at the same level, again
// unless a higher level also require_auth's the same addresses.
#[should_panic = "HostError: Error(Auth, InvalidAction)"]
fn test_auth_tree() {
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
                args: ().into_val(&e), // ???
                sub_invokes: &[
                    MockAuthInvoke {
                        contract: &contract_b_id,
                        fn_name: "fnb",
                        args: (&a,).into_val(&e),
                        sub_invokes: &[],
                    },
                    MockAuthInvoke {
                        contract: &contract_b_id,
                        fn_name: "fnb",
                        args: (&a,).into_val(&e),
                        sub_invokes: &[],
                    },
                ],
            },
        }])
        .fna(&contract_b_id, &a);

    assert_eq!(c, 1);

    println!("{:?}", e.auths());
}
