#![no_std]
use soroban_sdk::{contractimpl, contracttype, symbol, Env};

pub struct Contract;

#[derive(Default)]
#[contracttype]
pub struct Mem {
    counter: u32,
    balance: i128,
}

#[contractimpl]
impl Contract {
    fn get(env: &Env) -> Mem {
        match env.storage().get(symbol!("MEM")) {
            Some(Ok(mem)) => mem,
            _ => Mem::default(),
        }
    }

    pub fn set(env: Env, b: i128) {
        let mem = Self::get(&env);
        env.storage().set(
            symbol!("MEM"),
            Mem {
                counter: mem.counter + 1,
                balance: b,
            },
        )
    }

    pub fn counter(env: Env) -> u32 {
        let mem = Self::get(&env);
        mem.counter
    }

    pub fn balance(env: Env) -> i128 {
        let mem = Self::get(&env);
        mem.balance
    }
}
