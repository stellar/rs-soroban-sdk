//! [`contractevent`] events with a map data format omit void fields when publishing.
//!
//! An event declared with `data_format = "map"`, which is the default, publishes its data fields as
//! a map keyed by the field names. Every field used to be written to that map, including fields
//! whose value is void, such as an [`Option`] field that is `None`.
//!
//! In v28 a field whose value is void is omitted from the map, so the published event carries only
//! the fields that hold a value.
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
//!     let event = Transfer {
//!         to: symbol_short!("to"),
//!         to_muxed_id: None,
//!         amount: 1,
//!     };
//!     env.as_contract(&id, || {
//!         event.publish(&env);
//!     });
//!
//!     assert_eq!(env.events().all(), [event.to_xdr(&env, &id)]);
//!
//!     // The published data map holds only amount. The to_muxed_id field is
//!     // None, which is void, and so is omitted from the map. Before v28 the
//!     // map also held to_muxed_id with a void value.
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
//! map, because a field it expects to be present with a void value is now absent.
//!
//! A test that asserts on the data map of an event that has a void field needs updating, because
//! the field it asserted on is no longer in the map. Rather than asserting on each field, compare
//! the event with its `to_xdr` form, which is built the same way as the published event and so
//! stays correct as the packing changes:
//!
//! ```ignore
//! assert_eq!(env.events().all(), std::vec![event.to_xdr(&env, &id)]);
//! ```
//!
//! An event type can carry a field that is only sometimes meaningful without paying for it in every
//! event, so a single event can serve the shapes a contract needs rather than one event type per
//! shape.
//!
//! [`contractevent`]: crate::contractevent
//! [`contracttype`]: crate::contracttype
