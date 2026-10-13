//! A data library: a crate that only provides errors and events, for contracts
//! to use.
//!
//! The error, the event and the event with a lifetime are each used by a
//! separate contract, test_data_error_contract, test_data_event_contract and
//! test_data_ref_event_contract, because a contract that calls into this crate
//! for one of them links in the spec entries of all of them.
//!
//! The types are each in a data library of their own, used by
//! test_data_contract, for the same reason. They are only of the kinds whose
//! conversions to and from `Val` are inlined into the contract using them, so
//! that the contract need not call into their crates at all. A named-field
//! struct is left out because its conversions are not inlined.
#![no_std]
use soroban_sdk::{contracterror, contractevent};

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

// An event with a lifetime, so that its conversions and its spec anchor are
// generic over the lifetime.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefEvent<'a> {
    pub v: &'a u32,
}
