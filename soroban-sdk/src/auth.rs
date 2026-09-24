//! Auth contains types for building custom account contracts.
//!
//! A custom account is a contract that implements the
//! [`CustomAccountInterface`], which the host invokes via the contract's
//! `__check_auth` function whenever [`Address::require_auth`] is called for the
//! contract's address. The contract decides what makes an authorization valid:
//! it receives the signature payload, its own [`CustomAccountInterface::Signature`]
//! type, and the list of [`Context`] values describing every call being
//! authorized.
//!
//! ### Examples
//!
//! #### Implement a custom account contract
//!
//! The account below is an example implementation: it stores an ed25519 public
//! key at construction, verifies that the signature payload was signed by the
//! corresponding private key, and only authorizes contract calls. A real
//! account decides for itself what a valid signature and an acceptable
//! [`Context`] are.
//!
//! ```
//! use soroban_sdk::{
//!     auth::{Context, CustomAccountInterface},
//!     contract, contracterror, contractimpl, contracttype,
//!     crypto::Hash,
//!     BytesN, Env, Vec,
//! };
//!
//! #[contracterror]
//! #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
//! #[repr(u32)]
//! pub enum Error {
//!     NotInitialized = 1,
//!     UnsupportedContext = 2,
//! }
//!
//! #[contracttype]
//! pub enum DataKey {
//!     PublicKey,
//! }
//!
//! #[contract]
//! pub struct Account;
//!
//! #[contractimpl]
//! impl Account {
//!     pub fn __constructor(env: Env, public_key: BytesN<32>) {
//!         env.storage()
//!             .instance()
//!             .set(&DataKey::PublicKey, &public_key);
//!     }
//! }
//!
//! #[contractimpl]
//! impl CustomAccountInterface for Account {
//!     type Signature = BytesN<64>;
//!     type Error = Error;
//!
//!     fn __check_auth(
//!         env: Env,
//!         signature_payload: Hash<32>,
//!         signature: BytesN<64>,
//!         auth_contexts: Vec<Context>,
//!     ) -> Result<(), Error> {
//!         // Check that the payload was signed by the account's key. The host
//!         // panics if the signature does not verify.
//!         let public_key: BytesN<32> = env
//!             .storage()
//!             .instance()
//!             .get(&DataKey::PublicKey)
//!             .ok_or(Error::NotInitialized)?;
//!         env.crypto()
//!             .ed25519_verify(&public_key, &signature_payload.into(), &signature);
//!
//!         // Check what is being authorized. There is one context for every
//!         // `require_auth[_for_args]` call made on behalf of this account.
//!         // This example only checks the kind of each context, but an
//!         // account can inspect the contract, function, and arguments too,
//!         // e.g. to enforce spend limits.
//!         for context in auth_contexts.iter() {
//!             match context {
//!                 Context::Contract(_) => (),
//!                 // This account does not authorize contract deployments.
//!                 Context::CreateContractHostFn(_)
//!                 | Context::CreateContractWithCtorHostFn(_) => {
//!                     return Err(Error::UnsupportedContext)
//!                 }
//!             }
//!         }
//!
//!         Ok(())
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     use ed25519_dalek::{Signer, SigningKey};
//!     use soroban_sdk::{
//!         auth::ContractContext, testutils::Address as _, vec, Address, IntoVal, Symbol,
//!     };
//!
//!     let env = Env::default();
//!
//!     let signing_key = SigningKey::from_bytes(&[0; 32]);
//!     let public_key = BytesN::from_array(&env, &signing_key.verifying_key().to_bytes());
//!     let account = env.register(Account, (public_key,));
//!
//!     // The host calls `__check_auth` when the account's address is required
//!     // to authorize a call. Call it directly to test the account on its own.
//!     let payload = BytesN::from_array(&env, &[1; 32]);
//!     let signature =
//!         BytesN::from_array(&env, &signing_key.sign(&payload.to_array()).to_bytes());
//!     let contexts = vec![
//!         &env,
//!         Context::Contract(ContractContext {
//!             contract: Address::generate(&env),
//!             fn_name: Symbol::new(&env, "deposit"),
//!             args: vec![&env, 100_i128.into_val(&env)],
//!         }),
//!     ];
//!     assert_eq!(
//!         env.try_invoke_contract_check_auth::<Error>(
//!             &account,
//!             &payload,
//!             signature.into_val(&env),
//!             &contexts,
//!         ),
//!         Ok(())
//!     );
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```

use crate::{
    contractimpl_trait_macro, contracttype, crypto::Hash, Address, BytesN, Env, Error, Symbol, Val,
    Vec,
};

pub use crate::{ContractExecutable, ContractExecutableRef};

/// Context of a single authorized call performed by an address.
///
/// Custom account contracts that implement `__check_auth` special function
/// receive a list of `Context` values corresponding to all the calls that
/// need to be authorized.
#[derive(Clone, Debug)]
#[contracttype(crate_path = "crate")]
pub enum Context {
    /// Contract invocation.
    Contract(ContractContext),
    /// Contract that has a constructor with no arguments is created.
    CreateContractHostFn(CreateContractHostFnContext),
    /// Contract that has a constructor with 1 or more arguments is created.
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}

/// Authorization context of a single contract call.
///
/// This struct corresponds to a `require_auth_for_args` call for an address
/// from `contract` function with `fn_name` name and `args` arguments.
#[derive(Clone, Debug)]
#[contracttype(crate_path = "crate")]
pub struct ContractContext {
    pub contract: Address,
    pub fn_name: Symbol,
    pub args: Vec<Val>,
}

/// Authorization context for `create_contract` host function that creates a
/// new contract on behalf of authorizer address.
#[derive(Clone, Debug)]
#[contracttype(crate_path = "crate")]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: BytesN<32>,
}

/// Authorization context for `create_contract` host function that creates a
/// new contract on behalf of authorizer address.
/// This is the same as `CreateContractHostFnContext`, but also has
/// contract constructor arguments.
#[derive(Clone, Debug)]
#[contracttype(crate_path = "crate")]
pub struct CreateContractWithConstructorHostFnContext {
    pub executable: ContractExecutable,
    pub salt: BytesN<32>,
    pub constructor_args: Vec<Val>,
}

/// A node in the tree of authorizations performed on behalf of the current
/// contract as invoker of the contracts deeper in the call stack.
///
/// This is used as an argument of `authorize_as_current_contract` host function.
///
/// This tree corresponds `require_auth[_for_args]` calls on behalf of the
/// current contract.
#[derive(Clone)]
#[contracttype(crate_path = "crate")]
pub enum InvokerContractAuthEntry {
    /// Invoke a contract.
    Contract(SubContractInvocation),
    /// Create a contract passing 0 arguments to constructor.
    CreateContractHostFn(CreateContractHostFnContext),
    /// Create a contract passing 0 or more arguments to constructor.
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}

/// Value of contract node in InvokerContractAuthEntry tree.
#[derive(Clone)]
#[contracttype(crate_path = "crate")]
pub struct SubContractInvocation {
    pub context: ContractContext,
    pub sub_invocations: Vec<InvokerContractAuthEntry>,
}

/// Custom account interface that a contract implements to support being used
/// as a custom account for auth.
///
/// Once a contract implements the interface, call to [`Address::require_auth`]
/// for the contract's address will call its `__check_auth` implementation.
#[contractimpl_trait_macro(crate_path = "crate")]
pub trait CustomAccountInterface {
    type Signature;
    type Error: Into<Error>;

    /// Check that the signatures and auth contexts are valid.
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signatures: Self::Signature,
        auth_contexts: Vec<Context>,
    ) -> Result<(), Self::Error>;
}
