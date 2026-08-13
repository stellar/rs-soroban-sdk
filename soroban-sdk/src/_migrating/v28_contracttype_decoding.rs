//! [`contracttype`] structs tolerate missing and additional map fields when decoding.
//!
//! A [`contracttype`] struct is represented as a map, keyed by the field names. Prior to v28 that
//! map had to match the struct exactly: every field of the struct had to be present as a key, and
//! no other keys could be present. Any difference between the map and the struct was an error, and
//! decoding a value stored by a different version of a contract usually meant the contract trapped.
//!
//! In v28 decoding is tolerant of both differences, which makes it possible to evolve a stored data
//! type without migrating the data already stored:
//!
//! - **Fields absent from the map decode as void.** Void decodes into an [`Option`] field as
//!   `None`, so a field added to a struct as an `Option` reads back as `None` from data stored
//!   before the field existed. A field that is not an `Option` still errors, because void does not
//!   decode into it.
//!
//! - **Keys in the map that are not fields of the struct are ignored.** A field removed from a
//!   struct is discarded when reading data stored while the field still existed.
//!
//! Encoding is unchanged. Every field of the struct is written to the map, including `Option`
//! fields that are `None`, which are written as void.
//!
//! The change applies to structs with named fields. Tuple structs and enums are represented as
//! vecs, not maps, and are unaffected, as are [`contractevent`] and [`contracterror`] types.
//!
//! Decoding uses the [CAP-86] `sparse_map_unpack_to_slice` host function, and so contracts built
//! with v28 require protocol 28 or later.
//!
//! ## Changed Behaviour
//!
//! ### Additional fields are ignored
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! use soroban_sdk::{contracttype, map, symbol_short, Env, TryFromVal};
//!
//! #[contracttype]
//! #[derive(Debug, PartialEq)]
//! pub struct State {
//!     pub count: u32,
//! }
//!
//! # fn main() {
//! let env = Env::default();
//!
//! // A map containing a field the struct does not have. Before v28 this was an error. In v28 the
//! // additional field is ignored and discarded.
//! let val = map![
//!     &env,
//!     (symbol_short!("count"), 5u32),
//!     (symbol_short!("removed"), 9u32),
//! ]
//! .to_val();
//!
//! assert_eq!(State::try_from_val(&env, &val), Ok(State { count: 5 }));
//! # }
//! ```
//!
//! ### Absent fields decode as void
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! use soroban_sdk::{contracttype, map, symbol_short, ConversionError, Env, TryFromVal};
//!
//! #[contracttype]
//! #[derive(Debug, PartialEq)]
//! pub struct State {
//!     pub count: u32,
//!     pub label: Option<u32>,
//! }
//!
//! # fn main() {
//! let env = Env::default();
//!
//! // A map missing the `label` field, as stored before the field was added to the struct. Before
//! // v28 this was an error. In v28 `label` decodes as `None`.
//! let val = map![&env, (symbol_short!("count"), 5u32)].to_val();
//!
//! assert_eq!(
//!     State::try_from_val(&env, &val),
//!     Ok(State { count: 5, label: None }),
//! );
//!
//! // A map missing the `count` field is still an error, because void does not decode into a u32.
//! let val = map![&env, (symbol_short!("label"), 7u32)].to_val();
//!
//! assert_eq!(State::try_from_val(&env, &val), Err(ConversionError));
//! # }
//! ```
//!
//! ## Migrating
//!
//! No code changes are required. Contracts that already round trip their own data see no
//! difference, because a map written by the same struct always contains exactly its fields.
//!
//! Review any code that relied on decoding failing to detect a mismatch:
//!
//! - A struct where every field is an `Option` now decodes from any map, including an empty map and
//!   a map written for an unrelated type, producing a value with every field `None`. Decoding is no
//!   longer sufficient on its own to determine that a value is of the expected type. Where the type
//!   needs to be distinguished, include a field that is not an `Option`, or store the values under
//!   distinct storage keys.
//!
//! - Data written by a newer version of a contract decodes in an older version, with the fields the
//!   older version does not know about discarded. If that data is then written back, those fields
//!   are lost. Contracts that may be read by more than one version should treat a decode as a
//!   read of only the fields the current version defines.
//!
//! When adding a field to a stored struct, make it an `Option` so that existing stored data decodes
//! with the field as `None`. A field that is not an `Option` still requires the stored data to be
//! migrated, because there is no value to decode it from.
//!
//! [CAP-86]: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0086.md
//! [`contracttype`]: crate::contracttype
//! [`contractevent`]: crate::contractevent
//! [`contracterror`]: crate::contracterror
