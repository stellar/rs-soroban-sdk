//! A `MuxedAddress` may be a contract address with an id.
//!
//! Protocol 30 adds muxed contract addresses with [CAP-84]. A muxed contract address is a contract
//! address paired with a `u64` id, and its strkey starts with `W`.
//!
//! Before v30 a [`MuxedAddress`] with an id was always a muxed account address, `M...`. In v30 a
//! [`MuxedAddress`] with an id may also be a muxed contract address, `W...`, where
//! [`MuxedAddress::address`] returns the contract address, `C...`, and [`MuxedAddress::id`]
//! returns the id.
//!
//! ## Example
//!
//! ```
//! #![no_std]
//! use soroban_sdk::{contract, contractimpl, Env, MuxedAddress};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn deposit(_env: Env, to: MuxedAddress) -> Option<u64> {
//!         // The address is `G...` or `C...`, whether or not `to` is muxed.
//!         let _address = to.address();
//!         // The id is `Some` for a muxed account or muxed contract address.
//!         to.id()
//!     }
//! }
//!
//! mod test {
//!     use soroban_sdk::testutils::MuxedAddress as _;
//!     use soroban_sdk::{Env, MuxedAddress};
//!
//!     use super::*;
//!
//!     #[test]
//!     fn test_deposit_to_muxed_contract() {
//!         let env = Env::default();
//!         let contract = env.register(Contract, ());
//!         let client = ContractClient::new(&env, &contract);
//!
//!         let to = MuxedAddress::new(&contract, 123);
//!         assert!(to.to_strkey().to_string().starts_with('W'));
//!         assert_eq!(client.deposit(&to), Some(123));
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! ## Migrating
//!
//! A contract that takes a [`MuxedAddress`] and reads [`MuxedAddress::id`] must not assume that a
//! muxed address with an id is an account. Use [`MuxedAddress::address`] for the address that
//! holds a balance or authorizes, whether it is an account or a contract.
//!
//! [CAP-84]: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0084.md
//! [`MuxedAddress`]: crate::MuxedAddress
//! [`MuxedAddress::address`]: crate::MuxedAddress::address
//! [`MuxedAddress::id`]: crate::MuxedAddress::id
