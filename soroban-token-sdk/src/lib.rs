#![no_std]

use crate::metadata::Metadata;
use soroban_sdk::Env;

pub mod metadata;

#[derive(Clone)]
pub struct TokenUtils(Env);

impl TokenUtils {
    #[inline(always)]
    pub fn new(env: &Env) -> TokenUtils {
        TokenUtils(env.clone())
    }

    pub fn metadata(&self) -> Metadata {
        Metadata::new(&self.0)
    }
}
