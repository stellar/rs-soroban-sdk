//! [`contracttype`] structs tolerate missing and additional map fields when unpacking.
//!
//! A [`contracttype`] struct is represented as a map, keyed by the field names. Prior to v28 that
//! map had to match the struct exactly: every field of the struct had to be present as a key, and
//! no other keys could be present. Any difference between the map and the struct was an error, and
//! unpacking a value stored by a different version of a contract usually meant the contract
//! trapped.
//!
//! In v28 unpacking (reading from storage) is tolerant of both differences, which makes it possible
//! to evolve the data using some intuitive migration strategies:
//!
//! - **Fields absent from the map unpack as void.** Void unpacks into an [`Option`] field as
//!   `None`, so a field added to a struct as an `Option` reads back as `None` from data stored
//!   before the field existed. A field that is not an `Option` still errors, because void does not
//!   unpack into it.
//!
//! - **Keys in the map that are not fields of the struct are ignored.** A field removed from a
//!   struct is discarded when reading data stored while the field still existed.
//!
//!   ⚠️ The discarded fields are not remembered. If the value is packed and written back to
//!   storage, the stored data loses every field the struct being unpacked into did not have. When
//!   more than one version of a struct reads the same stored data, only the fields of the struct
//!   doing the writing survive.
//!
//! Packing (writing to storage) is unchanged. Every field of the struct is written to the map,
//! including `Option` fields that are `None`, which are written as void.
//!
//! The change applies to structs with named fields, when unpacking from a [`Val`], and when
//! unpacking from the [`ScVal`] and [`ScMap`] XDR types available under the `testutils` feature.
//! Tuple structs and enums are represented as vecs, not maps, and are unaffected, as are
//! [`contractevent`] and [`contracterror`] types.
//!
//! ## Changed Behaviour
//!
//! ### Additional fields are ignored
//!
//! ```
//! use soroban_sdk::{contracttype, map, symbol_short, Env, TryFromVal};
//!
//! #[contracttype]
//! #[derive(Debug, PartialEq)]
//! pub struct State {
//!     pub count: u32,
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!
//!     // A map containing a field the struct does not have. Before v28 this was an error. In v28
//!     // the additional field is ignored and discarded.
//!     let val = map![
//!         &env,
//!         (symbol_short!("count"), 5u32),
//!         (symbol_short!("removed"), 9u32),
//!     ]
//!     .to_val();
//!
//!     assert_eq!(State::try_from_val(&env, &val), Ok(State { count: 5 }));
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! ### Absent fields unpack as void
//!
//! ```
//! use soroban_sdk::{contracttype, map, symbol_short, ConversionError, Env, TryFromVal};
//!
//! #[contracttype]
//! #[derive(Debug, PartialEq)]
//! pub struct State {
//!     pub count: u32,
//!     pub label: Option<u32>,
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!
//!     // A map missing the `label` field, as stored before the field was added to the struct.
//!     // Before v28 this was an error. In v28 `label` unpacks as `None`.
//!     let val = map![&env, (symbol_short!("count"), 5u32)].to_val();
//!
//!     assert_eq!(
//!         State::try_from_val(&env, &val),
//!         Ok(State { count: 5, label: None }),
//!     );
//!
//!     // A map missing the `count` field is still an error, because void does not unpack into a
//!     // u32.
//!     let val = map![&env, (symbol_short!("label"), 7u32)].to_val();
//!
//!     assert_eq!(State::try_from_val(&env, &val), Err(ConversionError));
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! ## Migrating
//!
//! No code changes are required for most contracts, although the subtle change in behaviour could
//! be observable on contracts that expect type unpacking to fail in the cases that are now
//! tolerated. Contracts that only round trip their own data see no difference, because a map
//! written by the same struct always contains exactly its fields.
//!
//! Review any code that relied on unpacking failing to detect a mismatch. A struct where every
//! field is an `Option` now unpacks from any map, including an empty map and a map written for an
//! unrelated type, producing a value with every field `None`. Unpacking is no longer sufficient on
//! its own to determine that a value is of the expected type. Where the type needs to be
//! distinguished, include a field that is not an `Option`, or store the values under distinct
//! storage keys.
//!
//! When adding a field to a stored struct, make it an `Option` so that existing stored data unpacks
//! with the field as `None`. A field that is not an `Option` still requires the stored data to be
//! migrated, because there is no value to unpack it from.
//!
//! [`contracttype`]: crate::contracttype
//! [`contractevent`]: crate::contractevent
//! [`contracterror`]: crate::contracterror
//! [`Val`]: crate::Val
//! [`ScVal`]: crate::xdr::ScVal
//! [`ScMap`]: crate::xdr::ScMap
