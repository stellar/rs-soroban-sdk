#![no_std]
use stellar_contract_sdk::{Env, EnvTrait, RawVal};

#[no_mangle]
pub fn vec_err(e: Env, x: RawVal) -> RawVal {
    let v1 = e.vec_new();
    let v2 = e.vec_insert(v1, 5_u32.into(), x); // out of bound
    v2.to_raw()
}