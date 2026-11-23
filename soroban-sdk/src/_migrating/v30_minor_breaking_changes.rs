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
//!
//! ## `spec_xdr` fns return names before they are processed
//!
//! `spec_xdr` functions return entries with full-path names. For example, the 
//! spec entry for a `#[contracttype]` struct `State` will now be named 
//! `::my_contract::State`, not `State`.
//!
//! Tools that assemble a spec from the `spec_xdr` fns need to reduce the names 
//! with `soroban_spec::reduce::reduce`, as `stellar contract build` does. See 
//! [v30_spec_xdr_unprocessed] for details.
//!
//! When building with the Stellar CLI's `stellar contract build`, the resulting 
//! spec will already be reduced, so no action is needed.
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
