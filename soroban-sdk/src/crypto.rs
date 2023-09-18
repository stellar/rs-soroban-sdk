//! Crypto contains functions for cryptographic functions.
use crate::{env::internal, unwrap::UnwrapInfallible, Bytes, BytesN, Env};

/// Crypto provides access to cryptographic functions.
pub struct Crypto {
    env: Env,
}

impl Crypto {
    pub(crate) fn new(env: &Env) -> Crypto {
        Crypto { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Returns the SHA-256 hash of the data.
    pub fn sha256(&self, data: &Bytes) -> BytesN<32> {
        let env = self.env();
        let bin = internal::Env::compute_hash_sha256(env, data.into()).unwrap_infallible();
        unsafe { BytesN::unchecked_new(env.clone(), bin) }
    }

    /// Returns the Keccak-256 hash of the data.
    pub fn keccak256(&self, data: &Bytes) -> BytesN<32> {
        let env = self.env();
        let bin = internal::Env::compute_hash_keccak256(env, data.into()).unwrap_infallible();
        unsafe { BytesN::unchecked_new(env.clone(), bin) }
    }

    /// Verifies an ed25519 signature.
    ///
    /// The signature is verified as a valid signature of the message by the
    /// ed25519 public key.
    ///
    /// ### Panics
    ///
    /// If the signature verification fails.
    pub fn ed25519_verify(&self, public_key: &BytesN<32>, message: &Bytes, signature: &BytesN<64>) {
        let env = self.env();
        let _ = internal::Env::verify_sig_ed25519(
            env,
            public_key.to_object(),
            message.to_object(),
            signature.to_object(),
        );
    }

    /// Recovers the ECDSA secp256k1 public key.
    ///
    /// The public key returned is the SEC-1-encoded ECDSA secp256k1 public key
    /// that produced the 64-byte signature over a given 32-byte message digest,
    /// for a given recovery_id byte.
    pub fn secp256k1_recover(
        &self,
        message_digest: &BytesN<32>,
        signature: &BytesN<64>,
        recorvery_id: u32,
    ) -> BytesN<65> {
        let env = self.env();
        let bytes = internal::Env::recover_key_ecdsa_secp256k1(
            env,
            message_digest.to_object(),
            signature.to_object(),
            recorvery_id.into(),
        )
        .unwrap_infallible();
        unsafe { BytesN::unchecked_new(env.clone(), bytes) }
    }
}
