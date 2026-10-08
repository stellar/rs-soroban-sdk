#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::contract;
struct _Contract {
    env: soroban_sdk::Env,
}
impl _Contract {
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
struct ___ContractContractValue;
///Value of the [`_Contract`] contract, for registering the contract.
#[allow(non_upper_case_globals)]
const _Contract: ___ContractContractValue = ___ContractContractValue;
///_ContractArgs is a type for building arg lists for functions defined in "_Contract".
pub struct _ContractArgs;
///_ContractClient is a client for calling the contract defined in "_Contract".
pub struct _ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> _ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
