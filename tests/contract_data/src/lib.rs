#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: Symbol) {
        e.storage().persistent().set(&key, &val, None)
    }

    pub fn get(e: Env, key: Symbol) -> Option<Symbol> {
        e.storage().persistent().get(&key)
    }

    pub fn del(e: Env, key: Symbol) {
        e.storage().persistent().remove(&key)
    }
}
