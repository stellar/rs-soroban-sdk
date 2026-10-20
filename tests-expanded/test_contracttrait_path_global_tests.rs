#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{
    contract, contractimpl, Address, Bytes, BytesN, Duration, Map, String, Symbol, Timepoint, Vec,
    I256, U256,
};
use test_contracttrait_trait::{MyEnumUnit, MyEnumVariants, MyStruct};
pub struct ContractGlobalPath;
///ContractGlobalPathArgs is a type for building arg lists for functions defined in "ContractGlobalPath".
pub struct ContractGlobalPathArgs;
///ContractGlobalPathClient is a client for calling the contract defined in "ContractGlobalPath".
pub struct ContractGlobalPathClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    config: soroban_sdk::testutils::ClientInternalConfig<'a>,
}
impl<'a> ContractGlobalPathClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            config: soroban_sdk::testutils::ClientInternalConfig::default(),
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
    pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            config: self.config.set_auths(auths),
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            config: self.config.mock_auths(mock_auths),
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
            config: self.config.mock_all_auths(),
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
            config: self.config.mock_all_auths_allowing_non_root_auth(),
        }
    }
}
mod __contractglobalpath_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = soroban_sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(
        name: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl soroban_sdk::testutils::ContractFunctionRegister for ContractGlobalPath {
    fn register(name: &'static str, func: &'static __contractglobalpath_fn_set_registry::F) {
        __contractglobalpath_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for ContractGlobalPath {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contractglobalpath_fn_set_registry::call(func, env, args)
    }
}
impl ::test_contracttrait_trait::AllTypes for ContractGlobalPath {}
impl<'a> ContractGlobalPathClient<'a> {}
impl ContractGlobalPathArgs {}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __ContractGlobalPath_AllTypes_types {
    type u32;
    type i32;
    type u64;
    type i64;
    type u128;
    type i128;
    type bool;
    type Address;
    type Bytes;
    type BytesN;
    type String;
    type Symbol;
    type Vec;
    type Map;
    type Duration;
    type Timepoint;
    type I256;
    type U256;
    type Env;
    type MyStruct;
    type MyEnumUnit;
    type MyEnumVariants;
}
impl<T: ::test_contracttrait_trait::AllTypes> __ContractGlobalPath_AllTypes_types for T {
    type u32 = T::__AllTypes_u32;
    type i32 = T::__AllTypes_i32;
    type u64 = T::__AllTypes_u64;
    type i64 = T::__AllTypes_i64;
    type u128 = T::__AllTypes_u128;
    type i128 = T::__AllTypes_i128;
    type bool = T::__AllTypes_bool;
    type Address = T::__AllTypes_Address;
    type Bytes = T::__AllTypes_Bytes;
    type BytesN = T::__AllTypes_BytesN;
    type String = T::__AllTypes_String;
    type Symbol = T::__AllTypes_Symbol;
    type Vec = T::__AllTypes_Vec;
    type Map = T::__AllTypes_Map;
    type Duration = T::__AllTypes_Duration;
    type Timepoint = T::__AllTypes_Timepoint;
    type I256 = T::__AllTypes_I256;
    type U256 = T::__AllTypes_U256;
    type Env = T::__AllTypes_Env;
    type MyStruct = T::__AllTypes_MyStruct;
    type MyEnumUnit = T::__AllTypes_MyEnumUnit;
    type MyEnumVariants = T::__AllTypes_MyEnumVariants;
}
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u32` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_u32__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_u32(
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
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u32` instead")]
pub fn __ContractGlobalPath__test_u32__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_u32__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u32` instead")]
pub extern "C" fn __ContractGlobalPath__test_u32__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_u32__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i32` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_i32__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_i32(
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
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i32` instead")]
pub fn __ContractGlobalPath__test_i32__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_i32__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i32` instead")]
pub extern "C" fn __ContractGlobalPath__test_i32__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_i32__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u64` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_u64__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_u64(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u64` instead")]
pub fn __ContractGlobalPath__test_u64__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_u64__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u64` instead")]
pub extern "C" fn __ContractGlobalPath__test_u64__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_u64__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i64` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_i64__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_i64(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i64` instead")]
pub fn __ContractGlobalPath__test_i64__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_i64__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i64` instead")]
pub extern "C" fn __ContractGlobalPath__test_i64__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_i64__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u128` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_u128__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_u128(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u128` instead")]
pub fn __ContractGlobalPath__test_u128__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_u128__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u128` instead")]
pub extern "C" fn __ContractGlobalPath__test_u128__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_u128__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i128` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_i128__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_i128(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i128` instead")]
pub fn __ContractGlobalPath__test_i128__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_i128__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i128` instead")]
pub extern "C" fn __ContractGlobalPath__test_i128__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_i128__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bool` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_bool__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_bool(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bool` instead")]
pub fn __ContractGlobalPath__test_bool__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_bool__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bool` instead")]
pub extern "C" fn __ContractGlobalPath__test_bool__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_bool__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_address` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_address__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_address(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_address` instead")]
pub fn __ContractGlobalPath__test_address__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_address__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_address` instead")]
pub extern "C" fn __ContractGlobalPath__test_address__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_address__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bytes` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_bytes__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_bytes(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bytes` instead")]
pub fn __ContractGlobalPath__test_bytes__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_bytes__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bytes` instead")]
pub extern "C" fn __ContractGlobalPath__test_bytes__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_bytes__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bytes_n` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_bytes_n__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_bytes_n(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bytes_n` instead")]
pub fn __ContractGlobalPath__test_bytes_n__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_bytes_n__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_bytes_n` instead")]
pub extern "C" fn __ContractGlobalPath__test_bytes_n__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_bytes_n__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_string` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_string__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_string(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_string` instead")]
pub fn __ContractGlobalPath__test_string__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_string__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_string` instead")]
pub extern "C" fn __ContractGlobalPath__test_string__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_string__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_symbol` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_symbol__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_symbol(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_symbol` instead")]
pub fn __ContractGlobalPath__test_symbol__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_symbol__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_symbol` instead")]
pub extern "C" fn __ContractGlobalPath__test_symbol__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_symbol__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_vec` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_vec__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_vec(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_vec` instead")]
pub fn __ContractGlobalPath__test_vec__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_vec__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_vec` instead")]
pub extern "C" fn __ContractGlobalPath__test_vec__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_vec__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_map` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_map__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_map(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_map` instead")]
pub fn __ContractGlobalPath__test_map__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_map__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_map` instead")]
pub extern "C" fn __ContractGlobalPath__test_map__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_map__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_duration` instead"
)]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_duration__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_duration(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_duration` instead"
)]
pub fn __ContractGlobalPath__test_duration__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_duration__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_duration` instead"
)]
pub extern "C" fn __ContractGlobalPath__test_duration__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_duration__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_timepoint` instead"
)]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_timepoint__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_timepoint(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_timepoint` instead"
)]
pub fn __ContractGlobalPath__test_timepoint__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_timepoint__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_timepoint` instead"
)]
pub extern "C" fn __ContractGlobalPath__test_timepoint__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_timepoint__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i256` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_i256__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_i256(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i256` instead")]
pub fn __ContractGlobalPath__test_i256__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_i256__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_i256` instead")]
pub extern "C" fn __ContractGlobalPath__test_i256__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_i256__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u256` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_u256__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_u256(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u256` instead")]
pub fn __ContractGlobalPath__test_u256__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_u256__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_u256` instead")]
pub extern "C" fn __ContractGlobalPath__test_u256__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_u256__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_env_param` instead"
)]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_env_param__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_env_param(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_env_param` instead"
)]
pub fn __ContractGlobalPath__test_env_param__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __ContractGlobalPath__test_env_param__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_env_param` instead"
)]
pub extern "C" fn __ContractGlobalPath__test_env_param__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_env_param__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_struct` instead")]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_struct__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_struct(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_struct` instead")]
pub fn __ContractGlobalPath__test_struct__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_struct__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_struct` instead")]
pub extern "C" fn __ContractGlobalPath__test_struct__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_struct__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_enum_unit` instead"
)]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_enum_unit__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_enum_unit(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_enum_unit` instead"
)]
pub fn __ContractGlobalPath__test_enum_unit__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_enum_unit__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_enum_unit` instead"
)]
pub extern "C" fn __ContractGlobalPath__test_enum_unit__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_enum_unit__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_enum_variants` instead"
)]
#[allow(deprecated)]
pub fn __ContractGlobalPath__test_enum_variants__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractGlobalPath as ::test_contracttrait_trait::AllTypes>::test_enum_variants(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_enum_variants` instead"
)]
pub fn __ContractGlobalPath__test_enum_variants__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __ContractGlobalPath__test_enum_variants__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractGlobalPathClient::new(&env, &contract_id).test_enum_variants` instead"
)]
pub extern "C" fn __ContractGlobalPath__test_enum_variants__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractGlobalPath__test_enum_variants__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_u32__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    /// Test u32 values.
    /// Returns the input unchanged.
    static __SPEC_XDR_FN_TEST_U32: [u8; super::ContractGlobalPath::spec_xdr_test_u32().len()] =
        super::ContractGlobalPath::spec_xdr_test_u32();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    /// Test u32 values.
    /// Returns the input unchanged.
    const __SPEC_XDR_ENTRY_test_u32: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
            b"Test u32 values.\nReturns the input unchanged.",
        ),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u32"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Test u32 values.
    /// Returns the input unchanged.
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_u32(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_u32.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_u32.const_to_xdr() }
    }
}
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_i32__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    /// Test i32 values.
    static __SPEC_XDR_FN_TEST_I32: [u8; super::ContractGlobalPath::spec_xdr_test_i32().len()] =
        super::ContractGlobalPath::spec_xdr_test_i32();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    /// Test i32 values.
    const __SPEC_XDR_ENTRY_test_i32: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
            b"Test i32 values.",
        ),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i32"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Test i32 values.
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_i32(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_i32.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_i32.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_u64__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_U64: [u8; super::ContractGlobalPath::spec_xdr_test_u64().len()] =
        super::ContractGlobalPath::spec_xdr_test_u64();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_u64: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u64"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_u64(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_u64.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_u64.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_i64__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_I64: [u8; super::ContractGlobalPath::spec_xdr_test_i64().len()] =
        super::ContractGlobalPath::spec_xdr_test_i64();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_i64: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i64"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_i64(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_i64.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_i64.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_u128__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_U128: [u8; super::ContractGlobalPath::spec_xdr_test_u128().len()] =
        super::ContractGlobalPath::spec_xdr_test_u128();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_u128: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u128"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_u128(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_u128.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_u128.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_i128__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_I128: [u8; super::ContractGlobalPath::spec_xdr_test_i128().len()] =
        super::ContractGlobalPath::spec_xdr_test_i128();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_i128: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i128"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_i128(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_i128.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_i128.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_bool__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_BOOL: [u8; super::ContractGlobalPath::spec_xdr_test_bool().len()] =
        super::ContractGlobalPath::spec_xdr_test_bool();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_bool: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_bool"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_bool(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_bool.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_bool.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_address__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ADDRESS: [u8; super::ContractGlobalPath::spec_xdr_test_address()
        .len()] = super::ContractGlobalPath::spec_xdr_test_address();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_address: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_address"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_address(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_address.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_address.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_bytes__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_BYTES: [u8; super::ContractGlobalPath::spec_xdr_test_bytes().len()] =
        super::ContractGlobalPath::spec_xdr_test_bytes();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_bytes: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_bytes"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_bytes(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_bytes.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_bytes.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_bytes_n__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_BYTES_N: [u8; super::ContractGlobalPath::spec_xdr_test_bytes_n()
        .len()] = super::ContractGlobalPath::spec_xdr_test_bytes_n();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_bytes_n: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_bytes_n"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_bytes_n(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_bytes_n.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_bytes_n.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_string__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_STRING: [u8; super::ContractGlobalPath::spec_xdr_test_string()
        .len()] = super::ContractGlobalPath::spec_xdr_test_string();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_string: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_string"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_string(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_string.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_string.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_symbol__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_SYMBOL: [u8; super::ContractGlobalPath::spec_xdr_test_symbol()
        .len()] = super::ContractGlobalPath::spec_xdr_test_symbol();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_symbol: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_symbol"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_symbol(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_symbol.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_symbol.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_vec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_VEC: [u8; super::ContractGlobalPath::spec_xdr_test_vec().len()] =
        super::ContractGlobalPath::spec_xdr_test_vec();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_vec: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_vec"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_vec(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_vec.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_vec.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_map__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_MAP: [u8; super::ContractGlobalPath::spec_xdr_test_map().len()] =
        super::ContractGlobalPath::spec_xdr_test_map();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_map: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_map"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_map(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_map.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_map.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_duration__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_DURATION: [u8; super::ContractGlobalPath::spec_xdr_test_duration()
        .len()] = super::ContractGlobalPath::spec_xdr_test_duration();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_duration: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_duration"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_duration(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_duration.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_duration.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_timepoint__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_TIMEPOINT: [u8; super::ContractGlobalPath::spec_xdr_test_timepoint(
    )
    .len()] = super::ContractGlobalPath::spec_xdr_test_timepoint();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_timepoint: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"test_timepoint",
            ),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_timepoint(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_timepoint.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_timepoint.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_i256__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_I256: [u8; super::ContractGlobalPath::spec_xdr_test_i256().len()] =
        super::ContractGlobalPath::spec_xdr_test_i256();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_i256: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_i256"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_i256(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_i256.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_i256.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_u256__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_U256: [u8; super::ContractGlobalPath::spec_xdr_test_u256().len()] =
        super::ContractGlobalPath::spec_xdr_test_u256();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_u256: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_u256"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_u256(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_u256.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_u256.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_env_param__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ENV_PARAM: [u8; super::ContractGlobalPath::spec_xdr_test_env_param(
    )
    .len()] = super::ContractGlobalPath::spec_xdr_test_env_param();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_env_param: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"test_env_param",
            ),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_env_param(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_env_param.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_env_param.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_struct__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_STRUCT: [u8; super::ContractGlobalPath::spec_xdr_test_struct()
        .len()] = super::ContractGlobalPath::spec_xdr_test_struct();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_struct: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_struct"),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_struct(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_struct.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_struct.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_enum_unit__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ENUM_UNIT: [u8; super::ContractGlobalPath::spec_xdr_test_enum_unit(
    )
    .len()] = super::ContractGlobalPath::spec_xdr_test_enum_unit();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_enum_unit: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"test_enum_unit",
            ),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_enum_unit(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_enum_unit.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_enum_unit.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractGlobalPath__test_enum_variants__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ENUM_VARIANTS: [u8;
        super::ContractGlobalPath::spec_xdr_test_enum_variants().len()] =
        super::ContractGlobalPath::spec_xdr_test_enum_variants();
}
impl ContractGlobalPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_enum_variants: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"test_enum_variants",
            ),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"",
                    ),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"v",
                    ),
                    type_: <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
            ],
        ),
    });
    #[allow(non_snake_case)]
    /// Returns the XDR of this item's contract spec entry, as it is
    /// embedded in the contract.
    ///
    /// The entry names user-defined types and events by their fully
    /// qualified path, such as `::mycrate::MyType`, which `stellar contract
    /// build` reduces to simple names in the spec of a built contract. A
    /// spec assembled from this entry outside of a contract build must be
    /// reduced with `soroban_spec::reduce::reduce` before it is published or
    /// given to tools.
    pub const fn spec_xdr_test_enum_variants(
    ) -> [u8; ContractGlobalPath::__SPEC_XDR_ENTRY_test_enum_variants.const_xdr_len()] {
        const { ContractGlobalPath::__SPEC_XDR_ENTRY_test_enum_variants.const_to_xdr() }
    }
}
impl<'a> ContractGlobalPathClient<'a> {
    /// Call `test_u32` function on the contract.
    ///
    /// Test u32 values.
    /// Returns the input unchanged.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u32`] to receive errors instead of panicking.
    pub fn test_u32(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u32` function on the contract.
    ///
    /// Test u32 values.
    /// Returns the input unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u32(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i32` function on the contract.
    ///
    /// Test i32 values.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i32`] to receive errors instead of panicking.
    pub fn test_i32(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i32` function on the contract.
    ///
    /// Test i32 values.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i32(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u64` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u64`] to receive errors instead of panicking.
    pub fn test_u64(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u64` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u64(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i64` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i64`] to receive errors instead of panicking.
    pub fn test_i64(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i64` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i64(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u128` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u128`] to receive errors instead of panicking.
    pub fn test_u128(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u128` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u128(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i128` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i128`] to receive errors instead of panicking.
    pub fn test_i128(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i128` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i128(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bool` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_bool`] to receive errors instead of panicking.
    pub fn test_bool(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bool` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_bool(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_address` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_address`] to receive errors instead of panicking.
    pub fn test_address(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_address` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_address(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_bytes`] to receive errors instead of panicking.
    pub fn test_bytes(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_bytes(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes_n` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_bytes_n`] to receive errors instead of panicking.
    pub fn test_bytes_n(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_bytes_n` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_bytes_n(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_string` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_string`] to receive errors instead of panicking.
    pub fn test_string(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_string` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_string(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_symbol` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_symbol`] to receive errors instead of panicking.
    pub fn test_symbol(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_symbol` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_symbol(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_vec` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_vec`] to receive errors instead of panicking.
    pub fn test_vec(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_vec` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_vec(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_map` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_map`] to receive errors instead of panicking.
    pub fn test_map(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_map` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_map(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_duration` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_duration`] to receive errors instead of panicking.
    pub fn test_duration(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_duration` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_duration(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_timepoint` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_timepoint`] to receive errors instead of panicking.
    pub fn test_timepoint(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_timepoint` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_timepoint(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i256` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_i256`] to receive errors instead of panicking.
    pub fn test_i256(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_i256` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_i256(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u256` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_u256`] to receive errors instead of panicking.
    pub fn test_u256(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_u256` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_u256(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_env_param` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_env_param`] to receive errors instead of panicking.
    pub fn test_env_param(
        &self,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `test_env_param` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_env_param(
        &self,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32 as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `test_struct` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_struct`] to receive errors instead of panicking.
    pub fn test_struct(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_struct` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_struct(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_unit` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_enum_unit`] to receive errors instead of panicking.
    pub fn test_enum_unit(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_unit` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_enum_unit(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_variants` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_test_enum_variants`] to receive errors instead of panicking.
    pub fn test_enum_variants(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants,
        >,
    ) -> <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants as soroban_sdk::client::Output>::Ok{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
    /// Call `test_enum_variants` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_test_enum_variants(
        &self,
        v: impl soroban_sdk::client::Arg<
            <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants,
        >,
    ) -> Result<
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants as soroban_sdk::client::Output>::Ok,
            <<<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<
            <<ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants as soroban_sdk::client::Output>::Err,
            soroban_sdk::InvokeError,
        >,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::client::Arg::<
                    <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants,
                >::into_val_for_client(v, &self.env)],
            ),
        );
        res
    }
}
impl ContractGlobalPathArgs {
    /// Test u32 values.
    /// Returns the input unchanged.
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u32<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u32,) {
        (v,)
    }
    /// Test i32 values.
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i32<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u64<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i64<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u128<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::u128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i128<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::i128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bool<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::bool,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_address<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Address,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Bytes,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes_n<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::BytesN,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_string<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::String,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_symbol<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Symbol,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_vec<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Vec,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_map<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Map,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_duration<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Duration,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_timepoint<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::Timepoint,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i256<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::I256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u256<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::U256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_env_param<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_struct<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyStruct,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_unit<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumUnit,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_variants<'i>(
        v: &'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants,
    ) -> (&'i <ContractGlobalPath as __ContractGlobalPath_AllTypes_types>::MyEnumVariants,) {
        (v,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __ContractGlobalPath__test_contracttrait_trait_AllTypes__959aee9d42336ade92416504111dfbb4e37b0472bbb1e487310c05a170c39d28_ctor(
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
                    __ContractGlobalPath__test_contracttrait_trait_AllTypes__959aee9d42336ade92416504111dfbb4e37b0472bbb1e487310c05a170c39d28_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u32",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_u32__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i32",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_i32__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u64",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_u64__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i64",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_i64__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u128",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_u128__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i128",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_i128__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_bool",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_bool__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_address",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_address__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_bytes",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_bytes__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_bytes_n",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_bytes_n__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_string",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_string__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_symbol",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_symbol__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_vec",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_vec__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_map",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_map__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_duration",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_duration__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_timepoint",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_timepoint__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i256",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_i256__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u256",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_u256__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_env_param",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_env_param__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_struct",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_struct__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_enum_unit",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_enum_unit__invoke_raw_slice,
        );
        <ContractGlobalPath as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_enum_variants",
            #[allow(deprecated)]
            &__ContractGlobalPath__test_enum_variants__invoke_raw_slice,
        );
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __ContractGlobalPath__test_contracttrait_trait_AllTypes__e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor(
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
                    __ContractGlobalPath__test_contracttrait_trait_AllTypes__e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {}
}
mod test {
    use super::*;
    use soroban_sdk::{map, symbol_short, testutils::Address as _, vec, Env};
    extern crate test;
    #[rustc_test_marker = "test::test_global_path"]
    #[doc(hidden)]
    pub const test_global_path: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_global_path"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/contracttrait_path_global/src/lib.rs",
            start_line: 28usize,
            start_col: 8usize,
            end_line: 28usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_global_path()),
        ),
    };
    fn test_global_path() {
        let e = Env::default();
        let contract_id = e.register(ContractGlobalPath, ());
        let client = ContractGlobalPathClient::new(&e, &contract_id);
        match (&client.test_u32(&42u32), &42u32) {
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
        match (&client.test_i32(&-42i32), &-42i32) {
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
        match (&client.test_u64(&42u64), &42u64) {
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
        match (&client.test_i64(&-42i64), &-42i64) {
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
        match (&client.test_u128(&42u128), &42u128) {
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
        match (&client.test_i128(&-42i128), &-42i128) {
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
        match (&client.test_bool(&true), &true) {
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
        let addr = Address::generate(&e);
        match (&client.test_address(&addr), &addr) {
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
        let bytes = Bytes::from_slice(&e, &[1, 2, 3]);
        match (&client.test_bytes(&bytes), &bytes) {
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
        let bytes_n = BytesN::from_array(&e, &[0u8; 32]);
        match (&client.test_bytes_n(&bytes_n), &bytes_n) {
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
        let string = String::from_str(&e, "hello");
        match (&client.test_string(&string), &string) {
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
        let symbol = {
            #[allow(deprecated)]
            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test");
            SYMBOL
        };
        match (&client.test_symbol(&symbol), &symbol) {
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
        let vec_val = ::soroban_sdk::Vec::from_array(&e, [1u32, 2u32, 3u32]);
        match (&client.test_vec(&vec_val), &vec_val) {
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
        let map_val = ::soroban_sdk::Map::from_array(&e, [(1u32, 2u32), (3u32, 4u32)]);
        match (&client.test_map(&map_val), &map_val) {
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
        let duration_val = Duration::from_seconds(&e, 100);
        match (&client.test_duration(&duration_val), &duration_val) {
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
        let timepoint_val = Timepoint::from_unix(&e, 100);
        match (&client.test_timepoint(&timepoint_val), &timepoint_val) {
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
        let i256_val = I256::from_i128(&e, 42);
        match (&client.test_i256(&i256_val), &i256_val) {
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
        let u256_val = U256::from_u128(&e, 42);
        match (&client.test_u256(&u256_val), &u256_val) {
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
        match (&client.test_env_param(), &42) {
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
        let my_struct = MyStruct { a: 10, b: 20 };
        match (&client.test_struct(&my_struct), &my_struct) {
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
        match (&client.test_enum_unit(&MyEnumUnit::A), &MyEnumUnit::A) {
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
        let my_enum = MyEnumVariants::VarB(MyStruct { a: 1, b: 2 });
        match (&client.test_enum_variants(&my_enum), &my_enum) {
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
    test::test_main_static(&[&test_global_path])
}
