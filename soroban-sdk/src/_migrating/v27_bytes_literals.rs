//! [`bytes!`] and [`bytesn!`] no longer accept base10 (decimal) integer literals.
//!
//! The [`bytes!`] and [`bytesn!`] macros build a [`Bytes`] or [`BytesN`] from an integer literal by
//! interpreting the literal's bytes in big-endian order. The macros were always intended to be used
//! with hex (`0x`) and binary (`0b`) literals, where each byte of the resulting value is written
//! explicitly. Support for base10 (decimal) literals was accidental and incomplete, and exhibited
//! oddities because the byte width of a decimal literal is ambiguous (for example, does `256` mean
//! `[1, 0]` or `[0, 1, 0]`?). Rather than try to fix that incomplete and surprising behavior, the
//! underlying `bytes-lit` dependency removed decimal support entirely, keeping the feature focused
//! on its intended use case: hex and binary literals.
//!
//! Array literals (e.g. `bytes!(&env, [3, 2, 1])`) are unaffected and continue to accept decimal
//! values for each element.
//!
//! ## Migrating
//!
//! Rewrite any base10 integer literal passed to [`bytes!`] or [`bytesn!`] as a hex (`0x`) or binary
//! (`0b`) literal. A decimal literal will now fail to compile with:
//!
//! ```text
//! error: only positive hex (0x) and binary (0b) integer literals are supported
//! ```
//!
//! Before:
//!
//! ```compile_fail
//! use soroban_sdk::{bytes, Env};
//!
//! let env = Env::default();
//! let b = bytes!(&env, 1); // ❌ base10 literal no longer supported
//! ```
//!
//! After:
//!
//! ```
//! use soroban_sdk::{bytes, Env};
//!
//! let env = Env::default();
//! let b = bytes!(&env, 0x1); // 👈 👀 use a hex (0x) or binary (0b) literal
//! # assert_eq!(b, bytes!(&env, [1]));
//! ```
//!
//! [`bytes!`]: crate::bytes
//! [`bytesn!`]: crate::bytesn
//! [`Bytes`]: crate::Bytes
//! [`BytesN`]: crate::BytesN
