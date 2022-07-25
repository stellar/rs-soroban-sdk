#![no_std]
use stellar_contract_sdk::{contractimpl, Binary, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Note that anyone can create a contract here with any salt, so a users call to
    // this could be frontrun and the same salt taken.
    pub fn create(e: Env, c: Binary, s: Binary) {
        e.create_contract_from_contract(c, s);
    }
}
