//! [`contractevent`] events with a map data format omit void fields when publishing.
//!
//! An event declared with `data_format = "map"`, which is the default, publishes its data fields as
//! a map keyed by the field names. Every field used to be written to that map, including fields
//! whose value is void, such as an [`Option`] field that is `None`.
//!
//! In v28 a field whose value is void is omitted from the map, so the published event carries only
//! the fields that hold a value. A consumer unpacking the event into a struct reads an omitted
//! field back as `None` for an `Option` field, because [unpacking is sparse][v28_contracttype_unpacking],
//! and so the value round trips even though the map is smaller.
//!
//! This applies only to the top level of an event's data map. Packing everywhere else is unchanged:
//! a [`contracttype`] struct writes all of its fields whether it is stored, passed to a contract
//! function, or nested inside the value of an event field.
//!
//! ## Changed Behaviour
//!
//! ```
//! use soroban_sdk::{
//!     contract, contractevent, map, symbol_short, testutils::Events as _, vec, Env, Event,
//!     IntoVal, Symbol, Val,
//! };
//!
//! #[contractevent]
//! pub struct Transfer {
//!     #[topic]
//!     to: Symbol,
//!     to_muxed_id: Option<u64>,
//!     amount: i128,
//! }
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let id = env.register(Contract, ());
//!     env.as_contract(&id, || {
//!         Transfer {
//!             to: symbol_short!("to"),
//!             to_muxed_id: None,
//!             amount: 1,
//!         }
//!         .publish(&env);
//!     });
//!
//!     // The to_muxed_id field is None, which is void, and so is omitted from
//!     // the map. Before v28 the map also held to_muxed_id with a void value.
//!     let data: Val = map![
//!         &env,
//!         (
//!             symbol_short!("amount"),
//!             <_ as IntoVal<Env, Val>>::into_val(&1i128, &env)
//!         ),
//!     ]
//!     .to_val();
//!     assert_eq!(
//!         env.events().all(),
//!         vec![
//!             &env,
//!             (
//!                 id.clone(),
//!                 (symbol_short!("transfer"), symbol_short!("to")).into_val(&env),
//!                 data
//!             )
//!         ],
//!     );
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! ## Migrating
//!
//! No code changes are required to publish events. Review any consumer that reads the published
//! map, because a field it expects to be present with a void value is now absent, and any test that
//! asserts on the full data map of an event that has a void field.
//!
//! An event type can carry a field that is only sometimes meaningful without paying for it in every
//! event, so a single event can serve the shapes a contract needs rather than one event type per
//! shape.
//!
//! [`contractevent`]: crate::contractevent
//! [`contracttype`]: crate::contracttype
//! [v28_contracttype_unpacking]: super::v28_contracttype_unpacking
