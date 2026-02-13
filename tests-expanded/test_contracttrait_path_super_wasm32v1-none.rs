#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contracttrait, Env};
pub struct SuperPathTraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_super_path_trait as SuperPathTrait;
pub trait SuperPathTrait {
    fn super_path_method(env: &Env) -> u32 {
        let _ = env;
        300
    }
}
///SuperPathTraitClient is a client for calling the contract defined in "SuperPathTrait".
pub struct SuperPathTraitClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> SuperPathTraitClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> SuperPathTraitClient<'a> {
    pub fn super_path_method(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_super_path_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
///SuperPathTraitArgs is a type for building arg lists for functions defined in "SuperPathTrait".
pub struct SuperPathTraitArgs;
impl SuperPathTraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn super_path_method<'i>() -> () {
        ()
    }
}
impl SuperPathTraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_super_path_method() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11super_path_method\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
pub mod submodule {
    use soroban_sdk::{contract, contractimpl};
    pub struct ContractSuperPath;
    ///ContractSuperPathArgs is a type for building arg lists for functions defined in "ContractSuperPath".
    pub struct ContractSuperPathArgs;
    ///ContractSuperPathClient is a client for calling the contract defined in "ContractSuperPath".
    pub struct ContractSuperPathClient<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> ContractSuperPathClient<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl super::SuperPathTrait for ContractSuperPath {}
    impl<'a> ContractSuperPathClient<'a> {}
    impl ContractSuperPathArgs {}
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(
        note = "use `ContractSuperPathClient::new(&env, &contract_id).super_path_method` instead"
    )]
    pub fn __ContractSuperPath__super_path_method__invoke_raw(
        env: soroban_sdk::Env,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<ContractSuperPath as super::SuperPathTrait>::super_path_method(&env),
            &env,
        )
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(
        note = "use `ContractSuperPathClient::new(&env, &contract_id).super_path_method` instead"
    )]
    #[export_name = "super_path_method"]
    pub extern "C" fn __ContractSuperPath__super_path_method__invoke_raw_extern() -> soroban_sdk::Val
    {
        #[allow(deprecated)]
        __ContractSuperPath__super_path_method__invoke_raw(soroban_sdk::Env::default())
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __ContractSuperPath__super_path_method__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #[link_section = "contractspecv0"]
        pub static __SPEC_XDR_FN_SUPER_PATH_METHOD: [u8; 44usize] =
            super::ContractSuperPath::spec_xdr_super_path_method();
    }
    impl ContractSuperPath {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_super_path_method() -> [u8; 44usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x11super_path_method\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
        }
    }
    impl<'a> ContractSuperPathClient<'a> {
        pub fn super_path_method(&self) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_super_path_method(
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
                &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
    }
    impl ContractSuperPathArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn super_path_method<'i>() -> () {
            ()
        }
    }
}
