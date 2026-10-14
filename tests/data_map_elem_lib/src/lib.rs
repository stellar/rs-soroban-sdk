//! A data library providing only tuple structs for a contract to use as the key
//! and value types of a `Map`, without converting any key or value, so that the
//! contract links this crate in only through the `Map`'s spec anchor.
#![no_std]
use soroban_sdk::contracttype;

#[contracttype]
pub struct MapKey(pub u32);

#[contracttype]
pub struct MapVal(pub u32);
