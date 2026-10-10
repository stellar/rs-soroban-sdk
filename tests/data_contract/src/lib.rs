#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype};
use test_data_lib::{Enum, IntEnum, Outer, Tuple, Wrapped};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Wrapper(pub Wrapped);

impl Wrapper {
    // An inherent fn with the same name as the SpecAnchor trait's fn, which the
    // generated conversions call, must not clash with it.
    pub fn spec_anchor() -> u32 {
        0
    }
}

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
