use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct InnerUdt {
    pub c: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt {
    pub a: i32,
    pub b: Option<i32>,
    pub c: Option<InnerUdt>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());

    let a = Udt {
        a: 5,
        b: None,
        c: None,
    };
    let b = Udt {
        a: 10,
        b: Some(1),
        c: Some(InnerUdt { c: 2 }),
    };
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}
