//! Types, events, and errors cannot be named with the names of soroban-sdk types.
//!
//! A contract's spec refers to soroban-sdk types, such as [`Address`] or [`Symbol`], and to the
//! contract's own types, both by name. A user-defined type named like a soroban-sdk type could be
//! mistaken for the soroban-sdk type by a reader of the spec, and in particular by contracts built
//! with older SDKs that import the contract, as they map such a name to the soroban-sdk type.
//!
//! [`contracttype`] already rejected the names of soroban-sdk types. In v30 [`contractevent`] and
//! [`contracterror`] reject them too:
//!
//! ```text
//! error: type `Address` conflicts with a soroban_sdk type and cannot be used as a user-defined type
//! ```
//!
//! The names rejected are the names of the soroban-sdk types that can appear in a spec, such as
//! `Address`, `Bytes`, `BytesN`, `Duration`, `Hash`, `Map`, `MuxedAddress`, `String`, `Symbol`,
//! `Timepoint`, `Vec`, `U256`, `I256`, and the BLS12-381 and BN254 types.
//!
//! ## Migrating
//!
//! Rename any event or error enum named like a soroban-sdk type. Renaming an event changes the
//! name in its spec entry, and renaming an error enum changes the name its spec entry and the
//! functions that return it use, so clients generated from the spec will see the new name.
//!
//! [`Address`]: crate::Address
//! [`Symbol`]: crate::Symbol
//! [`contracttype`]: crate::contracttype
//! [`contractevent`]: crate::contractevent
//! [`contracterror`]: crate::contracterror
