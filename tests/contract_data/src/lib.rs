#![no_std]
use stellar_contract_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: Symbol) {
        e.contract().data().set(key, val)
    }

    pub fn del(e: Env, key: Symbol) {
        e.contract().data().remove(key)
    }
}
