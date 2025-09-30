#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal};
pub struct ContractA;
///ContractAArgs is a type for building arg lists for functions defined in "ContractA".
pub struct ContractAArgs;
///ContractAClient is a client for calling the contract defined in "ContractA".
pub struct ContractAClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractAClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl ContractA {
    pub fn fn1(a: Address) -> u64 {
        a.require_auth();
        2
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractA__fn1__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FN1: [u8; 44usize] = super::ContractA::spec_xdr_fn1();
}
impl ContractA {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn1() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03fn1\0\0\0\0\x01\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x13\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractAClient<'a> {
    pub fn fn1(&self, a: &Address) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        res
    }
    pub fn try_fn1(
        &self,
        a: &Address,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        res
    }
}
impl ContractAArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn1<'i>(a: &'i Address) -> (&'i Address,) {
        (a,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractA__fn1 {
    use super::*;
    #[deprecated(note = "use `ContractAClient::new(&env, &contract_id).fn1` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::ContractA>::fn1(
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractAClient::new(&env, &contract_id).fn1` instead")]
    #[export_name = "fn1"]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
pub struct ContractB;
///ContractBArgs is a type for building arg lists for functions defined in "ContractB".
pub struct ContractBArgs;
///ContractBClient is a client for calling the contract defined in "ContractB".
pub struct ContractBClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractBClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl ContractB {
    pub fn fn2(e: Env, a: Address, sub: Address) -> u64 {
        a.require_auth_for_args((1, 2).into_val(&e));
        let client = ContractAClient::new(&e, &sub);
        client.fn1(&a)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractB__fn2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FN2: [u8; 60usize] = super::ContractB::spec_xdr_fn2();
}
impl ContractB {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn2() -> [u8; 60usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03fn2\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x03sub\0\0\0\0\x13\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractBClient<'a> {
    pub fn fn2(&self, a: &Address, sub: &Address) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), sub.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_fn2(
        &self,
        a: &Address,
        sub: &Address,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), sub.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractBArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn2<'i>(a: &'i Address, sub: &'i Address) -> (&'i Address, &'i Address) {
        (a, sub)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractB__fn2 {
    use super::*;
    #[deprecated(note = "use `ContractBClient::new(&env, &contract_id).fn2` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::ContractB>::fn2(
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
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractBClient::new(&env, &contract_id).fn2` instead")]
    #[export_name = "fn2"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
