#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use sdk::{contract, contractimpl, contracttrait, Env};
pub struct CratePathTraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_crate_path_trait as CratePathTrait;
pub trait CratePathTrait {
    fn default_method(env: &Env) -> u32 {
        let _ = env;
        100
    }
    fn overridden_method(env: &Env) -> u32 {
        let _ = env;
        0
    }
}
///CratePathTraitClient is a client for calling the contract defined in "CratePathTrait".
pub struct CratePathTraitClient<'a> {
    pub env: sdk::Env,
    pub address: sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> CratePathTraitClient<'a> {
    pub fn new(env: &sdk::Env, address: &sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> CratePathTraitClient<'a> {
    pub fn default_method(&self) -> u32 {
        use core::ops::Not;
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_default_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn overridden_method(&self) -> u32 {
        use core::ops::Not;
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_overridden_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
///CratePathTraitArgs is a type for building arg lists for functions defined in "CratePathTrait".
pub struct CratePathTraitArgs;
impl CratePathTraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn default_method<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn overridden_method<'i>() -> () {
        ()
    }
}
impl CratePathTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_default_method() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0edefault_method\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl CratePathTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_overridden_method() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11overridden_method\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: sdk::Env,
    pub address: sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &sdk::Env, address: &sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl CratePathTrait for Contract {
    fn overridden_method(env: &Env) -> u32 {
        let _ = env;
        200
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__overridden_method__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_OVERRIDDEN_METHOD: [u8; 44usize] =
        super::Contract::spec_xdr_overridden_method();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_overridden_method() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11overridden_method\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn overridden_method(&self) -> u32 {
        use core::ops::Not;
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_overridden_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn overridden_method<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).overridden_method` instead")]
#[allow(deprecated)]
pub fn __Contract__overridden_method__invoke_raw(env: sdk::Env) -> sdk::Val {
    sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as CratePathTrait>::overridden_method(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).overridden_method` instead")]
#[export_name = "overridden_method"]
pub extern "C" fn __Contract__overridden_method__invoke_raw_extern() -> sdk::Val {
    #[allow(deprecated)]
    __Contract__overridden_method__invoke_raw(sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).default_method` instead")]
#[allow(deprecated)]
pub fn __Contract__default_method__invoke_raw(env: sdk::Env) -> sdk::Val {
    sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as CratePathTrait>::default_method(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).default_method` instead")]
#[export_name = "default_method"]
pub extern "C" fn __Contract__default_method__invoke_raw_extern() -> sdk::Val {
    #[allow(deprecated)]
    __Contract__default_method__invoke_raw(sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__default_method__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_DEFAULT_METHOD: [u8; 40usize] = super::Contract::spec_xdr_default_method();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_default_method() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0edefault_method\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn default_method(&self) -> u32 {
        use core::ops::Not;
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_default_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn default_method<'i>() -> () {
        ()
    }
}
