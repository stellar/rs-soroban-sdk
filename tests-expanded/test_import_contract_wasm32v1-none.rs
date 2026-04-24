#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Address, Env};
mod addcontract {}
pub struct Contract;
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
impl Contract {
    pub fn add_with(env: Env, contract_id: Address, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add_with__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_ADD_WITH: [u8; 88usize] = super::Contract::spec_xdr_add_with();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add_with() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08add_with\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractClient<'a> {
    pub fn add_with(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add_with");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_add_with(
        &self,
        contract_id: &Address,
        x: &u64,
        y: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add_with");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn add_with<'i>(
        contract_id: &'i Address,
        x: &'i u64,
        y: &'i u64,
    ) -> (&'i Address, &'i u64, &'i u64) {
        (contract_id, x, y)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
#[allow(deprecated)]
pub fn __Contract__add_with__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::add_with(
            env.clone(),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
#[export_name = "add_with"]
pub extern "C" fn __Contract__add_with__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__add_with__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
