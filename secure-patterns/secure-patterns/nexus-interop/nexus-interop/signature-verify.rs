use soroban_sdk::{Env, BytesN, Bytes};

pub fn verify_signature(e: &Env, msg: &Bytes, sig: &BytesN<64>, pubkey: &BytesN<32>) -> bool {
    e.crypto().ed25519_verify(pubkey, msg, sig)
}
