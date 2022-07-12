#![no_std]
use stellar_contract_sdk::{contractimpl, vec, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env, a: i32, b: i32) -> i32 {
        let vec = vec![&e, a, b];
        let mut s = 0;
        for i in vec.into_iter_unchecked() {
            s += i;
        }
        s
    }
}
