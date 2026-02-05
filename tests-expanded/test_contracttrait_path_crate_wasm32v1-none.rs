#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl};
pub mod traits {
    use soroban_sdk::{contracttrait, Env};
    pub struct CratePathTraitSpec;
    /// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
    pub use __contractimpl_for_crate_path_trait as CratePathTrait;
    pub trait CratePathTrait {
        fn crate_path_method(env: &Env) -> u32 {
            let _ = env;
            100
        }
    }
    ///CratePathTraitClient is a client for calling the contract defined in "CratePathTrait".
    pub struct CratePathTraitClient<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> CratePathTraitClient<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> CratePathTraitClient<'a> {
        pub fn crate_path_method(&self) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "crate_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_crate_path_method(
            &self,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "crate_path_method") },
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
        pub fn crate_path_method<'i>() -> () {
            ()
        }
    }
    impl CratePathTraitSpec {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_crate_path_method() -> [u8; 44usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x11crate_path_method\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
        }
    }
}
pub struct ContractCratePath;
///ContractCratePathArgs is a type for building arg lists for functions defined in "ContractCratePath".
pub struct ContractCratePathArgs;
///ContractCratePathClient is a client for calling the contract defined in "ContractCratePath".
pub struct ContractCratePathClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractCratePathClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl crate::traits::CratePathTrait for ContractCratePath {}
impl<'a> ContractCratePathClient<'a> {}
impl ContractCratePathArgs {}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractCratePathClient::new(&env, &contract_id).crate_path_method` instead"
)]
pub fn __ContractCratePath__crate_path_method__invoke_raw(
    env: soroban_sdk::Env,
) -> soroban_sdk::Val {
    use crate::traits::CratePathTrait;
    <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
        #[allow(deprecated)]
        &<ContractCratePath>::crate_path_method(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractCratePathClient::new(&env, &contract_id).crate_path_method` instead"
)]
#[export_name = "crate_path_method"]
pub extern "C" fn __ContractCratePath__crate_path_method__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractCratePath__crate_path_method__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractCratePath__crate_path_method__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_CRATE_PATH_METHOD: [u8; 44usize] =
        super::ContractCratePath::spec_xdr_crate_path_method();
}
impl ContractCratePath {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_crate_path_method() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11crate_path_method\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractCratePathClient<'a> {
    pub fn crate_path_method(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "crate_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_crate_path_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "crate_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractCratePathArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn crate_path_method<'i>() -> () {
        ()
    }
}
