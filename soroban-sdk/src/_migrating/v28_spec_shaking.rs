//! Spec shaking is always on, and the `export` argument has been removed.
//!
//! A contract's spec (the `contractspecv0` custom section in the Wasm binary) contains entries for
//! every function, type, and event defined by the contract. When types or events are defined but
//! not actually used at a contract boundary (parameters, return values, error returns, or event
//! publishes), their spec entries unnecessarily increase the size of the Wasm file. Spec shaking
//! removes them.
//!
//! Prior to v28 the SDK decided what to put in the spec from Rust visibility and the `export`
//! argument: `pub` types and all events got an entry, non-`pub` types did not, and `export` could
//! override either way. In soroban-sdk v27 the `experimental_spec_shaking_v2` feature offered an
//! alternative, where the SDK emits an entry for everything and post-build tooling within the
//! stellar-cli removes what the contract does not actually use. That feature is now the only
//! behaviour, and the feature flag has been removed.
//!
//! ## How It Works
//!
//! The SDK embeds 14-byte **markers** in the Wasm data section for each type and event. A marker
//! consists of a `SpEcV1` magic prefix followed by 8 bytes of a SHA-256 hash of the spec entry's
//! XDR.
//!
//! Markers are placed inside functions that are only called when the type is actually used:
//! - **Function parameters**: marker is triggered when deserializing the input.
//! - **Function return values**: marker is triggered when serializing the output.
//! - **Error returns**: marker is triggered via `Result<T, E>` serialization.
//! - **Error panics**: marker is triggered by [`panic_with_error!`] and
//!   [`Env::panic_with_error`], so an error type reaching the boundary by panic is marked even
//!   when it is never returned in a `Result`.
//! - **Event publishes**: marker is triggered inside the `publish()` call.
//! - **Nested types**: a type's marker function calls the marker functions of its field types, so
//!   nested types are transitively marked.
//! - **Container types**: `Vec<T>`, `Map<K, V>`, `Option<T>`, and `Result<T, E>` propagate markers
//!   to their inner types.
//!
//! The Rust compiler's dead code elimination (DCE) removes markers for types that are never used,
//! while keeping markers for types that are. Post-build tools then scan the Wasm data section for
//! `SpEcV1` markers, match them against spec entries, and strip any entry without a corresponding
//! marker.
//!
//! ## Build Requirements
//!
//! Because the SDK relies on the stellar-cli build system to do that final strip, contracts must be
//! built with `stellar contract build` from `stellar-cli` v25.2.0 or newer. Building a contract for
//! wasm with any other build system produces a build error:
//!
//! ```text
//! error: soroban-sdk requires stellar-cli v25.2.0+ to build a contract
//! ```
//!
//! The check only fires for wasm targets, so native builds and unit tests are unaffected.
//!
//! ## Changed Behaviour
//!
//! ### [`contracttype`] and [`contracterror`]
//!
//! Spec entries and markers are generated for all types regardless of Rust visibility, and
//! post-build tooling removes the entries that are not reachable from the contract interface.
//! Previously entries were only generated for `pub` types.
//!
//! ### [`contractevent`]
//!
//! Markers are embedded for all published events, so post-build tooling strips spec entries for
//! events that are never published at a contract boundary.
//!
//! ### [`contractimport!`]
//!
//! Imported types produce spec entries and markers in the importing contract, just like locally
//! defined types. Previously they produced no entries in the importing contract's spec, and callers
//! had to look at the imported contract's spec to find the type definitions.
//!
//! The importing contract's spec is now self-contained: it includes the type definitions for all
//! types used at its own boundary, regardless of where those types were originally defined.
//! Imported types used in the contract's function signatures or events keep their entries, and
//! imported types not used at any boundary have their entries stripped. A contract importing a
//! large interface therefore only carries spec entries for the types it actually uses.
//!
//! ## Migrating
//!
//! Build contracts with `stellar contract build` from `stellar-cli` v25.2.0 or newer.
//!
//! If the `experimental_spec_shaking_v2` feature is enabled, remove it from the `soroban-sdk`
//! dependency's feature list in `Cargo.toml`. The behaviour it enabled is now the default.
//!
//! Remove the `export` argument from [`contracttype`], [`contracterror`], and [`contractevent`]
//! annotations. It was deprecated in v27, and is now rejected:
//!
//! ```text
//! error: `export` is no longer supported, and contract spec export is now determined by
//! reachability from the contract boundary (functions, events, errors)
//! ```
//!
//! A type that was hidden with `export = false` is filtered from the spec automatically when it is
//! unused, and a type that was forced in with `export = true` is included whenever it is reachable.
//! For example, a type used only inside a contract — never at a function boundary — no longer
//! compiles when it asks to be hidden:
//!
//! ```compile_fail
//! use soroban_sdk::contracttype;
//!
//! #[contracttype(export = false)] // ❌ export is no longer supported
//! pub struct InternalState {
//!     pub counter: u32,
//! }
//! # fn main() {}
//! ```
//!
//! Drop the argument. Because `InternalState` is never reachable from a public contract function,
//! post-build tooling strips its spec entry automatically:
//!
//! ```
//! use soroban_sdk::contracttype;
//!
//! #[contracttype] // 👈 👀 no export argument; reachability determines the final spec
//! pub struct InternalState {
//!     pub counter: u32,
//! }
//! # fn main() {}
//! ```
//!
//! [`contracttype`]: crate::contracttype
//! [`contracterror`]: crate::contracterror
//! [`contractevent`]: crate::contractevent
//! [`contractimport!`]: crate::contractimport
//! [`panic_with_error!`]: crate::panic_with_error
//! [`Env::panic_with_error`]: crate::Env::panic_with_error
