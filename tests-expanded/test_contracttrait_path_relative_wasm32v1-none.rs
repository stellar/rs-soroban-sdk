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
    pub struct RelativePathTraitSpec;
    /// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
    pub use __contractimpl_for_relative_path_trait as RelativePathTrait;
    pub trait RelativePathTrait {
        fn relative_path_method(env: &Env) -> u32 {
            let _ = env;
            400
        }
    }
    ///RelativePathTraitClient is a client for calling the contract defined in "RelativePathTrait".
    pub struct RelativePathTraitClient<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> RelativePathTraitClient<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> RelativePathTraitClient<'a> {
        pub fn relative_path_method(&self) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_relative_path_method(
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
                &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
    }
    ///RelativePathTraitArgs is a type for building arg lists for functions defined in "RelativePathTrait".
    pub struct RelativePathTraitArgs;
    impl RelativePathTraitArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn relative_path_method<'i>() -> () {
            ()
        }
    }
    impl RelativePathTraitSpec {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_relative_path_method() -> [u8; 44usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x14relative_path_method\0\0\0\0\0\0\0\x01\0\0\0\x04"
        }
    }
}
pub struct ContractRelativePath;
///ContractRelativePathArgs is a type for building arg lists for functions defined in "ContractRelativePath".
pub struct ContractRelativePathArgs;
///ContractRelativePathClient is a client for calling the contract defined in "ContractRelativePath".
pub struct ContractRelativePathClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractRelativePathClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl traits::RelativePathTrait for ContractRelativePath {}
impl<'a> ContractRelativePathClient<'a> {}
impl ContractRelativePathArgs {}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractRelativePathClient::new(&env, &contract_id).relative_path_method` instead"
)]
pub fn __ContractRelativePath__relative_path_method__invoke_raw(
    env: soroban_sdk::Env,
) -> soroban_sdk::Val {
    use traits::RelativePathTrait;
    <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
        #[allow(deprecated)]
        &<ContractRelativePath>::relative_path_method(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractRelativePathClient::new(&env, &contract_id).relative_path_method` instead"
)]
#[export_name = "relative_path_method"]
pub extern "C" fn __ContractRelativePath__relative_path_method__invoke_raw_extern(
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractRelativePath__relative_path_method__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractRelativePath__relative_path_method__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_RELATIVE_PATH_METHOD: [u8; 44usize] =
        super::ContractRelativePath::spec_xdr_relative_path_method();
}
impl ContractRelativePath {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_relative_path_method() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x14relative_path_method\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractRelativePathClient<'a> {
    pub fn relative_path_method(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_relative_path_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractRelativePathArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn relative_path_method<'i>() -> () {
        ()
    }
}
