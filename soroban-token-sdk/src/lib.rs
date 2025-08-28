#![no_std]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
pub mod _migrating;

use crate::event::Events;
use crate::metadata::Metadata;
use soroban_sdk::Env;

pub mod event;
pub mod events;
pub mod metadata;
mod tests;

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
