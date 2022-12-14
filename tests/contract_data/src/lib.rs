#![no_std]
use soroban_sdk::{contractimpl, storage_map::StorageMap, symbol, Env, Symbol};

pub struct Contract;

const ATOB: u64 = symbol!("atob").to_raw().get_payload();

#[contractimpl]
impl Contract {
    fn mapping(e: &Env) -> StorageMap<ATOB, Symbol, u32> {
        e.storage_map()
    }

    pub fn put(e: Env, key: Symbol, val: u32) {
        Self::mapping(&e).set(key, val)
    }

    pub fn get(e: Env, key: Symbol) -> Option<u32> {
        Self::mapping(&e).get(key).map(|val| val.unwrap())
    }
}
