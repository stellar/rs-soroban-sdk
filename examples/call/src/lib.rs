#![no_std]
use stellar_contract_sdk::{Env, EnvTrait, RawVal, Symbol};

#[no_mangle]
pub fn del_call(e: Env, x: RawVal) -> RawVal {
    let mut obj = e.binary_new();
    for _i in 0..32 {
        obj = e.binary_push(obj, 1_u32.into());
    }
    let fun = Symbol::from_str("vec_err");
    let args = e.vec_new();
    let args = e.vec_push(args, x);
    e.call(obj, fun.into(), args)
}