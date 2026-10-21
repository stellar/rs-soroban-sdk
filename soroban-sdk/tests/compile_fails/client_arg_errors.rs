use soroban_sdk::{contract, contractimpl, Env};

// A client function only accepts values of its argument types, or references to
// them, and not values of other types that convert into them.

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn f(_env: Env, a: u64) -> u64 {
        a
    }
}

fn main() {
    let env = Env::default();
    let client = ContractClient::new(&env, &env.register(Contract, ()));
    client.f(1u32);
    client.f(&1u32);
}
