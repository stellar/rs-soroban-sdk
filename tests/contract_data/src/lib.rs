#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: Symbol) {
        e.storage().set(key, val)
    }

    pub fn get(e: Env, key: Symbol) -> Option<Symbol> {
        e.storage().get(key).map(|v| v.unwrap())
    }

    pub fn del(e: Env, key: Symbol) {
        e.storage().remove(key)
    }
}
