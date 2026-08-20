use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct C;

// `contracttrait` is meaningless on an inherent impl, and silently ignoring it
// would produce a contract with an empty spec.
#[contractimpl(contracttrait)]
impl C {
    pub fn f(env: Env) -> u32 {
        let _ = env;
        1
    }
}

fn main() {}
