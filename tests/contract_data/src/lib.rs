#![no_std]
use soroban_sdk::{contractimpl, Env, StorageMap, Symbol};

pub struct Contract;

static MAPPING: StorageMap<Symbol, u32> = StorageMap::new("asdf");

#[contractimpl]
impl Contract {
    pub fn put(e: Env, key: Symbol, val: u32) {
        MAPPING.set(&e, key, val)
    }

    pub fn get(e: Env, key: Symbol) -> Option<u32> {
        MAPPING.get(&e, key).map(|val| val.unwrap())
    }
}
