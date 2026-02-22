//! BN254 (alt_bn128) elliptic curve support.
//!
//! Support for BN254 (also known as alt_bn128), a pairing-friendly elliptic curve commonly used in
//! zero-knowledge proof systems.
//!
//! The BN254 functionality is accessed via `env.crypto().bn254()`.
//!
//! ## New Types
//!
//! - [`Bn254G1Affine`] - A point in the G1 group (64 bytes, Ethereum-compatible format)
//! - [`Bn254G2Affine`] - A point in the G2 group (128 bytes, Ethereum-compatible format)
//! - [`Fr`] - A scalar field element (32 bytes, internally a `U256`)
//! - [`Bn254Fp`] - A base field element (32 bytes)
//!
//! ## New Operations
//!
//! - `g1_add` - Add two G1 points
//! - `g1_mul` - Multiply a G1 point by a scalar
//! - `pairing_check` - Multi-pairing check for ZK proof verification
//!
//! G1 points also support arithmetic operations via Rust traits:
//! - `Add` - Add two G1 points
//! - `Mul<Fr>` - Multiply a G1 point by a scalar
//! - `Neg` - Negate a G1 point
//!
//! ## Example: Basic G1 Operations
//!
//! ```
//! use soroban_sdk::{Env, BytesN, U256};
//! use soroban_sdk::crypto::bn254::{Bn254G1Affine, Fr};
//!
//! # fn main() {
//! let env = Env::default();
//! let bn254 = env.crypto().bn254();
//!
//! // The generator point G1 for BN254
//! // G1 = (1, 2)
//! let g1_bytes: [u8; 64] = [
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // x = 1
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // y = 2
//! ];
//! let g1 = Bn254G1Affine::from_array(&env, &g1_bytes);
//!
//! // Add the generator to itself: G + G = 2G
//! let g1_doubled = bn254.g1_add(&g1, &g1);
//!
//! // Or use the Add trait
//! let g1_doubled_alt = g1.clone() + g1.clone();
//! assert_eq!(g1_doubled, g1_doubled_alt);
//!
//! // Scalar multiplication: 2 * G should equal G + G
//! let scalar: Fr = U256::from_u32(&env, 2).into();
//! let g1_times_2 = bn254.g1_mul(&g1, &scalar);
//! assert_eq!(g1_doubled, g1_times_2);
//!
//! // Or use the Mul trait
//! let g1_times_2_alt = g1.clone() * scalar;
//! assert_eq!(g1_doubled, g1_times_2_alt);
//! # }
//! ```
//!
//! ## Example: Point Negation
//!
//! ```
//! use soroban_sdk::{Env, BytesN};
//! use soroban_sdk::crypto::bn254::Bn254G1Affine;
//!
//! # fn main() {
//! let env = Env::default();
//! let bn254 = env.crypto().bn254();
//!
//! // G1 generator point (1, 2)
//! let g1_bytes: [u8; 64] = [
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
//! ];
//! let g1 = Bn254G1Affine::from_array(&env, &g1_bytes);
//!
//! // Negate the point: -G has same x but negated y
//! let neg_g1 = -g1.clone();
//!
//! // G + (-G) = point at infinity (all zeros)
//! let sum = bn254.g1_add(&g1, &neg_g1);
//! let infinity = Bn254G1Affine::from_array(&env, &[0u8; 64]);
//! assert_eq!(sum, infinity);
//! # }
//! ```
//!
//! ## Example: Pairing Check
//!
//! ```
//! use soroban_sdk::{Env, vec, Vec};
//! use soroban_sdk::crypto::bn254::{Bn254G1Affine, Bn254G2Affine};
//!
//! # fn main() {
//! let env = Env::default();
//! let bn254 = env.crypto().bn254();
//!
//! // Example G1 and G2 points from Ethereum test vectors - https://github.com/ethereum/go-ethereum/blob/master/core/vm/testdata/precompiles/bn256Pairing.json
//! let g1_bytes: [u8; 64] = [
//!     0x1c, 0x76, 0x47, 0x6f, 0x4d, 0xef, 0x4b, 0xb9, 0x45, 0x41, 0xd5, 0x7e, 0xbb, 0xa1, 0x19, 0x33,
//!     0x81, 0xff, 0xa7, 0xaa, 0x76, 0xad, 0xa6, 0x64, 0xdd, 0x31, 0xc1, 0x60, 0x24, 0xc4, 0x3f, 0x59,
//!     0x30, 0x34, 0xdd, 0x29, 0x20, 0xf6, 0x73, 0xe2, 0x04, 0xfe, 0xe2, 0x81, 0x1c, 0x67, 0x87, 0x45,
//!     0xfc, 0x81, 0x9b, 0x55, 0xd3, 0xe9, 0xd2, 0x94, 0xe4, 0x5c, 0x9b, 0x03, 0xa7, 0x6a, 0xef, 0x41,
//! ];
//! let g1 = Bn254G1Affine::from_array(&env, &g1_bytes);
//!
//! let g2_bytes: [u8; 128] = [
//!     0x20, 0x9d, 0xd1, 0x5e, 0xbf, 0xf5, 0xd4, 0x6c, 0x4b, 0xd8, 0x88, 0xe5, 0x1a, 0x93, 0xcf, 0x99,
//!     0xa7, 0x32, 0x96, 0x36, 0xc6, 0x35, 0x14, 0x39, 0x6b, 0x4a, 0x45, 0x20, 0x03, 0xa3, 0x5b, 0xf7,
//!     0x04, 0xbf, 0x11, 0xca, 0x01, 0x48, 0x3b, 0xfa, 0x8b, 0x34, 0xb4, 0x35, 0x61, 0x84, 0x8d, 0x28,
//!     0x90, 0x59, 0x60, 0x11, 0x4c, 0x8a, 0xc0, 0x40, 0x49, 0xaf, 0x4b, 0x63, 0x15, 0xa4, 0x16, 0x78,
//!     0x2b, 0xb8, 0x32, 0x4a, 0xf6, 0xcf, 0xc9, 0x35, 0x37, 0xa2, 0xad, 0x1a, 0x44, 0x5c, 0xfd, 0x0c,
//!     0xa2, 0xa7, 0x1a, 0xcd, 0x7a, 0xc4, 0x1f, 0xad, 0xbf, 0x93, 0x3c, 0x2a, 0x51, 0xbe, 0x34, 0x4d,
//!     0x12, 0x0a, 0x2a, 0x4c, 0xf3, 0x0c, 0x1b, 0xf9, 0x84, 0x5f, 0x20, 0xc6, 0xfe, 0x39, 0xe0, 0x7e,
//!     0xa2, 0xcc, 0xe6, 0x1f, 0x0c, 0x9b, 0xb0, 0x48, 0x16, 0x5f, 0xe5, 0xe4, 0xde, 0x87, 0x75, 0x50,
//! ];
//! let g2 = Bn254G2Affine::from_array(&env, &g2_bytes);
//!
//! // Create vectors of G1 and G2 points
//! let g1_vec: Vec<Bn254G1Affine> = vec![&env, g1];
//! let g2_vec: Vec<Bn254G2Affine> = vec![&env, g2];
//!
//! // Perform pairing check
//! // Returns true if e(G1[0], G2[0]) * e(G1[1], G2[1]) * ... = 1
//! let result = bn254.pairing_check(g1_vec, g2_vec);
//! // result will be true or false depending on the pairing equation
//! # }
//! ```
//!
//! [`Bn254G1Affine`]: crate::crypto::bn254::Bn254G1Affine
//! [`Bn254G2Affine`]: crate::crypto::bn254::Bn254G2Affine
//! [`Fr`]: crate::crypto::bn254::Fr
//! [`Bn254Fp`]: crate::crypto::bn254::Bn254Fp
