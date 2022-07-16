#![no_std]
use stellar_contract_sdk::{contract, contractimpl, Env, Vec};

contract!();

pub struct Contract;

#[contractimpl]
impl Contract {
    /// This function will generate an out-of-bound error on any inputs passed in
    pub fn vec_err(e: Env, val: u32) -> Vec<u32> {
        let mut v = Vec::new(&e);
        v.insert(1_u32, val);
        v
    }
}
