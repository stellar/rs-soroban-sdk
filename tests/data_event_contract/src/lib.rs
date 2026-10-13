#![no_std]
use soroban_sdk::{contract, contractimpl, Env};
use test_data_lib::Event;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn event(env: Env, v: u32) {
        Event { v }.publish(&env);
    }
}
