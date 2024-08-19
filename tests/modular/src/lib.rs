#![no_std]
use soroban_sdk::{contract, contractimpl};

mod feat1;
mod feat2;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn base() -> u32 {
        0
    }
}
