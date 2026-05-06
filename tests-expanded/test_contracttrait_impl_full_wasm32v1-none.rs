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
use test_contracttrait_trait::{AllTypes, MyEnumUnit, MyEnumVariants, MyStruct};
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
pub mod __Contract__test_u32__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    /// Test u32 values.
    /// Returns the input unchanged.
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_U32: [u8; 96usize] = super::Contract::spec_xdr_test_u32();
}
impl Contract {
    #[allow(non_snake_case)]
    /// Test u32 values.
    /// Returns the input unchanged.
    pub const fn spec_xdr_test_u32() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0-Test u32 values.\nReturns the input unchanged.\0\0\0\0\0\0\x08test_u32\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_U32: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\x8dR\xda\x86B-/\xc4kN\x18Z\xecF\xf8\xc8\xb8\xa7!U\x86\x99\x8bv@#\xd2\x92\x02\xd3\x96\x1d",
    [],
);
#[doc(hidden)]
/// Test i32 values.
#[allow(non_snake_case)]
pub mod __Contract__test_i32__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    /// Test i32 values.
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_I32: [u8; 64usize] = super::Contract::spec_xdr_test_i32();
}
impl Contract {
    #[allow(non_snake_case)]
    /// Test i32 values.
    pub const fn spec_xdr_test_i32() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\x10Test i32 values.\0\0\0\x08test_i32\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x05\0\0\0\x01\0\0\0\x05"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_I32: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"RO\xc6\xf8W\x07\xe9\xe3_\xe3J9D\xa5\x87\xc3\x9d`\x04c\xa5\xb3%\xe91\x07#E\xc2\xb3l\xd9",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_u64__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_U64: [u8; 48usize] = super::Contract::spec_xdr_test_u64();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_u64() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_u64\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_U64: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\xeaQ\x08\xd6\xb9\xb9\xe62\x87\xb7\x81$\xf6\x8f\x81\x98\xa5\xae\x1c/\xbc\x9c\x8c\xc4\x95\xc15\xbeg\xb4\xc4\t",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_i64__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_I64: [u8; 48usize] = super::Contract::spec_xdr_test_i64();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_i64() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_i64\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x07\0\0\0\x01\0\0\0\x07"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_I64: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\xa4\x97mN\xcb\x1b\x08}\x84\x02o\xa9;4\xf9*\x80\t\xe52\xe4\xe1.\x96\xfaaZ\x15\x1b\x80\x99L",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_u128__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_U128: [u8; 52usize] = super::Contract::spec_xdr_test_u128();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_u128() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_u128\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\n\0\0\0\x01\0\0\0\n"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_U128: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\xb8\xec\xfb\xc6\xf0k\xa6 ie\x91\xe1\x04{q\x94\xf1Q\x1b\xa6\xda\x1a\xae/Co\xcc\x0c\x94\xbe\t\x01",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_i128__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_I128: [u8; 52usize] = super::Contract::spec_xdr_test_i128();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_i128() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_i128\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0b\0\0\0\x01\0\0\0\x0b"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_I128: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"K\xdf\x89\x8c\xc4\xc7\xb8Fz\xcb\xff\xb7^,C\xfe\xf7\xde\xd3K\xa4Y\x03Xq\xa62X5z&\x1b",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_bool__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_BOOL: [u8; 52usize] = super::Contract::spec_xdr_test_bool();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_bool() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_bool\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\x01"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_BOOL: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"]\xb8\xa6\xf2A\xb4\x93\xa4\xfa\x06\x11\xf2\x9e;\x02}\xb8\\T\xef\xd3]\"\x7f\xd4A\xbf\x89\xe2~\xaa\xc9",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_address__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_ADDRESS: [u8; 52usize] = super::Contract::spec_xdr_test_address();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_address() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ctest_address\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x13\0\0\0\x01\0\0\0\x13"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_ADDRESS: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\xe1\xa1\xce]Y\n1X\xf5\xa1\xd0A\xf9\xfa\0,\xa9/2.\xfak\x83.\x9d\xf4\xbe-\xdc\xc8\x15Y",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_bytes__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_BYTES: [u8; 52usize] = super::Contract::spec_xdr_test_bytes();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_bytes() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ntest_bytes\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0e\0\0\0\x01\0\0\0\x0e"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_BYTES: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"5\xdd\x0c4\x83\xcc&m\xd3@{\x1d'\x01\x89b\xc4E\xe0s\x80w\xaa+\xa7ttU\xb8Z\x99|",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_bytes_n__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_BYTES_N: [u8; 60usize] = super::Contract::spec_xdr_test_bytes_n();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_bytes_n() -> [u8; 60usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ctest_bytes_n\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xee\0\0\0 \0\0\0\x01\0\0\x03\xee\0\0\0 "
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_BYTES_N: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"e\xc8ovtK?\xb2ixC\x15b\xb8\xfd\xc2\xd4pf\xc2,\xcd\x1b\xb6\xd5\x97\x16 \xbe\xcf\x81\xdd",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_string__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_STRING: [u8; 52usize] = super::Contract::spec_xdr_test_string();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_string() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_string\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x10\0\0\0\x01\0\0\0\x10"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_STRING: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"G\xfe^\xa9(|>\x01\xc9\xf4\x9f\xfe\xb8\x88\xde8K\xefn\xb7\xd8\xa8\xbb.\xca'\xa9C\xb9\xe2\x0f\x8d",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_symbol__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_SYMBOL: [u8; 52usize] = super::Contract::spec_xdr_test_symbol();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_symbol() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_symbol\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x11\0\0\0\x01\0\0\0\x11"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_SYMBOL: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\xb3\xbb?\xa3\xe95v]E\x90^\x17\x8dzlte\x02\n%\xe8\x93\xd2f\x16\xec\xc1\x12\x0c}(^",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_vec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_VEC: [u8; 56usize] = super::Contract::spec_xdr_test_vec();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_vec() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_vec\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xea\0\0\0\x04\0\0\0\x01\0\0\x03\xea\0\0\0\x04"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_VEC: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"il\x1aVT\xad'\\T\xdf\xbeC\x16\xda\xec\xf7\x93t\xea%'l\x06\x0f\xff\xbd9p_\xf3'\xef",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_map__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_MAP: [u8; 64usize] = super::Contract::spec_xdr_test_map();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_map() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08test_map\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x03\xec\0\0\0\x04\0\0\0\x04\0\0\0\x01\0\0\x03\xec\0\0\0\x04\0\0\0\x04"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_MAP: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\xe0I\xc0\xec\x08W\xe7\x18\xd3\x81E&\xcc!`\xc9\xf7\x81~p\xdd1j\xbc\x1f<\xc2\xdb&+%t",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_duration__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_DURATION: [u8; 56usize] =
        super::Contract::spec_xdr_test_duration();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_duration() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rtest_duration\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\t\0\0\0\x01\0\0\0\t"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_DURATION: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\"g\xca\xf3\x95\xc5f\"\x95\xb0\t\"\x12\x01\xf7\xa7\xa7\xe3\x9a\x9a\x90\xdc\x04\xe1\xfft\xd7\xe2\x01\xad\xeb@",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_timepoint__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_TIMEPOINT: [u8; 56usize] =
        super::Contract::spec_xdr_test_timepoint();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_timepoint() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_timepoint\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x08\0\0\0\x01\0\0\0\x08"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_TIMEPOINT: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\x98S\x95 \xc6\xb1\xfa*\x0f\r\x0b\x1f\x0fu\xe9C\xdc\xc17\x07\xf5\xe7\xe4\xa3\xdd\xe4\xd2\xe2\x01\x19\x07k",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_i256__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_I256: [u8; 52usize] = super::Contract::spec_xdr_test_i256();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_i256() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_i256\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\r\0\0\0\x01\0\0\0\r"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_I256: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\x7f\xef\xe5_|\xbb\xc6\xa9\xbd\xf1HP:jO8U\x8a|E\x03C\xc29\xd1c\xe5\xa1\xf8Q)?",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_u256__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_U256: [u8; 52usize] = super::Contract::spec_xdr_test_u256();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_u256() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\ttest_u256\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\0\x0c"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_U256: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\xe3\xff\x18\xda\xd9V\x04P.\xc3\x0bX\xbd\x95\xb2\03U_\x9a~\xb82\"t\xff\x90=,o\x11\x89",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_env_param__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_ENV_PARAM: [u8; 40usize] =
        super::Contract::spec_xdr_test_env_param();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_env_param() -> [u8; 40usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_env_param\0\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_ENV_PARAM: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\xf5\xeb\xae\xe9\xa1L\xe56?k \x12\x8eT\xf3\xfd\x0f\xbc2\xd6XL\xed-\xa9Y}\xe5L9\x85\x16",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_struct__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_STRUCT: [u8; 76usize] = super::Contract::spec_xdr_test_struct();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_struct() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0btest_struct\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\x08MyStruct\0\0\0\x01\0\0\x07\xd0\0\0\0\x08MyStruct"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_STRUCT: [u8; 106usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<106usize, 2usize>(
        0,
        *b"\xe3J\x02q\xbbS\x01G\xe1\x81W'^@\xb2\x8b8\xb1\xe3\x8c\xce'\xde\x8b\xc8]\xc0\xb7Dcl ",
        [
            <MyStruct as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            <MyStruct as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        ],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_enum_unit__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_ENUM_UNIT: [u8; 88usize] =
        super::Contract::spec_xdr_test_enum_unit();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_enum_unit() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0etest_enum_unit\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\nMyEnumUnit\0\0"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_ENUM_UNIT: [u8; 106usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    106usize,
    2usize,
>(
    0,
    *b"\x12fTH\xb0\xbd\xb3\xfb\x8f\r&\x857\xf4\xbc\x05\x16\xfa\xfc\x0ey\xee\x07\ra\x18\x80\xa1\x1e4\xbe+",
    [
        <MyEnumUnit as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        <MyEnumUnit as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
    ],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__test_enum_variants__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TEST_ENUM_VARIANTS: [u8; 100usize] =
        super::Contract::spec_xdr_test_enum_variants();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_test_enum_variants() -> [u8; 100usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x12test_enum_variants\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01v\0\0\0\0\0\x07\xd0\0\0\0\x0eMyEnumVariants\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0eMyEnumVariants\0\0"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TEST_ENUM_VARIANTS: [u8; 106usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    106usize,
    2usize,
>(
    0,
    *b"\xbf\xb8\x9d\xfbU@\x9d_\n\x9b\xb9<`\x90\xda\x14E\x17\xdd{\xd4<\xc2\xadL\xd7f\xaf\x84\xae\xbfe",
    [
        <MyEnumVariants as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
        <MyEnumVariants as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
    ],
);
impl<'a> ContractClient<'a> {
    /// Test u32 values.
    /// Returns the input unchanged.
    pub fn test_u32(&self, v: &u32) -> u32 {
        use core::ops::Not;
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
    /// Test u32 values.
    /// Returns the input unchanged.
    pub fn try_test_u32(
        &self,
        v: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    /// Test i32 values.
    pub fn test_i32(&self, v: &i32) -> i32 {
        use core::ops::Not;
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
    /// Test i32 values.
    pub fn try_test_i32(
        &self,
        v: &i32,
    ) -> Result<
        Result<i32, <i32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_u64(&self, v: &u64) -> u64 {
        use core::ops::Not;
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
    pub fn try_test_u64(
        &self,
        v: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_i64(&self, v: &i64) -> i64 {
        use core::ops::Not;
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
    pub fn try_test_i64(
        &self,
        v: &i64,
    ) -> Result<
        Result<i64, <i64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_u128(&self, v: &u128) -> u128 {
        use core::ops::Not;
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
    pub fn try_test_u128(
        &self,
        v: &u128,
    ) -> Result<
        Result<u128, <u128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_i128(&self, v: &i128) -> i128 {
        use core::ops::Not;
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
    pub fn try_test_i128(
        &self,
        v: &i128,
    ) -> Result<
        Result<i128, <i128 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_bool(&self, v: &bool) -> bool {
        use core::ops::Not;
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
    pub fn try_test_bool(
        &self,
        v: &bool,
    ) -> Result<
        Result<bool, <bool as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_address(&self, v: &Address) -> Address {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_address(
        &self,
        v: &Address,
    ) -> Result<
        Result<
            Address,
            <Address as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_address") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_bytes(&self, v: &Bytes) -> Bytes {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_bytes(
        &self,
        v: &Bytes,
    ) -> Result<
        Result<
            Bytes,
            <Bytes as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_bytes_n(&self, v: &BytesN<32>) -> BytesN<32> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_bytes_n(
        &self,
        v: &BytesN<32>,
    ) -> Result<
        Result<
            BytesN<32>,
            <BytesN<32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_bytes_n") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_string(&self, v: &String) -> String {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_string(
        &self,
        v: &String,
    ) -> Result<
        Result<
            String,
            <String as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_string") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_symbol(&self, v: &Symbol) -> Symbol {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_symbol(
        &self,
        v: &Symbol,
    ) -> Result<
        Result<
            Symbol,
            <Symbol as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_symbol") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_vec(&self, v: &Vec<u32>) -> Vec<u32> {
        use core::ops::Not;
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
    pub fn try_test_vec(
        &self,
        v: &Vec<u32>,
    ) -> Result<
        Result<
            Vec<u32>,
            <Vec<u32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_map(&self, v: &Map<u32, u32>) -> Map<u32, u32> {
        use core::ops::Not;
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
    pub fn try_test_map(
        &self,
        v: &Map<u32, u32>,
    ) -> Result<
        Result<
            Map<u32, u32>,
            <Map<u32, u32> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_duration(&self, v: &Duration) -> Duration {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_duration(
        &self,
        v: &Duration,
    ) -> Result<
        Result<
            Duration,
            <Duration as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_duration") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_timepoint(&self, v: &Timepoint) -> Timepoint {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_timepoint(
        &self,
        v: &Timepoint,
    ) -> Result<
        Result<
            Timepoint,
            <Timepoint as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_timepoint") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_i256(&self, v: &I256) -> I256 {
        use core::ops::Not;
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
    pub fn try_test_i256(
        &self,
        v: &I256,
    ) -> Result<
        Result<I256, <I256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_u256(&self, v: &U256) -> U256 {
        use core::ops::Not;
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
    pub fn try_test_u256(
        &self,
        v: &U256,
    ) -> Result<
        Result<U256, <U256 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    pub fn test_env_param(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_test_env_param(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_env_param") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn test_struct(&self, v: &MyStruct) -> MyStruct {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_struct(
        &self,
        v: &MyStruct,
    ) -> Result<
        Result<
            MyStruct,
            <MyStruct as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_struct") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_enum_unit(&self, v: &MyEnumUnit) -> MyEnumUnit {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_enum_unit(
        &self,
        v: &MyEnumUnit,
    ) -> Result<
        Result<
            MyEnumUnit,
            <MyEnumUnit as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_unit") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn test_enum_variants(&self, v: &MyEnumVariants) -> MyEnumVariants {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "test_enum_variants") },
            ::soroban_sdk::Vec::from_array(&self.env, [v.into_val(&self.env)]),
        );
        res
    }
    pub fn try_test_enum_variants(
        &self,
        v: &MyEnumVariants,
    ) -> Result<
        Result<
            MyEnumVariants,
            <MyEnumVariants as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u32<'i>(v: &'i u32) -> (&'i u32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i32<'i>(v: &'i i32) -> (&'i i32,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u64<'i>(v: &'i u64) -> (&'i u64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i64<'i>(v: &'i i64) -> (&'i i64,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u128<'i>(v: &'i u128) -> (&'i u128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i128<'i>(v: &'i i128) -> (&'i i128,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bool<'i>(v: &'i bool) -> (&'i bool,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_address<'i>(v: &'i Address) -> (&'i Address,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes<'i>(v: &'i Bytes) -> (&'i Bytes,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_bytes_n<'i>(v: &'i BytesN<32>) -> (&'i BytesN<32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_string<'i>(v: &'i String) -> (&'i String,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_symbol<'i>(v: &'i Symbol) -> (&'i Symbol,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_vec<'i>(v: &'i Vec<u32>) -> (&'i Vec<u32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_map<'i>(v: &'i Map<u32, u32>) -> (&'i Map<u32, u32>,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_duration<'i>(v: &'i Duration) -> (&'i Duration,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_timepoint<'i>(v: &'i Timepoint) -> (&'i Timepoint,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_i256<'i>(v: &'i I256) -> (&'i I256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_u256<'i>(v: &'i U256) -> (&'i U256,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_env_param<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_struct<'i>(v: &'i MyStruct) -> (&'i MyStruct,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_unit<'i>(v: &'i MyEnumUnit) -> (&'i MyEnumUnit,) {
        (v,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn test_enum_variants<'i>(v: &'i MyEnumVariants) -> (&'i MyEnumVariants,) {
        (v,)
    }
}
