//! The `spec_xdr` fns return spec entries as the SDK emits them, not as they appear in a built
//! contract.
//!
//! This change is largely an internal detail, included for visibility for tool builders who
//! assemble specs from the `spec_xdr` fns that [`contracttype`], [`contracterror`],
//! [`contractevent`], [`contractimpl`], and [`contracttrait`] generate.
//!
//! In v30 the SDK names each type, event, and error in the spec by its full Rust path, such as
//! `::my_contract::State` (see [v30_spec_markers]). When building, the stellar-cli shakes the spec
//! and reduces each name to a short name, such as `State`, numbering any that collide (see
//! [v30_duplicate_names]). The `spec_xdr` fns return the entries from before that processing, so
//! their names, and the names of the types they refer to, are the full paths, and the set of
//! entries is not shaken. They are not representative of the entries in the spec of a built
//! contract.
//!
//! The `soroban-token-spec` and `stellar-asset-spec` crates assemble their specs from these fns.
//! Their `xdr` fns now return the spec with its names reduced, and are no longer `const`. The unprocessed spec is available from their `const` `xdr_unprocessed` fns.
//!
//! ## Example
//!
//! ```
//! use soroban_sdk::{
//!     contracttype,
//!     xdr::{Limits, ReadXdr, ScSpecEntry},
//! };
//!
//! #[contracttype]
//! pub struct State {
//!     pub count: u32,
//! }
//!
//! # fn main() {
//! let entry = ScSpecEntry::from_xdr(State::spec_xdr(), Limits::none()).unwrap();
//! let ScSpecEntry::UdtStructV0(s) = entry else { panic!() };
//! let name = s.name.to_utf8_string_lossy();
//! // The name is the full path of the type, not the `State` in the spec of a built contract.
//! assert!(name.starts_with("::"));
//! assert!(name.ends_with("::State"));
//! # }
//! ```
//!
//! ## Migrating
//!
//! No code changes are required for contracts. Tools that assemble a spec from the `spec_xdr` fns
//! and publish it, or give it to other tools, need to reduce its names with
//! `soroban_spec::reduce::reduce`, as `stellar contract build` does. Tools that need the entries in
//! a built contract need to read the spec from the built Wasm. Callers of the `xdr` fns of
//! `soroban-token-spec` and `stellar-asset-spec` that need a `const` spec need to use
//! `xdr_unprocessed`.
//!
//! [v30_spec_markers]: super::v30_spec_markers
//! [v30_duplicate_names]: super::v30_duplicate_names
//! [`contracttype`]: crate::contracttype
//! [`contracterror`]: crate::contracterror
//! [`contractevent`]: crate::contractevent
//! [`contractimpl`]: crate::contractimpl
//! [`contracttrait`]: crate::contracttrait
