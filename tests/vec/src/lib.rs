#![no_std]
use soroban_sdk::{contractimpl, Env, Vec};

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
