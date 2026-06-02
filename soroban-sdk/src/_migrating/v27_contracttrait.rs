//! `cfg` and `cfg_attr` handling in [`contracttrait`] and `#[contractimpl(contracttrait)]`.
//!
//! [`contracttrait`] captures the signatures of default functions when the trait is defined, but
//! the wrappers for non-overridden defaults are generated later, at each
//! `#[contractimpl(contracttrait)]` site. A `cfg` or `cfg_attr` on a default function would
//! therefore be evaluated in the implementing crate's configuration rather than the trait-defining
//! crate's, so these attributes are now rejected on default functions.
//!
//! On `#[contractimpl(contracttrait)]` override methods, a direct `cfg` is supported — the default
//! wrapper is generated under the inverse condition — but `cfg_attr` is rejected because it is not
//! normalized by this handoff.
//!
//! ## `cfg`/`cfg_attr` on default functions
//!
//! `cfg` and `cfg_attr` are no longer accepted on the default functions of a [`contracttrait`]:
//!
//! ```compile_fail
//! use soroban_sdk::{contracttrait, Env};
//!
//! #[contracttrait]
//! pub trait Pausable {
//!     #[cfg(feature = "extra")] // ❌ cfg on a default function is rejected
//!     fn extra(env: Env) -> u32 {
//!         let _ = env;
//!         1
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! The build fails with:
//!
//! ```text
//! error: `cfg` and `cfg_attr` are not supported on `#[contracttrait]` default functions because
//! they would be evaluated where the default implementation is generated, not where the trait is
//! defined
//! ```
//!
//! To make an entire interface conditional, gate the trait (and its implementations) instead:
//!
//! ```
//! use soroban_sdk::{contracttrait, Env};
//!
//! #[cfg(feature = "extra")] // 👈 👀 gate the whole trait
//! #[contracttrait]
//! pub trait Pausable {
//!     fn extra(env: Env) -> u32 {
//!         let _ = env;
//!         1
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! To make a single function conditional, declare it without a default and gate the implementing
//! method at each `#[contractimpl(contracttrait)]` site, where a direct `cfg` is supported (see
//! below).
//!
//! ## `cfg_attr` on override methods
//!
//! On a `#[contractimpl(contracttrait)]` impl, a direct `cfg` is supported but `cfg_attr` is not:
//!
//! ```compile_fail
//! use soroban_sdk::{contract, contractimpl, contracttrait, Env};
//!
//! #[contracttrait]
//! pub trait Pausable {
//!     fn pause(env: Env) -> u32 {
//!         let _ = env;
//!         1
//!     }
//! }
//!
//! #[contract]
//! pub struct MyContract;
//!
//! #[contractimpl(contracttrait)]
//! impl Pausable for MyContract {
//!     #[cfg_attr(feature = "extra", allow(dead_code))] // ❌ cfg_attr is rejected
//!     fn pause(env: Env) -> u32 {
//!         let _ = env;
//!         2
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! The build fails with:
//!
//! ```text
//! error: `cfg_attr` is not supported on `#[contractimpl(contracttrait)]` methods because the
//! generated helper only supports direct `cfg` attrs for default override matching
//! ```
//!
//! Use a direct `cfg` (supported — the default is emitted under the inverse condition), expand the
//! `cfg_attr` to the attribute(s) it applies, or gate the whole impl block:
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, contracttrait, Env};
//!
//! #[contracttrait]
//! pub trait Pausable {
//!     fn pause(env: Env) -> u32 {
//!         let _ = env;
//!         1
//!     }
//! }
//!
//! #[contract]
//! pub struct MyContract;
//!
//! #[contractimpl(contracttrait)]
//! impl Pausable for MyContract {
//!     #[cfg(feature = "extra")] // 👈 👀 direct cfg is supported
//!     fn pause(env: Env) -> u32 {
//!         let _ = env;
//!         2
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! [`contracttrait`]: crate::contracttrait
