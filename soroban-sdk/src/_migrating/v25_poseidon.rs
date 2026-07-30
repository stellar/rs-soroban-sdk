//! Poseidon and Poseidon2 permutation functions.
//!
//! Protocol 25 exposes low-level Poseidon and Poseidon2 permutation host functions
//! for advanced cryptographic use cases. These are available through the `CryptoHazmat`
//! interface under the `hazmat-crypto` feature.
//!
//! Higher level non-hazmat functionality will be released in a separate crate.
//!
//! ## ⚠️ Hazardous Materials Warning
//!
//! These are low-level cryptographic primitives. Most users should use higher-level
//! constructions built on top of these permutations. Incorrect usage can lead to
//! security vulnerabilities.
//!
//! ## Enabling the Feature
//!
//! Add the `hazmat-crypto` feature to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! soroban-sdk = { version = "25", features = ["hazmat-crypto"] }
//! ```
//!
//! ## Poseidon Permutation
//!
//! The `poseidon_permutation` function performs the standard Poseidon permutation
//! with a full MDS matrix.
//!
//! Parameters:
//! - `input` - State vector of `U256` field elements
//! - `field` - Field identifier (e.g., `"BN254"`)
//! - `t` - State width (number of elements)
//! - `d` - S-box exponent (typically 5)
//! - `rounds_f` - Number of full rounds (must be even)
//! - `rounds_p` - Number of partial rounds
//! - `mds` - MDS matrix (`t × t` matrix of `U256` elements)
//! - `round_constants` - Round constants (one vector per round)
//!
//! ## Poseidon2 Permutation
//!
//! The `poseidon2_permutation` function performs the Poseidon2 permutation with an
//! optimized internal matrix representation.
//!
//! Parameters:
//! - `input` - State vector of `U256` field elements
//! - `field` - Field identifier (e.g., `"BN254"`)
//! - `t` - State width (number of elements)
//! - `d` - S-box exponent (typically 5)
//! - `rounds_f` - Number of full rounds (must be even)
//! - `rounds_p` - Number of partial rounds
//! - `mat_internal_diag_m_1` - Diagonal of internal matrix minus identity
//! - `round_constants` - Round constants (one vector per round)
//!
//! ## Example
//!
//! ```ignore
//! use soroban_sdk::{bytesn, vec, Env, Symbol, U256};
//! use soroban_sdk::crypto::CryptoHazmat;
//!
//! # fn main() {
//! let env = Env::default();
//!
//! // Define MDS matrix (2x2 for t=2)
//! let mds = vec![
//!     &env,
//!     vec![&env,
//!         U256::from_be_bytes(&env, &bytesn!(&env, 0x066f6f85d6f68a85ec10345351a23a3aaf07f38af8c952a7bceca70bd2af7ad5).into()),
//!         U256::from_be_bytes(&env, &bytesn!(&env, 0x2b9d4b4110c9ae997782e1509b1d0fdb20a7c02bbd8bea7305462b9f8125b1e8).into()),
//!     ],
//!     vec![&env,
//!         U256::from_be_bytes(&env, &bytesn!(&env, 0x0cc57cdbb08507d62bf67a4493cc262fb6c09d557013fff1f573f431221f8ff9).into()),
//!         U256::from_be_bytes(&env, &bytesn!(&env, 0x1274e649a32ed355a31a6ed69724e1adade857e86eb5c3a121bcd147943203c8).into()),
//!     ],
//! ];
//!
//! // Define round constants
//! let rc = vec![
//!     &env,
//!     vec![&env, U256::from_u32(&env, 1), U256::from_u32(&env, 2)],
//!     vec![&env, U256::from_u32(&env, 3), U256::from_u32(&env, 4)],
//!     vec![&env, U256::from_u32(&env, 5), U256::from_u32(&env, 6)],
//! ];
//!
//! let input = vec![&env, U256::from_u32(&env, 0), U256::from_u32(&env, 1)];
//!
//! let hazmat = CryptoHazmat::new(&env);
//! let result = hazmat.poseidon_permutation(
//!     &input,
//!     Symbol::new(&env, "BN254"),
//!     2,  // t: state width
//!     5,  // d: s-box exponent
//!     2,  // rounds_f: full rounds
//!     1,  // rounds_p: partial rounds
//!     &mds,
//!     &rc,
//! );
//!
//! assert_eq!(result.len(), 2);
//! # }
//! ```
