use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contractimpl, xdr, Address, Env, TryFromVal};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store(env: Env, k: i32, v: i32) {
        env.storage().persistent().set(&k, &v)
    }
    pub fn get(env: Env, k: i32) -> i32 {
        env.storage().persistent().get(&k).unwrap()
    }
}

#[test]
fn test() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let contract_id_xdr = xdr::ScAddress::try_from(&contract_id).unwrap();
    let client = ContractClient::new(&e, &contract_id);

    client.store(&2, &4);
    assert_eq!(client.get(&2), 4);

    let snapshot = e.to_ledger_snapshot();

    let e = Env::from_ledger_snapshot(snapshot);
    let contract_id = Address::try_from_val(&e, &contract_id_xdr).unwrap();
    e.register_contract(&contract_id, Contract);
    let client = ContractClient::new(&e, &contract_id);

    assert_eq!(client.get(&2), 4);
}
