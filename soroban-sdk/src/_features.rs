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
//! - Unused entries shaken out using extra-root markers and exact spec graph pruning
//!
//! A contract's spec (the `contractspecv0` custom section in the Wasm binary)
//! contains entries for every function, type, and event defined by the contract.
//! When types or events are defined but not actually used at a contract boundary
//! (parameters, return values, error returns, or event publishes), their spec
//! entries are dead weight. Spec shaking removes them.
//!
//! ### How It Works
//!
//! Function entries in `contractspecv0` are always roots. The macros also emit
//! a removable sidecar graph in the private `contractspecv0.rssdk.graphv0`
//! custom section. Each graph record is keyed by the SHA-256 hash of a full spec
//! entry's XDR and lists the exact child spec-entry IDs referenced by that
//! entry. Generated UDTs implement the hidden `SpecTypeId` helper so function,
//! event, and UDT graph records can refer to exact type specs instead of only
//! their `ScSpecTypeDef::Udt` names.
//! `export = false` types still get this hidden exact ID when v2 is enabled,
//! but they do not emit public spec entries or UDT graph records of their own.
//! A reachable graph reference to such a type is invalid for post-build
//! shaking because there is no matching spec entry in `contractspecv0` to keep.
//! SDK-owned types used at canonical contract boundaries are configured to emit
//! normally under v2; see [`contracttype`] below.
//!
//! Events and errors thrown through `panic_with_error!` or
//! `assert_with_error!` need one extra reachability signal because those use
//! sites are not visible from function specs alone. For each exported event or
//! contract error, the SDK embeds a 14-byte marker in reachable code that uses
//! it. A marker consists of a `SpEcV1` magic prefix followed by 8 bytes of a
//! SHA-256 hash of the spec entry's XDR.
//!
//! Post-build tools (e.g. `stellar-cli`) scan the Wasm data section for
//! markers, read the sidecar graph, keep all functions and marked extra roots,
//! traverse UDT references by exact spec ID, rewrite `contractspecv0`, and
//! remove `contractspecv0.rssdk.graphv0`. When a reachable function, event, or
//! UDT entry references UDTs, its graph record must be present and complete.
//! Missing graph records, missing references, references to unknown spec IDs,
//! and references to non-UDT spec entries are rejected instead of falling back
//! to name-based matching.
//!
//! ### Changed Behaviour
//!
//! When this feature is enabled the following macros change behaviour:
//!
//! #### [`contracttype`]
//!
//! Without this feature, spec entries are only generated for `pub` types (or
//! when `export = true` is explicitly set). With this feature, spec entries
//! are generated for all types regardless of visibility, unless `export = false`
//! is explicitly set. This ensures all types can participate in spec graph
//! pruning. Types with `export = false` do not emit public spec entries, but
//! they still get the hidden `SpecTypeId` helper so graph records can identify
//! them exactly. A reachable graph edge to one of these hidden-only types is
//! rejected during strict post-build validation because the referenced spec
//! entry is absent.
//!
//! SDK-owned types that appear at canonical contract boundaries, such as
//! `auth::Context` in `__check_auth` no longer use `export = false` under v2.
//! As a result, they behave like ordinary public UDTs in v2, and without v2 they
//! remain hidden. This keeps specs complete and without ambiguities between
//! SDK-owned types and user-defined types with the same name.
//!
//! #### [`contracterror`]
//!
//! Same as [`contracttype`]: without this feature, spec entries are only
//! generated for `pub` types. With this feature, spec entries are generated for
//! all error enums regardless of visibility, unless
//! `export = false` is explicitly set. Error enums with `export = false` still
//! get the hidden `SpecTypeId` helper for exact graph references, but cannot be
//! the target of a reachable graph edge in a valid shaken v2 spec.
//! They also do not implement the hidden `SpecShakingMarker` hook required by
//! `panic_with_error!` and `assert_with_error!` when this feature is enabled;
//! remove `export = false` for typed error enums that may be thrown through those
//! macros, or throw a raw `Error` value instead.
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
//! `export = true`. Imported types produce spec entries in the importing
//! contract, just like locally defined types. This changes the
//! contract's spec to be self-contained — it includes the type definitions for
//! all types used at the contract boundary, regardless of where those types
//! were originally defined. Specifically:
//!
//! - Imported types that are used in the contract's function signatures or
//!   published events will be reachable from the spec graph and their spec
//!   entries will be kept after shaking.
//! - Imported types that are **not** used at any contract boundary will have
//!   their spec entries stripped.
//!
//! This ensures that a contract importing a large interface only includes spec
//! entries for the types it actually uses, while still producing a
//! self-contained spec.
//! Raw Wasm built without post-build shaking can still contain pre-shake
//! candidates, including SDK-owned boundary types or duplicate UDT names that
//! would otherwise be stripped. Prefer importing Wasms produced by
//! `stellar contract build` or another pipeline that runs `shake_contract_spec`.
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
