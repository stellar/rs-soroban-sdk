#![no_std]
use stellar_contract_sdk::{contract, contractimpl, BigInt};

contract!();

pub struct Contract;

#[contractimpl(tests_if = "testutils")]
impl Contract {
    pub fn add(a: BigInt, b: BigInt) -> BigInt {
        a + b
    }
}
