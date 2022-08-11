#![no_std]
use soroban_sdk::{contractimpl, BigInt};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: BigInt, b: BigInt) -> BigInt {
        a + b
    }
}
