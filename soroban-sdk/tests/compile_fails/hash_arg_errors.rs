use soroban_sdk::{contract, contractimpl, crypto::Hash};

// A Hash is only a contract function argument as the signature payload of
// __check_auth, because only there does the host guarantee it is a hash.

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn f(_hash: Hash<32>) {}
}

fn main() {}
