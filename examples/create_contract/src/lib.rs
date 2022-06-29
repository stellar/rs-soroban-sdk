#![no_std]
use stellar_contract_sdk::{contractfn, Binary, Env};

// Note that anyone can create a contract here with any salt, so a users call to
// this could be frontrun and the same salt taken.
#[contractfn]
pub fn create(e: Env, c: Binary, s: Binary) {
    e.create_contract_using_parent_id(c, s)
}
