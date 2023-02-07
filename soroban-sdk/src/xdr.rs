/// XDR contains types for building and generating XDR values.

// XDR types needed by macros.
#[doc(hidden)]
pub use super::env::xdr::HostFunction;

// XDR generic types and traits.
#[cfg(not(target_family = "wasm"))]
pub use super::env::xdr::ReadXdrIter;

// XDR types
pub use super::env::xdr::*;
