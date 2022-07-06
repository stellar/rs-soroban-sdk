#![no_std]
use stellar_contract_sdk::{contractfn, Env, Binary, FixedLengthBinary};

#[contractfn]
pub fn bin_new(e: Env, l: u32) -> Binary {
    let buf: [u8; 4] = [0,1,2,3];
    e.binary_new_from_linear_memory(buf.as_ptr() as u32, l)
}

#[contractfn]
pub fn from_guest(e: Env, b: Binary, i: u32, j: u32, l: u32) -> Binary {
    let buf: [u8; 4] = [0,1,2,3];
    if j+l > buf.len() as u32 {
        panic!("index out of array bound")
    }
    let j: u32 = unsafe { buf.as_ptr().add(j as usize) as u32};
    e.binary_copy_from_linear_memory(b, i, j, l)
}

#[contractfn]
pub fn to_guest(e: Env, b: Binary, i: u32, j: u32, l: u32){
    let buf: [u8; 4] = [0; 4];    
    if j+l > buf.len() as u32 {
        panic!("index out of array bound")
    }        
    let j: u32 = unsafe { buf.as_ptr().add(j as usize) as u32};
    e.binary_copy_to_linear_memory(b.clone(), i, j, l);
    for idx in j..buf.len() as u32 {
        assert_eq!(buf[idx as usize], b.get(i + idx));
    }
}