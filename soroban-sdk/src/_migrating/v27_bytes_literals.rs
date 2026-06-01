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
//! ## Building bytes from a decimal value
//!
//! To build bytes from a value you have in mind in decimal, write the value as a hex literal
//! instead. For a single byte, write the decimal value's hex equivalent (e.g. `255` is `0xff`).
//! Note that hex makes the byte width explicit, so left-pad with zeros to get the byte count you
//! want (e.g. `0x00ff` for two bytes). Alternatively, use an array literal, which still accepts
//! decimal values for each byte.
//!
//! ```
//! use soroban_sdk::{bytes, Env};
//!
//! let env = Env::default();
//!
//! // The decimal value 255 written as a single-byte hex literal.
//! let b = bytes!(&env, 0xff);
//! assert_eq!(b, bytes!(&env, [255]));
//!
//! // Or as an array literal, where decimal values are still accepted per byte.
//! let b = bytes!(&env, [255]);
//! assert_eq!(b, bytes!(&env, 0xff));
//! ```
//!
//! [`bytes!`]: crate::bytes
//! [`bytesn!`]: crate::bytesn
//! [`Bytes`]: crate::Bytes
//! [`BytesN`]: crate::BytesN
