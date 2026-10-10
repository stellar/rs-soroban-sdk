#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype};
use test_spec_lib_no_lto::{Enum, IntEnum, Outer, Tuple, Wrapped};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Wrapper(pub Wrapped);

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn int_enum(v: IntEnum) -> IntEnum {
        v
    }

    pub fn enum_(v: Enum) -> Enum {
        v
    }

    pub fn tuple(v: Tuple) -> Tuple {
        v
    }

    pub fn outer(v: Outer) -> u32 {
        v.0 .0
    }

    pub fn wrapper(v: Wrapper) -> u32 {
        v.0 .0
    }
}

#[cfg(test)]
mod test;
