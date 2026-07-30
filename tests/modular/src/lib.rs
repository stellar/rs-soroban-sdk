#![no_std]
use soroban_sdk::{contract, contractimpl};

mod feat1;
mod feat2;
mod test;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn zero() -> u32 {
        0
    }
}
