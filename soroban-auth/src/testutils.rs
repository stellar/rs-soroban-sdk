#![cfg(feature = "testutils")]
#![cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]

//! Utilities intended for use when testing contracts that use
//! [`soroban_auth`](crate).

pub mod ed25519 {
    use core::panic;

    use ed25519_dalek::Keypair;
    use soroban_sdk::{
        testutils::ed25519::{Generate, Sign},
        BytesN, Env, IntoVal, RawVal, Symbol, Vec,
    };

    use crate::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};

    /// Generate an ed25519 identifier and signer that can sign
    /// [`SignaturePayload`]s for that identifier.
    pub fn generate(
        env: &Env,
    ) -> (
        Identifier,
        impl Sign<SignaturePayload, Signature = [u8; 64]>,
    ) {
        let signer = <Keypair as Generate<_, SignaturePayload>>::generate();
        (
            Identifier::Ed25519(signer.public.to_bytes().into_val(env)),
            signer,
        )
    }

    /// Sign a [`SignaturePayload`] constructed using the arguments.
    ///
    /// The returned [`Signature`] can be verified by [`verify`](crate::verify)
    /// with the same arguments within the specified contract.
    pub fn sign(
        env: &Env,
        signer: (
            &Identifier,
            &impl Sign<SignaturePayload, Signature = [u8; 64]>,
        ),
        contract: &BytesN<32>,
        function: Symbol,
        args: impl IntoVal<Env, Vec<RawVal>>,
    ) -> Signature {
        let (public_key, signer) = if let (Identifier::Ed25519(public_key), signer) = &signer {
            (public_key, signer)
        } else {
            panic!("identifier must be ed25519")
        };
        let payload = SignaturePayload::V0(SignaturePayloadV0 {
            network: env.ledger().network_passphrase(),
            contract: contract.clone(),
            function,
            args: args.into_val(env),
        });
        let signature = match signer.sign(payload) {
            Ok(signature) => signature,
            Err(_) => panic!("error signing signature payload"),
        };
        Signature::Ed25519(Ed25519Signature {
            public_key: public_key.into_val(env),
            signature: signature.into_val(env),
        })
    }
}

// TODO: Add account module that contains utilities for producing signatures for
// accounts.
// pub mod account {
// }
