//! ML-DSA (FIPS 204) signature verification.
//!
//! Protocol 30 adds host functions for verifying ML-DSA signatures with [CAP-87]. ML-DSA is the
//! module-lattice-based digital signature algorithm standardized by NIST in [FIPS 204], and is the
//! first signature scheme supported by Soroban whose security does not rely on the hardness of the
//! elliptic curve discrete logarithm problem, which does not hold against a sufficiently capable
//! quantum computer.
//!
//! All three standardized parameter sets are supported, with [`Crypto::ml_dsa_44_verify`],
//! [`Crypto::ml_dsa_65_verify`], and [`Crypto::ml_dsa_87_verify`]. Each verifies a signature of
//! the full message, for a context string, using the pure `ML-DSA.Verify` interface of FIPS 204,
//! and panics if the signature is not valid. The context is a domain separation string of at
//! most 255 bytes chosen by the signer, and is empty for most signers.
//!
//! The parameter sets differ in security category and in the size of their encoded keys and
//! signatures:
//!
//! | Parameter set | Public key     | Signature      |
//! | ------------- | -------------- | -------------- |
//! | ML-DSA-44     | `BytesN<1312>` | `BytesN<2420>` |
//! | ML-DSA-65     | `BytesN<1952>` | `BytesN<3309>` |
//! | ML-DSA-87     | `BytesN<2592>` | `BytesN<4627>` |
//!
//! ## Example
//!
//! ```
//! #![no_std]
//! use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     /// Panics if `signature` is not a valid ML-DSA-65 signature of `message` by `public_key`.
//!     pub fn verify(env: Env, public_key: BytesN<1952>, message: Bytes, signature: BytesN<3309>) {
//!         // The context is the domain separation string the signer used. Most signers use
//!         // none, which is verified with empty bytes.
//!         let context = Bytes::new(&env);
//!         env.crypto()
//!             .ml_dsa_65_verify(&public_key, &message, &signature, &context);
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! [CAP-87]: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0087.md
//! [FIPS 204]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf
//! [`Crypto::ml_dsa_44_verify`]: crate::crypto::Crypto::ml_dsa_44_verify
//! [`Crypto::ml_dsa_65_verify`]: crate::crypto::Crypto::ml_dsa_65_verify
//! [`Crypto::ml_dsa_87_verify`]: crate::crypto::Crypto::ml_dsa_87_verify
