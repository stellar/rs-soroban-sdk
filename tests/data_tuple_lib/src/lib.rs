//! A data library providing only a tuple struct, so that a contract using it
//! links this crate in only through the tuple struct's spec anchor.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub struct Tuple(pub u32, pub u32);
