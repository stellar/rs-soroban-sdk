#![no_std]
use soroban_sdk::RawVal;
use soroban_sdk::{contracttype, BytesN, Symbol, Vec};

#[cfg(any(feature = "testutils", test))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
pub mod testutils;

#[contracttype]
#[derive(Clone)]
pub struct AuthorizationContext {
    pub contract: BytesN<32>,
    pub fn_name: Symbol,
    pub args: Vec<RawVal>,
}
