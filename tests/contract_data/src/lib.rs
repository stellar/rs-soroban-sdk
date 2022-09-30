#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: Symbol) {
        e.data().set(key, val)
    }

    pub fn del(e: Env, key: Symbol) {
        e.data().remove(key)
    }
}
