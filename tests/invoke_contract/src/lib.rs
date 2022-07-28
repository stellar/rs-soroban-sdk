#![no_std]
use soroban_sdk::{contractimpl, Env, FixedBinary, IntoVal, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn delegate(e: Env, val: u32) {
        let buff = [1u8; 32];
        let cid = e.binary_new_from_linear_memory(buff.as_ptr() as u32, 32);
        let cid: FixedBinary<32> = cid.try_into().unwrap();
        let fun = Symbol::from_str("vec_err");
        let args = Vec::from_array(&e, [val.into_env_val(&e); 1]);
        let _: Vec<u32> = e.invoke_contract(cid, fun, args);
    }
}
