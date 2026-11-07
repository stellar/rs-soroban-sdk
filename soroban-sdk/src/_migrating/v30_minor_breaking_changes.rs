//! Minor breaking changes in v30.
//!
//! These changes are unlikely to affect most contracts, but may require changes to some contracts,
//! tests, or tools.
//!
//! ## `#[contracttype]` integer enums can't be passed to `panic_with_error!`
//!
//! `#[contracttype]` integer enums can no longer be passed to [`panic_with_error!`]. Use
//! [`#[contracterror]`][contracterror].
//!
//! Their support, for integer enums that implement a conversion into [`Error`], was accidental.
//! Only [`contracterror`] types are intended to be used with [`panic_with_error!`] and
//! [`Env::panic_with_error`].
//!
//! Before:
//!
//! ```compile_fail
//! use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Env};
//!
//! #[contracttype]
//! #[derive(Copy, Clone, Debug, Eq, PartialEq)]
//! #[repr(u32)]
//! pub enum ContractError {
//!     NotFound = 1,
//! }
//!
//! impl From<ContractError> for soroban_sdk::Error {
//!     fn from(e: ContractError) -> Self {
//!         soroban_sdk::Error::from_contract_error(e as u32)
//!     }
//! }
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn get(env: Env) {
//!         panic_with_error!(&env, ContractError::NotFound); // ❌ not a contracterror
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! After:
//!
//! ```
//! use soroban_sdk::{contract, contracterror, contractimpl, panic_with_error, Env};
//!
//! #[contracterror] // 👈 👀
//! #[derive(Copy, Clone, Debug, Eq, PartialEq)]
//! #[repr(u32)]
//! pub enum ContractError {
//!     NotFound = 1,
//! }
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn get(env: Env) {
//!         panic_with_error!(&env, ContractError::NotFound);
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! ## `#[contract]` types can't have fields or generics
//!
//! `#[contract]` types must be `struct Contract;` or `struct Contract {}`, with no fields or
//! generics.
//!
//! The SDK never constructs the contract type, so fields were silently ignored, and generics caused
//! [`contractimpl`] to fail with an unhelpful error. [`contractimpl`] also no longer accepts a
//! generic type. Store state in [`Env::storage`] instead of in fields.
//!
//! Before:
//!
//! ```compile_fail
//! use soroban_sdk::{contract, contractimpl, Address};
//!
//! #[contract]
//! pub struct Contract {
//!     admin: Address, // ❌ fields are not supported
//! }
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn hello() {}
//! }
//! # fn main() {}
//! ```
//!
//! After:
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};
//!
//! #[contract]
//! pub struct Contract; // 👈 👀
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn __constructor(env: Env, admin: Address) {
//!         env.storage().instance().set(&symbol_short!("admin"), &admin);
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! ## Hand-written `Event` impls can't be published
//!
//! Hand-written [`Event`] impls can't `publish()`. Use [`#[contractevent]`][contractevent].
//!
//! Publishing an event now also includes the event's spec in the contract (see
//! [v30_spec_markers]), which only [`contractevent`] generates.
//!
//! Before:
//!
//! ```compile_fail
//! use soroban_sdk::{
//!     contract, contractimpl, symbol_short, vec, Env, Event, IntoVal, Val, Vec,
//! };
//!
//! pub struct Ping {
//!     pub count: u32,
//! }
//!
//! impl Event for Ping {
//!     fn topics(&self, env: &Env) -> Vec<Val> {
//!         vec![env, symbol_short!("ping").into_val(env)]
//!     }
//!     fn data(&self, env: &Env) -> Val {
//!         self.count.into_val(env)
//!     }
//! }
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn ping(env: Env) {
//!         Ping { count: 1 }.publish(&env); // ❌ not a contractevent
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! After:
//!
//! ```
//! use soroban_sdk::{contract, contractevent, contractimpl, Env};
//!
//! #[contractevent(data_format = "single-value")] // 👈 👀
//! pub struct Ping {
//!     pub count: u32,
//! }
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn ping(env: Env) {
//!         Ping { count: 1 }.publish(&env);
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! ## `soroban_sdk::xdr` is stellar-xdr v30
//!
//! [`soroban_sdk::xdr`][crate::xdr] is now stellar-xdr v30. [`ScAddress`] has a new
//! [`MuxedContract`][ScAddress::MuxedContract] variant.
//!
//! The variant holds a muxed contract address (see [v30_muxed_contracts]). Code that matches
//! exhaustively on [`ScAddress`], or on other XDR types changed by Protocol 30, needs to handle the
//! new variants.
//!
//! Before:
//!
//! ```compile_fail
//! use soroban_sdk::xdr::ScAddress;
//!
//! fn kind(address: &ScAddress) -> &'static str {
//!     match address { // ❌ MuxedContract is not covered
//!         ScAddress::Account(_) => "account",
//!         ScAddress::Contract(_) => "contract",
//!         ScAddress::MuxedAccount(_) => "muxed account",
//!         ScAddress::ClaimableBalance(_) => "claimable balance",
//!         ScAddress::LiquidityPool(_) => "liquidity pool",
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! After:
//!
//! ```
//! use soroban_sdk::xdr::ScAddress;
//!
//! fn kind(address: &ScAddress) -> &'static str {
//!     match address {
//!         ScAddress::Account(_) => "account",
//!         ScAddress::Contract(_) => "contract",
//!         ScAddress::MuxedAccount(_) => "muxed account",
//!         ScAddress::MuxedContract(_) => "muxed contract", // 👈 👀
//!         ScAddress::ClaimableBalance(_) => "claimable balance",
//!         ScAddress::LiquidityPool(_) => "liquidity pool",
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! ## Tests default to protocol 30
//!
//! Tests default to protocol 30, so test snapshots change.
//!
//! The ledger of an [`Env`] created in tests has protocol version 30 instead of 28. Test snapshot
//! JSON files record the protocol version, so they change the next time the tests run. Commit the
//! updated snapshots. A test that depends on the behaviour of an earlier protocol can set the
//! protocol version itself.
//!
//! ```
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//! use soroban_sdk::{testutils::Ledger as _, Env};
//!
//! let env = Env::default();
//! assert_eq!(env.ledger().protocol_version(), 30);
//!
//! // To test against an earlier protocol.
//! env.ledger().set_protocol_version(29);
//! assert_eq!(env.ledger().protocol_version(), 29);
//! # }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() {}
//! ```
//!
//! ## `spec_xdr` fns return names before they are processed
//!
//! `spec_xdr` functions return entries with full-path names, before `stellar contract build`
//! processes them.
//!
//! Tools that assemble a spec from the `spec_xdr` fns need to reduce the names with
//! `soroban_spec::reduce::reduce`, as `stellar contract build` does. See
//! [v30_spec_xdr_unprocessed] for details.
//!
//! ```
//! use soroban_sdk::{
//!     contracttype,
//!     xdr::{Limits, ReadXdr, ScSpecEntry},
//! };
//!
//! #[contracttype]
//! pub struct State {
//!     pub count: u32,
//! }
//!
//! # fn main() {
//! let entry = ScSpecEntry::from_xdr(State::spec_xdr(), Limits::none()).unwrap();
//! let ScSpecEntry::UdtStructV0(s) = entry else { panic!() };
//! let name = s.name.to_utf8_string_lossy();
//! // 👇 👀 The full path, such as ::my_contract::State, not State.
//! assert!(name.starts_with("::"));
//! assert!(name.ends_with("::State"));
//! # }
//! ```
//!
//! ## `soroban-token-spec` and `stellar-asset-spec` `xdr` fns
//!
//! `soroban-token-spec` and `stellar-asset-spec` now require std, and their `xdr()` returns reduced
//! names and isn't `const`. Use `xdr_unprocessed()` for the `const` spec.
//!
//! `xdr()` returns the spec as it appears in a built contract, with short names. The spec with
//! full-path names is returned by the `const` fn `xdr_unprocessed()`. See
//! [v30_spec_xdr_unprocessed] for details.
//!
//! Before:
//!
//! ```ignore
//! const SPEC: &[u8] = soroban_token_spec::xdr(); // ❌ no longer const
//! ```
//!
//! After:
//!
//! ```ignore
//! // The spec as it appears in a built contract.
//! let spec: Vec<u8> = soroban_token_spec::xdr();
//!
//! // The spec before names are reduced, in a const.
//! const SPEC_UNPROCESSED: &[u8] = soroban_token_spec::xdr_unprocessed(); // 👈 👀
//! ```
//!
//! ## `contractimport!` events use their declared data format
//!
//! [`contractimport!`] events use their declared data format instead of always `map`.
//!
//! An event declared with a `data_format` of `single-value` or `vec` previously encoded its data as
//! a map when the event type was imported into another contract with [`contractimport!`], and so
//! did not match the events the imported contract publishes. Events published with imported event
//! types now encode their data in the declared format.
//!
//! For example, an event declared in one contract as:
//!
//! ```
//! use soroban_sdk::{contractevent, Address};
//!
//! #[contractevent(data_format = "single-value")]
//! pub struct Transfer {
//!     #[topic]
//!     pub from: Address,
//!     pub amount: i128,
//! }
//! # fn main() {}
//! ```
//!
//! When imported with [`contractimport!`] and published by another contract, previously had data
//! encoded as the map `{ amount: i128 }`, and now has data encoded as the single value
//! `amount: i128`, the same as when published by the contract that declares it.
//!
//! ## Test clients restore auths after a call panics
//!
//! Test clients restore `mock_auths`, `set_auths`, and `mock_all_auths` after a call panics.
//!
//! The auths set on a generated client for one call are restored to the previous auths when the
//! call returns. Previously if the call panicked, such as inside [`std::panic::catch_unwind`], the
//! auths were not restored and stayed in effect for the calls that followed. Tests that relied on
//! the auths staying in effect need to set them again.
//!
//! ```
//! # #[cfg(feature = "testutils")]
//! # mod test {
//! use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, Env};
//! use std::panic::{catch_unwind, AssertUnwindSafe};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn fail() {
//!         panic!("failed")
//!     }
//!
//!     pub fn need_auth(user: Address) {
//!         user.require_auth();
//!     }
//! }
//!
//! pub fn test() {
//!     let env = Env::default();
//!     let contract_id = env.register(Contract, ());
//!     let client = ContractClient::new(&env, &contract_id);
//!     let user = Address::generate(&env);
//!
//!     let result = catch_unwind(AssertUnwindSafe(|| {
//!         client.mock_all_auths().fail();
//!     }));
//!     assert!(result.is_err());
//!
//!     // 👇 👀 The mocked auths no longer apply after the panicking call.
//!     assert!(client.try_need_auth(&user).is_err());
//! }
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//! #     test::test();
//! # }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() {}
//! ```
//!
//! ## `MuxedAddress::try_from_val` returns an error for non-address values
//!
//! `MuxedAddress::try_from_val` returns an error instead of panicking on non-address values.
//!
//! Converting an [`ScVal`][crate::xdr::ScVal] that is not an address into a [`MuxedAddress`]
//! returns a [`ConversionError`], the same as converting it into an [`Address`].
//!
//! ```
//! use soroban_sdk::{xdr::ScVal, Env, MuxedAddress, TryFromVal};
//!
//! # fn main() {
//! let env = Env::default();
//! let val = ScVal::U32(1);
//! // 👇 👀 Previously panicked.
//! assert!(MuxedAddress::try_from_val(&env, &val).is_err());
//! # }
//! ```
//!
//! [v30_muxed_contracts]: super::v30_muxed_contracts
//! [v30_spec_markers]: super::v30_spec_markers
//! [v30_spec_xdr_unprocessed]: super::v30_spec_xdr_unprocessed
//! [`panic_with_error!`]: crate::panic_with_error
//! [`Env::panic_with_error`]: crate::Env::panic_with_error
//! [`Env::storage`]: crate::Env::storage
//! [`Env`]: crate::Env
//! [`Error`]: crate::Error
//! [`Event`]: crate::Event
//! [`MuxedAddress`]: crate::MuxedAddress
//! [`Address`]: crate::Address
//! [`ConversionError`]: crate::ConversionError
//! [`ScAddress`]: crate::xdr::ScAddress
//! [ScAddress::MuxedContract]: crate::xdr::ScAddress::MuxedContract
//! [`contractimport!`]: crate::contractimport
//! [contracttype]: crate::contracttype
//! [contracterror]: crate::contracterror
//! [`contracterror`]: crate::contracterror
//! [contractevent]: crate::contractevent
//! [`contractevent`]: crate::contractevent
//! [`contractimpl`]: crate::contractimpl
