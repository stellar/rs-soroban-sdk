//! Resource limit enforcement in tests.
//!
//! ## Breaking Change: Tests May Fail Due to Resource Limits
//!
//! By default, [`Env::default()`] now enforces mainnet resource limits for contract invocations in
//! tests. **If your contract exceeds any resource limit, your tests will panic** with details
//! about which limits were exceeded.
//!
//! This provides an early warning that a contract might be too resource-heavy to run on mainnet.
//!
//! **If you see test failures after upgrading**, and you wish to test without mainnet limits
//! (e.g., while experimenting or optimizing), see [Disabling Resource
//! Limits](#disabling-resource-limits) below.
//!
//! ## New Default Behavior
//!
//! When creating a new `Env` with [`Env::default()`], mainnet resource limits are automatically
//! enforced. No changes to existing test code are required to benefit from this protection.
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, Env};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn execute() {
//!         // ... code
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default(); // Mainnet limits enforced automatically
//!     let contract_id = env.register(Contract, ());
//!     let client = ContractClient::new(&env, &contract_id);
//!     client.execute(); // Will panic if resource limit exceeded
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! ## Disabling Resource Limits
//!
//! For experimental contracts that are still being optimized, resource limit enforcement can be
//! disabled using [`CostEstimate::disable_resource_limits()`]:
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, Env};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn execute() {
//!         // ... resource-heavy code
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     env.cost_estimate().disable_resource_limits(); // Disable resource limit
//!
//!     let contract_id = env.register(Contract, ());
//!     let client = ContractClient::new(&env, &contract_id);
//!     client.execute(); // Won't panic even if limits exceeded
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! ## Custom Resource Limits
//!
//! Custom resource limits can be enforced using [`CostEstimate::enforce_resource_limits()`]:
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, Env};
//! use soroban_sdk::testutils::cost_estimate::NetworkInvocationResourceLimits;
//! use soroban_env_host::InvocationResourceLimits;
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn execute() {
//!         // ... code
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!
//!     // Use custom limits (this example uses mainnet limits as a base)
//!     let mut limits = InvocationResourceLimits::mainnet();
//!     limits.instructions = 100_000_000;  // Reduce instruction limit
//!     env.cost_estimate().enforce_resource_limits(limits);
//!
//!     let contract_id = env.register(Contract, ());
//!     let client = ContractClient::new(&env, &contract_id);
//!     client.execute(); // Uses the custom limits
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! ## Mainnet Resource Limits
//!
//! The [`NetworkInvocationResourceLimits`] trait provides the `mainnet()` method on
//! [`InvocationResourceLimits`] to get the current mainnet limits:
//!
//! - Instructions: 600,000,000
//! - Memory: 41,943,040 bytes
//! - Disk read entries: 100
//! - Write entries: 50
//! - Ledger entries: 100
//! - Disk read bytes: 200,000
//! - Write bytes: 132,096
//! - Contract events size: 16,384 bytes
//! - Max contract data key size: 250 bytes
//! - Max contract data entry size: 65,536 bytes
//! - Max contract code entry size: 131,072 bytes
//!
//! Note: These values are not pulled dynamically. The SDK will be updated from time-to-time to
//! pick up changes to mainnet limits. These changes may occur in any major, minor, or patch
//! release.
//!
//! [`Env::default()`]: crate::Env::default
//! [`CostEstimate::disable_resource_limits()`]: crate::testutils::cost_estimate::CostEstimate::disable_resource_limits
//! [`CostEstimate::enforce_resource_limits()`]: crate::testutils::cost_estimate::CostEstimate::enforce_resource_limits
//! [`InvocationResourceLimits`]: soroban_env_host::InvocationResourceLimits
//! [`NetworkInvocationResourceLimits`]: crate::testutils::cost_estimate::NetworkInvocationResourceLimits
