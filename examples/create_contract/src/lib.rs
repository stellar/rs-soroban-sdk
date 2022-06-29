#![no_std]
use stellar_contract_sdk::{contractfn, Env, Object};

// Note that anyone can create a contract here with any salt, so a users call to
// this could be frontrun and the same salt taken.
#[contractfn]
pub fn create(e: Env, c: Object, s: Object) {
    e.create_contract_using_parent_id(
        c.in_env(&e).try_into().unwrap(),
        s.in_env(&e).try_into().unwrap(),
    )
}
