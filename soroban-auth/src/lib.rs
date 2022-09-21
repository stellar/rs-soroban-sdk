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

use soroban_sdk::{serde::Serialize, Account, BytesN, Env, IntoVal, RawVal, Symbol, Vec};

pub mod testutils;

mod public_types;
pub use crate::public_types::{
    AccountSignatures, Ed25519Signature, Identifier, Signature, SignaturePayload,
    SignaturePayloadV0,
};

fn verify_ed25519_signature(
    env: &Env,
    auth: &Ed25519Signature,
    function: Symbol,
    args: Vec<RawVal>,
) {
    let msg = SignaturePayloadV0 {
        function,
        contract: env.get_current_contract(),
        network: env.ledger().network_passphrase(),
        args,
    };
    let msg_bin = SignaturePayload::V0(msg).serialize(env);

    env.verify_sig_ed25519(&auth.public_key, &msg_bin, &auth.signature);
}

fn verify_account_signatures(
    env: &Env,
    auth: &AccountSignatures,
    function: Symbol,
    args: Vec<RawVal>,
) {
    let acc = Account::from_public_key(&auth.account_id).unwrap();

    let msg = SignaturePayloadV0 {
        function,
        contract: env.get_current_contract(),
        network: env.ledger().network_passphrase(),
        args,
    };
    let msg_bytes = SignaturePayload::V0(msg).serialize(env);

    let threshold = acc.medium_threshold();
    let mut weight = 0u32;

    let sigs = &auth.signatures;
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

        env.verify_sig_ed25519(&sig.public_key, &msg_bytes, &sig.signature);

        weight = weight
            .checked_add(acc.signer_weight(&sig.public_key))
            .expect("weight overflow");

        prev_pk = Some(sig.public_key);
    }

    if weight < threshold {
        panic!("insufficient signing weight")
    }
}

/// Verify that a [`Signature`] is a valid signature of a [`SignaturePayload`]
/// containing the provided arguments by the [`Identifier`] contained within the
/// [`Signature`].
///
/// Verify that the given signature is a signature of the [`SignaturePayload`]
/// that contain `function`, and `args`.
///
/// Three types of signature are accepted:
///
/// - Contract Signature
///
///   An invoking contract can sign the message by simply making the invocation.
///   No actual signature of [`SignaturePayload`] is required.
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
pub fn verify(env: &Env, sig: &Signature, function: Symbol, args: impl IntoVal<Env, Vec<RawVal>>) {
    match sig {
        Signature::Contract => {
            env.get_invoking_contract();
        }
        Signature::Ed25519(e) => verify_ed25519_signature(env, &e, function, args.into_val(env)),
        Signature::Account(a) => verify_account_signatures(env, &a, function, args.into_val(env)),
    }
}

#[doc(hidden)]
#[deprecated(note = "use soroban_auth::verify(...)")]
pub fn check_auth(
    env: &Env,
    sig: &Signature,
    function: Symbol,
    args: impl IntoVal<Env, Vec<RawVal>>,
) {
    verify(env, sig, function, args)
}
