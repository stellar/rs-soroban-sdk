use soroban_sdk::{contracttrait, symbol_short, Env, Symbol};

use crate::Administratable;

const STORAGE_KEY: &Symbol = &symbol_short!("PAUSED");

#[contracttrait(add_impl_type = true)]
pub trait Pausable: Administratable {
    fn is_paused(env: &Env) -> bool;

    fn pause(env: &Env);

    fn unpause(env: &Env);
}

pub struct PausableBase;

impl Administratable for PausableBase {}

impl Pausable for PausableBase {
    type Impl = Self;
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
