//! Spec shaking v2 is always on, and the `export` argument is deprecated.
//!
//! The `export` argument on [`contracttype`], [`contracterror`], and [`contractevent`] controlled
//! whether a type contributes a contract spec entry. Under spec shaking v1, it was a retention
//! hint: `export = false` suppressed the entry, hiding the type from the contract spec, and
//! `export = true` forced one to be emitted.
//!
//! [Spec shaking v2][`_spec_shaking`] is now always on, and the final spec is instead determined by
//! *reachability* from the contract boundary. The macros emit a spec entry and a marker for every
//! type, and post-build tooling removes the entries for types that are not reachable from any
//! public contract function. As a result, `export` no longer has any effect: it cannot hide a type
//! that remains reachable from a public boundary (the entry is kept regardless), and it is
//! redundant for a type that is already reachable. Worse, `export = false` can conflict with exact
//! spec shaking by dropping a marker for an entry that is still reachable, leading to missing
//! coverage.
//!
//! Setting `export` therefore now emits a deprecation warning at the macro call site, and the
//! argument will be removed entirely in a future release.
//!
//! Because the SDK relies on post-build tooling to shake the spec, contracts must be built with
//! `stellar contract build` from `stellar-cli` v25.2.0 or newer. Building a contract for wasm with
//! another build system produces a build error unless the
//! `SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2` environment variable is set to signal that
//! the build system shakes the spec.
//!
//! ## Migrating
//!
//! Remove the `export` argument from `contracttype`, `contracterror`, and `contractevent`
//! annotations. A type that was hidden with `export = false` is filtered from the spec
//! automatically when it is unused, and a type that was forced in with `export = true` is included
//! whenever it is reachable.
//!
//! For example, a type used only inside a contract — never at a function boundary — was previously
//! hidden from the spec with `export = false`:
//!
//! ```
//! # #![allow(deprecated)]
//! use soroban_sdk::contracttype;
//!
//! #[contracttype(export = false)] // 👈 👀 hint to hide the unused type from the spec
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
//! [`_spec_shaking`]: crate::_spec_shaking
