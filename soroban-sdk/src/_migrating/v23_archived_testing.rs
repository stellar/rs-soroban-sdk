//! Accessing archived persistent entries in tests no longer results in an error.
//!
//! Prior to protocol 23 the SDK used to emulate the failure when an archived
//! ledger entry was accessed in tests. This behavior has never represented the
//! actual behavior of the network, just one possible scenario when an archived
//! entry is present in the transaction footprint.
//!
//! In protocol 23 automatic entry restoration has been introduced, which makes
//! it possible for a transaction to restore an archived entry before accessing
//! it. As this behavior will become the most common case on the network, the
//! SDK has been changed to emulate automatic restoration in tests as well.
//!
//! Note, that instance storage is a persistent entry as well, so it is subject
//! to the same change.
//!
//! ## Example
//!
//! Consider the following simple contract that extends entry TTL
//! along with the test that relies on the error on archived entry access in
//! SDK 22:
//!
//! ```
//! #![no_std]
//! use soroban_sdk::{contract, contractimpl, contracttype, Env};
//!
//! #[contract]
//! struct Contract;
//!
//! #[contracttype]
//! enum DataKey {
//!     Key,
//! }
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn create_and_extend_entry(env: Env) {
//!         env.storage().persistent().set(&DataKey::Key, &123_u32);
//!         // Extend the entry to live for at least 1_000_000 ledgers.
//!         env.storage()
//!             .persistent()
//!             .extend_ttl(&DataKey::Key, 1_000_000, 1_000_000);
//!     }
//!
//!     pub fn read_entry(env: Env) -> u32 {
//!         env.storage().persistent().get(&DataKey::Key).unwrap()
//!     }
//! }
//!
//! mod test {
//!     extern crate std;
//!     use soroban_sdk::testutils::{storage::Persistent, Ledger};
//!
//!     use super::*;
//!
//!     #[test]
//!     fn test_entry_archived() {
//!         let env = Env::default();
//!         let contract = env.register(Contract, ());
//!         let client = ContractClient::new(&env, &contract);
//!         client.create_and_extend_entry();
//!         let current_ledger = env.ledger().sequence();
//!         assert_eq!(client.read_entry(), 123);
//!
//!         // Bump ledger sequence past entry TTL.
//!         env.ledger()
//!             .set_sequence_number(current_ledger + 1_000_000 + 1);
//!         let res = client.try_read_entry();
//!         // 👀 In SDK 22 `res` would be an error because the entry is archived.
//!         // 👀 In SDK 23 `res` is Ok(123) because the entry is automatically restored.
//!         assert!(res.is_err());
//!     }
//! }
//!
//! # fn main() { }
//! ```
//!
//! The best way to address this change is to update the tests to explicitly
//! verify the expected entry TTL after the extension. This way there is no need
//! to rely on the storage behavior, and also the test becomes more robust as
//! it enforces the exact expected TTL value, so there is no risk of bumping
//! the ledger sequence further than the expected TTL and still having the test
//! pass.
//!
//! The example test above can be re-written as follows:
//!
//! ```
//! #![no_std]
//! use soroban_sdk::{contract, contractimpl, contracttype, Env};
//!
//! #[contract]
//! struct Contract;
//!
//! #[contracttype]
//! enum DataKey {
//!     Key,
//! }
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn create_and_extend_entry(env: Env) {
//!         env.storage().persistent().set(&DataKey::Key, &123_u32);
//!         // Extend the entry to live for at least 1_000_000 ledgers.
//!         env.storage()
//!             .persistent()
//!             .extend_ttl(&DataKey::Key, 1_000_000, 1_000_000);
//!     }
//!
//!     pub fn read_entry(env: Env) -> u32 {
//!         env.storage().persistent().get(&DataKey::Key).unwrap()
//!     }
//! }
//!
//! #[cfg(test)]
//! mod test {
//!     extern crate std;
//!     use soroban_sdk::testutils::{storage::Persistent, Ledger};
//!
//!     use super::*;
//!
//!     #[test]
//!     fn test_entry_ttl_extended() {
//!         let env = Env::default();
//!         let contract = env.register(Contract, ());
//!         let client = ContractClient::new(&env, &contract);
//!         client.create_and_extend_entry();
//!         assert_eq!(client.read_entry(), 123);
//!
//!         // 👀 Verify that the entry TTL was extended correctly by 1000000 ledgers.
//!         env.as_contract(&contract, || {
//!             assert_eq!(env.storage().persistent().get_ttl(&DataKey::Key), 1_000_000);
//!         });
//!     }
//!     
//!     // 👀 This test is not really necessary, but it demonstrates the
//!     // auto-restoration behavior in tests.
//!     #[test]
//!     fn test_auto_restore() {
//!         let env = Env::default();
//!         let contract = env.register(Contract, ());
//!         let client = ContractClient::new(&env, &contract);
//!         client.create_and_extend_entry();
//!         let current_ledger = env.ledger().sequence();
//!
//!         // Bump ledger sequence past entry TTL.
//!         env.ledger()
//!             .set_sequence_number(current_ledger + 1_000_000 + 1);
//!         // 👀 Entry can still be accessed because automatic restoration is emulated
//!         // in tests.
//!         assert_eq!(client.read_entry(), 123);
//!
//!         // 👀 Automatic restoration is also accounted for in cost_estimate():
//!         let resources = env.cost_estimate().resources();
//!         // Even though `read_entry` call is normally read-only, auto-restoration
//!         // will cause 2 entry writes here: 1 for the contract instance, another
//!         // one for the restored entry.
//!         assert_eq!(resources.write_entries, 2);
//!         // 2 rent bumps will happen as well for the respective entries.
//!         assert_eq!(resources.persistent_entry_rent_bumps, 2);
//!
//!         // 👀 Entry TTL after auto-restoration can be observed via get_ttl().
//!         env.as_contract(&contract, || {
//!             // Auto-restored entries have their TTL extended by the minimum
//!             // possible TTL worth of ledgers (`min_persistent_entry_ttl`),
//!             // including the ledger in which they were restored (that's why
//!             // we subtract 1 here).
//!             assert_eq!(
//!                 env.storage().persistent().get_ttl(&DataKey::Key),
//!                 env.ledger().get().min_persistent_entry_ttl - 1
//!             );
//!         });
//!     }
//! }
//!
//! # fn main() { }
//! ```
//!
