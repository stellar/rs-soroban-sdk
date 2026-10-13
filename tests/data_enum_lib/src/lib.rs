//! A data library providing only an enum with tuple variants, so that a contract
//! using it links this crate in only through the enum's spec anchor.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub enum Enum {
    A(u32),
    B(u32),
}
