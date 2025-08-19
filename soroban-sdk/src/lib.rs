//! Soroban SDK supports writing smart contracts for the Wasm-powered [Soroban] smart contract
//! runtime, deployed on [Stellar].
//!
//! ### Docs
//!
//! See [developers.stellar.org] for documentation about building smart contracts for [Stellar].
//!
//! [developers.stellar.org]: https://developers.stellar.org
//! [Stellar]: https://stellar.org
//! [Soroban]: https://stellar.org/soroban
//!
//! ### Migrating Major Versions
//!
//! See [_migrating] for a summary of how to migrate from one major version to another.
//!
//! ### Examples
//!
//! ```rust
//! use soroban_sdk::{contract, contractimpl, vec, symbol_short, BytesN, Env, Symbol, Vec};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
//!         vec![&env, symbol_short!("Hello"), to]
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_id = env.register(Contract, ());
//!     let client = ContractClient::new(&env, &contract_id);
//!
//!     let words = client.hello(&symbol_short!("Dev"));
//!
//!     assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! More examples are available at:
//! - <https://developers.stellar.org/docs/build/smart-contracts/example-contracts>
//! - <https://developers.stellar.org/docs/build/guides>

#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

pub mod _migrating;

#[cfg(all(target_family = "wasm", feature = "testutils"))]
compile_error!("'testutils' feature is not supported on 'wasm' target");

// When used in a no_std contract, provide a panic handler as one is required.
#[cfg(target_family = "wasm")]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// Here we provide a `#[global_allocator]` that is a minimal non-freeing bump
// allocator, appropriate for a WASM blob that runs a single contract call.
#[cfg(all(feature = "alloc", target_family = "wasm"))]
mod alloc;

/// This const block contains link sections that need to end up in the final
/// build of any contract using the SDK.
///
/// In Rust's build system sections only get included into the final build if
/// the object file containing those sections are processed by the linker, but
/// as an optimization step if no code is called in an object file it is
/// discarded.  This has the unfortunate effect of causing anything else in
/// those object files, such as link sections, to be discarded. Placing anything
/// that must be included in the build inside an exported static or function
/// ensures the object files won't be discarded. wasm-bindgen does a similar
/// thing to this with a function, and so this seems to be a reasonably
/// accepted way to work around this limitation in the build system. The SDK
/// uses a static exported with name `_` that becomes a global because a global
/// is more unnoticeable, and takes up less bytes.
///
/// The const block has no affect on the above problem and exists only to group
/// the static and link sections under a shared cfg.
///
/// See https://github.com/stellar/rs-soroban-sdk/issues/383 for more details.
#[cfg(target_family = "wasm")]
const _: () = {
    /// This exported static is guaranteed to end up in the final binary of any
    /// importer, as a global. It exists to ensure the link sections are
    /// included in the final build artifact. See notes above.
    #[export_name = "_"]
    static __: () = ();

    #[link_section = "contractenvmetav0"]
    static __ENV_META_XDR: [u8; env::internal::meta::XDR.len()] = env::internal::meta::XDR;

    // Rustc version.
    contractmeta!(key = "rsver", val = env!("RUSTC_VERSION"),);

    // Rust Soroban SDK version.
    contractmeta!(
        key = "rssdkver",
        val = concat!(env!("CARGO_PKG_VERSION"), "#", env!("GIT_REVISION")),
    );
};

// Re-exports of dependencies used by macros.
#[doc(hidden)]
pub mod reexports_for_macros {
    pub use ::bytes_lit;
    #[cfg(any(test, feature = "testutils"))]
    pub use ::ctor;
}

/// Assert in contract asserts that the contract is currently executing within a
/// contract. The macro maps to code when testutils are enabled or in tests,
/// otherwise maps to nothing.
#[macro_export]
macro_rules! assert_in_contract {
    ($env:expr $(,)?) => {{
        {
            #[cfg(any(test, feature = "testutils"))]
            assert!(
                ($env).in_contract(),
                "this function is not accessible outside of a contract, wrap \
                the call with `env.as_contract()` to access it from a \
                particular contract"
            );
        }
    }};
}

/// Create a short [Symbol] constant with the given string.
///
/// A short symbol's maximum length is 9 characters. For longer symbols, use
/// [Symbol::new] to create the symbol at runtime.
///
/// Valid characters are `a-zA-Z0-9_`.
///
/// The [Symbol] is generated at compile time and returned as a const.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{symbol_short, Symbol};
///
/// let symbol = symbol_short!("a_str");
/// assert_eq!(symbol, symbol_short!("a_str"));
/// ```
pub use soroban_sdk_macros::symbol_short;

/// Generates conversions from the repr(u32) enum from/into an `Error`.
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
/// use soroban_sdk::{contract, contracterror, contractimpl, Env};
///
/// #[contracterror]
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// #[repr(u32)]
/// pub enum Error {
///     MyError = 1,
///     AnotherError = 2,
/// }
///
/// #[contract]
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
///     let contract_id = env.register(Contract, ());
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
/// # use soroban_sdk::{contract, contracterror, contractimpl, Env};
/// #
/// # #[contracterror]
/// # #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// # #[repr(u32)]
/// # pub enum Error {
/// #     MyError = 1,
/// #     AnotherError = 2,
/// # }
/// #
/// # #[contract]
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
///     let contract_id = env.register(Contract, ());
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
/// The path given is relative to the workspace root, and not the current
/// file.
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
///     let contract_b_id = env.register(ContractB, ());
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

/// Marks a type as being the type that contract functions are attached for.
///
/// Use `#[contractimpl]` on impl blocks of this type to make those functions
/// contract functions.
///
/// Note that a crate only ever exports a single contract. While there can be
/// multiple types in a crate with `#[contract]`, when built as a wasm file and
/// deployed the combination of all contract functions and all contracts within
/// a crate will be seen as a single contract.
///
/// ### Examples
///
/// Define a contract with one function, `hello`, and call it from within a test
/// using the generated client.
///
/// ```
/// use soroban_sdk::{contract, contractimpl, vec, symbol_short, BytesN, Env, Symbol, Vec};
///
/// #[contract]
/// pub struct HelloContract;
///
/// #[contractimpl]
/// impl HelloContract {
///     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
///         vec![&env, symbol_short!("Hello"), to]
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register(HelloContract, ());
///     let client = HelloContractClient::new(&env, &contract_id);
///
///     let words = client.hello(&symbol_short!("Dev"));
///
///     assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
pub use soroban_sdk_macros::contract;

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
/// use soroban_sdk::{contract, contractimpl, vec, symbol_short, BytesN, Env, Symbol, Vec};
///
/// #[contract]
/// pub struct HelloContract;
///
/// #[contractimpl]
/// impl HelloContract {
///     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
///         vec![&env, symbol_short!("Hello"), to]
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register(HelloContract, ());
///     let client = HelloContractClient::new(&env, &contract_id);
///
///     let words = client.hello(&symbol_short!("Dev"));
///
///     assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
pub use soroban_sdk_macros::contractimpl;

/// Adds a serialized SCMetaEntry::SCMetaV0 to the WASM contracts custom section
/// under the section name 'contractmetav0'. Contract developers can use this to
/// append metadata to their contract.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{contract, contractimpl, contractmeta, vec, symbol_short, BytesN, Env, Symbol, Vec};
///
/// contractmeta!(key="desc", val="hello world contract");
///
/// #[contract]
/// pub struct HelloContract;
///
/// #[contractimpl]
/// impl HelloContract {
///     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
///         vec![&env, symbol_short!("Hello"), to]
///     }
/// }
///
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register(HelloContract, ());
///     let client = HelloContractClient::new(&env, &contract_id);
///
///     let words = client.hello(&symbol_short!("Dev"));
///
///     assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
pub use soroban_sdk_macros::contractmeta;

/// Generates conversions from the struct/enum from/into a `Val`.
///
/// There are some constraints on the types that are supported:
/// - Enums with integer values must have an explicit integer literal for every
/// variant.
/// - Enums with unit variants are supported.
/// - Enums with tuple-like variants with a maximum of one tuple field are
/// supported. The tuple field must be of a type that is also convertible to and
/// from `Val`.
/// - Enums with struct-like variants are not supported.
/// - Structs are supported. All fields must be of a type that is also
/// convertible to and from `Val`.
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
/// use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};
///
/// #[contracttype]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct State {
///     pub count: u32,
///     pub last_incr: u32,
/// }
///
/// #[contract]
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
///         env.storage().persistent().set(&symbol_short!("STATE"), &state);
///
///         // Return the count to the caller.
///         state.count
///     }
///
///     /// Return the current state.
///     pub fn get_state(env: Env) -> State {
///         env.storage().persistent()
///             .get(&symbol_short!("STATE"))
///             .unwrap_or_else(|| State::default()) // If no value set, assume 0.
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register(Contract, ());
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
/// use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Symbol, Env};
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
/// #[contract]
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     /// Set the color.
///     pub fn set(env: Env, c: Color) {
///         env.storage().persistent().set(&symbol_short!("COLOR"), &c);
///     }
///
///     /// Get the color.
///     pub fn get(env: Env) -> Option<Color> {
///         env.storage().persistent()
///             .get(&symbol_short!("COLOR"))
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register(Contract, ());
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

/// Generates conversions from the struct into a published event.
///
/// Fields of the struct become topics and data parameters in the published event.
///
/// Includes the event in the contract spec so that clients can generate bindings
/// for the type and downstream systems can understand the meaning of the event.
///
/// ### Examples
///
/// #### Define an Event
///
/// The event will have a single fixed topic matching the name of the struct in lower snake
/// case. The fixed topic will appear before any topics listed as fields. In the example
/// below, the topics for the event will be:
/// - `"my_event"`
/// - u32 value from the `my_topic` field
///
/// The event's data will be a [`Map`], containing a key-value pair for each field with the key
/// being the name as a [`Symbol`]. In the example below, the data for the event will be:
/// - key: my_event_data => val: u32
/// - key: more_event_data => val: u64
///
/// ```
/// #![no_std]
/// use soroban_sdk::contractevent;
///
/// // Define the event using the `contractevent` attribute macro.
/// #[contractevent]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct MyEvent {
///     // Mark fields as topics, for the value to be included in the events topic list so
///     // that downstream systems know to index it.
///     #[topic]
///     pub my_topic: u32,
///     // Fields not marked as topics will appear in the events data section.
///     pub my_event_data: u32,
///     pub more_event_data: u64,
/// }
///
/// # fn main() { }
/// ```
///
/// #### Define an Event with Custom Topics
///
/// Define a contract event with a custom list of fixed topics.
///
/// The fixed topics can be change to another value. In the example
/// below, the topics for the event will be:
/// - `"my_contract"`
/// - `"an_event"`
/// - u32 value from the `my_topic` field
///
/// ```
/// #![no_std]
/// use soroban_sdk::contractevent;
///
/// // Define the event using the `contractevent` attribute macro.
/// #[contractevent(topics = ["my_contract", "an_event"])]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct MyEvent {
///     // Mark fields as topics, for the value to be included in the events topic list so
///     // that downstream systems know to index it.
///     #[topic]
///     pub my_topic: u32,
///     // Fields not marked as topics will appear in the events data section.
///     pub my_event_data: u32,
///     pub more_event_data: u64,
/// }
///
/// # fn main() { }
/// ```
///
/// #### Define an Event with Other Data Formats
///
/// The data format of the event is a map by default, but can alternatively be defined as a `vec`
/// or `single-value`.
///
/// ##### Vec
///
/// In the example below, the data for the event will be a [`Vec`] containing:
/// - u32
/// - u64
///
/// ```
/// #![no_std]
/// use soroban_sdk::contractevent;
///
/// // Define the event using the `contractevent` attribute macro.
/// #[contractevent(data_format = "vec")]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct MyEvent {
///     // Mark fields as topics, for the value to be included in the events topic list so
///     // that downstream systems know to index it.
///     #[topic]
///     pub my_topic: u32,
///     // Fields not marked as topics will appear in the events data section.
///     pub my_event_data: u32,
///     pub more_event_data: u64,
/// }
///
/// # fn main() { }
/// ```
///
/// ##### Single Value
///
/// In the example below, the data for the event will be a u32.
///
/// When the data format is a single value there must be no more than one data field.
///
/// ```
/// #![no_std]
/// use soroban_sdk::contractevent;
///
/// // Define the event using the `contractevent` attribute macro.
/// #[contractevent(data_format = "single-value")]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct MyEvent {
///     // Mark fields as topics, for the value to be included in the events topic list so
///     // that downstream systems know to index it.
///     #[topic]
///     pub my_topic: u32,
///     // Fields not marked as topics will appear in the events data section.
///     pub my_event_data: u32,
/// }
///
/// # fn main() { }
/// ```
///
/// #### A Full Example
///
/// Defining an event, publishing it in a contract, and testing it.
///
/// ```
/// #![no_std]
/// use soroban_sdk::{contract, contractevent, contractimpl, contracttype, symbol_short, Env, Symbol};
///
/// // Define the event using the `contractevent` attribute macro.
/// #[contractevent]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct Increment {
///     // Mark fields as topics, for the value to be included in the events topic list so
///     // that downstream systems know to index it.
///     #[topic]
///     pub change: u32,
///     // Fields not marked as topics will appear in the events data section.
///     pub count: u32,
/// }
///
/// #[contracttype]
/// #[derive(Clone, Default, Debug, Eq, PartialEq)]
/// pub struct State {
///     pub count: u32,
///     pub last_incr: u32,
/// }
///
/// #[contract]
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     /// Increment increments an internal counter, and returns the value.
///     /// Publishes an event about the change in the counter.
///     pub fn increment(env: Env, incr: u32) -> u32 {
///         // Get the current count.
///         let mut state = Self::get_state(env.clone());
///
///         // Increment the count.
///         state.count += incr;
///         state.last_incr = incr;
///
///         // Save the count.
///         env.storage().persistent().set(&symbol_short!("STATE"), &state);
///
///         // Publish an event about the change.
///         Increment {
///             change: incr,
///             count: state.count,
///         }.publish(&env);
///
///         // Return the count to the caller.
///         state.count
///     }
///
///     /// Return the current state.
///     pub fn get_state(env: Env) -> State {
///         env.storage().persistent()
///             .get(&symbol_short!("STATE"))
///             .unwrap_or_else(|| State::default()) // If no value set, assume 0.
///     }
/// }
///
/// #[test]
/// fn test() {
/// # }
/// # #[cfg(feature = "testutils")]
/// # fn main() {
///     let env = Env::default();
///     let contract_id = env.register(Contract, ());
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
pub use soroban_sdk_macros::contractevent;

/// Generates a type that helps build function args for a contract trait.
pub use soroban_sdk_macros::contractargs;

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
/// use soroban_sdk::{contract, contractclient, contractimpl, vec, symbol_short, BytesN, Env, Symbol, Vec};
///
/// #[contractclient(name = "Client")]
/// pub trait HelloInteface {
///     fn hello(env: Env, to: Symbol) -> Vec<Symbol>;
/// }
///
/// #[contract]
/// pub struct HelloContract;
///
/// #[contractimpl]
/// impl HelloContract {
///     pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
///         vec![&env, symbol_short!("Hello"), to]
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
///     let contract_id = env.register(HelloContract, ());
///
///     // Create a client for the hello contract, that was constructed using
///     // the trait.
///     let client = Client::new(&env, &contract_id);
///
///     let words = client.hello(&symbol_short!("Dev"));
///
///     assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
/// }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
pub use soroban_sdk_macros::contractclient;

/// Generates a contract spec for a trait or impl.
///
/// Note that [`contractimpl`] also generates a contract spec and it is in most
/// cases not necessary to use this macro.
#[doc(hidden)]
pub use soroban_sdk_macros::contractspecfn;

/// Import a contract from its WASM file, generating a constant holding the
/// contract file.
///
/// Note that [`contractimport`] also automatically imports the contract file
/// into a constant, and so it is usually unnecessary to use [`contractfile`]
/// directly, unless you specifically want to only load the contract file
/// without generating a client for it.
pub use soroban_sdk_macros::contractfile;

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

/// An internal panic! variant that avoids including the string
/// when building for wasm (since it's just pointless baggage).
#[cfg(target_family = "wasm")]
macro_rules! sdk_panic {
    ($_msg:literal) => {
        panic!()
    };
    () => {
        panic!()
    };
}
#[cfg(not(target_family = "wasm"))]
macro_rules! sdk_panic {
    ($msg:literal) => {
        panic!($msg)
    };
    () => {
        panic!()
    };
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
mod muxed_address;
mod symbol;

pub use env::{ConversionError, Env};

/// Raw value of the Soroban smart contract platform that types can be converted
/// to and from for storing, or passing between contracts.
///
pub use env::Val;

/// Used to do conversions between values in the Soroban environment.
pub use env::FromVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::IntoVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::TryFromVal;
/// Used to do conversions between values in the Soroban environment.
pub use env::TryIntoVal;

// Used by generated code only.
#[doc(hidden)]
pub use env::EnvBase;
#[doc(hidden)]
pub use env::Error;
#[doc(hidden)]
pub use env::MapObject;
#[doc(hidden)]
pub use env::SymbolStr;
#[doc(hidden)]
pub use env::VecObject;

mod try_from_val_for_contract_fn;
#[doc(hidden)]
#[allow(deprecated)]
pub use try_from_val_for_contract_fn::TryFromValForContractFn;

#[doc(hidden)]
#[deprecated(note = "use storage")]
pub mod data {
    #[doc(hidden)]
    #[deprecated(note = "use storage::Storage")]
    pub use super::storage::Storage as Data;
}
pub mod auth;
mod bytes;
pub mod crypto;
pub mod deploy;
mod error;
pub use error::InvokeError;
pub mod events;
pub use events::{Event, Topics};
pub mod iter;
pub mod ledger;
pub mod logs;
mod map;
pub mod prng;
pub mod storage;
pub mod token;
mod vec;
pub use address::Address;
pub use bytes::{Bytes, BytesN};
pub use map::Map;
pub use muxed_address::MuxedAddress;
pub use symbol::Symbol;
pub use vec::Vec;
mod num;
pub use num::{Duration, Timepoint, I256, U256};
mod string;
pub use string::String;
mod tuple;

mod constructor_args;
pub use constructor_args::ConstructorArgs;

pub mod xdr;

pub mod testutils;

mod arbitrary_extra;

mod tests;
