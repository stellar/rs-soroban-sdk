//! A data library providing only a tuple struct for a contract to use as the
//! element type of a `Vec`, without converting any element, so that the
//! contract never calls the tuple struct's spec anchor.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub struct VecElem(pub u32);
