//! A data library: a crate that only provides types, errors and events, for
//! contracts to use.
//!
//! The types are only of the kinds whose conversions to and from `Val` are
//! inlined into the contract using them, so that the contract need not call
//! into this crate at all. A named-field struct is left out because its
//! conversions are not inlined.
//!
//! The types, the error and the event are each used by a separate contract,
//! test_data_contract, test_data_error_contract and test_data_event_contract,
//! because a contract that calls into this crate for one of them links in the
//! spec entries of all of them.
#![no_std]
use soroban_sdk::{contracterror, contractevent, contracttype};

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

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    A = 1,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub v: u32,
}
