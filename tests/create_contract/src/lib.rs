#![no_std]
use soroban_sdk::{contractimpl, Bytes, BytesN, CurrentNamespace, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    // Note that anyone can create a contract here with any salt, so a users call to
    // this could be frontrun and the same salt taken.
    pub fn create(e: Env, c: Bytes, s: BytesN<32>) {
        let _contract_id = e.contract_id(CurrentNamespace, &s);
        let _contract_id = e.deployer(CurrentNamespace).deploy_wasm(&s, c);
    }
}
