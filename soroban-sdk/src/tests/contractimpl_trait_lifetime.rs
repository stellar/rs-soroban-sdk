use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, Env};

// A trait with a lifetime parameter, implemented for any lifetime. The lifetime
// is declared by the impl, and the functions generated to export the trait's
// functions are outside of the impl.

pub trait Scale<'a> {
    fn scale(env: Env, x: &'a u32) -> u32;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl<'a> Scale<'a> for Contract {
    fn scale(_env: Env, x: &'a u32) -> u32 {
        x * 2
    }
}

#[test]
fn test_trait_with_lifetime() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());
    let client = ContractClient::new(&e, &contract_id);

    assert_eq!(client.scale(&3), 6);
}
