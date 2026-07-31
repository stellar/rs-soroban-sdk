use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contractimpl, contracttype, Env};

mod inner {
    use crate::{self as soroban_sdk};
    use soroban_sdk::contracttype;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[contracttype]
    pub struct Inner {
        pub a: i32,
        pub b: i32,
    }
}

use inner::Inner as Renamed;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Outer {
    pub inner: Renamed,
    pub c: i32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Outer, b: Outer) -> (Outer, Outer) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());

    let a = Outer {
        inner: Renamed { a: 5, b: 7 },
        c: 1,
    };
    let b = Outer {
        inner: Renamed { a: 10, b: 14 },
        c: 2,
    };
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}
