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
impl AllTypes for Contract {}
impl<'a> ContractClient<'a> {}
impl ContractArgs {}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __Contract_AllTypes_project {
    type u32;
    type i32;
    type u64;
    type i64;
    type u128;
    type i128;
    type bool;
    type Address;
    type Bytes;
    type BytesN<const N0: usize>;
    type String;
    type Symbol;
    type Vec<T0>;
    type Map<T0, T1>;
    type Duration;
    type Timepoint;
    type I256;
    type U256;
    type Env;
    type MyStruct;
    type MyEnumUnit;
    type MyEnumVariants;
}
impl<__T: AllTypes> __Contract_AllTypes_project for __T {
    type u32 = __T::__AllTypes_u32;
    type i32 = __T::__AllTypes_i32;
    type u64 = __T::__AllTypes_u64;
    type i64 = __T::__AllTypes_i64;
    type u128 = __T::__AllTypes_u128;
    type i128 = __T::__AllTypes_i128;
    type bool = __T::__AllTypes_bool;
    type Address = __T::__AllTypes_Address;
    type Bytes = __T::__AllTypes_Bytes;
    type BytesN<const N0: usize> = __T::__AllTypes_BytesN<N0>;
    type String = __T::__AllTypes_String;
    type Symbol = __T::__AllTypes_Symbol;
    type Vec<T0> = __T::__AllTypes_Vec<T0>;
    type Map<T0, T1> = __T::__AllTypes_Map<T0, T1>;
    type Duration = __T::__AllTypes_Duration;
    type Timepoint = __T::__AllTypes_Timepoint;
    type I256 = __T::__AllTypes_I256;
    type U256 = __T::__AllTypes_U256;
    type Env = __T::__AllTypes_Env;
    type MyStruct = __T::__AllTypes_MyStruct;
    type MyEnumUnit = __T::__AllTypes_MyEnumUnit;
    type MyEnumVariants = __T::__AllTypes_MyEnumVariants;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract_AllTypes_types {
    #[allow(non_camel_case_types)]
    pub type u32 = <super::Contract as super::__Contract_AllTypes_project>::u32;
    #[allow(non_camel_case_types)]
    pub type i32 = <super::Contract as super::__Contract_AllTypes_project>::i32;
    #[allow(non_camel_case_types)]
    pub type u64 = <super::Contract as super::__Contract_AllTypes_project>::u64;
    #[allow(non_camel_case_types)]
    pub type i64 = <super::Contract as super::__Contract_AllTypes_project>::i64;
    #[allow(non_camel_case_types)]
    pub type u128 = <super::Contract as super::__Contract_AllTypes_project>::u128;
    #[allow(non_camel_case_types)]
    pub type i128 = <super::Contract as super::__Contract_AllTypes_project>::i128;
    #[allow(non_camel_case_types)]
    pub type bool = <super::Contract as super::__Contract_AllTypes_project>::bool;
    #[allow(non_camel_case_types)]
    pub type Address = <super::Contract as super::__Contract_AllTypes_project>::Address;
    #[allow(non_camel_case_types)]
    pub type Bytes = <super::Contract as super::__Contract_AllTypes_project>::Bytes;
    #[allow(non_camel_case_types)]
    pub type BytesN<const N0: usize> =
        <super::Contract as super::__Contract_AllTypes_project>::BytesN<N0>;
    #[allow(non_camel_case_types)]
    pub type String = <super::Contract as super::__Contract_AllTypes_project>::String;
    #[allow(non_camel_case_types)]
    pub type Symbol = <super::Contract as super::__Contract_AllTypes_project>::Symbol;
    #[allow(non_camel_case_types)]
    pub type Vec<T0> = <super::Contract as super::__Contract_AllTypes_project>::Vec<T0>;
    #[allow(non_camel_case_types)]
    pub type Map<T0, T1> = <super::Contract as super::__Contract_AllTypes_project>::Map<T0, T1>;
    #[allow(non_camel_case_types)]
    pub type Duration = <super::Contract as super::__Contract_AllTypes_project>::Duration;
    #[allow(non_camel_case_types)]
    pub type Timepoint = <super::Contract as super::__Contract_AllTypes_project>::Timepoint;
    #[allow(non_camel_case_types)]
    pub type I256 = <super::Contract as super::__Contract_AllTypes_project>::I256;
    #[allow(non_camel_case_types)]
    pub type U256 = <super::Contract as super::__Contract_AllTypes_project>::U256;
    #[allow(non_camel_case_types)]
    pub type Env = <super::Contract as super::__Contract_AllTypes_project>::Env;
    #[allow(non_camel_case_types)]
    pub type MyStruct = <super::Contract as super::__Contract_AllTypes_project>::MyStruct;
    #[allow(non_camel_case_types)]
    pub type MyEnumUnit = <super::Contract as super::__Contract_AllTypes_project>::MyEnumUnit;
    #[allow(non_camel_case_types)]
    pub type MyEnumVariants =
        <super::Contract as super::__Contract_AllTypes_project>::MyEnumVariants;
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
#[export_name = "test_u32"]
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
#[export_name = "test_i32"]
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
#[export_name = "test_u64"]
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
#[export_name = "test_i64"]
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
#[export_name = "test_u128"]
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
#[export_name = "test_i128"]
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
#[export_name = "test_bool"]
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
#[export_name = "test_address"]
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
#[export_name = "test_bytes"]
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
#[export_name = "test_bytes_n"]
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
#[export_name = "test_string"]
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
#[export_name = "test_symbol"]
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
#[export_name = "test_vec"]
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
#[export_name = "test_map"]
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
#[export_name = "test_duration"]
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
#[export_name = "test_timepoint"]
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
#[export_name = "test_i256"]
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
#[export_name = "test_u256"]
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
#[export_name = "test_env_param"]
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
#[export_name = "test_struct"]
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
#[export_name = "test_enum_unit"]
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
#[export_name = "test_enum_variants"]
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::i32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::u64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::u128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::i128 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::BytesN<
                        32,
                    > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::BytesN<
                    32,
                > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::Symbol as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::Vec<
                        __Contract_AllTypes_types::u32,
                    > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::Vec<
                    __Contract_AllTypes_types::u32,
                > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_TEST_MAP: [u8; super::Contract::spec_xdr_test_map().len()] =
        super::Contract::spec_xdr_test_map();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_map: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_map"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"v"),
                        type_: <__Contract_AllTypes_types::Map<
                            __Contract_AllTypes_types::u32,
                            __Contract_AllTypes_types::u32,
                        > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <__Contract_AllTypes_types::Map<
                        __Contract_AllTypes_types::u32,
                        __Contract_AllTypes_types::u32,
                    > as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::Duration as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::Timepoint as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::I256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::U256 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_TEST_ENV_PARAM: [u8; super::Contract::spec_xdr_test_env_param().len()] =
        super::Contract::spec_xdr_test_env_param();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_test_env_param: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"test_env_param"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <__Contract_AllTypes_types::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                ]),
            },
        );
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::MyStruct as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::MyEnumUnit as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    #[link_section = "contractspecv0"]
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
                    type_: <__Contract_AllTypes_types::MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                },
            ],
        ),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <__Contract_AllTypes_types::MyEnumVariants as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::u32>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::u32> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::u32>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::u32>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::u32>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::u32> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::u32>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::u32>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::i32>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::i32> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::i32>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::i32>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::i32>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::i32> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::i32>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::i32>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::u64>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::u64> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::u64>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::u64>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::u64>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::u64> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::u64>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::u64>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::i64>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::i64> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::i64>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::i64>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::i64>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::i64> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::i64>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::i64>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::u128>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::u128> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::u128>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::u128>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::u128>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::u128> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::u128>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::u128>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::i128>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::i128> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::i128>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::i128>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::i128>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::i128> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::i128>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::i128>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::bool>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::bool> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::bool>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::bool>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::bool>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::bool> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::bool>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::bool>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Address>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::Address> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Address>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Address>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Address>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::Address> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Address>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Address>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Bytes>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::Bytes> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Bytes>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Bytes>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Bytes>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::Bytes> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Bytes>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Bytes>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::BytesN<32>>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::BytesN<32>> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::BytesN<32>>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::BytesN<32>>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::BytesN<32>>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::BytesN<32>> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::BytesN<32>>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::BytesN<32>>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::String>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::String> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::String>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::String>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::String>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::String> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::String>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::String>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Symbol>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::Symbol> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Symbol>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Symbol>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Symbol>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::Symbol> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Symbol>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Symbol>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>>
    {
        use core::ops::Not;
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
                [::core::borrow::Borrow::<
                    __Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>,
                >::borrow(&soroban_sdk::IntoArg::<
                    __Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>,
                >::into_arg(v))
                .into_val(&self.env)],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>>
    {
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
                [::core::borrow::Borrow::<
                    __Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>,
                >::borrow(&soroban_sdk::IntoArg::<
                    __Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>,
                >::into_arg(v))
                .into_val(&self.env)],
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
        v: impl soroban_sdk::IntoArg<
            __Contract_AllTypes_types::Map<
                __Contract_AllTypes_types::u32,
                __Contract_AllTypes_types::u32,
            >,
        >,
    ) -> soroban_sdk::client::Ret<
        __Contract_AllTypes_types::Map<
            __Contract_AllTypes_types::u32,
            __Contract_AllTypes_types::u32,
        >,
    > {
        use core::ops::Not;
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
                [::core::borrow::Borrow::<
                    __Contract_AllTypes_types::Map<
                        __Contract_AllTypes_types::u32,
                        __Contract_AllTypes_types::u32,
                    >,
                >::borrow(&soroban_sdk::IntoArg::<
                    __Contract_AllTypes_types::Map<
                        __Contract_AllTypes_types::u32,
                        __Contract_AllTypes_types::u32,
                    >,
                >::into_arg(v))
                .into_val(&self.env)],
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
        v: impl soroban_sdk::IntoArg<
            __Contract_AllTypes_types::Map<
                __Contract_AllTypes_types::u32,
                __Contract_AllTypes_types::u32,
            >,
        >,
    ) -> soroban_sdk::client::TryRet<
        __Contract_AllTypes_types::Map<
            __Contract_AllTypes_types::u32,
            __Contract_AllTypes_types::u32,
        >,
    > {
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
                [::core::borrow::Borrow::<
                    __Contract_AllTypes_types::Map<
                        __Contract_AllTypes_types::u32,
                        __Contract_AllTypes_types::u32,
                    >,
                >::borrow(&soroban_sdk::IntoArg::<
                    __Contract_AllTypes_types::Map<
                        __Contract_AllTypes_types::u32,
                        __Contract_AllTypes_types::u32,
                    >,
                >::into_arg(v))
                .into_val(&self.env)],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Duration>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::Duration> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Duration>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Duration>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Duration>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::Duration> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Duration>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Duration>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Timepoint>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::Timepoint> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Timepoint>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Timepoint>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::Timepoint>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::Timepoint> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::Timepoint>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::Timepoint>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::I256>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::I256> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::I256>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::I256>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::I256>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::I256> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::I256>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::I256>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::U256>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::U256> {
        use core::ops::Not;
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::U256>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::U256>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::U256>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::U256> {
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
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::U256>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::U256>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
    pub fn test_env_param(&self) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::u32> {
        use core::ops::Not;
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
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::u32> {
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::MyStruct>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::MyStruct> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::MyStruct>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::MyStruct>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::MyStruct>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::MyStruct> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::MyStruct>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::MyStruct>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::MyEnumUnit>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::MyEnumUnit> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::MyEnumUnit>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::MyEnumUnit>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::MyEnumUnit>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::MyEnumUnit> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    ::core::borrow::Borrow::<__Contract_AllTypes_types::MyEnumUnit>::borrow(
                        &soroban_sdk::IntoArg::<__Contract_AllTypes_types::MyEnumUnit>::into_arg(v),
                    )
                    .into_val(&self.env),
                ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::MyEnumVariants>,
    ) -> soroban_sdk::client::Ret<__Contract_AllTypes_types::MyEnumVariants> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self
            .env
            .invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        ::core::borrow::Borrow::<
                            __Contract_AllTypes_types::MyEnumVariants,
                        >::borrow(
                                &soroban_sdk::IntoArg::<
                                    __Contract_AllTypes_types::MyEnumVariants,
                                >::into_arg(v),
                            )
                            .into_val(&self.env),
                    ],
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
        v: impl soroban_sdk::IntoArg<__Contract_AllTypes_types::MyEnumVariants>,
    ) -> soroban_sdk::client::TryRet<__Contract_AllTypes_types::MyEnumVariants> {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self
            .env
            .try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        ::core::borrow::Borrow::<
                            __Contract_AllTypes_types::MyEnumVariants,
                        >::borrow(
                                &soroban_sdk::IntoArg::<
                                    __Contract_AllTypes_types::MyEnumVariants,
                                >::into_arg(v),
                            )
                            .into_val(&self.env),
                    ],
                ),
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
        v: &'i __Contract_AllTypes_types::u32,
    ) -> (&'i __Contract_AllTypes_types::u32,) {
        (v,)
    }
    /// Test i32 values.
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i32<'i>(
        v: &'i __Contract_AllTypes_types::i32,
    ) -> (&'i __Contract_AllTypes_types::i32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u64<'i>(
        v: &'i __Contract_AllTypes_types::u64,
    ) -> (&'i __Contract_AllTypes_types::u64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i64<'i>(
        v: &'i __Contract_AllTypes_types::i64,
    ) -> (&'i __Contract_AllTypes_types::i64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u128<'i>(
        v: &'i __Contract_AllTypes_types::u128,
    ) -> (&'i __Contract_AllTypes_types::u128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i128<'i>(
        v: &'i __Contract_AllTypes_types::i128,
    ) -> (&'i __Contract_AllTypes_types::i128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bool<'i>(
        v: &'i __Contract_AllTypes_types::bool,
    ) -> (&'i __Contract_AllTypes_types::bool,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_address<'i>(
        v: &'i __Contract_AllTypes_types::Address,
    ) -> (&'i __Contract_AllTypes_types::Address,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes<'i>(
        v: &'i __Contract_AllTypes_types::Bytes,
    ) -> (&'i __Contract_AllTypes_types::Bytes,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes_n<'i>(
        v: &'i __Contract_AllTypes_types::BytesN<32>,
    ) -> (&'i __Contract_AllTypes_types::BytesN<32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_string<'i>(
        v: &'i __Contract_AllTypes_types::String,
    ) -> (&'i __Contract_AllTypes_types::String,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_symbol<'i>(
        v: &'i __Contract_AllTypes_types::Symbol,
    ) -> (&'i __Contract_AllTypes_types::Symbol,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_vec<'i>(
        v: &'i __Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>,
    ) -> (&'i __Contract_AllTypes_types::Vec<__Contract_AllTypes_types::u32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_map<'i>(
        v: &'i __Contract_AllTypes_types::Map<
            __Contract_AllTypes_types::u32,
            __Contract_AllTypes_types::u32,
        >,
    ) -> (
        &'i __Contract_AllTypes_types::Map<
            __Contract_AllTypes_types::u32,
            __Contract_AllTypes_types::u32,
        >,
    ) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_duration<'i>(
        v: &'i __Contract_AllTypes_types::Duration,
    ) -> (&'i __Contract_AllTypes_types::Duration,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_timepoint<'i>(
        v: &'i __Contract_AllTypes_types::Timepoint,
    ) -> (&'i __Contract_AllTypes_types::Timepoint,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i256<'i>(
        v: &'i __Contract_AllTypes_types::I256,
    ) -> (&'i __Contract_AllTypes_types::I256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u256<'i>(
        v: &'i __Contract_AllTypes_types::U256,
    ) -> (&'i __Contract_AllTypes_types::U256,) {
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
        v: &'i __Contract_AllTypes_types::MyStruct,
    ) -> (&'i __Contract_AllTypes_types::MyStruct,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_unit<'i>(
        v: &'i __Contract_AllTypes_types::MyEnumUnit,
    ) -> (&'i __Contract_AllTypes_types::MyEnumUnit,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_variants<'i>(
        v: &'i __Contract_AllTypes_types::MyEnumVariants,
    ) -> (&'i __Contract_AllTypes_types::MyEnumVariants,) {
        (v,)
    }
}
