//! A data library providing only a tuple struct nested in another, so that a
//! contract using the outer struct links this crate in only through the spec
//! anchors of the two structs.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub struct Inner(pub u32);

#[contracttype]
pub struct Outer(pub Inner);
