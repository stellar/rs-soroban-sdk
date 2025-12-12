#![cfg(any(test, feature = "hazmat-address"))]
#![cfg_attr(
    feature = "docs",
    doc(cfg(any(feature = "hazmat", feature = "hazmat-address")))
)]

//! Address payload extraction and construction.
//!
//! This module provides types and functions for extracting raw payloads from
//! addresses and constructing addresses from raw payloads. This is useful for
//! cross-chain interoperability where raw addresses need to be passed between
//! different systems.
//!
//! # Warning
//!
//! For account addresses, the Ed25519 public key corresponds to the account's
//! master key, which depending on the configuration of that account may or may
//! not be a signer of the account. Do not use this for custom Ed25519 signature
//! verification as a form of authentication.

use crate::{unwrap::UnwrapOptimized, Address, Bytes, BytesN, Env};

/// The payload contained in an [`Address`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressPayload {
    /// A 32-byte Ed25519 public key from an account address (G...).
    ///
    /// # Warning
    ///
    /// The Ed25519 public key corresponds to the account's master key, which
    /// depending on the configuration of that account may or may not be a
    /// signer of the account. Do not use this for custom Ed25519 signature
    /// verification as a form of authentication.
    AccountEd25519PublicKey(BytesN<32>),
    /// A 32-byte contract hash from a contract address (C...).
    ContractIDHash(BytesN<32>),
}

impl AddressPayload {
    /// Constructs an [`Address`] from this payload.
    ///
    /// This is the inverse of [`from_address`][AddressPayload::from_address].
    ///
    /// # Warning
    ///
    /// For account addresses, the Ed25519 public key corresponds to the
    /// account's master key, which depending on the configuration of that
    /// account may or may not be a signer of the account. Do not use this for
    /// custom Ed25519 signature verification as a form of authentication.
    pub fn to_address(&self, env: &Env) -> Address {
        use crate::xdr::FromXdr;
        // Build XDR header and get payload bytes based on payload type:
        let (header, payload_bytes): (&[u8], &BytesN<32>) = match self {
            AddressPayload::AccountEd25519PublicKey(bytes) => (
                &[
                    0, 0, 0, 18, // ScVal::Address
                    0, 0, 0, 0, // ScAddress::Account
                    0, 0, 0, 0, // PublicKey::PublicKeyTypeEd25519
                ],
                bytes,
            ),
            AddressPayload::ContractIDHash(bytes) => (
                &[
                    0, 0, 0, 18, // ScVal::Address
                    0, 0, 0, 1, // ScAddress::Contract
                ],
                bytes,
            ),
        };

        let mut xdr = Bytes::from_slice(env, header);
        xdr.append(&Bytes::from(payload_bytes.clone()));

        Address::from_xdr(env, &xdr).unwrap_optimized()
    }

    /// Extracts an [`AddressPayload`] from an [`Address`].
    ///
    /// Returns:
    /// - For contract addresses (C...), returns [`AddressPayload::ContractIDHash`]
    ///   containing the 32-byte contract hash.
    /// - For account addresses (G...), returns [`AddressPayload::AccountEd25519PublicKey`]
    ///   containing the 32-byte Ed25519 public key.
    ///
    /// Returns `None` if the address type is not recognized. This may occur if
    /// a new address type has been introduced to the network that this version
    /// of this library is not aware of.
    ///
    /// # Warning
    ///
    /// For account addresses, the returned Ed25519 public key corresponds to
    /// the account's master key, which depending on the configuration of that
    /// account may or may not be a signer of the account. Do not use this for
    /// custom Ed25519 signature verification as a form of authentication.
    pub fn from_address(address: &Address) -> Option<Self> {
        use crate::xdr::ToXdr;
        let xdr = address.to_xdr(address.env());
        // Skip over ScVal discriminant because we know it is an ScAddress.
        let xdr = xdr.slice(4..);
        // Decode ScAddress
        let addr_type: BytesN<4> = xdr.slice(0..4).try_into().unwrap_optimized();
        match addr_type.to_array() {
            // Decode ScAddress::Account
            [0, 0, 0, 0] => {
                // Decode PublicKey
                let public_key_type: BytesN<4> = xdr.slice(4..8).try_into().unwrap_optimized();
                match public_key_type.to_array() {
                    // Decode PublicKey::PublicKeyTypeEd25519
                    [0, 0, 0, 0] => {
                        let ed25519: BytesN<32> = xdr.slice(8..40).try_into().unwrap_optimized();
                        Some(AddressPayload::AccountEd25519PublicKey(ed25519))
                    }
                    _ => None,
                }
            }
            // Decode ScAddress::Contract
            [0, 0, 0, 1] => {
                let hash: BytesN<32> = xdr.slice(4..36).try_into().unwrap_optimized();
                Some(AddressPayload::ContractIDHash(hash))
            }
            _ => None,
        }
    }
}
