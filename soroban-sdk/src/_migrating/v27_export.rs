//! The `export` argument is deprecated under the [`experimental_spec_shaking_v2`] feature.
//!
//! The `export` argument on [`contracttype`], [`contracterror`], and [`contractevent`] controls
//! whether a type contributes a contract spec entry. Under spec shaking v1 (the default), it is a
//! retention hint: `export = false` suppresses the entry, hiding the type from the contract spec,
//! and `export = true` forces one to be emitted.
//!
//! Under spec shaking v2, the final spec is instead determined by *reachability* from the contract
//! boundary. The macros emit a spec entry and a marker for every type, and post-build tooling
//! removes the entries for types that are not reachable from any public contract function. As a
//! result, `export` no longer has any effect: it cannot hide a type that remains reachable from a
//! public boundary (the entry is kept regardless), and it is redundant for a type that is already
//! reachable. Worse, `export = false` can conflict with exact spec shaking by dropping a marker
//! for an entry that is still reachable, leading to missing coverage.
//!
//! Setting `export` therefore now emits a deprecation warning at the macro call site, and the
//! argument will be removed entirely in a future release. Default (v1) builds are unaffected.
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
//! use soroban_sdk::contracttype;
//!
//! #[contracttype(export = false)] // 👈 👀 hint to hide the unused type from the spec
//! pub struct InternalState {
//!     pub counter: u32,
//! }
//! # fn main() {}
//! ```
//!
//! Under spec shaking v2, drop the argument. Because `InternalState` is never reachable from a
//! public contract function, post-build tooling strips its spec entry automatically:
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
//! [`experimental_spec_shaking_v2`]: crate::_features#experimental_spec_shaking_v2
