#![no_std]
use stellar_contract_sdk::{Env, Symbol};

#[no_mangle]
pub fn put(e: Env, key: Symbol, val: Symbol) {
    e.put_contract_data(key, val)
}

#[no_mangle]
pub fn del(e: Env, key: Symbol) {
    e.del_contract_data(key)
}
