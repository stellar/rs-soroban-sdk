//! A data library providing only a tuple struct for another data library to use
//! as the field of its own type, so that a contract using that type only as the
//! element type of a `Vec` links this crate in only through the spec anchor of
//! that type.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub struct ElemInner(pub u32);
