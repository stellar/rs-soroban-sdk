use soroban_sdk::{contracttrait, symbol_short, Env, Symbol};

use crate::Administratable;

const STORAGE_KEY: &Symbol = &symbol_short!("PAUSED");

#[contracttrait]
pub trait Pausable: Administratable {
    fn is_paused(env: &Env) -> bool {
        env.storage().persistent().get(STORAGE_KEY).unwrap_or(false)
    }

    fn pause(env: &Env) {
        Self::admin(env).require_auth();
        env.storage().persistent().set(STORAGE_KEY, &true);
    }

    fn unpause(env: &Env) {
        Self::admin(env).require_auth();
        env.storage().persistent().set(STORAGE_KEY, &false)
    }
}
