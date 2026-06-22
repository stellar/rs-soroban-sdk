use soroban_sdk::{Env, Symbol};

pub fn risk_score(storage_writes: u32, external_calls: u32) -> u32 {
    let mut score = 0;
    score += storage_writes * 2;
    score += external_calls * 5;
    score
}

pub fn emit_risk(e: &Env, tag: Symbol, score: u32) {
    e.events().publish(tag, score);
}
