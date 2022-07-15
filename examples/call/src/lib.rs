#![no_std]
use stellar_contract_sdk::{contractimpl, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn delegate(e: Env, val: u32) -> Vec<u32> {
        let mut obj = e.binary_new();
        for _i in 0..32 {
            obj = e.binary_push(obj, 1_u32.into());
        }
        let fun = Symbol::from_str("vec_err");
        let args = e.vec_new();
        let args = e.vec_push(args, x);
        e.call(obj, fun.into(), args)
    }
}
