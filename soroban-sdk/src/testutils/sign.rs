#![cfg(any(test, feature = "testutils"))]

/// Sign implementations produce signatures for types that can be represented as
/// the MSG.
pub trait Sign<MSG> {
    type Signature;
    type Error;
    /// Sign produces a signature for MSGs.
    fn sign(&self, m: MSG) -> Result<Self::Signature, Self::Error>;
}

// TODO: Add a Verify interface and ed25519 implementation to counter the Sign
// interface.

pub mod ed25519 {
    use xdr::WriteXdr;

    use crate::xdr;

    #[derive(Debug)]
    pub enum Error<E: std::error::Error> {
        XdrError(xdr::Error),
        Ed25519SignatureError(ed25519_dalek::SignatureError),
        ConversionError(E),
    }

    impl<E: std::error::Error> std::error::Error for Error<E> {
        #[must_use]
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::XdrError(e) => e.source(),
                Self::Ed25519SignatureError(e) => e.source(),
                Self::ConversionError(e) => e.source(),
            }
        }
    }

    impl<E: std::error::Error> std::fmt::Display for Error<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Self::XdrError(e) => write!(f, "{}", e),
                Self::Ed25519SignatureError(e) => write!(f, "{}", e),
                Self::ConversionError(e) => write!(f, "{}", e),
            }
        }
    }

    impl<E: std::error::Error> From<xdr::Error> for Error<E> {
        fn from(e: xdr::Error) -> Self {
            Error::XdrError(e)
        }
    }

    impl<E: std::error::Error> From<ed25519_dalek::SignatureError> for Error<E> {
        fn from(e: ed25519_dalek::SignatureError) -> Self {
            Error::Ed25519SignatureError(e)
        }
    }

    pub use super::Sign;

    impl<S, M> Sign<M> for S
    where
        S: ed25519_dalek::Signer<ed25519_dalek::Signature>,
        M: TryInto<xdr::ScVal>,
        <M as TryInto<xdr::ScVal>>::Error: std::error::Error,
    {
        type Error = Error<<M as TryInto<xdr::ScVal>>::Error>;
        type Signature = [u8; 64];
        fn sign(&self, m: M) -> Result<Self::Signature, Self::Error> {
            let mut buf = Vec::<u8>::new();
            let val: xdr::ScVal = m.try_into().map_err(Self::Error::ConversionError)?;
            val.write_xdr(&mut buf)?;
            Ok(self.try_sign(&buf)?.to_bytes())
        }
    }

    #[cfg(test)]
    mod test {
        use ed25519_dalek::{Keypair, PublicKey, SecretKey};

        use super::Sign;

        #[test]
        fn sign() {
            let sk = SecretKey::from_bytes(
                &hex::decode("5acc7253295dfc356c046297925a369f3d2762d00afdf2583ecbe92180b07c37")
                    .unwrap(),
            )
            .unwrap();
            let pk = PublicKey::from(&sk);
            let kp = Keypair {
                secret: sk,
                public: pk,
            };
            let sig = kp.sign(128i64).unwrap();
            assert_eq!(
                hex::encode(sig),
                // Verified with https://go.dev/play/p/BYht__ahx7y.
                "082f78fb1864f6914de4c3c4e3e0c6e7c63a6a866aa81bda8042f74155cb95e7d29958061a11568f03db137cbf17c8b7bbf6193b2901af9888bbdf150c7be00a",
            );
        }
    }
}
