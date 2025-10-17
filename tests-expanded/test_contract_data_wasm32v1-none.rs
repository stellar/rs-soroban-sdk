#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Env, Symbol};
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
impl<'i> Contract {
    pub fn put(e: Env, key: &Symbol, val: &Symbol) {
        e.storage().persistent().set(key, val)
    }
    pub fn get(e: Env, key: &'i mut Symbol) -> Option<Symbol> {
        e.storage().persistent().get(key)
    }
    pub fn del(e: Env, key: &Symbol) {
        e.storage().persistent().remove(key)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__put__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_PUT: [u8; 56usize] = super::Contract::spec_xdr_put();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_put() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03put\0\0\0\0\x02\0\0\0\0\0\0\0\x03key\0\0\0\0\x11\0\0\0\0\0\0\0\x03val\0\0\0\0\x11\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET: [u8; 48usize] = super::Contract::spec_xdr_get();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03get\0\0\0\0\x01\0\0\0\0\0\0\0\x03key\0\0\0\0\x11\0\0\0\x01\0\0\x03\xe8\0\0\0\x11"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__del__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DEL: [u8; 40usize] = super::Contract::spec_xdr_del();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_del() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03del\0\0\0\0\x01\0\0\0\0\0\0\0\x03key\0\0\0\0\x11\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn put(&self, key: &Symbol, val: &Symbol) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("put");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [key.into_val(&self.env), val.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_put(
        &self,
        key: &Symbol,
        val: &Symbol,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("put");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [key.into_val(&self.env), val.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn get(&self, key: &Symbol) -> Option<Symbol> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn try_get(
        &self,
        key: &Symbol,
    ) -> Result<
        Result<
            Option<Symbol>,
            <Option<Symbol> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn del(&self, key: &Symbol) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("del");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn try_del(
        &self,
        key: &Symbol,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("del");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn put<'i>(key: &'i Symbol, val: &'i Symbol) -> (&'i Symbol, &'i Symbol) {
        (key, val)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get<'i>(key: &'i Symbol) -> (&'i Symbol,) {
        (key,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn del<'i>(key: &'i Symbol) -> (&'i Symbol,) {
        (key,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__put {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).put` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::put(
                env.clone(),
                &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
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
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).put` instead")]
    #[export_name = "put"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::get(
                env.clone(),
                &mut <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get` instead")]
    #[export_name = "get"]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__del {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).del` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::del(
                env.clone(),
                &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).del` instead")]
    #[export_name = "del"]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
