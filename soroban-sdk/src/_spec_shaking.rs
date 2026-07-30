//! # Spec Shaking
//!
//! Spec shaking is the mechanism that controls which type, event, and function
//! definitions appear in a contract's spec.
//!
//! A contract's spec (the `contractspecv0` custom section in the Wasm binary)
//! contains entries for every function, type, and event defined by the contract.
//! When types or events are defined but not actually used at a contract boundary
//! (parameters, return values, error returns, or event publishes), their spec
//! entries are dead weight. Spec shaking removes them.
//!
//! Everything is exported (types, events, functions, imports), and unused
//! entries are shaken out using dead code / spec elimination.
//!
//! ## How It Works
//!
//! The SDK embeds 14-byte **markers** in the Wasm data section for each exported
//! type and event. A marker consists of a `SpEcV1` magic prefix followed by 8
//! bytes of a SHA-256 hash of the spec entry's XDR.
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
//! ## Macro Behaviour
//!
//! ### [`contracttype`]
//!
//! Spec entries and markers are generated for all types regardless of
//! visibility, and post-build tooling removes entries that are not reachable
//! from the contract interface. The `export` argument is a no-op and emits a
//! deprecation warning at the macro call site; it will be removed in a future
//! release.
//!
//! ### [`contracterror`]
//!
//! Same as [`contracttype`]: post-build tooling removes unreachable error enum
//! entries. The `export` argument is a no-op and emits a deprecation warning; it
//! will be removed in a future release.
//!
//! ### [`contractevent`]
//!
//! Markers are embedded for all events, allowing post-build tools to strip
//! spec entries for events that are never published at a contract boundary.
//! The `export` argument is a no-op and emits a deprecation warning; it will be
//! removed in a future release.
//!
//! ### [`contractimport!`]
//!
//! Imported types produce spec entries and markers in the importing contract,
//! just like locally defined types. A contract's spec is self-contained — it
//! includes the type definitions for all types used at the contract boundary,
//! regardless of where those types were originally defined. Specifically:
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
//! ## Build Requirements
//!
//! Because the SDK relies on post-build tooling to shake the spec, contracts
//! must be built with `stellar contract build` from `stellar-cli` v25.2.0 or
//! newer. Building a contract for wasm with another build system (e.g. with
//! `cargo build --target wasm32v1-none`) produces a build error unless the
//! `SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2` environment variable is
//! set to signal that the build system shakes the spec. The check only fires for
//! wasm targets; native builds (e.g. unit tests) are unaffected.
//!
//! [`contracttype`]: crate::contracttype
//! [`contracterror`]: crate::contracterror
//! [`contractevent`]: crate::contractevent
//! [`contractimport!`]: crate::contractimport
