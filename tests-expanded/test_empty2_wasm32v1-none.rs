#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
pub struct Contract {
    env: soroban_sdk::Env,
}
impl Contract {
    pub fn env(&self) -> &soroban_sdk::Env {
        &self.env
    }
    #[doc(hidden)]
    pub fn __soroban_new(env: soroban_sdk::Env) -> Self {
        Self { env }
    }
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct __ContractContractValue;
///Value of the [`Contract`] contract, for registering the contract.
#[allow(non_upper_case_globals)]
pub const Contract: __ContractContractValue = __ContractContractValue;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl Contract {}
impl<'a> ContractClient<'a> {}
impl ContractArgs {}
