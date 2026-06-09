//! Exercises delegated `__check_auth` via [`Address::delegate_auth`].
//!
//! A custom account may delegate (part of) its authentication to a set of other
//! addresses rather than performing all of it itself. The user chooses which of
//! the registered signers to authenticate with by attaching them to the
//! transaction's authorization payload as delegated signers.

use crate as soroban_sdk;

use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

use soroban_sdk::xdr::{
    self, HashIdPreimage, HashIdPreimageSorobanAuthorizationWithAddress, InvokeContractArgs,
    Limits, ScAddress, ScErrorCode, ScErrorType, ScVal, SorobanAddressCredentials,
    SorobanAddressCredentialsWithDelegates, SorobanAuthorizationEntry, SorobanAuthorizedFunction,
    SorobanAuthorizedInvocation, SorobanCredentials, SorobanDelegateSignature, StringM, VecM,
    WriteXdr,
};
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contracttype,
    crypto::Hash,
    vec, Address, BytesN, Env, Error, Symbol, TryFromVal, Vec,
};

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ModularAccountError {
    UnknownDelegate = 1,
}

#[contracttype]
pub enum DataKey {
    PublicKey,
    Signers,
    AuthorizedCalls,
}

// A custom account that can delegate authentication to a set of registered
// signer addresses.
#[contract]
pub struct ModularAccount;

#[contractimpl]
impl ModularAccount {
    pub fn __constructor(env: Env, public_key: BytesN<32>, signers: Vec<Address>) {
        env.storage()
            .instance()
            .set(&DataKey::PublicKey, &public_key);
        env.storage().instance().set(&DataKey::Signers, &signers);
    }
}

#[contractimpl]
impl CustomAccountInterface for ModularAccount {
    type Signature = BytesN<64>;
    type Error = ModularAccountError;

    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: BytesN<64>,
        auth_contexts: Vec<Context>,
    ) -> Result<(), ModularAccountError> {
        // Even though we use delegated authentication, the account can still
        // perform the regular verification if necessary.
        let public_key: BytesN<32> = env.storage().instance().get(&DataKey::PublicKey).unwrap();
        env.crypto()
            .ed25519_verify(&public_key, &signature_payload.into(), &signature);
        record_authorized_calls(&env, &auth_contexts);

        // The signers the user attached to the transaction for this account's
        // authorization.
        let delegates = env.get_delegated_signers();

        let signers: Vec<Address> = env.storage().instance().get(&DataKey::Signers).unwrap();
        for delegate in delegates.iter() {
            // The host can not validate the delegates, so the account has to
            // check that each one is actually a registered signer.
            if !signers.contains(&delegate) {
                return Err(ModularAccountError::UnknownDelegate);
            }
            // Forward the current authentication context to the delegate.
            delegate.delegate_auth();
        }
        Ok(())
    }
}

// An account that performs ed25519 verification and is used as a delegate
// signer of `ModularAccount`.
#[contract]
pub struct DelegateAccount;

#[contractimpl]
impl DelegateAccount {
    pub fn __constructor(env: Env, public_key: BytesN<32>) {
        env.storage()
            .instance()
            .set(&DataKey::PublicKey, &public_key);
    }
}

#[contractimpl]
impl CustomAccountInterface for DelegateAccount {
    type Signature = BytesN<64>;
    type Error = crate::Error;

    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: BytesN<64>,
        auth_contexts: Vec<Context>,
    ) -> Result<(), crate::Error> {
        let public_key: BytesN<32> = env.storage().instance().get(&DataKey::PublicKey).unwrap();
        env.crypto()
            .ed25519_verify(&public_key, &signature_payload.into(), &signature);
        record_authorized_calls(&env, &auth_contexts);
        Ok(())
    }
}

// A contract with an operation that requires the account's authorization.
#[contract]
pub struct Protected;

#[contractimpl]
impl Protected {
    pub fn protected(account: Address) {
        account.require_auth();
    }
}

fn sign(env: &Env, key: &SigningKey, payload: &[u8; 32]) -> ScVal {
    let sig: [u8; 64] = key.sign(payload).to_bytes();
    ScVal::try_from_val(env, &BytesN::from_array(env, &sig).to_val()).unwrap()
}

// Appends the function name from each contract-call context to a per-account
// log in instance storage so the test can verify what the account approved.
fn record_authorized_calls(env: &Env, auth_contexts: &Vec<Context>) {
    let mut calls: Vec<Symbol> = env
        .storage()
        .instance()
        .get(&DataKey::AuthorizedCalls)
        .unwrap_or_else(|| Vec::new(env));
    for ctx in auth_contexts.iter() {
        if let Context::Contract(c) = ctx {
            calls.push_back(c.fn_name);
        }
    }
    env.storage()
        .instance()
        .set(&DataKey::AuthorizedCalls, &calls);
}

#[test]
fn test_delegate_auth() {
    let env = Env::default();

    let account_key = SigningKey::from_bytes(&[1u8; 32]);
    let key_a = SigningKey::from_bytes(&[2u8; 32]);
    let key_b = SigningKey::from_bytes(&[3u8; 32]);

    let delegate_a = env.register(
        DelegateAccount,
        (BytesN::from_array(&env, &key_a.verifying_key().to_bytes()),),
    );
    let delegate_b = env.register(
        DelegateAccount,
        (BytesN::from_array(&env, &key_b.verifying_key().to_bytes()),),
    );

    // Register the account with its own key and both delegates as signers.
    let account = env.register(
        ModularAccount,
        (
            BytesN::from_array(&env, &account_key.verifying_key().to_bytes()),
            vec![&env, delegate_a.clone(), delegate_b.clone()],
        ),
    );
    let protected = env.register(Protected, ());

    let account_addr: ScAddress = account.clone().try_into().unwrap();

    let nonce = 123;
    let signature_expiration_ledger = 100;

    // Create authorized invocation for `protected` call.
    let root_invocation = SorobanAuthorizedInvocation {
        function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
            contract_address: protected.clone().try_into().unwrap(),
            function_name: StringM::try_from("protected").unwrap().into(),
            args: std::vec![ScVal::Address(account_addr.clone())]
                .try_into()
                .unwrap(),
        }),
        sub_invocations: VecM::default(),
    };

    // Build the signature payload by hashing
    // `HashIdPreimage::SorobanAuthorizationWithAddress` preimage required for
    // `AddressWithDelegates` credentials.
    let network_id = env.ledger().network_id();
    let preimage = HashIdPreimage::SorobanAuthorizationWithAddress(
        HashIdPreimageSorobanAuthorizationWithAddress {
            network_id: network_id.to_array().into(),
            nonce,
            signature_expiration_ledger,
            invocation: root_invocation.clone(),
            address: account_addr.clone(),
        },
    );
    let preimage_xdr = preimage.to_xdr(Limits::none()).unwrap();
    let payload: [u8; 32] = Sha256::digest(&preimage_xdr).into();

    // Negative scenario: attach a delegate signer that isn't registered with the
    // account as a delegate. The account should reject the authorization with
    // an `UnknownDelegate` error.
    let unknown_key = SigningKey::from_bytes(&[4u8; 32]);
    let unknown_delegate = env.register(
        DelegateAccount,
        (BytesN::from_array(
            &env,
            &unknown_key.verifying_key().to_bytes(),
        ),),
    );
    let mut bad_delegates = std::vec![
        SorobanDelegateSignature {
            address: delegate_a.clone().try_into().unwrap(),
            signature: sign(&env, &key_a, &payload),
            nested_delegates: VecM::default(),
        },
        SorobanDelegateSignature {
            address: unknown_delegate.clone().try_into().unwrap(),
            signature: sign(&env, &unknown_key, &payload),
            nested_delegates: VecM::default(),
        },
    ];
    bad_delegates.sort_by(|x, y| x.address.cmp(&y.address));
    env.set_auths(&[SorobanAuthorizationEntry {
        credentials: SorobanCredentials::AddressWithDelegates(
            SorobanAddressCredentialsWithDelegates {
                address_credentials: SorobanAddressCredentials {
                    address: account_addr.clone(),
                    nonce,
                    signature_expiration_ledger,
                    signature: sign(&env, &account_key, &payload),
                },
                delegates: bad_delegates.try_into().unwrap(),
            },
        ),
        root_invocation: root_invocation.clone(),
    }]);
    // The call will fail due to the auth failure.
    assert!(ProtectedClient::new(&env, &protected)
        .try_protected(&account)
        .is_err());

    // Each delegate signs the same payload with its own distinct key.
    let mut delegates = std::vec![
        SorobanDelegateSignature {
            address: delegate_a.clone().try_into().unwrap(),
            signature: sign(&env, &key_a, &payload),
            nested_delegates: VecM::default(),
        },
        SorobanDelegateSignature {
            address: delegate_b.clone().try_into().unwrap(),
            signature: sign(&env, &key_b, &payload),
            nested_delegates: VecM::default(),
        },
    ];
    // Delegates must be sorted by address.
    delegates.sort_by(|x, y| x.address.cmp(&y.address));

    // Build the full authorization entry with `AddressWithDelegates`
    // credentials containing both delegates.
    env.set_auths(&[SorobanAuthorizationEntry {
        credentials: SorobanCredentials::AddressWithDelegates(
            SorobanAddressCredentialsWithDelegates {
                address_credentials: SorobanAddressCredentials {
                    address: account_addr.clone(),
                    nonce,
                    signature_expiration_ledger,
                    // Also include the account's own signature in the
                    // credentials as its required by our test contract.
                    signature: sign(&env, &account_key, &payload),
                },
                delegates: delegates.try_into().unwrap(),
            },
        ),
        root_invocation: root_invocation.clone(),
    }]);

    // Call the `protected` function with the enforced authorization payload
    // above.
    // Note, that testing delegated auth via
    // `env.try_invoke_contract_check_auth` is not supported at the moment, so
    // only `set_auths` + a wrapper call can be used to test the full flow.
    ProtectedClient::new(&env, &protected).protected(&account);

    // Both account and its delegates observe a single call to
    // `protected` in their authorization contexts.
    let expected = vec![&env, Symbol::new(&env, "protected")];
    for addr in [&account, &delegate_a, &delegate_b] {
        let calls: Vec<Symbol> = env.as_contract(addr, || {
            env.storage()
                .instance()
                .get(&DataKey::AuthorizedCalls)
                .unwrap()
        });
        assert_eq!(calls, expected);
    }
}
