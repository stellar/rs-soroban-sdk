use crate::{self as soroban_sdk};
use ml_dsa::{ExpandedSigningKey, MlDsa44, MlDsa65, MlDsa87, MlDsaParams, Seed};
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

const MSG: &[u8] = b"CAP-0087 ML-DSA test message";

#[contract]
pub struct MlDsaContract;

#[contractimpl]
impl MlDsaContract {
    pub fn verify_44(env: Env, pk: BytesN<1312>, msg: Bytes, sig: BytesN<2420>, ctx: Bytes) {
        env.crypto().ml_dsa_44_verify(&pk, &msg, &sig, &ctx);
    }

    pub fn verify_65(env: Env, pk: BytesN<1952>, msg: Bytes, sig: BytesN<3309>, ctx: Bytes) {
        env.crypto().ml_dsa_65_verify(&pk, &msg, &sig, &ctx);
    }

    pub fn verify_87(env: Env, pk: BytesN<2592>, msg: Bytes, sig: BytesN<4627>, ctx: Bytes) {
        env.crypto().ml_dsa_87_verify(&pk, &msg, &sig, &ctx);
    }
}

fn signing_key<P: MlDsaParams>(seed_byte: u8) -> ExpandedSigningKey<P> {
    let seed = [seed_byte; 32];
    ExpandedSigningKey::<P>::from_seed(&Seed::try_from(&seed[..]).unwrap())
}

#[test]
fn test_ml_dsa_44_verify() {
    let env = Env::default();
    let contract_id = env.register(MlDsaContract, ());
    let client = MlDsaContractClient::new(&env, &contract_id);

    let sk = signing_key::<MlDsa44>(1);
    let pk: [u8; 1312] = <[u8; 1312]>::try_from(&sk.verifying_key().encode()[..]).unwrap();
    let pk = BytesN::from_array(&env, &pk);
    let msg = Bytes::from_slice(&env, MSG);

    // Empty context.
    let sig: [u8; 2420] =
        <[u8; 2420]>::try_from(&sk.sign_deterministic(MSG, &[]).unwrap().encode()[..]).unwrap();
    client.verify_44(
        &pk,
        &msg,
        &BytesN::from_array(&env, &sig),
        &Bytes::new(&env),
    );

    // Maximum-length (255-byte) context.
    let ctx = [7u8; 255];
    let sig: [u8; 2420] =
        <[u8; 2420]>::try_from(&sk.sign_deterministic(MSG, &ctx).unwrap().encode()[..]).unwrap();
    client.verify_44(
        &pk,
        &msg,
        &BytesN::from_array(&env, &sig),
        &Bytes::from_slice(&env, &ctx),
    );
}

#[test]
fn test_ml_dsa_65_verify() {
    let env = Env::default();
    let contract_id = env.register(MlDsaContract, ());
    let client = MlDsaContractClient::new(&env, &contract_id);

    let sk = signing_key::<MlDsa65>(1);
    let pk: [u8; 1952] = <[u8; 1952]>::try_from(&sk.verifying_key().encode()[..]).unwrap();
    let pk = BytesN::from_array(&env, &pk);
    let msg = Bytes::from_slice(&env, MSG);

    let sig: [u8; 3309] =
        <[u8; 3309]>::try_from(&sk.sign_deterministic(MSG, &[]).unwrap().encode()[..]).unwrap();
    client.verify_65(
        &pk,
        &msg,
        &BytesN::from_array(&env, &sig),
        &Bytes::new(&env),
    );

    let ctx = [7u8; 255];
    let sig: [u8; 3309] =
        <[u8; 3309]>::try_from(&sk.sign_deterministic(MSG, &ctx).unwrap().encode()[..]).unwrap();
    client.verify_65(
        &pk,
        &msg,
        &BytesN::from_array(&env, &sig),
        &Bytes::from_slice(&env, &ctx),
    );
}

#[test]
fn test_ml_dsa_87_verify() {
    let env = Env::default();
    let contract_id = env.register(MlDsaContract, ());
    let client = MlDsaContractClient::new(&env, &contract_id);

    let sk = signing_key::<MlDsa87>(1);
    let pk: [u8; 2592] = <[u8; 2592]>::try_from(&sk.verifying_key().encode()[..]).unwrap();
    let pk = BytesN::from_array(&env, &pk);
    let msg = Bytes::from_slice(&env, MSG);

    let sig: [u8; 4627] =
        <[u8; 4627]>::try_from(&sk.sign_deterministic(MSG, &[]).unwrap().encode()[..]).unwrap();
    client.verify_87(
        &pk,
        &msg,
        &BytesN::from_array(&env, &sig),
        &Bytes::new(&env),
    );

    let ctx = [7u8; 255];
    let sig: [u8; 4627] =
        <[u8; 4627]>::try_from(&sk.sign_deterministic(MSG, &ctx).unwrap().encode()[..]).unwrap();
    client.verify_87(
        &pk,
        &msg,
        &BytesN::from_array(&env, &sig),
        &Bytes::from_slice(&env, &ctx),
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Crypto, InvalidInput)")]
fn test_ml_dsa_44_verify_invalid_sig() {
    let env = Env::default();

    let sk = signing_key::<MlDsa44>(1);
    let pk: [u8; 1312] = <[u8; 1312]>::try_from(&sk.verifying_key().encode()[..]).unwrap();
    let sig: [u8; 2420] =
        <[u8; 2420]>::try_from(&sk.sign_deterministic(MSG, &[]).unwrap().encode()[..]).unwrap();

    // Message modified after signing.
    env.crypto().ml_dsa_44_verify(
        &BytesN::from_array(&env, &pk),
        &Bytes::from_slice(&env, b"CAP-0087 ML-DSA test messagf"),
        &BytesN::from_array(&env, &sig),
        &Bytes::new(&env),
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Crypto, InvalidInput)")]
fn test_ml_dsa_44_verify_context_mismatch() {
    let env = Env::default();

    let sk = signing_key::<MlDsa44>(1);
    let pk: [u8; 1312] = <[u8; 1312]>::try_from(&sk.verifying_key().encode()[..]).unwrap();
    let sig: [u8; 2420] =
        <[u8; 2420]>::try_from(&sk.sign_deterministic(MSG, b"ctx-a").unwrap().encode()[..])
            .unwrap();

    env.crypto().ml_dsa_44_verify(
        &BytesN::from_array(&env, &pk),
        &Bytes::from_slice(&env, MSG),
        &BytesN::from_array(&env, &sig),
        &Bytes::from_slice(&env, b"ctx-b"),
    );
}
