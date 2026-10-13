use soroban_sdk::{contract, contractimpl, crypto::Hash, Bytes, Env};

// A Hash can't be a contract function return type, because a caller receiving
// it has no guarantee it is a hash.

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn f(env: Env, data: Bytes) -> Hash<32> {
        env.crypto().sha256(&data)
    }
}

fn main() {}
