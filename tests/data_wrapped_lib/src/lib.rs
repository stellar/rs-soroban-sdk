//! A data library providing only a tuple struct for a contract to wrap in a type
//! of its own, so that the contract links this crate in only through the spec
//! anchor of the wrapped struct.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Wrapped(pub u32);
