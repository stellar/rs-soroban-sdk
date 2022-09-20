#![no_std]

use soroban_sdk::{serde::Serialize, Account, BytesN, Env, IntoVal, RawVal, Symbol, Vec};

mod public_types;
pub use crate::public_types::{
    AccountSignatures, Ed25519Signature, Identifier, Signature, SignaturePayload,
    SignaturePayloadV0,
};

fn check_ed25519_auth(env: &Env, auth: &Ed25519Signature, function: Symbol, args: Vec<RawVal>) {
    let msg = SignaturePayloadV0 {
        function,
        contract: env.get_current_contract(),
        network: env.ledger().network_passphrase(),
        args,
    };
    let msg_bin = SignaturePayload::V0(msg).serialize(env);

    env.verify_sig_ed25519(&auth.public_key, &msg_bin, &auth.signature);
}

fn check_account_auth(env: &Env, auth: &AccountSignatures, function: Symbol, args: Vec<RawVal>) {
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

/// Verifies a Signature. It's important to note that this module does
/// not provide replay protection. That will need to be implemented by
/// the user.
pub fn check_auth(
    env: &Env,
    sig: &Signature,
    function: Symbol,
    args: impl IntoVal<Env, Vec<RawVal>>,
) {
    match sig {
        Signature::Contract => {
            env.get_invoking_contract();
        }
        Signature::Ed25519(kea) => check_ed25519_auth(env, &kea, function, args.into_val(env)),
        Signature::Account(kaa) => check_account_auth(env, &kaa, function, args.into_val(env)),
    }
}
