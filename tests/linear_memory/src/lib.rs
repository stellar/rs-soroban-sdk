#![no_std]
use stellar_contract_sdk::{contractimpl, Binary, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn bin_new(e: Env, len: u32) -> Binary {
        let buf: [u8; 4] = [0, 1, 2, 3];
        e.binary_new_from_linear_memory(buf.as_ptr() as u32, len)
    }

    pub fn from_guest(e: Env, b: Binary, b_pos: u32, buf_off: u32, len: u32) -> Binary {
        let buf: [u8; 4] = [0, 1, 2, 3];
        assert!(buf_off + len <= buf.len() as u32);
        let lm_pos: u32 = unsafe { buf.as_ptr().add(buf_off as usize) as u32 };
        e.binary_copy_from_linear_memory(b, b_pos, lm_pos, len)
    }

    pub fn to_guest(e: Env, b: Binary, b_pos: u32, buf_off: u32, len: u32) {
        let buf: [u8; 4] = [0; 4];
        assert!(buf_off + len <= buf.len() as u32);
        let lm_pos: u32 = unsafe { buf.as_ptr().add(buf_off as usize) as u32 };
        e.binary_copy_to_linear_memory(b.clone(), b_pos, lm_pos, len);
        for idx in lm_pos..buf.len() as u32 {
            assert!(buf[idx as usize] == b.get_unchecked((b_pos + idx) as usize));
        }
    }
}
