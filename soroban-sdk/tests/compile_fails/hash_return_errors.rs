use soroban_sdk::{contract, contractimpl, crypto::Hash};

// A Hash cannot be returned from a contract function, because its cryptographic
// provenance cannot be guaranteed when decoded by clients.

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn f() -> Hash<32> {
        panic!()
    }
}

fn main() {}
