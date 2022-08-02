#![no_std]
use soroban_sdk::{contractimpl, Bytes, BytesN, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Note that anyone can create a contract here with any salt, so a users call to
    // this could be frontrun and the same salt taken.
    pub fn create(e: Env, c: Bytes, s: BytesN<32>) {
        e.create_contract_from_contract(c, s);
    }
}
