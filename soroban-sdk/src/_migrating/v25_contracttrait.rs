//! [`contracttrait`] macro for reusable contract interfaces.
//!
//! _Note: This feature was released in v23.4.0 but is being included in the migration notes for the
//! next major version, v25._
//!
//! The [`contracttrait`] macro enables defining reusable contract interfaces as Rust traits with
//! default implementations. Contracts can implement these traits, and the default implementations
//! are automatically exported as contract functions.
//!
//! ## Generated Functionality
//!
//! When applied to a trait, [`contracttrait`] generates:
//!
//! - `{TraitName}Client` - A client for invoking the trait's functions on a contract
//! - `{TraitName}Args` - An enum of function arguments for the trait's functions
//! - `{TraitName}Spec` - The contract specification for the trait's functions
//!
//! These names can be customized using macro arguments (e.g., `client_name`, `args_name`,
//! `spec_name`).
//!
//! ## When to Use
//!
//! Use [`contracttrait`] when you want the trait to represent a contract interface.
//!
//! The [`contracttrait`] will make it possible to:
//! - Access a generated client for any contract implementing the interface.
//! - Automatically export default implementations that contracts can optionally override
//! - Share common functionality across contracts
//!
//! ## How It Works
//!
//! 1. **Define a trait** with [`contracttrait`]
//!
//! 2. **Implement the trait** with [`contractimpl`], including the `contracttrait` option:
//!    `#[contractimpl(contracttrait)]`
//!
//! 3. **Override default functions as needed** - Contracts can provide their own implementations of
//!    any function with default implementations.
//!
//! ## Patterns For Use
//!
//! Place traits that use [`contracttrait`] into a library crate, to share and make those traits
//! available to other crates and developers.
//!
//! [`contracttrait`]: crate::contracttrait
//! [`contractimpl`]: crate::contractimpl
//!
//! ## Example: Defining and Implementing a Trait
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, contracttrait, Env};
//!
//! // Define a trait with default implementations
//! #[contracttrait]
//! pub trait Pausable {
//!     fn is_paused(env: &Env) -> bool {
//!         env.storage().instance().has(&"paused")
//!     }
//!
//!     fn pause(env: &Env) {
//!         env.storage().instance().set(&"paused", &true);
//!     }
//!
//!     fn unpause(env: &Env) {
//!         env.storage().instance().remove(&"paused");
//!     }
//! }
//!
//! #[contract]
//! pub struct MyContract;
//!
//! // Implement the trait - default functions are automatically exported
//! #[contractimpl(contracttrait)]
//! impl Pausable for MyContract {}
//!
//! #[contractimpl]
//! impl MyContract {
//!     pub fn do_something(env: &Env) {
//!         if Self::is_paused(env) {
//!             panic!("contract is paused");
//!         }
//!         // ... rest of the function
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! ## Example: Overriding Default Implementations
//!
//! Contracts can override specific functions while keeping the defaults for others:
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, contracttrait, Env};
//!
//! // Define a trait with default implementations
//! #[contracttrait]
//! pub trait Pausable {
//!     fn is_paused(env: &Env) -> bool {
//!         env.storage().instance().has(&"paused")
//!     }
//!
//!     fn pause(env: &Env) {
//!         env.storage().instance().set(&"paused", &true);
//!     }
//!
//!     fn unpause(env: &Env) {
//!         env.storage().instance().remove(&"paused");
//!     }
//! }
//!
//! #[contract]
//! pub struct MyContract;
//!
//! // Implement the trait - override default implementations as needed
//! #[contractimpl(contracttrait)]
//! impl Pausable for MyContract {
//!     // Override is_paused with custom logic that returns false when not set
//!     fn is_paused(env: &Env) -> bool {
//!         env.storage().instance().get(&"paused").unwrap_or(false)
//!     }
//!     // pause() and unpause() use the default implementations
//! }
//!
//! #[contractimpl]
//! impl MyContract {
//!     pub fn do_something(env: &Env) {
//!         if Self::is_paused(env) {
//!             panic!("contract is paused");
//!         }
//!         // ... rest of the function
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! ## Example: Using the Generated Client
//!
//! The generated `{TraitName}Client` can be used to call any contract that implements the trait:
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, contracttrait, Env};
//!
//! // Define a trait with default implementations
//! #[contracttrait]
//! pub trait Pausable {
//!     fn is_paused(env: &Env) -> bool {
//!         env.storage().instance().has(&"paused")
//!     }
//!
//!     fn pause(env: &Env) {
//!         env.storage().instance().set(&"paused", &true);
//!     }
//!
//!     fn unpause(env: &Env) {
//!         env.storage().instance().remove(&"paused");
//!     }
//! }
//!
//! #[contract]
//! pub struct MyContract;
//!
//! // Implement the trait - default functions are automatically exported
//! #[contractimpl(contracttrait)]
//! impl Pausable for MyContract {}
//!
//! #[contractimpl]
//! impl MyContract {
//!     pub fn do_something(env: &Env) {
//!         if Self::is_paused(env) {
//!             panic!("contract is paused");
//!         }
//!         // ... rest of the function
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_id = env.register(MyContract, ());
//!     let client = PausableClient::new(&env, &contract_id);
//!
//!     assert!(!client.is_paused());
//!     client.pause();
//!     assert!(client.is_paused());
//!     client.unpause();
//!     assert!(!client.is_paused());
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
