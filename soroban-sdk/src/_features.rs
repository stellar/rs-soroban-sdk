//! # Features
//!
//! The SDK provides several Cargo features that enable additional functionality.
//!
//! ## `testutils`
//!
//! Enables test utilities for writing tests that interact with contracts.
//! Required for using [`Env::default()`] in tests, generating test addresses,
//! and other test helpers. Only available for non-Wasm targets.
//!
//! ## `alloc`
//!
//! Enables the [`alloc`][crate::alloc] module, providing access to the global
//! allocator for use in contracts.
//!
//! ## `hazmat-crypto`
//!
//! Exposes low-level cryptographic primitives (e.g.
//! [`CryptoHazmat`][crate::crypto::CryptoHazmat]) that are easy to misuse.
//! Use with care.
//!
//! ## `hazmat-address`
//!
//! Exposes low-level address primitives (e.g.
//! [`Address::to_payload`][crate::Address::to_payload],
//! [`Address::from_payload`][crate::Address::from_payload]) that are easy to
//! misuse. Use with care.
//!
//! ## `experimental_spec_shaking_v2`
//!
//! Enables v2 spec shaking, an improved mechanism for controlling which type,
//! event, and function definitions appear in a contract's spec.
//!
//! ### Spec Shaking v1 (default, no feature flag)
//!
//! - Lib imports (via `contractimport!`): exported
//! - Wasm imports (via `contractimport!`): not exported
//! - `pub` types: exported
//! - Non-`pub` types: not exported
//! - All events: exported
//! - All functions: exported
//!
//! ### Spec Shaking v2 (this feature)
//!
//! - Everything exported (types, events, functions, imports)
//! - Unused entries shaken out using dead code / spec elimination
//!
//! A contract's spec (the `contractspecv0` custom section in the Wasm binary)
//! contains entries for every function, type, and event defined by the contract.
//! When types or events are defined but not actually used at a contract boundary
//! (parameters, return values, error returns, or event publishes), their spec
//! entries are dead weight. Spec shaking removes them.
//!
//! ### How It Works
//!
//! When this feature is enabled, the SDK embeds 14-byte **markers** in the Wasm
//! data section for each exported type and event. A marker consists of a
//! `SpEcV1` magic prefix followed by 8 bytes of a SHA-256 hash of the spec
//! entry's XDR.
//!
//! Markers are placed inside functions that are only called when the type is
//! actually used:
//! - **Function parameters**: marker is triggered when deserializing the input.
//! - **Function return values**: marker is triggered when serializing the output.
//! - **Error returns**: marker is triggered via `Result<T, E>` serialization.
//! - **Event publishes**: marker is triggered inside the `publish()` call.
//! - **Nested types**: a type's marker function calls the marker functions of
//!   its field types, so nested types are transitively marked.
//! - **Container types**: `Vec<T>`, `Map<K, V>`, `Option<T>`, and `Result<T, E>`
//!   propagate markers to their inner types.
//!
//! The Rust compiler's dead code elimination (DCE) removes markers for types
//! that are never used, while keeping markers for types that are.
//!
//! Post-build tools (e.g. `stellar-cli`) scan the Wasm data section for
//! `SpEcV1` markers, match them against spec entries, and strip any entries
//! without a corresponding marker.
//!
//! ### Changed Behaviour
//!
//! When this feature is enabled the following macros change behaviour:
//!
//! #### [`contracttype`]
//!
//! Without this feature, spec entries are only generated for `pub` types (or
//! when `export = true` is explicitly set). With this feature, spec entries
//! and markers are generated for all types regardless of visibility, unless
//! `export = false` is explicitly set. This ensures all types can participate
//! in spec shaking.
//!
//! #### [`contracterror`]
//!
//! Same as [`contracttype`]: without this feature, spec entries are only
//! generated for `pub` types. With this feature, spec entries and markers are
//! generated for all error enums regardless of visibility, unless
//! `export = false` is explicitly set.
//!
//! #### [`contractevent`]
//!
//! Markers are embedded for all events, allowing post-build tools to strip
//! spec entries for events that are never published at a contract boundary.
//!
//! #### [`contractimport!`]
//!
//! Without this feature, [`contractimport!`] generates imported types with
//! `export = false`. Imported types do not produce spec entries in the
//! importing contract's spec. They are purely local Rust types used for
//! serialization. The importing contract's spec only contains its own function
//! definitions, and callers must look at the imported contract's spec to find
//! the type definitions.
//!
//! With this feature, [`contractimport!`] generates imported types with
//! `export = true`. Imported types produce spec entries and markers in the
//! importing contract, just like locally defined types. This changes the
//! contract's spec to be self-contained — it includes the type definitions for
//! all types used at the contract boundary, regardless of where those types
//! were originally defined. Specifically:
//!
//! - Imported types that are used in the contract's function signatures or
//!   events will have their markers survive DCE and their spec entries will be
//!   kept after shaking.
//! - Imported types that are **not** used at any contract boundary will have
//!   their markers eliminated by DCE and their spec entries will be stripped.
//!
//! This ensures that a contract importing a large interface only includes spec
//! entries for the types it actually uses, while still producing a
//! self-contained spec.
//!
//! ### Build Requirements
//!
//! This feature requires building with `stellar contract build` from
//! `stellar-cli` v25.2.0 or newer. Building a contract for wasm (e.g. with
//! `cargo build --target wasm32v1-none`) will produce a build error unless the
//! `SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2` environment variable is
//! set. The check only fires for wasm targets; native builds (e.g. unit tests)
//! are unaffected.
//!
//! [`contracttype`]: crate::contracttype
//! [`contracterror`]: crate::contracterror
//! [`contractevent`]: crate::contractevent
//! [`contractimport!`]: crate::contractimport
