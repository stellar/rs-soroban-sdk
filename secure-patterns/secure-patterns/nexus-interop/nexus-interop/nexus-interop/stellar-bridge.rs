use soroban_sdk::{Env, Address, Bytes};

pub fn emit_bridge_event(e: &Env, from: Address, to_chain: Bytes, payload: Bytes) {
    e.events().publish(
        (from, to_chain),
        payload,
    );
}
