#![no_std]

use crate::event::Events;
use crate::metadata::Metadata;
use soroban_sdk::Env;

pub mod event;
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

    pub fn events(&self) -> Events {
        Events::new(&self.0)
    }
}
