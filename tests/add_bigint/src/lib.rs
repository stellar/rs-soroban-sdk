#![no_std]
use stellar_contract_sdk::{contractimpl, BigInt};

pub struct Contract;

#[contractimpl(tests_if = "testutils")]
impl Contract {
    pub fn add(a: BigInt, b: BigInt) -> BigInt {
        a + b
    }
}
