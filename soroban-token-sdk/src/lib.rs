#![no_std]

use soroban_sdk::{contracttype, symbol_short, unwrap::UnwrapOptimized, Env, String, Symbol};

const METADATA_KEY: Symbol = symbol_short!("METADATA");

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

#[derive(Clone)]
pub struct TokenUtils(Env);

impl TokenUtils {
    #[inline(always)]
    pub fn new(env: &Env) -> TokenUtils {
        TokenUtils(env.clone())
    }

    #[inline(always)]
    pub fn set_metadata(&self, metadata: &TokenMetadata) {
        self.0.storage().persistent().set(&METADATA_KEY, metadata);
    }

    #[inline(always)]
    pub fn get_metadata(&self) -> TokenMetadata {
        self.0
            .storage()
            .persistent()
            .get(&METADATA_KEY)
            .unwrap_optimized()
    }
}
