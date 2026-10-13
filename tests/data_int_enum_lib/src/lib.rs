//! A data library providing only an int enum, so that a contract using it links
//! this crate in only through the int enum's spec anchor.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntEnum {
    A = 1,
    B = 2,
}
