//! Each natively registered contract gets its own contract code entry.
//!
//! Contracts registered natively with [`Env::register`] or [`Env::register_at`] each get their own
//! contract code entry, instead of all native contracts sharing a single empty Wasm entry. The
//! change comes from the host (`soroban-env-host`), which now uploads a distinct code entry per
//! native contract so that native contracts behave like Wasm contracts.
//!
//! ## Changed Behaviour
//!
//! Prior to v28 every natively registered contract pointed at the same contract code entry, keyed
//! by the hash of empty bytes. Two natively registered contracts therefore shared one code entry,
//! and operations on that entry — extending its TTL, for example — affected every native contract
//! in the test.
//!
//! In v28 each registration uploads its own code entry under its own Wasm hash. Native contracts
//! now behave like Wasm contracts, where each distinct Wasm has its own entry, and only contracts
//! deployed from the same Wasm hash share one.
//!
//! Two consequences follow:
//!
//! - Tests that rely on native contracts sharing a code entry need updating. A test asserting that
//!   extending the code TTL of one contract extends it for another no longer holds.
//!
//! - The Wasm hash a native contract is registered under is no longer the hash of empty bytes, so
//!   test snapshot JSON files containing natively registered contracts change when upgrading.
//!   Regenerate them.
//!
//! ## Migrating
//!
//! To test contract deployments where a single contract code entry is shared by multiple contracts,
//! upload the contract once with [`Env::upload`] and deploy it multiple times with
//! [`DeployerWithAddress::deploy_contract`], the same way a Wasm contract would be uploaded once and
//! deployed many times.
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, ContractExecutable, Env};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn hello(env: Env) { /* ... */ }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # fn main() {
//!     let env = Env::default();
//!     env.mock_all_auths();
//!
//!     // Upload the contract once, getting a Wasm hash for it. 👈 👀
//!     let wasm_hash = env.upload(Contract);
//!
//!     // Deploy it as many times as required. Every instance shares the one
//!     // contract code entry that the upload created. 👈 👀
//!     let deployer = Address::generate(&env);
//!     let contract_a = env
//!         .deployer()
//!         .with_address(deployer.clone(), [0u8; 32])
//!         .deploy_contract(ContractExecutable::Wasm(wasm_hash.clone()), ());
//!     let contract_b = env
//!         .deployer()
//!         .with_address(deployer, [1u8; 32])
//!         .deploy_contract(ContractExecutable::Wasm(wasm_hash), ());
//! }
//! ```
//!
//! Use [`Env::upload_at`] instead of [`Env::upload`] to specify the Wasm hash the contract is
//! uploaded to, rather than having one generated.
//!
//! [`Env::register`]: crate::Env::register
//! [`Env::register_at`]: crate::Env::register_at
//! [`Env::upload`]: crate::Env::upload
//! [`Env::upload_at`]: crate::Env::upload_at
//! [`DeployerWithAddress::deploy_contract`]: crate::deploy::DeployerWithAddress::deploy_contract
