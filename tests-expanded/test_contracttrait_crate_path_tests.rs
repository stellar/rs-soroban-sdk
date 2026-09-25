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
    set_auths: Option<&'a [sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> CratePathTraitClient<'a> {
    pub fn new(env: &sdk::Env, address: &sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
impl<'a> CratePathTraitClient<'a> {
    pub fn default_method(&self) -> u32 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_default_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn overridden_method(&self) -> u32 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_overridden_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
    set_auths: Option<&'a [sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &sdk::Env, address: &sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
mod __contract_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(name: &str, env: sdk::Env, args: &[sdk::Val]) -> Option<sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl sdk::testutils::ContractFunctionSet for Contract {
    fn call(&self, func: &str, env: sdk::Env, args: &[sdk::Val]) -> Option<sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
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
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_overridden_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "overridden_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
pub fn __Contract__overridden_method__invoke_raw_slice(
    env: sdk::Env,
    args: &[sdk::Val],
) -> sdk::Val {
    if args.len() != 0usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                0usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__overridden_method__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).overridden_method` instead")]
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
pub fn __Contract__default_method__invoke_raw_slice(env: sdk::Env, args: &[sdk::Val]) -> sdk::Val {
    if args.len() != 0usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                0usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__default_method__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).default_method` instead")]
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
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_default_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as sdk::TryFromVal<sdk::Env, sdk::Val>>::Error>,
        Result<sdk::Error, sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ sdk::Symbol::new(&self.env, "default_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__CratePathTrait__9a3c48b70b85bd1de039b6d3d222a30f0062bb1b2d01fc634c0b9ee632adf879_ctor(
) {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::ctor::__support::CtorRetType {
                unsafe {
                    __Contract__CratePathTrait__9a3c48b70b85bd1de039b6d3d222a30f0062bb1b2d01fc634c0b9ee632adf879_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as sdk::testutils::ContractFunctionRegister>::register(
            "default_method",
            #[allow(deprecated)]
            &__Contract__default_method__invoke_raw_slice,
        );
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__CratePathTrait__ade0c2da31aff18d6ecaa9598b6cd6f921f3f9db13ccca6f9ea0e4b11c84b923_ctor(
) {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::ctor::__support::CtorRetType {
                unsafe {
                    __Contract__CratePathTrait__ade0c2da31aff18d6ecaa9598b6cd6f921f3f9db13ccca6f9ea0e4b11c84b923_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as sdk::testutils::ContractFunctionRegister>::register(
            "overridden_method",
            #[allow(deprecated)]
            &__Contract__overridden_method__invoke_raw_slice,
        );
    }
}
mod test {
    use super::*;
    use sdk::Env;
    extern crate test;
    #[rustc_test_marker = "test::test_crate_path"]
    #[doc(hidden)]
    pub const test_crate_path: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_crate_path"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/contracttrait_crate_path/src/lib.rs",
            start_line: 41usize,
            start_col: 8usize,
            end_line: 41usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_crate_path()),
        ),
    };
    fn test_crate_path() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        match (&client.default_method(), &100) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&client.overridden_method(), &200) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_crate_path])
}
