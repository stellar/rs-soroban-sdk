//! Soroban SDK supports writing programs for the Soroban smart contract
//! platform.
//!
//! ### Docs
//!
//! See [soroban.stellar.org](https://soroban.stellar.org) for documentation.
//!
//! ### Examples
//!
//! ```rust
//! use soroban_sdk::{contractimpl, symbol, vec, BytesN, Env, Symbol, Vec};
//!
//! pub struct HelloContract;
//!
//! #[contractimpl]
//! impl HelloContract {
//!     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
//!         vec![&env, symbol!("Hello"), to]
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_id = env.register_contract(None, HelloContract);
//!     let client = HelloContractClient::new(&env, &contract_id);
//!
//!     let words = client.hello(&symbol!("Dev"));
//!
//!     assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! More examples are available at <https://soroban.stellar.org/docs/category/examples>.

#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

#[cfg(not(target_family = "wasm"))]
extern crate std;

#[cfg(all(target_family = "wasm", feature = "testutils"))]
compile_error!("'testutils' feature is not supported on 'wasm' target");

#[cfg(target_family = "wasm")]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// __link_sections returns and does nothing, but it contains link sections that
/// should be ensured end up in the final build of any contract using the SDK.
///
/// In Rust's build system sections only get included into the final build if
/// the object file containing those sections are processed by the linker, but
/// as an optimization step if no code is called in an object file it is
/// discarded.  This has the unfortunate effect of causing anything else in
/// those object files, such as link sections, to be discarded. Placing anything
/// that must be included in the build inside an exported function ensures the
/// object files won't be discarded. wasm-bindgen does a similar thing to this,
/// and so this seems to be a reasonably accepted way to work around this
/// limitation in the build system.
///
/// This has an unfortunate side-effect that all contracts will have a function
/// in the resulting WASM named `_`, however this function won't be rendered in
/// the contract specification. The overhead of this is very minimal on file
/// size.
///
/// See https://github.com/stellar/rs-soroban-sdk/issues/383 for more details.
#[cfg(target_family = "wasm")]
#[export_name = "_"]
fn __link_sections() {
    #[link_section = "contractenvmetav0"]
    static __ENV_META_XDR: [u8; env::meta::XDR.len()] = env::meta::XDR;
}

#[doc(hidden)]
pub use bytes_lit::bytes as __bytes_lit_bytes;
#[doc(hidden)]
pub use bytes_lit::bytesmin as __bytes_lit_bytesmin;

/// Generates conversions from the repr(u32) enum from/into a `Status`.
///
/// There are some constraints on the types that are supported:
/// - Enum must derive `Copy`.
/// - Enum variants must have an explicit integer literal.
/// - Enum variants must have a value convertible to u32.
///
/// Includes the type in the contract spec so that clients can generate bindings
/// for the type.
///
/// ### Examples
///
/// Defining an error and capturing errors using the `try_` variant.
///
/// ```
/// use soroban_sdk::{contracterror, contractimpl, Env};
///
/// #[contracterror]
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// #[repr(u32)]
/// pub enum Error {
///     MyError = 1,
///     AnotherError = 2,
/// }
///
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     pub fn causeerror(env: Env) -> Result<(), Error> {
///         Err(Error::MyError)
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///
///     // Register the contract defined in this crate.
///     let contract_id = env.register_contract(None, Contract);
///
///     // Create a client for calling the contract.
///     let client = ContractClient::new(&env, &contract_id);
///
///     // Invoke contract causeerror function, but use the try_ variant that
///     // will capture the error so we can inspect.
///     let result = client.try_causeerror();
///     assert_eq!(result, Err(Ok(Error::MyError)));
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
///
/// Testing invocations that cause errors with `should_panic` instead of `try_`.
///
/// ```should_panic
/// # use soroban_sdk::{contracterror, contractimpl, Env};
/// #
/// # #[contracterror]
/// # #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// # #[repr(u32)]
/// # pub enum Error {
/// #     MyError = 1,
/// #     AnotherError = 2,
/// # }
/// #
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn causeerror(env: Env) -> Result<(), Error> {
/// #         Err(Error::MyError)
/// #     }
/// # }
/// #
/// #[test]
/// #[should_panic(expected = "ContractError(1)")]
/// fn test() {
/// # panic!("ContractError(1)");
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///
///     // Register the contract defined in this crate.
///     let contract_id = env.register_contract(None, Contract);
///
///     // Create a client for calling the contract.
///     let client = ContractClient::new(&env, &contract_id);
///
///     // Invoke contract causeerror function.
///     client.causeerror();
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
pub use soroban_sdk_macros::contracterror;

/// Import a contract from its WASM file, generating a client, types, and
/// constant holding the contract file.
///
/// Generates in the current module:
/// - A `Contract` trait that matches the contracts interface.
/// - A `ContractClient` struct that has functions for each function in the
/// contract.
/// - Types for all contract types defined in the contract.
///
/// ### Examples
///
/// ```ignore
/// use soroban_sdk::{contractimpl, BytesN, Env, Symbol};
///
/// mod contract_a {
///     soroban_sdk::contractimport!(file = "contract_a.wasm");
/// }
///
/// pub struct ContractB;
///
/// #[contractimpl]
/// impl ContractB {
///     pub fn add_with(env: Env, contract_id: BytesN<32>, x: u32, y: u32) -> u32 {
///         let client = contract_a::ContractClient::new(&env, contract_id);
///         client.add(&x, &y)
///     }
/// }
///
/// #[test]
/// fn test() {
///     let env = Env::default();
///
///     // Register contract A using the imported WASM.
///     let contract_a_id = env.register_contract_wasm(None, contract_a::WASM);
///
///     // Register contract B defined in this crate.
///     let contract_b_id = env.register_contract(None, ContractB);
///
///     // Create a client for calling contract B.
///     let client = ContractBClient::new(&env, &contract_b_id);
///
///     // Invoke contract B via its client.
///     let sum = client.add_with(&contract_a_id, &5, &7);
///     assert_eq!(sum, 12);
/// }
/// ```
pub use soroban_sdk_macros::contractimport;

/// Exports the publicly accessible functions to the Soroban environment.
///
/// Functions that are publicly accessible in the implementation are invocable
/// by other contracts, or directly by transactions, when deployed.
///
/// ### Examples
///
/// Define a contract with one function, `hello`, and call it from within a test
/// using the generated client.
///
/// ```
/// use soroban_sdk::{contractimpl, symbol, vec, BytesN, Env, Symbol, Vec};
///
/// pub struct HelloContract;
///
/// #[contractimpl]
/// impl HelloContract {
///     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
///         vec![&env, symbol!("Hello"), to]
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register_contract(None, HelloContract);
///     let client = HelloContractClient::new(&env, &contract_id);
///
///     let words = client.hello(&symbol!("Dev"));
///
///     assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
pub use soroban_sdk_macros::contractimpl;

/// Generates conversions from the struct/enum from/into a `RawVal`.
///
/// There are some constraints on the types that are supported:
/// - Enums with integer values must have an explicit integer literal for every
/// variant.
/// - Enums with unit variants are supported.
/// - Enums with tuple-like variants with a maximum of one tuple field are
/// supported. The tuple field must be of a type that is also convertible to and
/// from `RawVal`.
/// - Enums with struct-like variants are not supported.
/// - Structs are supported. All fields must be of a type that is also
/// convertible to and from `RawVal`.
/// - All variant names, field names, and type names must be 10-characters or
/// less in length.
///
/// Includes the type in the contract spec so that clients can generate bindings
/// for the type.
///
/// ### Examples
///
/// Defining a contract type that is a struct and use it in a contract.
///
/// ```
/// #![no_std]
/// use soroban_sdk::{contractimpl, contracttype, symbol, Env, Symbol};
///
/// #[contracttype]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct State {
///     pub count: u32,
///     pub last_incr: u32,
/// }
///
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     /// Increment increments an internal counter, and returns the value.
///     pub fn increment(env: Env, incr: u32) -> u32 {
///         // Get the current count.
///         let mut state = Self::get_state(env.clone());
///
///         // Increment the count.
///         state.count += incr;
///         state.last_incr = incr;
///
///         // Save the count.
///         env.data().set(symbol!("STATE"), &state);
///
///         // Return the count to the caller.
///         state.count
///     }
///
///     /// Return the current state.
///     pub fn get_state(env: Env) -> State {
///         env.data()
///             .get(symbol!("STATE"))
///             .unwrap_or_else(|| Ok(State::default())) // If no value set, assume 0.
///             .unwrap() // Panic if the value of COUNTER is not a State.
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register_contract(None, Contract);
///     let client = ContractClient::new(&env, &contract_id);
///
///     assert_eq!(client.increment(&1), 1);
///     assert_eq!(client.increment(&10), 11);
///     assert_eq!(
///         client.get_state(),
///         State {
///             count: 11,
///             last_incr: 10,
///         },
///     );
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
///
/// Defining contract types that are three different types of enums and using
/// them in a contract.
///
/// ```
/// #![no_std]
/// use soroban_sdk::{contractimpl, contracttype, symbol, Env};
///
/// /// A tuple enum is stored as a two-element vector containing the name of
/// /// the enum variant as a Symbol, then the value in the tuple.
/// #[contracttype]
/// #[derive(Clone, Debug, Eq, PartialEq)]
/// pub enum Color {
///     Red(Intensity),
///     Blue(Shade),
/// }
///
/// /// A unit enum is stored as a single-element vector containing the name of
/// /// the enum variant as a Symbol.
/// #[contracttype]
/// #[derive(Clone, Debug, Eq, PartialEq)]
/// pub enum Shade {
///     Light,
///     Dark,
/// }
///
/// /// An integer enum is stored as its integer value.
/// #[contracttype]
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// #[repr(u32)]
/// pub enum Intensity {
///     Low = 1,
///     High = 2,
/// }
///
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     /// Set the color.
///     pub fn set(env: Env, c: Color) {
///         env.data().set(symbol!("COLOR"), c);
///     }
///
///     /// Get the color.
///     pub fn get(env: Env) -> Option<Color> {
///         env.data()
///             .get(symbol!("COLOR"))
///             .map(Result::unwrap) // Panic if the value of COLOR is not a Color.
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register_contract(None, Contract);
///     let client = ContractClient::new(&env, &contract_id);
///
///     assert_eq!(client.get(), None);
///
///     client.set(&Color::Red(Intensity::High));
///     assert_eq!(client.get(), Some(Color::Red(Intensity::High)));
///
///     client.set(&Color::Blue(Shade::Light));
///     assert_eq!(client.get(), Some(Color::Blue(Shade::Light)));
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
pub use soroban_sdk_macros::contracttype;

/// Generates a client for a contract trait.
///
/// Can be used to create clients for contracts that live outside the current
/// crate, using a trait that has been published as a standard or shared
/// interface.
///
/// Primarily useful when needing to generate a client for someone elses
/// contract where they have only shared a trait interface.
///
/// Note that [`contractimpl`] also automatically generates a client, and so it
/// is unnecessary to use [`contractclient`] for contracts that live in the
/// current crate.
///
/// Note that [`contractimport`] also automatically generates a client when
/// importing someone elses contract where they have shared a .wasm file.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{contractclient, contractimpl, symbol, vec, BytesN, Env, Symbol, Vec};
///
/// #[contractclient(name = "Client")]
/// pub trait HelloInteface {
///     fn hello(env: Env, to: Symbol) -> Vec<Symbol>;
/// }
///
/// pub struct HelloContract;
///
/// #[contractimpl]
/// impl HelloContract {
///     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
///         vec![&env, symbol!("Hello"), to]
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///
///     // Register the hello contract.
///     let contract_id = env.register_contract(None, HelloContract);
///
///     // Create a client for the hello contract, that was constructed using
///     // the trait.
///     let client = Client::new(&env, &contract_id);
///
///     let words = client.hello(&symbol!("Dev"));
///
///     assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
pub use soroban_sdk_macros::contractclient;

/// Import a contract from its WASM file, generating a constant holding the
/// contract file.
///
/// Note that [`contractimport`] also automatically imports the contract file
/// into a constant, and so it is usually unnecessary to use [`contractfile`]
/// directly, unless you specifically want to only load the contract file
/// without generating a client for it.
pub use soroban_sdk_macros::contractfile;

/// Create a [Symbol] with the given string.
///
/// A symbol's maximum length is 10 characters.
///
/// Valid characters are `a-zA-Z0-9_`.
///
/// The [Symbol] is generated at compile time and returned as a const.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{symbol, Symbol};
///
/// let symbol = symbol!("a_str");
/// assert_eq!(symbol, Symbol::from_str("a_str"));
/// ```
///
/// ```
/// use soroban_sdk::{symbol, Symbol};
///
/// const symbol: Symbol = symbol!("a_str");
/// assert_eq!(symbol, Symbol::from_str("a_str"));
/// ```
pub use soroban_sdk_macros::symbol;

/// Panic with the given error.
///
/// The first argument in the list must be a reference to an [Env].
///
/// The second argument is an error value. The error value will be given to any
/// calling contract.
///
/// Equivalent to `panic!`, but with an error value instead of a string. The
/// error value will be given to any calling contract.
///
/// See [`contracterror`] for how to define an error type.
#[macro_export]
macro_rules! panic_with_error {
    ($env:expr, $error:expr) => {{
        $env.panic_with_error($error);
        unreachable!();
    }};
}

#[doc(hidden)]
#[deprecated(note = "use panic_with_error!")]
#[macro_export]
macro_rules! panic_error {
    ($env:expr, $error:expr) => {{
        $crate::panic_with_error!($env, $error);
    }};
}

/// Assert a condition and panic with the given error if it is false.
///
/// The first argument in the list must be a reference to an [Env].
///
/// The second argument is an expression that if resolves to `false` will cause
/// a panic with the error in the third argument.
///
/// The third argument is an error value. The error value will be given to any
/// calling contract.
///
/// Equivalent to `assert!`, but with an error value instead of a string. The
/// error value will be given to any calling contract.
///
/// See [`contracterror`] for how to define an error type.
#[macro_export]
macro_rules! assert_with_error {
    ($env:expr, $cond:expr, $error:expr) => {{
        if !($cond) {
            $crate::panic_with_error!($env, $error);
        }
    }};
}

#[doc(hidden)]
pub mod unwrap;

mod env;

mod address;
pub mod xdr;

pub use env::ConversionError;

pub use env::Env;
/// Raw value of the Soroban smart contract platform that types can be converted
/// to and from for storing, or passing between contracts.
///
pub use env::RawVal;

/// Used to do conversions between values in the Soroban environment.
pub use env::FromVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::IntoVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::TryFromVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::TryIntoVal;

pub use env::Symbol;

mod envhidden {
    pub use super::env::Object;
    pub use super::env::Status;
}
#[doc(hidden)]
pub use envhidden::*;

#[doc(hidden)]
#[deprecated(note = "use storage")]
pub mod data {
    #[doc(hidden)]
    #[deprecated(note = "use storage::Storage")]
    pub use super::storage::Storage as Data;
}

pub mod accounts;
mod bytes;
pub mod crypto;
pub mod deploy;
pub mod events;
pub mod iter;
pub mod ledger;
pub mod logging;
mod map;
mod set;
pub mod storage;
mod vec;
pub use accounts::AccountId;
pub use address::Address;
pub use bytes::{Bytes, BytesN};
pub use map::Map;
pub use set::Set;
pub use vec::Vec;

pub mod serde;

pub mod testutils;

mod tests;
