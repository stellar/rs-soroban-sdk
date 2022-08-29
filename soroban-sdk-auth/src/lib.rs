#![no_std]

use soroban_sdk::{serde::Serialize, Account, BigInt, BytesN, Env, RawVal, Symbol, Vec};

pub mod public_types;
use crate::public_types::{
    AccountSignatures, Ed25519Signature, Identifier, Message, MessageV0, Signature,
};

/// Users of this module will need to pass a struct that implements NonceAuth
/// NonceAuth should manage a nonce in the contracts storage
///
///
/// Example implementation for NonceAuth -
/// ```
/// struct WrappedAuth(Signature);
///
/// impl NonceAuth for WrappedAuth {
///
/// fn read_nonce(e: &Env, id: Identifier) -> BigInt {
///    let key = DataKey::Nonce(id);
///    if let Some(nonce) = e.contract_data().get(key) {
///       nonce.unwrap()
///    } else {
///        BigInt::zero(e)
///    }
/// }
///
/// fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt {
///     let key = DataKey::Nonce(id.clone());
///     let nonce = Self::read_nonce(e, id);
///     e.contract_data()
///         .set(key, nonce.clone() + BigInt::from_u32(e, 1));
///     nonce
/// }
///
/// fn get_keyed_auth(&self) -> &Signature {
///     &self.0
/// }
/// }
/// ```
pub trait NonceAuth {
    /// Return the nonce stored in the contract.
    fn read_nonce(e: &Env, id: Identifier) -> BigInt;
    /// Return the nonce stored in the contract, and then increment it.
    fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt;
    /// Return the Signature used for authorization.
    fn get_keyed_auth(&self) -> &Signature;
}

fn check_ed25519_auth(env: &Env, auth: &Ed25519Signature, function: Symbol, args: Vec<RawVal>) {
    let msg = MessageV0 {
        function,
        contrct_id: env.get_current_contract(),
        network_id: env.ledger().network_passphrase(),
        args,
    };
    let msg_bin = Message::V0(msg).serialize(env);

    env.verify_sig_ed25519(
        auth.public_key.clone().into(),
        msg_bin,
        auth.signature.clone().into(),
    );
}

fn check_account_auth(env: &Env, auth: &AccountSignatures, function: Symbol, args: Vec<RawVal>) {
    let acc = Account::from_public_key(&auth.account_id).unwrap();

    let msg = MessageV0 {
        function,
        contrct_id: env.get_current_contract(),
        network_id: env.ledger().network_passphrase(),
        args,
    };
    let msg_bytes = Message::V0(msg).serialize(env);

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

        env.verify_sig_ed25519(
            sig.public_key.clone().into(),
            msg_bytes.clone(),
            sig.signature.into(),
        );

        weight = weight
            .checked_add(acc.signer_weight(&sig.public_key))
            .expect("weight overflow");

        prev_pk = Some(sig.public_key);
    }

    if weight < threshold {
        panic!("insufficient signing weight")
    }
}

/// Checks a Signature that should be wrapped in a struct that implements NonceAuth
/// Note that the nonce is expected to be 0 if the signature is of type Signature::Contract
pub fn check_auth<T>(env: &Env, auth: &T, nonce: BigInt, function: Symbol, args: Vec<RawVal>)
where
    T: NonceAuth,
{
    match auth.get_keyed_auth() {
        Signature::Contract => {
            if nonce != BigInt::from_i32(env, 0) {
                panic!("nonce should be zero for Contract")
            }
            env.get_invoking_contract();
        }
        Signature::Ed25519(kea) => {
            let stored_nonce =
                auth.read_and_increment_nonce(env, Identifier::Ed25519(kea.public_key.clone()));
            if nonce != stored_nonce {
                panic!("incorrect nonce")
            }
            check_ed25519_auth(env, &kea, function, args)
        }
        Signature::Account(kaa) => {
            let stored_nonce =
                auth.read_and_increment_nonce(env, Identifier::Account(kaa.account_id.clone()));
            if nonce != stored_nonce {
                panic!("incorrect nonce")
            }
            check_account_auth(env, &kaa, function, args)
        }
    }
}
