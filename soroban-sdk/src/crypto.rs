//! Crypto contains functions for cryptographic functions.
use crate::{env::internal, Bytes, BytesN, Env};

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
        let bin = internal::Env::compute_hash_sha256(env, data.into());
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
            message.to_object(),
            public_key.to_object(),
            signature.to_object(),
        );
    }
}
