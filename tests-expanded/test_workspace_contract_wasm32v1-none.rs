#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
use test_workspace_lib::Value;
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
impl Contract {
    pub fn value() -> Value {
        return Value { value: 13 };
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__value__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_VALUE: [u8; super::Contract::spec_xdr_value().len()] =
        super::Contract::spec_xdr_value();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_value: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"value"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Value as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_value() -> [u8; Contract::__SPEC_XDR_ENTRY_value.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_value.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    /// Call `value` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_value`] to receive errors instead of panicking.
    pub fn value(&self) -> <Value as soroban_sdk::client::Output>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("value");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `value` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_value(
        &self,
    ) -> Result<
        Result<
            <Value as soroban_sdk::client::Output>::Ok,
            <<Value as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<Value as soroban_sdk::client::Output>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("value");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn value<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).value` instead")]
#[allow(deprecated)]
pub fn __Contract__value__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::value(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).value` instead")]
#[export_name = "value"]
pub extern "C" fn __Contract__value__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__value__invoke_raw(soroban_sdk::Env::default())
}
