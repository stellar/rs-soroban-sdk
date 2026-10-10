//! A data library: a crate that only provides types, for contracts to use.
//!
//! The types are only of the kinds whose conversions to and from `Val` are
//! inlined into the contract using them, so that the contract need not call
//! into this crate at all. A named-field struct is left out because its
//! conversions are not inlined.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntEnum {
    A = 1,
    B = 2,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Enum {
    A(u32),
    B(u32),
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tuple(pub u32, pub u32);

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Inner(pub u32);

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Outer(pub Inner);

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Wrapped(pub u32);
