//! A data library providing only a tuple struct for a contract to use as the
//! element type of a `Vec` field of a type of its own, without converting any
//! element, so that the contract links this crate in only through the spec
//! anchor of the field.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub struct FieldElem(pub u32);
