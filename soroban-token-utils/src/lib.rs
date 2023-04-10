#![no_std]

use soroban_sdk::{contracttype, Bytes, ConversionError, Env, Symbol};

const METADATA_KEY: Symbol = Symbol::short("METADATA");

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: Bytes,
    pub symbol: Bytes,
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
        self.0.storage().set(&METADATA_KEY, metadata);
    }

    #[inline(always)]
    pub fn get_metadata_unchecked(&self) -> Result<TokenMetadata, ConversionError> {
        self.0.storage().get_unchecked(&METADATA_KEY)
    }
}
