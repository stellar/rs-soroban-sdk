//! [`contractevent`] replaces [`Events::publish`].
//!
//! The [`contractevent`] macro provides a type-safe way to define and publish events, and
//! includes the event into the contract interface specification so that tooling, SDKs, and
//! generated clients can understand the events published.
//!
//! ## Example
//!
//! For example, consider the following event publishing code:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Address, IntoVal, Map, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123u32;
//! #     env.as_contract(&id, || {
//! // Define and publish the event:
//! env.events().publish(
//!     // Event topics
//!     (symbol_short!("increment"), &addr),
//!     // Event data
//!     Map::<Symbol, Val>::from_array(&env, [
//!         (symbol_short!("count"), count.into())
//!     ]),
//! );
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("increment"), &addr).into_val(&env),
//!             // Event data
//!             Map::<Symbol, Val>::from_array(&env, [
//!                 (symbol_short!("count"), count.into())
//!             ]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! Replace it with the following code using [`contractevent`]:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Map, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123;
//! #     env.as_contract(&id, || {
//! // Define the event:
//! #[contractevent]
//! pub struct Increment {
//!   #[topic]
//!   addr: Address,
//!   count: u32,
//! }
//!
//! // Publish the event:
//! Increment {
//!   addr: addr.clone(),
//!   count: count,
//! }.publish(&env);
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("increment"), &addr).into_val(&env),
//!             // Event data
//!             Map::<Symbol, Val>::from_array(&env, [
//!                 (symbol_short!("count"), count.into())
//!             ]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! ## Example: Vec Data
//!
//! By default the parameters not marked as `#[topic]`s are collected into a [`Map`] like in the
//! example above. If transitioning events that publish parameters in a [`Vec`], follow the
//! this example.
//!
//! Consider the following event publishing code:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Address, IntoVal, Vec, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123u32;
//! #     env.as_contract(&id, || {
//! // Define and publish the event:
//! env.events().publish(
//!     // Event topics
//!     (symbol_short!("increment"), &addr),
//!     // Event data
//!     Vec::<Val>::from_array(&env, [count.into()]),
//! );
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("increment"), &addr).into_val(&env),
//!             // Event data
//!             Vec::<Val>::from_array(&env, [count.into()]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! Replace it with the following code using [`contractevent`]:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Vec, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123;
//! #     env.as_contract(&id, || {
//! // Define the event:
//! #[contractevent(data_format = "vec")]
//! pub struct Increment {
//!   #[topic]
//!   addr: Address,
//!   count: u32,
//! }
//!
//! // Publish the event:
//! Increment {
//!   addr: addr.clone(),
//!   count: count,
//! }.publish(&env);
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("increment"), &addr).into_val(&env),
//!             // Event data
//!             Vec::<Val>::from_array(&env, [count.into()]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! ## Example: Other Data
//!
//! If transitioning events that publish some other type directly into the event's data field,
//! follow the this example.
//!
//! Consider the following event publishing code:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Address, IntoVal, Vec, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123u32;
//! #     env.as_contract(&id, || {
//! // Define and publish the event:
//! env.events().publish(
//!     // Event topics
//!     (symbol_short!("increment"), &addr),
//!     // Event data
//!     count,
//! );
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("increment"), &addr).into_val(&env),
//!             // Event data
//!             count.into(),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! Replace it with the following code using [`contractevent`]:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Map, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123;
//! #     env.as_contract(&id, || {
//! // Define the event:
//! #[contractevent(data_format = "single-value")]
//! pub struct Increment {
//!   #[topic]
//!   addr: Address,
//!   count: u32,
//! }
//!
//! // Publish the event:
//! Increment {
//!   addr: addr.clone(),
//!   count: count,
//! }.publish(&env);
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("increment"), &addr).into_val(&env),
//!             // Event data
//!             count.into(),
//!         ),
//!     ]
//! );
//! # }
//! ```
//! ## Example: Customising Topics
//!
//! By default the topics of an event are made up of a single static topic that is the event's name
//! converted to snake_case, along with any dynamic topics specified by `#[topic]` on the field.
//!
//! ### Custom Static Topic
//!
//! The static topic can be changed using the `topics = [...]` option on [`contractevent`]:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Map, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123;
//! #     env.as_contract(&id, || {
//! #[contractevent(topics = ["count_chn"])]
//! pub struct Increment {
//!   #[topic]
//!   addr: Address,
//!   count: u32,
//! }
//! #
//! #         Increment {
//! #           addr: addr.clone(),
//! #           count: count,
//! #         }.publish(&env);
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("count_chn"), &addr,).into_val(&env),
//!             // Event data
//!             Map::<Symbol, Val>::from_array(&env, [
//!                 (symbol_short!("count"), count.into())
//!             ]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! ### Multiple Static Topics
//!
//! Multiple static topics can be set using the `topics = [...]` option on [`contractevent`], with
//! up to two values:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Map, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123;
//! #     env.as_contract(&id, || {
//! #[contractevent(topics = ["count", "increment"])]
//! pub struct Increment {
//!   #[topic]
//!   addr: Address,
//!   count: u32,
//! }
//! #
//! #         Increment {
//! #           addr: addr.clone(),
//! #           count: count,
//! #         }.publish(&env);
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (symbol_short!("count"), symbol_short!("increment"), &addr,).into_val(&env),
//!             // Event data
//!             Map::<Symbol, Val>::from_array(&env, [
//!                 (symbol_short!("count"), count.into())
//!             ]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! ### No Static Topics
//!
//! Zero static topics can be specified with the following configuration where `topics = []` is
//! provided to [`contractevent`]:
//!
//! ```
//! # #![cfg(feature = "testutils")]
//! # use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, vec, Env, Address, IntoVal, Map, Symbol, Val, testutils::{Address as _, Events as _}};
//! #
//! # #[contract]
//! # pub struct Contract;
//! #
//! # fn main() {
//! #     let env = Env::default();
//! #     let id = env.register(Contract, ());
//! #     let addr = Address::generate(&env);
//! #     let count = 123;
//! #     env.as_contract(&id, || {
//! #[contractevent(topics = [])]
//! pub struct Increment {
//!   #[topic]
//!   addr: Address,
//!   count: u32,
//! }
//! #
//! #         Increment {
//! #           addr: addr.clone(),
//! #           count: count,
//! #         }.publish(&env);
//! #     });
//!
//! // Assert in tests on the published topics and data:
//! assert_eq!(
//!     env.events().all(),
//!     vec![&env,
//!         (
//!             id.clone(),
//!             // Event topics
//!             (&addr,).into_val(&env),
//!             // Event data
//!             Map::<Symbol, Val>::from_array(&env, [
//!                 (symbol_short!("count"), count.into())
//!             ]).into_val(&env),
//!         ),
//!     ]
//! );
//! # }
//! ```
//!
//! [`Events::publish`]: crate::events::Events::publish
//! [`Address`]: crate::MuxedAddress
//! [`MuxedAddress`]: crate::MuxedAddress
//! [`contractevent`]: crate::contractevent
//! [`Map`]: crate::Map
