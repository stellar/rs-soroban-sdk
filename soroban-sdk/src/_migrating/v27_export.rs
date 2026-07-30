//! The `export` argument was deprecated under the `experimental_spec_shaking_v2` feature.
//!
//! The `export` argument on [`contracttype`], [`contracterror`], and [`contractevent`] controlled
//! whether a type contributed a contract spec entry. Under spec shaking v1 (the v27 default), it was
//! a retention hint: `export = false` suppressed the entry, hiding the type from the contract spec,
//! and `export = true` forced one to be emitted.
//!
//! Under spec shaking v2, the final spec is instead determined by *reachability* from the contract
//! boundary. The macros emit a spec entry and a marker for every type, and post-build tooling
//! removes the entries for types that are not reachable from any public contract function. As a
//! result, `export` no longer had any effect: it could not hide a type that remained reachable from
//! a public boundary (the entry was kept regardless), and it was redundant for a type that was
//! already reachable. Worse, `export = false` could conflict with exact spec shaking by dropping a
//! marker for an entry that was still reachable, leading to missing coverage.
//!
//! Setting `export` therefore emitted a deprecation warning at the macro call site in v27, while
//! default (v1) builds were unaffected. As of v28 the feature is gone, spec shaking is always on,
//! and `export` is rejected outright. See
//! [`v28_spec_shaking`][crate::_migrating::v28_spec_shaking].
//!
//! ## Migrating
//!
//! Remove the `export` argument from `contracttype`, `contracterror`, and `contractevent`
//! annotations. A type that was hidden with `export = false` is filtered from the spec
//! automatically when it is unused, and a type that was forced in with `export = true` is included
//! whenever it is reachable.
//!
//! For example, a type used only inside a contract — never at a function boundary — was previously
//! hidden from the spec with `export = false`. On this version of the SDK the argument no longer
//! compiles:
//!
//! ```compile_fail
//! use soroban_sdk::contracttype;
//!
//! #[contracttype(export = false)] // ❌ `export` is no longer supported
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
