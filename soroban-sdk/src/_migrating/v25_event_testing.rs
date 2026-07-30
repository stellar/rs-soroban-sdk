//! `Events::all()` return type changed from `Vec<(Address, Vec<Val>, Val)>` to [`ContractEvents`].
//!
//! The [`ContractEvents`] struct provides a more ergonomic interface for asserting on
//! emitted events. It can be compared directly with:
//! - `[xdr::ContractEvent; _]`
//! - `std::vec::Vec<xdr::ContractEvent>`
//! - `Vec<(Address, Vec<Val>, Val)>` (maintains backward compatibility with the old format)
//!
//! The [`ContractEvents`] struct also provides utility methods:
//! - `filter_by_contract` - filter events by contract address
//! - `events` - get the underlying XDR events
//!
//! Additionally, events defined with [`contractevent`] now have a `to_xdr` method available with
//! `testutils` feature that converts the event to its XDR representation for comparison.
//!
//! ## Example: Using the old comparison style
//!
//! The old comparison style using `Vec<(Address, Vec<Val>, Val)>` still works:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     env.as_contract(&id, || {
//! #[contractevent]
//! pub struct MyEvent {
//!     #[topic]
//!     name: Symbol,
//!     value: u32,
//! }
//!
//! MyEvent {
//!     name: symbol_short!("hello"),
//!     value: 42,
//! }.publish(&env);
//! #     });
//!
//! // The old comparison style still works:
//! use soroban_sdk::Map;
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             (symbol_short!("my_event"), symbol_short!("hello")).into_val(&env),
//!             Map::<Symbol, Val>::from_array(&env, [
//!                 (symbol_short!("value"), 42u32.into())
//!             ]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! ## Example: Using XDR comparison (new recommended style)
//!
//! The new style uses `to_xdr` on the event for cleaner assertions:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Symbol, Val, testutils::{Address as _, Events as _}, Event};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #
//! #[contractevent]
//! pub struct MyEvent {
//!     #[topic]
//!     name: Symbol,
//!     value: u32,
//! }
//!
//! let event = MyEvent {
//!     name: symbol_short!("hello"),
//!     value: 42,
//! };
//!
//! #     env.as_contract(&id, || {
//! event.publish(&env);
//! #     });
//!
//! // New style: compare with XDR directly
//! assert_eq!(
//!     env.events().all(),
//!     std::vec![event.to_xdr(&env, &id)],
//! );
//! # }
//! ```
//!
//! ## Example: Filtering events by contract
//!
//! Use `filter_by_contract` to get events from a specific contract:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Symbol, Val, testutils::{Address as _, Events as _}, Event};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let contract_a = env.register(Contract, ());
//! #     let contract_b = env.register(Contract, ());
//! #
//! #[contractevent]
//! pub struct MyEvent {
//!     #[topic]
//!     name: Symbol,
//!     value: u32,
//! }
//!
//! let event_a = MyEvent {
//!     name: symbol_short!("hello"),
//!     value: 1,
//! };
//! let event_b = MyEvent {
//!     name: symbol_short!("world"),
//!     value: 2,
//! };
//!
//! env.as_contract(&contract_a, || {
//!     event_a.publish(&env);
//!     env.as_contract(&contract_b, || {
//!         event_b.publish(&env);
//!     });
//! });
//!
//! // Filter to get only events from contract_a
//! assert_eq!(
//!     env.events().all().filter_by_contract(&contract_a),
//!     std::vec![event_a.to_xdr(&env, &contract_a)],
//! );
//!
//! // Filter to get only events from contract_b
//! assert_eq!(
//!     env.events().all().filter_by_contract(&contract_b),
//!     std::vec![event_b.to_xdr(&env, &contract_b)],
//! );
//! # }
//! ```
//!
//! [`ContractEvents`]: crate::testutils::ContractEvents
//! [`contractevent`]: crate::contractevent
