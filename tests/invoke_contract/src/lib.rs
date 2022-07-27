#![no_std]
use stellar_contract_sdk::{contractimpl, Env, IntoVal, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn delegate(e: Env, val: u32) {
        let buff = [1u8; 32];
        let cid = e.binary_new_from_linear_memory(buff.as_ptr() as u32, 32);
        let fun = Symbol::from_str("vec_err");
        let args = Vec::from_array(&e, [val.into_env_val(&e); 1]);
        e.invoke_contract::<Vec<u32>>(cid, fun.into(), args);
    }
}
