//! Crypto contains functions for cryptographic functions.

use crate::{
    env::internal, unwrap::UnwrapInfallible, Bytes, BytesN, ConversionError, Env, IntoVal,
    TryFromVal, Val,
};

/// A wrapper type for a cryptographic hash.
///
/// This struct is designed to be used in contexts where a hash value generated
/// by a secure cryptographic function is required.  It can only be constructed
/// via secure manners, i.e. output from a secure hash function, or received
/// from the host (e.g. via `CustomAccountInterface`)
pub struct Hash<const N: usize>(BytesN<N>);

impl<const N: usize> Hash<N> {
    /// Constructs a new `Hash` from a fixed-length bytes array.
    ///
    /// This is intended for test-only, since `Hash` type is only meant to be
    /// constructed via secure manners.
    #[cfg(test)]
    pub fn from_bytes(bytes: BytesN<N>) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> IntoVal<Env, Val> for Hash<N> {
    fn into_val(&self, e: &Env) -> Val {
        self.0.into_val(e)
    }
}

impl<const N: usize> IntoVal<Env, BytesN<N>> for Hash<N> {
    fn into_val(&self, _e: &Env) -> BytesN<N> {
        self.0.clone()
    }
}

impl<const N: usize> Into<BytesN<N>> for Hash<N> {
    fn into(self) -> BytesN<N> {
        self.0
    }
}

impl<const N: usize> Into<[u8; N]> for Hash<N> {
    fn into(self) -> [u8; N] {
        self.0.into()
    }
}

impl<const N: usize> TryFromVal<Env, Val> for Hash<N> {
    type Error = ConversionError;

    fn try_from_val(env: &Env, v: &Val) -> Result<Self, Self::Error> {
        Ok(Hash(BytesN::<N>::try_from_val(env, v)?))
    }
}

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
    pub fn sha256(&self, data: &Bytes) -> Hash<32> {
        let env = self.env();
        let bin = internal::Env::compute_hash_sha256(env, data.into()).unwrap_infallible();
        unsafe { Hash(BytesN::unchecked_new(env.clone(), bin)) }
    }

    /// Returns the Keccak-256 hash of the data.
    pub fn keccak256(&self, data: &Bytes) -> Hash<32> {
        let env = self.env();
        let bin = internal::Env::compute_hash_keccak256(env, data.into()).unwrap_infallible();
        unsafe { Hash(BytesN::unchecked_new(env.clone(), bin)) }
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
        message_digest: &Hash<32>,
        signature: &BytesN<64>,
        recorvery_id: u32,
    ) -> BytesN<65> {
        let env = self.env();
        CryptoHazmat::new(env).secp256k1_recover(&message_digest.0, signature, recorvery_id)
    }

    /// Verifies the ECDSA secp256r1 signature.
    ///
    /// The SEC-1-encoded public key is provided along with the message,
    /// verifies the 64-byte signature.
    pub fn secp256r1_verify(
        &self,
        public_key: &BytesN<65>,
        message_digest: &Hash<32>,
        signature: &BytesN<64>,
    ) {
        let env = self.env();
        CryptoHazmat::new(env).secp256r1_verify(public_key, &message_digest.0, signature)
    }
}

/// Hazardous Materials
///
/// Cryptographic functions under [CryptoHazmat] are low-leveled which can be
/// insecure if misused. They are not generally recommended. Using them
/// incorrectly can introduce security vulnerabilities. Please use [Crypto] if
/// possible.
pub struct CryptoHazmat {
    env: Env,
}

impl CryptoHazmat {
    pub(crate) fn new(env: &Env) -> CryptoHazmat {
        CryptoHazmat { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Recovers the ECDSA secp256k1 public key.
    ///
    /// The public key returned is the SEC-1-encoded ECDSA secp256k1 public key
    /// that produced the 64-byte signature over a given 32-byte message digest,
    /// for a given recovery_id byte.
    ///
    /// WARNING: The `message_digest` must be produced by a secure cryptographic
    /// hash function on the message, otherwise the attacker can potentially
    /// forge signatures.
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

    /// Verifies the ECDSA secp256r1 signature.
    ///
    /// The SEC-1-encoded public key is provided along with a 32-byte message
    /// digest, verifies the 64-byte signature.
    ///
    /// WARNING: The `message_digest` must be produced by a secure cryptographic
    /// hash function on the message, otherwise the attacker can potentially
    /// forge signatures.
    pub fn secp256r1_verify(
        &self,
        public_key: &BytesN<65>,
        message_digest: &BytesN<32>,
        signature: &BytesN<64>,
    ) {
        let env = self.env();
        let _ = internal::Env::verify_sig_ecdsa_secp256r1(
            env,
            public_key.to_object(),
            message_digest.to_object(),
            signature.to_object(),
        )
        .unwrap_infallible();
    }
}
