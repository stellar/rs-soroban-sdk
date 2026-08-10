#![feature(prelude_import)]
#![no_std]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, contracttrait, Env};
pub struct SelfPathTraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_self_path_trait as SelfPathTrait;
pub trait SelfPathTrait {
    fn self_path_method(env: &Env) -> u32 {
        let _ = env;
        200
    }
}
///SelfPathTraitClient is a client for calling the contract defined in "SelfPathTrait".
pub struct SelfPathTraitClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> SelfPathTraitClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> SelfPathTraitClient<'a> {
    pub fn self_path_method(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "self_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_self_path_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "self_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
///SelfPathTraitArgs is a type for building arg lists for functions defined in "SelfPathTrait".
pub struct SelfPathTraitArgs;
impl SelfPathTraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn self_path_method<'i>() -> () {
        ()
    }
}
impl SelfPathTraitSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_VIEW_self_path_method: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(
                b"self_path_method",
            )),
            inputs: soroban_sdk::xdr::VecMView::new(&[]),
            outputs: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSpecTypeDefView::U32]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_self_path_method(
    ) -> [u8; SelfPathTraitSpec::__SPEC_XDR_VIEW_self_path_method.const_xdr_len()] {
        SelfPathTraitSpec::__SPEC_XDR_VIEW_self_path_method.const_to_xdr()
    }
}
pub struct ContractSelfPath;
///ContractSelfPathArgs is a type for building arg lists for functions defined in "ContractSelfPath".
pub struct ContractSelfPathArgs;
///ContractSelfPathClient is a client for calling the contract defined in "ContractSelfPath".
pub struct ContractSelfPathClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractSelfPathClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl self::SelfPathTrait for ContractSelfPath {}
impl<'a> ContractSelfPathClient<'a> {}
impl ContractSelfPathArgs {}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractSelfPathClient::new(&env, &contract_id).self_path_method` instead"
)]
#[allow(deprecated)]
pub fn __ContractSelfPath__self_path_method__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractSelfPath as self::SelfPathTrait>::self_path_method(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractSelfPathClient::new(&env, &contract_id).self_path_method` instead"
)]
#[export_name = "self_path_method"]
pub extern "C" fn __ContractSelfPath__self_path_method__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractSelfPath__self_path_method__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractSelfPath__self_path_method__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_SELF_PATH_METHOD: [u8;
        super::ContractSelfPath::__SPEC_XDR_VIEW_self_path_method.const_xdr_len()] =
        super::ContractSelfPath::spec_xdr_self_path_method();
}
impl ContractSelfPath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_VIEW_self_path_method: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::new(b""),
            name: soroban_sdk::xdr::ScSymbolView(soroban_sdk::xdr::StringMView::new(
                b"self_path_method",
            )),
            inputs: soroban_sdk::xdr::VecMView::new(&[]),
            outputs: soroban_sdk::xdr::VecMView::new(&[soroban_sdk::xdr::ScSpecTypeDefView::U32]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_self_path_method(
    ) -> [u8; ContractSelfPath::__SPEC_XDR_VIEW_self_path_method.const_xdr_len()] {
        ContractSelfPath::__SPEC_XDR_VIEW_self_path_method.const_to_xdr()
    }
}
impl<'a> ContractSelfPathClient<'a> {
    pub fn self_path_method(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "self_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_self_path_method(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "self_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractSelfPathArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn self_path_method<'i>() -> () {
        ()
    }
}
