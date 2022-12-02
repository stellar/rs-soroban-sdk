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

    /// Computes a SHA-256 hash.
    pub fn sha256(&self, msg: &Bytes) -> BytesN<32> {
        let env = self.env();
        let bin_obj = internal::Env::compute_hash_sha256(env, msg.into());
        unsafe { BytesN::unchecked_new(bin_obj.in_env(env)) }
    }

    /// Verifies an ed25519 signature.
    ///
    /// The ed25519 signature (`sig`) is verified as a valid signature of the
    /// message (`msg`) by the ed25519 public key (`pk`).
    ///
    /// ### Panics
    ///
    /// Will panic if the signature verification fails.
    ///
    /// ### TODO
    ///
    /// Return a [Result] instead of panicking.
    pub fn ed25519_verify(&self, pk: &BytesN<32>, msg: &Bytes, sig: &BytesN<64>) {
        let env = self.env();
        let _ = internal::Env::verify_sig_ed25519(
            env,
            msg.to_object(),
            pk.to_object(),
            sig.to_object(),
        );
    }
}
