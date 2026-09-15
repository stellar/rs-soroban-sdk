//! Custom accounts see a new `ContractExecutable::ExternalRef` variant when authorizing deployments.
//!
//! When a custom account authorizes a contract deployment, the host passes a
//! [`Context::CreateContractHostFn`] or [`Context::CreateContractWithCtorHostFn`] to
//! `__check_auth`. Both carry a [`ContractExecutable`] describing the executable of the contract
//! being created. In v28 the enum gains a second variant, [`ContractExecutable::ExternalRef`],
//! which refers to an executable reference entry owned by a contract (see [`ExecutableRefs`]),
//! alongside the existing [`ContractExecutable::Wasm`] variant. The change comes from
//! [CAP-85](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0085.md).
//!
//! ## Changed Behaviour
//!
//! Custom accounts that were built with an earlier SDK cannot decode a context whose executable
//! is an `ExternalRef`. When asked to authorize such a deployment, their `__check_auth` fails
//! and the deployment is not authorized.
//!
//! ## Migrating
//!
//! Add a match arm for [`ContractExecutable::ExternalRef`] and decide whether the account
//! authorizes it. An account that only ever authorized Wasm deployments can keep doing so by
//! rejecting the new variant.
//!
//! ```
//! use soroban_sdk::{
//!     auth::{Context, CustomAccountInterface},
//!     contract, contracterror, contractimpl,
//!     crypto::Hash,
//!     ContractExecutable, Env, Vec,
//! };
//!
//! #[contract]
//! pub struct Account;
//!
//! #[contracterror]
//! #[derive(Copy, Clone, Debug, Eq, PartialEq)]
//! pub enum Error {
//!     UnsupportedExecutable = 1,
//! }
//!
//! #[contractimpl]
//! impl CustomAccountInterface for Account {
//!     type Signature = ();
//!     type Error = Error;
//!
//!     fn __check_auth(
//!         env: Env,
//!         signature_payload: Hash<32>,
//!         signature: (),
//!         auth_contexts: Vec<Context>,
//!     ) -> Result<(), Error> {
//!         // ... verify the signature ...
//!
//!         for context in auth_contexts.iter() {
//!             let executable = match context {
//!                 Context::Contract(_) => continue,
//!                 Context::CreateContractHostFn(c) => c.executable,
//!                 Context::CreateContractWithCtorHostFn(c) => c.executable,
//!             };
//!             match executable {
//!                 ContractExecutable::Wasm(_wasm_hash) => (),
//!                 // 👇 👀 New in v28. This account does not authorize deployments
//!                 // from executable references.
//!                 ContractExecutable::ExternalRef(_) => {
//!                     return Err(Error::UnsupportedExecutable)
//!                 }
//!             }
//!         }
//!         Ok(())
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     use soroban_sdk::{
//!         auth::CreateContractHostFnContext, testutils::Address as _, vec, Address, BytesN,
//!         ContractExecutableRef, IntoVal, String,
//!     };
//!
//!     let env = Env::default();
//!     let account = env.register(Account, ());
//!     let payload = BytesN::from_array(&env, &[0; 32]);
//!
//!     // The host calls `__check_auth` when the account's address is required
//!     // to authorize a call. Call it directly to test the account on its own.
//!     let wasm = vec![
//!         &env,
//!         Context::CreateContractHostFn(CreateContractHostFnContext {
//!             executable: ContractExecutable::Wasm(BytesN::from_array(&env, &[1; 32])),
//!             salt: BytesN::from_array(&env, &[0; 32]),
//!         }),
//!     ];
//!     assert_eq!(
//!         env.try_invoke_contract_check_auth::<Error>(&account, &payload, ().into_val(&env), &wasm),
//!         Ok(())
//!     );
//!
//!     let external_ref = vec![
//!         &env,
//!         Context::CreateContractHostFn(CreateContractHostFnContext {
//!             executable: ContractExecutable::ExternalRef(ContractExecutableRef {
//!                 owner: Address::generate(&env),
//!                 tag: String::from_str(&env, "v1"),
//!             }),
//!             salt: BytesN::from_array(&env, &[0; 32]),
//!         }),
//!     ];
//!     assert_eq!(
//!         env.try_invoke_contract_check_auth::<Error>(
//!             &account,
//!             &payload,
//!             ().into_val(&env),
//!             &external_ref
//!         ),
//!         Err(Ok(Error::UnsupportedExecutable))
//!     );
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! [`Context::CreateContractHostFn`]: crate::auth::Context::CreateContractHostFn
//! [`Context::CreateContractWithCtorHostFn`]: crate::auth::Context::CreateContractWithCtorHostFn
//! [`ContractExecutable`]: crate::ContractExecutable
//! [`soroban_sdk::ContractExecutable`]: crate::ContractExecutable
//! [`ContractExecutable::Wasm`]: crate::ContractExecutable::Wasm
//! [`ContractExecutable::ExternalRef`]: crate::ContractExecutable::ExternalRef
//! [`ExecutableRefs`]: crate::executable_refs::ExecutableRefs
