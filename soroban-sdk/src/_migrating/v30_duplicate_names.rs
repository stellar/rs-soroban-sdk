//! Duplicate type, event, and error names in specs are given unique names.
//!
//! Before v30 a contract's spec named each type, event, and error by its Rust name alone. Two types
//! with the same name defined in different modules, or in different crates, produced two spec
//! entries with the same name, and a reader of the spec, such as a client generator, could not
//! tell which entry a function referred to.
//!
//! In v30 the SDK names each of them in the spec by its full Rust path, such as
//! `::my_contract::a::State`, and `stellar contract build` reduces each name back to its last
//! segment, such as `State`. Where two names would reduce to the same name, the one whose full
//! path sorts first keeps it, and the others are given unique names by adding a number to the end,
//! such as `State2`. Every reference to a type in the spec is updated to match, so functions,
//! events, and other types refer to the right entry. A name that would collide with the name of a
//! soroban-sdk type, or with an item that client generators add, such as `Client`, is numbered in
//! the same way.
//!
//! `contractimport!` and the stellar-cli's generated bindings reduce the names of an imported spec
//! in the same way.
//!
//! ## Example
//!
//! ```
//! #![no_std]
//! use soroban_sdk::{contract, contractimpl, Env};
//!
//! mod a {
//!     use soroban_sdk::contracttype;
//!
//!     #[contracttype]
//!     pub struct State {
//!         pub count: u32,
//!     }
//! }
//!
//! mod b {
//!     use soroban_sdk::contracttype;
//!
//!     #[contracttype]
//!     pub struct State {
//!         pub enabled: bool,
//!     }
//! }
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn get(_env: Env, a: a::State, b: b::State) -> (a::State, b::State) {
//!         (a, b)
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! Built with `stellar contract build`, the spec of the contract above contains a struct named
//! `State` for `a::State` and a struct named `State2` for `b::State`, and the `get` function
//! refers to each by its new name.
//!
//! ## Migrating
//!
//! No code changes are required. When upgrading the SDK on an existing contract that has duplicate
//! names, expect the contract spec to change, as all but one of the entries sharing a name are
//! renamed. Clients generated from the spec of such a contract will see the new names. To keep a
//! name unchanged, give each type, event, and error in the contract a unique name.
