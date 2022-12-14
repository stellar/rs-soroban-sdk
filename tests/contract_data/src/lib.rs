#![no_std]
use soroban_sdk::{contractimpl, symbol, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: u32) {
        e.storage_map::<Symbol, _, _>(symbol!("atob")).set(key, val)
    }

    pub fn get(e: Env, key: Symbol) -> Option<u32> {
        e.storage_map::<Symbol, _, _>(symbol!("atob"))
            .get(key)
            .map(|val| val.unwrap())
    }
}
