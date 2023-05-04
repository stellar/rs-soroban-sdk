//! Demonstrates how an authorization is not required to be consumed by an
//! invocation.
//!
//! It's possible to have dangling authorizations that may be optionally
//! consumed, or never consumed.
//!
//! Because authorizations cannot always be grouped, it's not possible to group
//! potentially related optional auths together to ensure that their nonce is
//! consumed. This means it's possible for an auth to be exposed on chain that
//! could be executed by someone in isolation, even after the transaction
//! succeeds.

use crate as soroban_sdk;

use soroban_sdk::{
    contractimpl,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    xdr, Address, Env, IntoVal,
};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Address, x: i32, y: i32) -> i32 {
        a.require_auth();
        x + y
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let a = Address::random(&e);

    let c = client
        .mock_auths(&[
            MockAuth {
                address: &a,
                nonce: 0,
                invoke: &MockAuthInvoke {
                    contract: &contract_id,
                    fn_name: "add",
                    args: (&a, 10, 12).into_val(&e),
                    sub_invokes: &[],
                },
            },
            MockAuth {
                address: &a,
                nonce: 1,
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

    assert_eq!(
        e.to_snapshot().ledger_entries[0].1.data,
        xdr::LedgerEntryData::ContractData(xdr::ContractDataEntry {
            contract_id: xdr::Hash(contract_id.to_array()),
            key: xdr::ScVal::LedgerKeyNonce(xdr::ScNonceKey {
                nonce_address: xdr::ScAddress::Contract(xdr::Hash(
                    a.contract_id().unwrap().to_array()
                )),
            },),
            // The nonce is 1 because the second auth is never consumed.
            val: xdr::ScVal::U64(1,),
        },)
    );
}
