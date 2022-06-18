#![no_std]
use stellar_contract_sdk::{contractfn, Env, Symbol};

#[contractfn]
pub fn put(e: Env, key: Symbol, val: Symbol) {
    e.put_contract_data(key, val)
}

#[contractfn]
pub fn del(e: Env, key: Symbol) {
    e.del_contract_data(key)
}
