#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
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
impl<'a, 'b> Contract
where
    'a: 'b,
{
    pub fn exec(i1: u32, i2: &u32, i3: &'b u32) -> u32 {
        i1 + i2 + *i3
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__exec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EXEC: [u8; 76usize] = super::Contract::spec_xdr_exec();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x04exec\0\0\0\x03\0\0\0\0\0\0\0\x02i1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02i2\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02i3\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn exec(&self, i1: &u32, i2: &u32, i3: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    i1.into_val(&self.env),
                    i2.into_val(&self.env),
                    i3.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_exec(
        &self,
        i1: &u32,
        i2: &u32,
        i3: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    i1.into_val(&self.env),
                    i2.into_val(&self.env),
                    i3.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec<'i>(i1: &'i u32, i2: &'i u32, i3: &'i u32) -> (&'i u32, &'i u32, &'i u32) {
        (i1, i2, i3)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__exec {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::exec(
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
                &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_2),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
    #[export_name = "exec"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
