#![no_std]
use soroban_sdk::{contract, contractimpl, U256};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn run(a: U256, b: U256) {
        if a < b {
            panic!("unexpected")
        }
    }
}
