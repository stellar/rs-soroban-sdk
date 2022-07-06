#![no_std]
use stellar_contract_sdk::{contractfn, Env, Binary, FixedLengthBinary};

#[contractfn]
pub fn bin_new(e: Env, len: u32) -> Binary {
    let buf: [u8; 4] = [0,1,2,3];
    e.binary_new_from_linear_memory(buf.as_ptr() as u32, len)
}

#[contractfn]
pub fn from_guest(e: Env, b: Binary, offset_ho: u32, offset_lm: u32, len: u32) -> Binary {
    let buf: [u8; 4] = [0,1,2,3];
    if offset_lm+len > buf.len() as u32 {
        panic!("index out of array bound")
    }
    let pos_lm: u32 = unsafe { buf.as_ptr().add(offset_lm as usize) as u32};
    e.binary_copy_from_linear_memory(b, offset_ho, pos_lm, len)
}

#[contractfn]
pub fn to_guest(e: Env, b: Binary, offset_ho: u32, offset_lm: u32, len: u32){
    let buf: [u8; 4] = [0; 4];    
    if offset_lm+len > buf.len() as u32 {
        panic!("index out of array bound")
    }        
    let pos_lm: u32 = unsafe { buf.as_ptr().add(offset_lm as usize) as u32};
    e.binary_copy_to_linear_memory(b.clone(), offset_ho, pos_lm, len);
    for idx in pos_lm..buf.len() as u32 {
        assert_eq!(buf[idx as usize], b.get(offset_ho + idx));
    }
}