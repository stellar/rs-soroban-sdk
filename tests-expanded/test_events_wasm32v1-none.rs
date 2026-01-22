#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, MuxedAddress};
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
pub struct Transfer {
    from: Address,
    to: Address,
    amount: i128,
    to_muxed_id: Option<u64>,
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_EVENT_TRANSFER: [u8; 144usize] = Transfer::spec_xdr();
impl Transfer {
    pub const fn spec_xdr() -> [u8; 144usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x08Transfer\0\0\0\x01\0\0\0\x08transfer\0\0\0\x04\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\0\0\0\0\x0bto_muxed_id\0\0\0\x03\xe8\0\0\0\x06\0\0\0\0\0\0\0\x02"
    }
}
impl soroban_sdk::IncludeSpecMarker for Transfer {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <i128 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <Option<u64> as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 12usize] = *b"SpEc;\xc1i\xa0H>\x8d\xf1";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::Event for Transfer {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            },
            {
                let v: soroban_sdk::Val = self.to.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 2usize] = ["amount", "to_muxed_id"];
        let vals: [soroban_sdk::Val; 2usize] =
            [self.amount.into_val(env), self.to_muxed_id.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl Transfer {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
impl Contract {
    pub fn transfer(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from: from.clone(),
            to: to.address(),
            amount,
            to_muxed_id: to.id(),
        }
        .publish(&env);
    }
    pub fn failed_transfer(env: Env, from: Address, to: Address, amount: i128) {
        Transfer {
            from: from.clone(),
            to: to.clone(),
            amount,
            to_muxed_id: None,
        }
        .publish(&env);
        {
            ::core::panicking::panic_fmt(format_args!("fail"));
        };
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TRANSFER: [u8; 80usize] = super::Contract::spec_xdr_transfer();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_transfer() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08transfer\0\0\0\x03\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x14\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__failed_transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FAILED_TRANSFER: [u8; 88usize] =
        super::Contract::spec_xdr_failed_transfer();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_failed_transfer() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ffailed_transfer\0\0\0\0\x03\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x13\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn transfer(&self, from: &Address, to: impl Into<MuxedAddress>, amount: &i128) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into().into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_transfer(
        &self,
        from: &Address,
        to: impl Into<MuxedAddress>,
        amount: &i128,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into().into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn failed_transfer(&self, from: &Address, to: &Address, amount: &i128) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "failed_transfer") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_failed_transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: &i128,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "failed_transfer") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn transfer<'i>(
        from: &'i Address,
        to: &'i MuxedAddress,
        amount: &'i i128,
    ) -> (&'i Address, &'i MuxedAddress, &'i i128) {
        (from, to, amount)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn failed_transfer<'i>(
        from: &'i Address,
        to: &'i Address,
        amount: &'i i128,
    ) -> (&'i Address, &'i Address, &'i i128) {
        (from, to, amount)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__transfer {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::transfer(
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
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_2),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
    #[export_name = "transfer"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__failed_transfer {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::failed_transfer(
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
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_2),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
    #[export_name = "failed_transfer"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
