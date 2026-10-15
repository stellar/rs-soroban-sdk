#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

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

#[cfg(test)]
mod test {
    use ml_dsa::{MlDsa44, MlDsa65, MlDsa87, MlDsaParams, SigningKey};
    use soroban_sdk::{Bytes, BytesN, Env};

    use crate::{Contract, ContractClient};

    mod wasm {
        soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/test_ml_dsa.wasm");
    }

    const MSG: &[u8] = b"test_ml_dsa message";
    const CTX: &[u8] = b"test_ml_dsa context";

    /// Signs `msg` under `ctx` with a key derived from `seed`, returning the
    /// encoded verifying key and signature. Key generation and signing are
    /// both deterministic, so the fixtures, and the test snapshots that record
    /// them, are stable across runs.
    fn sign<P: MlDsaParams>(env: &Env, seed: u8, msg: &[u8], ctx: &[u8]) -> (Bytes, Bytes) {
        let sk = SigningKey::<P>::from_seed(&[seed; 32].into());
        let sk = sk.expanded_key();
        let sig = sk.sign_deterministic(msg, ctx).unwrap();
        (
            Bytes::from_slice(env, &sk.verifying_key().encode()),
            Bytes::from_slice(env, &sig.encode()),
        )
    }

    /// Generates the same set of tests for each parameter set, run both
    /// natively and as wasm, where the host function imports must link.
    macro_rules! ml_dsa_tests {
        ($mod:ident, $params:ty, $contract_fn:ident, $pk_len:literal, $sig_len:literal) => {
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
                fn test_native() {
                    let env = Env::default();
                    let contract_id = env.register(Contract, ());
                    let client = ContractClient::new(&env, &contract_id);

                    let msg = Bytes::from_slice(&env, MSG);
                    let ctx = Bytes::from_slice(&env, CTX);
                    let (pk, sig) = sign(&env, 1, MSG, CTX);

                    client.$contract_fn(&pk, &msg, &sig, &ctx);
                }

                #[test]
                fn test_wasm() {
                    let env = Env::default();
                    let contract_id = env.register(wasm::WASM, ());
                    let client = wasm::Client::new(&env, &contract_id);

                    let msg = Bytes::from_slice(&env, MSG);
                    let ctx = Bytes::from_slice(&env, CTX);
                    let (pk, sig) = sign(&env, 1, MSG, CTX);

                    client.$contract_fn(&pk, &msg, &sig, &ctx);
                }

                #[test]
                #[should_panic(expected = "Error(Crypto, InvalidInput)")]
                fn test_wasm_invalid_signature() {
                    let env = Env::default();
                    let contract_id = env.register(wasm::WASM, ());
                    let client = wasm::Client::new(&env, &contract_id);

                    let msg = Bytes::from_slice(&env, MSG);
                    let ctx = Bytes::new(&env);
                    // Signed under a context the verifier is not given.
                    let (pk, sig) = sign(&env, 1, MSG, CTX);

                    client.$contract_fn(&pk, &msg, &sig, &ctx);
                }
            }
        };
    }

    ml_dsa_tests!(ml_dsa_44, MlDsa44, verify_44, 1312, 2420);
    ml_dsa_tests!(ml_dsa_65, MlDsa65, verify_65, 1952, 3309);
    ml_dsa_tests!(ml_dsa_87, MlDsa87, verify_87, 2592, 4627);
}
