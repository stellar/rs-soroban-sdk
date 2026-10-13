use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

// A contract impl written in a user's macro, with the argument's type passed
// in to the macro, so that the type carries different hygiene to the impl.
macro_rules! contract_fn {
    ($ty:ident) => {
        #[contractimpl]
        impl Contract {
            pub fn add(a: $ty, b: $ty) -> $ty {
                a + b
            }
        }
    };
}

contract_fn!(u32);

#[test]
fn test_functional() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());

    let c = ContractClient::new(&e, &contract_id).add(&10, &12);
    assert_eq!(c, 22);
}
