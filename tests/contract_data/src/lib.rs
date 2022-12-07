#![no_std]
use soroban_sdk::{contractimpl, storage_map::StorageMap, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    fn mapping(e: &Env) -> StorageMap<Symbol, u32> {
        e.storage_map()
    }

    pub fn put(e: Env, key: Symbol, val: u32) {
        Self::mapping(&e).set(key, val)
    }

    pub fn get(e: Env, key: Symbol) -> Option<u32> {
        Self::mapping(&e).get(key).map(|val| val.unwrap())
    }
}
