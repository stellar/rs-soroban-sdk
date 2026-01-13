#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, contracttype, Env};
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
pub enum DataKey {
    Persistent(u32),
    Temp(u32),
    Instance(u32),
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_DATAKEY: [u8; 112usize] = DataKey::spec_xdr();
impl DataKey {
    pub const fn spec_xdr() -> [u8; 112usize] {
        *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07DataKey\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\nPersistent\0\0\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x04Temp\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\0\0\0\0\0\x08Instance\0\0\0\x01\0\0\0\x04"
    }
}
impl soroban_sdk::IncludeSpecMarker for DataKey {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <u32 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <u32 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <u32 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc\x14\x94}~\xec\x15\x94\x84";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for DataKey {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
        const CASES: &'static [&'static str] = &["Persistent", "Temp", "Instance"];
        let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
        let mut iter = vec.try_iter();
        let discriminant: soroban_sdk::Symbol = iter
            .next()
            .ok_or(soroban_sdk::ConversionError)??
            .try_into_val(env)
            .map_err(|_| soroban_sdk::ConversionError)?;
        Ok(
            match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?) as usize
            {
                0 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Persistent(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                1 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Temp(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                2 => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::ConversionError);
                    }
                    Self::Instance(
                        iter.next()
                            .ok_or(soroban_sdk::ConversionError)??
                            .try_into_val(env)?,
                    )
                }
                _ => Err(soroban_sdk::ConversionError {})?,
            },
        )
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, DataKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &DataKey,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{TryFromVal, TryIntoVal};
        match val {
            DataKey::Persistent(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Persistent")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            DataKey::Temp(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Temp")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
            DataKey::Instance(ref value0) => {
                let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                    soroban_sdk::Symbol::try_from_val(env, &"Instance")?.to_val(),
                    value0.try_into_val(env)?,
                );
                tup.try_into_val(env).map_err(Into::into)
            }
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &DataKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&DataKey,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, DataKey>>::try_from_val(env, *val)
    }
}
impl Contract {
    pub fn __constructor(env: Env, init_key: u32, init_value: i64) {
        env.storage()
            .persistent()
            .set(&DataKey::Persistent(init_key), &init_value);
        env.storage()
            .temporary()
            .set(&DataKey::Temp(init_key * 2), &(init_value * 2));
        env.storage()
            .instance()
            .set(&DataKey::Instance(init_key * 3), &(init_value * 3));
    }
    pub fn get_data(env: Env, key: DataKey) -> Option<i64> {
        match key {
            DataKey::Persistent(_) => env.storage().persistent().get(&key),
            DataKey::Temp(_) => env.storage().temporary().get(&key),
            DataKey::Instance(_) => env.storage().instance().get(&key),
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract____constructor__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN___CONSTRUCTOR: [u8; 80usize] =
        super::Contract::spec_xdr___constructor();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr___constructor() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\r__constructor\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x08init_key\0\0\0\x04\0\0\0\0\0\0\0\ninit_value\0\0\0\0\0\x07\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_data__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_DATA: [u8; 64usize] = super::Contract::spec_xdr_get_data();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_data() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08get_data\0\0\0\x01\0\0\0\0\0\0\0\x03key\0\0\0\x07\xd0\0\0\0\x07DataKey\0\0\0\0\x01\0\0\x03\xe8\0\0\0\x07"
    }
}
impl<'a> ContractClient<'a> {
    pub fn get_data(&self, key: &DataKey) -> Option<i64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_data");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [key.into_val(&self.env)]),
        );
        res
    }
    pub fn try_get_data(
        &self,
        key: &DataKey,
    ) -> Result<
        Result<
            Option<i64>,
            <Option<i64> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_data");
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
    pub fn __constructor<'i>(init_key: &'i u32, init_value: &'i i64) -> (&'i u32, &'i i64) {
        (init_key, init_value)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_data<'i>(key: &'i DataKey) -> (&'i DataKey,) {
        (key,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract____constructor {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::__constructor(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__constructor` instead")]
    #[export_name = "__constructor"]
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
pub mod __Contract__get_data {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_data` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::get_data(
                env.clone(),
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_data` instead")]
    #[export_name = "get_data"]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
