#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Env};

/// Referenced by `hello`, so its spec entry must survive.
#[contracttype]
pub struct Used {
    pub n: u32,
}

/// Never referenced by anything the contract exports, so its spec entry must
/// be shaken out.
#[contracttype]
pub struct Unused {
    pub n: u32,
}

/// Published by `hello`, so its spec entry must survive.
#[contractevent]
pub struct Live {
    pub n: u32,
}

/// Never published, so its spec entry must be shaken out.
#[contractevent]
pub struct Dead {
    pub n: u32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, used: Used) -> u32 {
        Live { n: used.n }.publish(&env);
        used.n
    }
}
