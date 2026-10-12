//! Contracts must be built with `stellar contract build` from a stellar-cli of equal or greater
//! major version.
//!
//! The soroban-sdk and the stellar-cli work together to produce a contract's spec. The SDK writes
//! a spec entry for every type, event, and error the contract defines, and the stellar-cli removes
//! the entries the contract does not use and shortens the names of the types, as described in
//! [v30_duplicate_names] and [v30_spec_markers]. A spec is only correct once the stellar-cli has
//! done both, and the way it does them changes with the major version of the SDK.
//!
//! ## Build Requirements
//!
//! Contracts using soroban-sdk v30 must be built with `stellar contract build` from `stellar-cli`
//! v30.0.0 or newer. In general, a contract must be built by a stellar-cli whose major version is
//! equal to or greater than the major version of the soroban-sdk it uses. Building a contract for
//! wasm with an older stellar-cli, or with any other build system, produces a build error:
//!
//! ```text
//! error: soroban-sdk requires stellar-cli v30.0.0+ to build a contract
//! ```
//!
//! The check only fires for wasm targets, so native builds and unit tests are unaffected.
//!
//! ## Migrating
//!
//! Upgrade the stellar-cli used to build the contract, locally and in CI, to v30.0.0 or newer, and
//! build the contract with `stellar contract build`.
//!
//! [v30_duplicate_names]: super::v30_duplicate_names
//! [v30_spec_markers]: super::v30_spec_markers
