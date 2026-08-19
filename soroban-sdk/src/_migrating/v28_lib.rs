//! The `lib` argument has been removed, and has no replacement.
//!
//! The `lib` argument on [`contracttype`], [`contracterror`], and [`contractevent`] set the `lib`
//! field of the type's contract spec entry. Bindings generators read that field and, when it was
//! set, emitted a type alias pointing at a crate of that name instead of generating the type:
//!
//! ```text
//! pub type StructA = ::libname::StructA;
//! ```
//!
//! The intent was to let a contract declare that a type it uses is defined in a shared library
//! crate, so that generated bindings could reuse the library's definition rather than duplicating
//! it. The idea was never completed: nothing shipped that produced the referenced crate, an alias
//! to a crate the consumer does not depend on does not compile, and the argument remained
//! undocumented. No contract on Mainnet sets it.
//!
//! The argument is therefore gone, along with the code generation that consumed the spec field.
//! There is no replacement. The `lib` field remains in the spec XDR for compatibility, but the SDK
//! always emits it empty and ignores it when generating code, so [`contractimport!`] now always
//! generates a complete type definition.
//!
//! ## Migrating
//!
//! Remove the `lib` argument from `contracttype`, `contracterror`, and `contractevent`
//! annotations. Setting it is now rejected:
//!
//! ```text
//! error: Unknown field: `lib`
//! ```
//!
//! For example, a type that named a library crate no longer compiles:
//!
//! ```compile_fail
//! use soroban_sdk::contracttype;
//!
//! #[contracttype(lib = "libname")] // ❌ lib is no longer supported
//! pub struct Point {
//!     pub x: u32,
//!     pub y: u32,
//! }
//! # fn main() {}
//! ```
//!
//! Drop the argument. The type is defined as usual, and bindings generated for a contract using it
//! contain its full definition:
//!
//! ```
//! use soroban_sdk::contracttype;
//!
//! #[contracttype] // 👈 👀 no lib argument
//! pub struct Point {
//!     pub x: u32,
//!     pub y: u32,
//! }
//! # fn main() {}
//! ```
//!
//! To share types across contracts, define them in a common crate and depend on that crate from
//! each contract, as with any other Rust code. Each contract's spec, and the bindings generated
//! from it, remain self-contained.
//!
//! [`contracttype`]: crate::contracttype
//! [`contracterror`]: crate::contracterror
//! [`contractevent`]: crate::contractevent
//! [`contractimport!`]: crate::contractimport
