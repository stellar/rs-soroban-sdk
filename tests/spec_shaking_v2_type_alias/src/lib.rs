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

/// A Rust type alias for a primitive.
pub type Amount = i128;

/// A Rust type alias for a container type.
pub type Items = soroban_sdk::Vec<Item>;

/// A contract type whose field is declared with a type alias.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wrapper {
    pub item: ItemAlias,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// The marker keeps the `Item` entry, but the function spec references UDT
    /// `ItemAlias`, which has no entry.
    pub fn use_udt_alias(_env: Env, _item: ItemAlias) {}

    /// `map_type` treats the alias token `Amount` as a UDT reference, but no
    /// UDT entry named `Amount` exists (the real type is the primitive `i128`),
    /// so the reference dangles.
    pub fn use_primitive_alias(_env: Env, _amount: Amount) {}

    /// `map_type` treats the alias token `Items` as a UDT reference, but the
    /// real type is `Vec<Item>` (a container, not a UDT). The element type
    /// `Item` is kept via the marker, but the `Items` reference dangles.
    pub fn use_container_alias(_env: Env, _items: Items) {}

    /// The struct `Wrapper` is kept, but its field references UDT `ItemAlias`,
    /// which has no entry — the dangling reference lives inside a type spec,
    /// not just a function spec.
    pub fn use_field_alias(_env: Env, _w: Wrapper) {}
}

#[cfg(test)]
mod test;
