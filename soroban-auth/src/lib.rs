//! Soroban auth provides basic authentication capabilities to Soroban
//! contracts.
//!
//! For contracts that require basic authentication capabilities this crate may
//! do some of the heavy lifting for supporting authentication by Stellar
//! accounts, ed25519 signatures, or other contracts. For contracts that require
//! more bespoke authentication this crate may not be suitable.
//!
//! See [`verify`] for how to use.
//!
//! See [`testutils`] for test utilities.
//!
//! **The utilities in this crate provide no replay protection. Contracts must
//! provide their own mechanism suitable for replay prevention that prevents
//! contract invocations to be replayable if it is important they are not.**
#![no_std]
#![cfg_attr(feature = "docs", feature(doc_cfg))]

mod tests;

use soroban_sdk::{
    serde::Serialize, unwrap::UnwrapOptimized, BytesN, Env, IntoVal, RawVal, Symbol, Vec,
};

pub mod testutils;

mod public_types;
pub use crate::public_types::{
    AccountSignatures, Ed25519Signature, Identifier, Signature, SignaturePayload,
    SignaturePayloadV0,
};

const MAX_ACCOUNT_SIGNATURES: u32 = 20;

fn verify_ed25519_signature(env: &Env, auth: &Ed25519Signature, name: Symbol, args: Vec<RawVal>) {
    let msg = SignaturePayloadV0 {
        name,
        contract: env.get_current_contract(),
        network: env.ledger().network_passphrase(),
        args,
    };
    let msg_bin = SignaturePayload::V0(msg).serialize(env);

    env.crypto()
        .ed25519_verify(&auth.public_key, &msg_bin, &auth.signature);
}

fn verify_account_signatures(env: &Env, auth: &AccountSignatures, name: Symbol, args: Vec<RawVal>) {
    let acc = env.accounts().get(&auth.account_id).unwrap_optimized();

    let msg = SignaturePayloadV0 {
        name,
        contract: env.get_current_contract(),
        network: env.ledger().network_passphrase(),
        args,
    };
    let msg_bytes = SignaturePayload::V0(msg).serialize(env);

    let threshold = acc.medium_threshold();
    let mut weight = 0u32;

    let sigs = &auth.signatures;

    // Check if there is too many signatures: there shouldn't be more
    // signatures then the amount of account signers.
    if sigs.len() > MAX_ACCOUNT_SIGNATURES {
        panic!("too many account signatures");
    }

    let mut prev_pk: Option<BytesN<32>> = None;
    for sig in sigs.iter().map(Result::unwrap) {
        // Cannot take multiple signatures from the same key
        if let Some(prev) = prev_pk {
            if prev == sig.public_key {
                panic!("signature duplicate")
            }
            if prev > sig.public_key {
                panic!("signature out of order")
            }
        }

        env.crypto()
            .ed25519_verify(&sig.public_key, &msg_bytes, &sig.signature);
        let signer_weight = acc.signer_weight(&sig.public_key);
        if signer_weight == 0 {
            panic!("signature doesn't belong to account");
        }
        // A signature's weight can be at most u8::MAX, hence overflow isn't
        // possible here as u8::MAX * MAX_ACCOUNT_SIGNATURES is < u32::MAX.
        let signer_weight: u32 = signer_weight.into();
        weight += signer_weight;

        prev_pk = Some(sig.public_key);
    }

    if weight < threshold.into() {
        panic!("insufficient signing weight")
    }
}

/// Verify that a [`Signature`] is a valid signature of a [`SignaturePayload`]
/// containing the provided arguments by the [`Identifier`] contained within the
/// [`Signature`].
///
/// Verify that the given signature is a signature of the [`SignaturePayload`]
/// that contain `name`, and `args`.
///
/// Three types of signature are accepted:
///
/// - Contract Signature
///
///   An invoking account or contract can sign the message by simply making the
///   invocation. No actual signature of [`SignaturePayload`] is required.
///
/// - Ed25519 Signature
///
///   An ed25519 key can sign [`SignaturePayload`] and include that signature in
///   the `sig` field.
///
/// - Account Signatures
///
///   An account's signers can sign [`SignaturePayload`] and include those
///   signatures in the `sig` field.
///
/// **This function provides no replay protection. Contracts must provide their
/// own mechanism suitable for replay prevention that prevents contract
/// invocations to be replayable if it is important they are not.**
pub fn verify(env: &Env, sig: &Signature, name: Symbol, args: impl IntoVal<Env, Vec<RawVal>>) {
    match sig {
        Signature::Invoker => {}
        Signature::Ed25519(e) => verify_ed25519_signature(env, e, name, args.into_val(env)),
        Signature::Account(a) => verify_account_signatures(env, a, name, args.into_val(env)),
    }
}
