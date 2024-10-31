//! # Migrating from v20 to v21
//!
//! ## `CustomAccountInterface`'s `__check_auth` function `signature_payload` parameter changes from type `BytesN<32>` to `Hash<32>`.
//!
//! ### Before
//!
//! ```
//! # use soroban_sdk_21 as soroban_sdk;
//! use soroban_sdk::auth::CustomAccountInterface;
//!
//! #[contract]
//! pub struct Contract;
//!
//! impl CustomAccountInterface for Contract {
//!     type Signature = ();
//!     type Error: Into<Error> = u32;
//!
//!     fn __check_auth(
//!         env: Env,
//!         signature_payload: BytesN<32>,
//!         signatures: Self::Signature,
//!         auth_contexts: Vec<Context>,
//!     ) -> Result<(), Self::Error> {
//!         // ...
//!     }
//! }
//! ```
//!
//! ### After
//!
//! ```
//! use soroban_sdk::auth::CustomAccountInterface;
//!
//! #[contract]
//! pub struct Contract;
//!
//! impl CustomAccountInterface for Contract {
//!     type Signature = ();
//!     type Error: Into<Error> = u32;
//!
//!     fn __check_auth(
//!         env: Env,
//!         signature_payload: Hash<32>,
//!         signatures: Self::Signature,
//!         auth_contexts: Vec<Context>,
//!     ) -> Result<(), Self::Error> {
//!         // ...
//!     }
//! }
//! ```
//!
