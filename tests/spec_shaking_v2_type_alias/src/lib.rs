#![no_std]
//! Edge cases for `experimental_spec_shaking_v2`: Rust type aliases used at a
//! contract boundary (issue #1857).
//!
//! `map_type` turns a type *token* into a spec type by its syntactic name, so a
//! type alias becomes a UDT reference under the alias's name (e.g. `ItemAlias`).
//! The spec-shaking marker, however, resolves *through* the alias to the real
//! type, so the surviving spec entry keeps the real name (e.g. `Item`). After
//! shaking, the spec references a UDT name that has no entry — a dangling
//! reference that makes the spec invalid.
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub value: u32,
}

/// A plain Rust type alias for a contract type.
pub type ItemAlias = Item;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// The marker keeps the `Item` entry, but the function spec references UDT
    /// `ItemAlias`, which has no entry.
    pub fn use_udt_alias(_env: Env, _item: ItemAlias) {}
}

#[cfg(test)]
mod test;
