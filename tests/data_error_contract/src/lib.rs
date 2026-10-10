#![no_std]
use soroban_sdk::{contract, contractimpl};
use test_data_lib::Error;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn error(fail: bool) -> Result<u32, Error> {
        if fail {
            Err(Error::A)
        } else {
            Ok(1)
        }
    }
}
