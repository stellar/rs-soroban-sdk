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
