use crate::{self as soroban_sdk};
use ml_dsa::{MlDsa44, MlDsa65, MlDsa87, MlDsaParams, SigningKey};
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

const MSG: &[u8] = b"soroban-sdk ml-dsa test message";
const CTX: &[u8] = b"soroban-sdk ml-dsa test context";

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn verify_44(
        env: Env,
        public_key: BytesN<1312>,
        message: Bytes,
        signature: BytesN<2420>,
        context: Bytes,
    ) {
        env.crypto()
            .ml_dsa_44_verify(&public_key, &message, &signature, &context);
    }

    pub fn verify_65(
        env: Env,
        public_key: BytesN<1952>,
        message: Bytes,
        signature: BytesN<3309>,
        context: Bytes,
    ) {
        env.crypto()
            .ml_dsa_65_verify(&public_key, &message, &signature, &context);
    }

    pub fn verify_87(
        env: Env,
        public_key: BytesN<2592>,
        message: Bytes,
        signature: BytesN<4627>,
        context: Bytes,
    ) {
        env.crypto()
            .ml_dsa_87_verify(&public_key, &message, &signature, &context);
    }
}

/// Signs `msg` under `ctx` with a key derived from `seed`, returning the
/// encoded verifying key and signature. Key generation and signing are both
/// deterministic, so the fixtures, and the test snapshots that record them,
/// are stable across runs.
fn sign<P: MlDsaParams>(env: &Env, seed: u8, msg: &[u8], ctx: &[u8]) -> (Bytes, Bytes) {
    let sk = SigningKey::<P>::from_seed(&[seed; 32].into());
    let sk = sk.expanded_key();
    let sig = sk.sign_deterministic(msg, ctx).unwrap();
    (
        Bytes::from_slice(env, &sk.verifying_key().encode()),
        Bytes::from_slice(env, &sig.encode()),
    )
}

/// Generates the same set of tests for each parameter set, since the three
/// verify functions differ only in the parameter set and the sizes of the key
/// and signature.
macro_rules! ml_dsa_tests {
    ($mod:ident, $params:ty, $verify:ident, $contract_fn:ident, $pk_len:literal, $sig_len:literal) => {
        mod $mod {
            use super::*;

            fn sign(
                env: &Env,
                seed: u8,
                msg: &[u8],
                ctx: &[u8],
            ) -> (BytesN<$pk_len>, BytesN<$sig_len>) {
                let (pk, sig) = super::sign::<$params>(env, seed, msg, ctx);
                (pk.try_into().unwrap(), sig.try_into().unwrap())
            }

            #[test]
            fn test_verify() {
                let env = Env::default();
                let contract_id = env.register(Contract, ());
                let client = ContractClient::new(&env, &contract_id);

                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::new(&env);
                let (pk, sig) = sign(&env, 1, MSG, &[]);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
                client.$contract_fn(&pk, &msg, &sig, &ctx);
            }

            #[test]
            fn test_verify_with_context() {
                let env = Env::default();
                let contract_id = env.register(Contract, ());
                let client = ContractClient::new(&env, &contract_id);

                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::from_slice(&env, CTX);
                let (pk, sig) = sign(&env, 1, MSG, CTX);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
                client.$contract_fn(&pk, &msg, &sig, &ctx);
            }

            #[test]
            fn test_verify_with_max_len_context() {
                let env = Env::default();
                let contract_id = env.register(Contract, ());
                let client = ContractClient::new(&env, &contract_id);

                let max_ctx = [0xab; 255];
                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::from_slice(&env, &max_ctx);
                let (pk, sig) = sign(&env, 1, MSG, &max_ctx);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
                client.$contract_fn(&pk, &msg, &sig, &ctx);
            }

            #[test]
            fn test_verify_empty_message() {
                let env = Env::default();

                let msg = Bytes::new(&env);
                let ctx = Bytes::new(&env);
                let (pk, sig) = sign(&env, 1, &[], &[]);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
            }

            #[test]
            #[should_panic(expected = "Error(Crypto, InvalidInput)")]
            fn test_verify_wrong_message() {
                let env = Env::default();

                let mut wrong_msg = MSG.to_vec();
                wrong_msg[0] ^= 1;
                let msg = Bytes::from_slice(&env, &wrong_msg);
                let ctx = Bytes::new(&env);
                let (pk, sig) = sign(&env, 1, MSG, &[]);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
            }

            #[test]
            #[should_panic(expected = "Error(Crypto, InvalidInput)")]
            fn test_verify_wrong_context() {
                let env = Env::default();

                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::new(&env);
                let (pk, sig) = sign(&env, 1, MSG, CTX);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
            }

            #[test]
            #[should_panic(expected = "Error(Crypto, InvalidInput)")]
            fn test_verify_wrong_public_key() {
                let env = Env::default();

                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::new(&env);
                let (_, sig) = sign(&env, 1, MSG, &[]);
                let (pk, _) = sign(&env, 2, MSG, &[]);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
            }

            #[test]
            #[should_panic(expected = "Error(Crypto, InvalidInput)")]
            fn test_verify_corrupted_signature() {
                let env = Env::default();

                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::new(&env);
                let (pk, sig) = sign(&env, 1, MSG, &[]);
                let mut corrupted_sig = sig.to_array();
                corrupted_sig[0] ^= 1;
                let sig = BytesN::from_array(&env, &corrupted_sig);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
            }

            #[test]
            #[should_panic(expected = "Error(Crypto, InvalidInput)")]
            fn test_verify_context_too_long() {
                let env = Env::default();

                let msg = Bytes::from_slice(&env, MSG);
                let ctx = Bytes::from_slice(&env, &[0; 256]);
                let (pk, sig) = sign(&env, 1, MSG, &[]);

                env.crypto().$verify(&pk, &msg, &sig, &ctx);
            }
        }
    };
}

ml_dsa_tests!(ml_dsa_44, MlDsa44, ml_dsa_44_verify, verify_44, 1312, 2420);
ml_dsa_tests!(ml_dsa_65, MlDsa65, ml_dsa_65_verify, verify_65, 1952, 3309);
ml_dsa_tests!(ml_dsa_87, MlDsa87, ml_dsa_87_verify, verify_87, 2592, 4627);
