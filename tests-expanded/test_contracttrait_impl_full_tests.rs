#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
use test_contracttrait_trait::AllTypes;
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    config: soroban_sdk::testutils::ClientInternalConfig<'a>,
}
impl<'a> ContractClient<'a> {
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
mod __contract_fn_set_registry {
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
impl soroban_sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for Contract {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
    }
}
impl AllTypes for Contract {}
impl<'a> ContractClient<'a> {}
impl ContractArgs {}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u32__arg_0 {
    type u32;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u32__arg_0 for T {
    type u32 = T::__test_u32__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u32__ret {
    type u32;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u32__ret for T {
    type u32 = T::__test_u32__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i32__arg_0 {
    type i32;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i32__arg_0 for T {
    type i32 = T::__test_i32__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i32__ret {
    type i32;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i32__ret for T {
    type i32 = T::__test_i32__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u64__arg_0 {
    type u64;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u64__arg_0 for T {
    type u64 = T::__test_u64__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u64__ret {
    type u64;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u64__ret for T {
    type u64 = T::__test_u64__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i64__arg_0 {
    type i64;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i64__arg_0 for T {
    type i64 = T::__test_i64__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i64__ret {
    type i64;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i64__ret for T {
    type i64 = T::__test_i64__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u128__arg_0 {
    type u128;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u128__arg_0 for T {
    type u128 = T::__test_u128__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u128__ret {
    type u128;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u128__ret for T {
    type u128 = T::__test_u128__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i128__arg_0 {
    type i128;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i128__arg_0 for T {
    type i128 = T::__test_i128__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i128__ret {
    type i128;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i128__ret for T {
    type i128 = T::__test_i128__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_bool__arg_0 {
    type bool;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_bool__arg_0 for T {
    type bool = T::__test_bool__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_bool__ret {
    type bool;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_bool__ret for T {
    type bool = T::__test_bool__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_address__arg_0 {
    type Address;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_address__arg_0 for T {
    type Address = T::__test_address__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_address__ret {
    type Address;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_address__ret for T {
    type Address = T::__test_address__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_bytes__arg_0 {
    type Bytes;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_bytes__arg_0 for T {
    type Bytes = T::__test_bytes__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_bytes__ret {
    type Bytes;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_bytes__ret for T {
    type Bytes = T::__test_bytes__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_bytes_n__arg_0 {
    type BytesN;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_bytes_n__arg_0 for T {
    type BytesN = T::__test_bytes_n__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_bytes_n__ret {
    type BytesN;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_bytes_n__ret for T {
    type BytesN = T::__test_bytes_n__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_string__arg_0 {
    type String;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_string__arg_0 for T {
    type String = T::__test_string__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_string__ret {
    type String;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_string__ret for T {
    type String = T::__test_string__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_symbol__arg_0 {
    type Symbol;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_symbol__arg_0 for T {
    type Symbol = T::__test_symbol__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_symbol__ret {
    type Symbol;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_symbol__ret for T {
    type Symbol = T::__test_symbol__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_vec__arg_0 {
    type Vec;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_vec__arg_0 for T {
    type Vec = T::__test_vec__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_vec__ret {
    type Vec;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_vec__ret for T {
    type Vec = T::__test_vec__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_map__arg_0 {
    type Map;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_map__arg_0 for T {
    type Map = T::__test_map__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_map__ret {
    type Map;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_map__ret for T {
    type Map = T::__test_map__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_duration__arg_0 {
    type Duration;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_duration__arg_0 for T {
    type Duration = T::__test_duration__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_duration__ret {
    type Duration;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_duration__ret for T {
    type Duration = T::__test_duration__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_timepoint__arg_0 {
    type Timepoint;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_timepoint__arg_0 for T {
    type Timepoint = T::__test_timepoint__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_timepoint__ret {
    type Timepoint;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_timepoint__ret for T {
    type Timepoint = T::__test_timepoint__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i256__arg_0 {
    type I256;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i256__arg_0 for T {
    type I256 = T::__test_i256__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_i256__ret {
    type I256;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_i256__ret for T {
    type I256 = T::__test_i256__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u256__arg_0 {
    type U256;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u256__arg_0 for T {
    type U256 = T::__test_u256__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_u256__ret {
    type U256;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_u256__ret for T {
    type U256 = T::__test_u256__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_env_param__arg_0 {
    type Env;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_env_param__arg_0 for T {
    type Env = T::__test_env_param__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_env_param__ret {
    type u32;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_env_param__ret for T {
    type u32 = T::__test_env_param__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_struct__arg_0 {
    type MyStruct;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_struct__arg_0 for T {
    type MyStruct = T::__test_struct__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_struct__ret {
    type MyStruct;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_struct__ret for T {
    type MyStruct = T::__test_struct__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_enum_unit__arg_0 {
    type MyEnumUnit;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_enum_unit__arg_0 for T {
    type MyEnumUnit = T::__test_enum_unit__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_enum_unit__ret {
    type MyEnumUnit;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_enum_unit__ret for T {
    type MyEnumUnit = T::__test_enum_unit__ret;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_enum_variants__arg_0 {
    type MyEnumVariants;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_enum_variants__arg_0 for T {
    type MyEnumVariants = T::__test_enum_variants__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract__AllTypes__test_enum_variants__ret {
    type MyEnumVariants;
}
impl<T: AllTypes + ?Sized> __Contract__AllTypes__test_enum_variants__ret for T {
    type MyEnumVariants = T::__test_enum_variants__ret;
}
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u32` instead")]
#[allow(deprecated)]
pub fn __Contract__test_u32__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_u32(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u32` instead")]
pub fn __Contract__test_u32__invoke_raw_slice(
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
    __Contract__test_u32__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u32` instead")]
pub extern "C" fn __Contract__test_u32__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_u32__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i32` instead")]
#[allow(deprecated)]
pub fn __Contract__test_i32__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_i32(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i32` instead")]
pub fn __Contract__test_i32__invoke_raw_slice(
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
    __Contract__test_i32__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i32` instead")]
pub extern "C" fn __Contract__test_i32__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_i32__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u64` instead")]
#[allow(deprecated)]
pub fn __Contract__test_u64__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_u64(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u64` instead")]
pub fn __Contract__test_u64__invoke_raw_slice(
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
    __Contract__test_u64__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u64` instead")]
pub extern "C" fn __Contract__test_u64__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_u64__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i64` instead")]
#[allow(deprecated)]
pub fn __Contract__test_i64__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_i64(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i64` instead")]
pub fn __Contract__test_i64__invoke_raw_slice(
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
    __Contract__test_i64__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i64` instead")]
pub extern "C" fn __Contract__test_i64__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_i64__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u128` instead")]
#[allow(deprecated)]
pub fn __Contract__test_u128__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_u128(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u128` instead")]
pub fn __Contract__test_u128__invoke_raw_slice(
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
    __Contract__test_u128__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u128` instead")]
pub extern "C" fn __Contract__test_u128__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_u128__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i128` instead")]
#[allow(deprecated)]
pub fn __Contract__test_i128__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_i128(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i128` instead")]
pub fn __Contract__test_i128__invoke_raw_slice(
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
    __Contract__test_i128__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i128` instead")]
pub extern "C" fn __Contract__test_i128__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_i128__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bool` instead")]
#[allow(deprecated)]
pub fn __Contract__test_bool__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_bool(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bool` instead")]
pub fn __Contract__test_bool__invoke_raw_slice(
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
    __Contract__test_bool__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bool` instead")]
pub extern "C" fn __Contract__test_bool__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_bool__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_address` instead")]
#[allow(deprecated)]
pub fn __Contract__test_address__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_address(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_address` instead")]
pub fn __Contract__test_address__invoke_raw_slice(
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
    __Contract__test_address__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_address` instead")]
pub extern "C" fn __Contract__test_address__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_address__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes` instead")]
#[allow(deprecated)]
pub fn __Contract__test_bytes__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_bytes(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes` instead")]
pub fn __Contract__test_bytes__invoke_raw_slice(
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
    __Contract__test_bytes__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes` instead")]
pub extern "C" fn __Contract__test_bytes__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_bytes__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes_n` instead")]
#[allow(deprecated)]
pub fn __Contract__test_bytes_n__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_bytes_n(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes_n` instead")]
pub fn __Contract__test_bytes_n__invoke_raw_slice(
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
    __Contract__test_bytes_n__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_bytes_n` instead")]
pub extern "C" fn __Contract__test_bytes_n__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_bytes_n__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_string` instead")]
#[allow(deprecated)]
pub fn __Contract__test_string__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_string(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_string` instead")]
pub fn __Contract__test_string__invoke_raw_slice(
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
    __Contract__test_string__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_string` instead")]
pub extern "C" fn __Contract__test_string__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_string__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_symbol` instead")]
#[allow(deprecated)]
pub fn __Contract__test_symbol__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_symbol(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_symbol` instead")]
pub fn __Contract__test_symbol__invoke_raw_slice(
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
    __Contract__test_symbol__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_symbol` instead")]
pub extern "C" fn __Contract__test_symbol__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_symbol__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_vec` instead")]
#[allow(deprecated)]
pub fn __Contract__test_vec__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_vec(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_vec` instead")]
pub fn __Contract__test_vec__invoke_raw_slice(
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
    __Contract__test_vec__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_vec` instead")]
pub extern "C" fn __Contract__test_vec__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_vec__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_map` instead")]
#[allow(deprecated)]
pub fn __Contract__test_map__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_map(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_map` instead")]
pub fn __Contract__test_map__invoke_raw_slice(
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
    __Contract__test_map__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_map` instead")]
pub extern "C" fn __Contract__test_map__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_map__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_duration` instead")]
#[allow(deprecated)]
pub fn __Contract__test_duration__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_duration(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_duration` instead")]
pub fn __Contract__test_duration__invoke_raw_slice(
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
    __Contract__test_duration__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_duration` instead")]
pub extern "C" fn __Contract__test_duration__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_duration__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_timepoint` instead")]
#[allow(deprecated)]
pub fn __Contract__test_timepoint__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_timepoint(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_timepoint` instead")]
pub fn __Contract__test_timepoint__invoke_raw_slice(
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
    __Contract__test_timepoint__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_timepoint` instead")]
pub extern "C" fn __Contract__test_timepoint__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_timepoint__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i256` instead")]
#[allow(deprecated)]
pub fn __Contract__test_i256__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_i256(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i256` instead")]
pub fn __Contract__test_i256__invoke_raw_slice(
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
    __Contract__test_i256__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_i256` instead")]
pub extern "C" fn __Contract__test_i256__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_i256__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u256` instead")]
#[allow(deprecated)]
pub fn __Contract__test_u256__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_u256(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u256` instead")]
pub fn __Contract__test_u256__invoke_raw_slice(
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
    __Contract__test_u256__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_u256` instead")]
pub extern "C" fn __Contract__test_u256__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_u256__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_env_param` instead")]
#[allow(deprecated)]
pub fn __Contract__test_env_param__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_env_param(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_env_param` instead")]
pub fn __Contract__test_env_param__invoke_raw_slice(
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
    __Contract__test_env_param__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_env_param` instead")]
pub extern "C" fn __Contract__test_env_param__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_env_param__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_struct` instead")]
#[allow(deprecated)]
pub fn __Contract__test_struct__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_struct(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_struct` instead")]
pub fn __Contract__test_struct__invoke_raw_slice(
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
    __Contract__test_struct__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_struct` instead")]
pub extern "C" fn __Contract__test_struct__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_struct__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_unit` instead")]
#[allow(deprecated)]
pub fn __Contract__test_enum_unit__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_enum_unit(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_unit` instead")]
pub fn __Contract__test_enum_unit__invoke_raw_slice(
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
    __Contract__test_enum_unit__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_unit` instead")]
pub extern "C" fn __Contract__test_enum_unit__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_enum_unit__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_variants` instead")]
#[allow(deprecated)]
pub fn __Contract__test_enum_variants__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AllTypes>::test_enum_variants(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_variants` instead")]
pub fn __Contract__test_enum_variants__invoke_raw_slice(
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
    __Contract__test_enum_variants__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).test_enum_variants` instead")]
pub extern "C" fn __Contract__test_enum_variants__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__test_enum_variants__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
/// Test u32 values.
/// Returns the input unchanged.
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_u32__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    /// Test u32 values.
    /// Returns the input unchanged.
    static __SPEC_XDR_FN_TEST_U32: [u8; super::Contract::spec_xdr_test_u32().len()] =
        super::Contract::spec_xdr_test_u32();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_u32__arg_0>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_u32__ret>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u32() -> [u8; Contract::__SPEC_XDR_ENTRY_test_u32.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_u32.const_to_xdr() }
    }
}
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_i32__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    /// Test i32 values.
    static __SPEC_XDR_FN_TEST_I32: [u8; super::Contract::spec_xdr_test_i32().len()] =
        super::Contract::spec_xdr_test_i32();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_i32__arg_0>::i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_i32__ret>::i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i32() -> [u8; Contract::__SPEC_XDR_ENTRY_test_i32.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_i32.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_u64__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_U64: [u8; super::Contract::spec_xdr_test_u64().len()] =
        super::Contract::spec_xdr_test_u64();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_u64__arg_0>::u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_u64__ret>::u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u64() -> [u8; Contract::__SPEC_XDR_ENTRY_test_u64.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_u64.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_i64__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_I64: [u8; super::Contract::spec_xdr_test_i64().len()] =
        super::Contract::spec_xdr_test_i64();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_i64__arg_0>::i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_i64__ret>::i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i64() -> [u8; Contract::__SPEC_XDR_ENTRY_test_i64.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_i64.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_u128__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_U128: [u8; super::Contract::spec_xdr_test_u128().len()] =
        super::Contract::spec_xdr_test_u128();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_u128__arg_0>::u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_u128__ret>::u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u128() -> [u8; Contract::__SPEC_XDR_ENTRY_test_u128.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_test_u128.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_i128__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_I128: [u8; super::Contract::spec_xdr_test_i128().len()] =
        super::Contract::spec_xdr_test_i128();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_i128__arg_0>::i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_i128__ret>::i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i128() -> [u8; Contract::__SPEC_XDR_ENTRY_test_i128.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_test_i128.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_bool__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_BOOL: [u8; super::Contract::spec_xdr_test_bool().len()] =
        super::Contract::spec_xdr_test_bool();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_bool__arg_0>::bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_bool__ret>::bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_bool() -> [u8; Contract::__SPEC_XDR_ENTRY_test_bool.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_test_bool.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_address__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ADDRESS: [u8; super::Contract::spec_xdr_test_address().len()] =
        super::Contract::spec_xdr_test_address();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_address__arg_0>::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_address__ret>::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_address.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_address.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_bytes__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_BYTES: [u8; super::Contract::spec_xdr_test_bytes().len()] =
        super::Contract::spec_xdr_test_bytes();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_bytes__arg_0>::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_bytes__ret>::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_bytes() -> [u8; Contract::__SPEC_XDR_ENTRY_test_bytes.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_test_bytes.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_bytes_n__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_BYTES_N: [u8; super::Contract::spec_xdr_test_bytes_n().len()] =
        super::Contract::spec_xdr_test_bytes_n();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_bytes_n__arg_0>::BytesN as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_bytes_n__ret>::BytesN as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_bytes_n.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_bytes_n.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_string__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_STRING: [u8; super::Contract::spec_xdr_test_string().len()] =
        super::Contract::spec_xdr_test_string();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_string__arg_0>::String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_string__ret>::String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_string.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_string.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_symbol__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_SYMBOL: [u8; super::Contract::spec_xdr_test_symbol().len()] =
        super::Contract::spec_xdr_test_symbol();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_symbol__arg_0>::Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_symbol__ret>::Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_symbol.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_symbol.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_vec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_VEC: [u8; super::Contract::spec_xdr_test_vec().len()] =
        super::Contract::spec_xdr_test_vec();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_vec__arg_0>::Vec as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_vec__ret>::Vec as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_vec() -> [u8; Contract::__SPEC_XDR_ENTRY_test_vec.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_vec.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_map__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_MAP: [u8; super::Contract::spec_xdr_test_map().len()] =
        super::Contract::spec_xdr_test_map();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_map__arg_0>::Map as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_map__ret>::Map as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_map() -> [u8; Contract::__SPEC_XDR_ENTRY_test_map.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_map.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_duration__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_DURATION: [u8; super::Contract::spec_xdr_test_duration().len()] =
        super::Contract::spec_xdr_test_duration();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_duration__arg_0>::Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_duration__ret>::Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_duration.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_duration.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_timepoint__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_TIMEPOINT: [u8; super::Contract::spec_xdr_test_timepoint().len()] =
        super::Contract::spec_xdr_test_timepoint();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_timepoint__arg_0>::Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_timepoint__ret>::Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_timepoint.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_timepoint.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_i256__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_I256: [u8; super::Contract::spec_xdr_test_i256().len()] =
        super::Contract::spec_xdr_test_i256();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_i256__arg_0>::I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_i256__ret>::I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_i256() -> [u8; Contract::__SPEC_XDR_ENTRY_test_i256.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_test_i256.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_u256__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_U256: [u8; super::Contract::spec_xdr_test_u256().len()] =
        super::Contract::spec_xdr_test_u256();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_u256__arg_0>::U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_u256__ret>::U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_test_u256() -> [u8; Contract::__SPEC_XDR_ENTRY_test_u256.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_test_u256.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_env_param__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ENV_PARAM: [u8; super::Contract::spec_xdr_test_env_param().len()] =
        super::Contract::spec_xdr_test_env_param();
}
impl Contract {
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
                <<Contract as __Contract__AllTypes__test_env_param__ret>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_env_param.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_env_param.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_struct__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_STRUCT: [u8; super::Contract::spec_xdr_test_struct().len()] =
        super::Contract::spec_xdr_test_struct();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_struct__arg_0>::MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_struct__ret>::MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_struct.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_struct.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_enum_unit__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ENUM_UNIT: [u8; super::Contract::spec_xdr_test_enum_unit().len()] =
        super::Contract::spec_xdr_test_enum_unit();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_enum_unit__arg_0>::MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_enum_unit__ret>::MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_enum_unit.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_enum_unit.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__test_enum_variants__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_TEST_ENUM_VARIANTS: [u8; super::Contract::spec_xdr_test_enum_variants()
        .len()] = super::Contract::spec_xdr_test_enum_variants();
}
impl Contract {
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
                    type_: <<Contract as __Contract__AllTypes__test_enum_variants__arg_0>::MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<Contract as __Contract__AllTypes__test_enum_variants__ret>::MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_test_enum_variants.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_test_enum_variants.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
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
        v: &<Contract as __Contract__AllTypes__test_u32__arg_0>::u32,
    ) -> <Contract as __Contract__AllTypes__test_u32__ret>::u32 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u32__arg_0>::u32,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_u32__ret>::u32,
            <<Contract as __Contract__AllTypes__test_u32__ret>::u32 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i32__arg_0>::i32,
    ) -> <Contract as __Contract__AllTypes__test_i32__ret>::i32 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i32__arg_0>::i32,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_i32__ret>::i32,
            <<Contract as __Contract__AllTypes__test_i32__ret>::i32 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i32");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u64__arg_0>::u64,
    ) -> <Contract as __Contract__AllTypes__test_u64__ret>::u64 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u64__arg_0>::u64,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_u64__ret>::u64,
            <<Contract as __Contract__AllTypes__test_u64__ret>::u64 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i64__arg_0>::i64,
    ) -> <Contract as __Contract__AllTypes__test_i64__ret>::i64 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i64__arg_0>::i64,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_i64__ret>::i64,
            <<Contract as __Contract__AllTypes__test_i64__ret>::i64 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i64");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u128__arg_0>::u128,
    ) -> <Contract as __Contract__AllTypes__test_u128__ret>::u128 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u128__arg_0>::u128,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_u128__ret>::u128,
            <<Contract as __Contract__AllTypes__test_u128__ret>::u128 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i128__arg_0>::i128,
    ) -> <Contract as __Contract__AllTypes__test_i128__ret>::i128 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i128__arg_0>::i128,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_i128__ret>::i128,
            <<Contract as __Contract__AllTypes__test_i128__ret>::i128 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i128");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_bool__arg_0>::bool,
    ) -> <Contract as __Contract__AllTypes__test_bool__ret>::bool {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_bool__arg_0>::bool,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_bool__ret>::bool,
            <<Contract as __Contract__AllTypes__test_bool__ret>::bool as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_bool");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_address__arg_0>::Address,
    ) -> <Contract as __Contract__AllTypes__test_address__ret>::Address {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_address__arg_0>::Address,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_address__ret>::Address,
            <<Contract as __Contract__AllTypes__test_address__ret>::Address as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_bytes__arg_0>::Bytes,
    ) -> <Contract as __Contract__AllTypes__test_bytes__ret>::Bytes {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_bytes__arg_0>::Bytes,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_bytes__ret>::Bytes,
            <<Contract as __Contract__AllTypes__test_bytes__ret>::Bytes as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_bytes_n__arg_0>::BytesN,
    ) -> <Contract as __Contract__AllTypes__test_bytes_n__ret>::BytesN {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_bytes_n__arg_0>::BytesN,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_bytes_n__ret>::BytesN,
            <<Contract as __Contract__AllTypes__test_bytes_n__ret>::BytesN as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_string__arg_0>::String,
    ) -> <Contract as __Contract__AllTypes__test_string__ret>::String {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_string__arg_0>::String,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_string__ret>::String,
            <<Contract as __Contract__AllTypes__test_string__ret>::String as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_symbol__arg_0>::Symbol,
    ) -> <Contract as __Contract__AllTypes__test_symbol__ret>::Symbol {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_symbol__arg_0>::Symbol,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_symbol__ret>::Symbol,
            <<Contract as __Contract__AllTypes__test_symbol__ret>::Symbol as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_vec__arg_0>::Vec,
    ) -> <Contract as __Contract__AllTypes__test_vec__ret>::Vec {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_vec__arg_0>::Vec,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_vec__ret>::Vec,
            <<Contract as __Contract__AllTypes__test_vec__ret>::Vec as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_vec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_map__arg_0>::Map,
    ) -> <Contract as __Contract__AllTypes__test_map__ret>::Map {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_map__arg_0>::Map,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_map__ret>::Map,
            <<Contract as __Contract__AllTypes__test_map__ret>::Map as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_map");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_duration__arg_0>::Duration,
    ) -> <Contract as __Contract__AllTypes__test_duration__ret>::Duration {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_duration__arg_0>::Duration,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_duration__ret>::Duration,
            <<Contract as __Contract__AllTypes__test_duration__ret>::Duration as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_timepoint__arg_0>::Timepoint,
    ) -> <Contract as __Contract__AllTypes__test_timepoint__ret>::Timepoint {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_timepoint__arg_0>::Timepoint,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_timepoint__ret>::Timepoint,
            <<Contract as __Contract__AllTypes__test_timepoint__ret>::Timepoint as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i256__arg_0>::I256,
    ) -> <Contract as __Contract__AllTypes__test_i256__ret>::I256 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_i256__arg_0>::I256,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_i256__ret>::I256,
            <<Contract as __Contract__AllTypes__test_i256__ret>::I256 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_i256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u256__arg_0>::U256,
    ) -> <Contract as __Contract__AllTypes__test_u256__ret>::U256 {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_u256__arg_0>::U256,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_u256__ret>::U256,
            <<Contract as __Contract__AllTypes__test_u256__ret>::U256 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("test_u256");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
    pub fn test_env_param(&self) -> <Contract as __Contract__AllTypes__test_env_param__ret>::u32 {
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
            <Contract as __Contract__AllTypes__test_env_param__ret>::u32,
            <<Contract as __Contract__AllTypes__test_env_param__ret>::u32 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
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
        v: &<Contract as __Contract__AllTypes__test_struct__arg_0>::MyStruct,
    ) -> <Contract as __Contract__AllTypes__test_struct__ret>::MyStruct {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_struct__arg_0>::MyStruct,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_struct__ret>::MyStruct,
            <<Contract as __Contract__AllTypes__test_struct__ret>::MyStruct as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_enum_unit__arg_0>::MyEnumUnit,
    ) -> <Contract as __Contract__AllTypes__test_enum_unit__ret>::MyEnumUnit {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_enum_unit__arg_0>::MyEnumUnit,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_enum_unit__ret>::MyEnumUnit,
            <<Contract as __Contract__AllTypes__test_enum_unit__ret>::MyEnumUnit as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_enum_variants__arg_0>::MyEnumVariants,
    ) -> <Contract as __Contract__AllTypes__test_enum_variants__ret>::MyEnumVariants {
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
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
        v: &<Contract as __Contract__AllTypes__test_enum_variants__arg_0>::MyEnumVariants,
    ) -> Result<
        Result<
            <Contract as __Contract__AllTypes__test_enum_variants__ret>::MyEnumVariants,
            <<Contract as __Contract__AllTypes__test_enum_variants__ret>::MyEnumVariants as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        let _call_scope = soroban_sdk::testutils::ClientCallScope::enter(&self.env, self.config);
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    /// Test u32 values.
    /// Returns the input unchanged.
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u32<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_u32__arg_0>::u32,
    ) -> (&'i <Contract as __Contract__AllTypes__test_u32__arg_0>::u32,) {
        (v,)
    }
    /// Test i32 values.
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i32<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_i32__arg_0>::i32,
    ) -> (&'i <Contract as __Contract__AllTypes__test_i32__arg_0>::i32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u64<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_u64__arg_0>::u64,
    ) -> (&'i <Contract as __Contract__AllTypes__test_u64__arg_0>::u64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i64<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_i64__arg_0>::i64,
    ) -> (&'i <Contract as __Contract__AllTypes__test_i64__arg_0>::i64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u128<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_u128__arg_0>::u128,
    ) -> (&'i <Contract as __Contract__AllTypes__test_u128__arg_0>::u128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i128<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_i128__arg_0>::i128,
    ) -> (&'i <Contract as __Contract__AllTypes__test_i128__arg_0>::i128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bool<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_bool__arg_0>::bool,
    ) -> (&'i <Contract as __Contract__AllTypes__test_bool__arg_0>::bool,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_address<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_address__arg_0>::Address,
    ) -> (&'i <Contract as __Contract__AllTypes__test_address__arg_0>::Address,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_bytes__arg_0>::Bytes,
    ) -> (&'i <Contract as __Contract__AllTypes__test_bytes__arg_0>::Bytes,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes_n<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_bytes_n__arg_0>::BytesN,
    ) -> (&'i <Contract as __Contract__AllTypes__test_bytes_n__arg_0>::BytesN,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_string<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_string__arg_0>::String,
    ) -> (&'i <Contract as __Contract__AllTypes__test_string__arg_0>::String,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_symbol<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_symbol__arg_0>::Symbol,
    ) -> (&'i <Contract as __Contract__AllTypes__test_symbol__arg_0>::Symbol,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_vec<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_vec__arg_0>::Vec,
    ) -> (&'i <Contract as __Contract__AllTypes__test_vec__arg_0>::Vec,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_map<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_map__arg_0>::Map,
    ) -> (&'i <Contract as __Contract__AllTypes__test_map__arg_0>::Map,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_duration<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_duration__arg_0>::Duration,
    ) -> (&'i <Contract as __Contract__AllTypes__test_duration__arg_0>::Duration,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_timepoint<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_timepoint__arg_0>::Timepoint,
    ) -> (&'i <Contract as __Contract__AllTypes__test_timepoint__arg_0>::Timepoint,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i256<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_i256__arg_0>::I256,
    ) -> (&'i <Contract as __Contract__AllTypes__test_i256__arg_0>::I256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u256<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_u256__arg_0>::U256,
    ) -> (&'i <Contract as __Contract__AllTypes__test_u256__arg_0>::U256,) {
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
        v: &'i <Contract as __Contract__AllTypes__test_struct__arg_0>::MyStruct,
    ) -> (&'i <Contract as __Contract__AllTypes__test_struct__arg_0>::MyStruct,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_unit<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_enum_unit__arg_0>::MyEnumUnit,
    ) -> (&'i <Contract as __Contract__AllTypes__test_enum_unit__arg_0>::MyEnumUnit,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_variants<'i>(
        v: &'i <Contract as __Contract__AllTypes__test_enum_variants__arg_0>::MyEnumVariants,
    ) -> (&'i <Contract as __Contract__AllTypes__test_enum_variants__arg_0>::MyEnumVariants,) {
        (v,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__AllTypes__959aee9d42336ade92416504111dfbb4e37b0472bbb1e487310c05a170c39d28_ctor() {
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
                    __Contract__AllTypes__959aee9d42336ade92416504111dfbb4e37b0472bbb1e487310c05a170c39d28_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u32",
            #[allow(deprecated)]
            &__Contract__test_u32__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i32",
            #[allow(deprecated)]
            &__Contract__test_i32__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u64",
            #[allow(deprecated)]
            &__Contract__test_u64__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i64",
            #[allow(deprecated)]
            &__Contract__test_i64__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u128",
            #[allow(deprecated)]
            &__Contract__test_u128__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i128",
            #[allow(deprecated)]
            &__Contract__test_i128__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_bool",
            #[allow(deprecated)]
            &__Contract__test_bool__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_address",
            #[allow(deprecated)]
            &__Contract__test_address__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_bytes",
            #[allow(deprecated)]
            &__Contract__test_bytes__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_bytes_n",
            #[allow(deprecated)]
            &__Contract__test_bytes_n__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_string",
            #[allow(deprecated)]
            &__Contract__test_string__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_symbol",
            #[allow(deprecated)]
            &__Contract__test_symbol__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_vec",
            #[allow(deprecated)]
            &__Contract__test_vec__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_map",
            #[allow(deprecated)]
            &__Contract__test_map__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_duration",
            #[allow(deprecated)]
            &__Contract__test_duration__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_timepoint",
            #[allow(deprecated)]
            &__Contract__test_timepoint__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_i256",
            #[allow(deprecated)]
            &__Contract__test_i256__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_u256",
            #[allow(deprecated)]
            &__Contract__test_u256__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_env_param",
            #[allow(deprecated)]
            &__Contract__test_env_param__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_struct",
            #[allow(deprecated)]
            &__Contract__test_struct__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_enum_unit",
            #[allow(deprecated)]
            &__Contract__test_enum_unit__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "test_enum_variants",
            #[allow(deprecated)]
            &__Contract__test_enum_variants__invoke_raw_slice,
        );
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__AllTypes__e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor() {
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
                    __Contract__AllTypes__e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855_ctor();
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
    use soroban_sdk::{Address, Bytes, BytesN, Duration, String, Timepoint, I256, U256};
    use test_contracttrait_trait::{MyEnumUnit, MyEnumVariants, MyStruct};
    extern crate test;
    #[rustc_test_marker = "test::test_cross_crate_default_fns"]
    #[doc(hidden)]
    pub const test_cross_crate_default_fns: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_cross_crate_default_fns"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/contracttrait_impl_full/src/lib.rs",
            start_line: 19usize,
            start_col: 8usize,
            end_line: 19usize,
            end_col: 36usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_cross_crate_default_fns()),
        ),
    };
    fn test_cross_crate_default_fns() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
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
    test::test_main_static(&[&test_cross_crate_default_fns])
}
