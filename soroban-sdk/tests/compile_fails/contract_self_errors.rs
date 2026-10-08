use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn by_value(self) {}
}

#[contract]
pub struct Contract2;

#[contractimpl]
impl Contract2 {
    pub fn by_mut_ref(&mut self) {}
}

fn main() {}
