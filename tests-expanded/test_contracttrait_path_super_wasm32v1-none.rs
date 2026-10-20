#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contracttrait, Env};
pub struct SuperPathTraitSpec;
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __SuperPathTrait_Env {
    type __SuperPathTrait_Env;
}
impl<T: ?Sized> __SuperPathTrait_Env for T {
    type __SuperPathTrait_Env = Env;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __SuperPathTrait_u32 {
    type __SuperPathTrait_u32;
}
impl<T: ?Sized> __SuperPathTrait_u32 for T {
    type __SuperPathTrait_u32 = u32;
}
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_super_path_trait as SuperPathTrait;
pub trait SuperPathTrait: __SuperPathTrait_Env + __SuperPathTrait_u32 {
    fn super_path_method(env: &Env) -> u32 {
        let _ = env;
        300
    }
}
///SuperPathTraitClient is a client for calling the contract defined in "SuperPathTrait".
pub struct SuperPathTraitClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> SuperPathTraitClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> SuperPathTraitClient<'a> {
    /// Call `super_path_method` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_super_path_method`] to receive errors instead of panicking.
    pub fn super_path_method(&self) -> <u32 as soroban_sdk::client::Output>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `super_path_method` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_super_path_method(
        &self,
    ) -> Result<
        Result<
            <u32 as soroban_sdk::client::Output>::Ok,
            <<u32 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<u32 as soroban_sdk::client::Output>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
///SuperPathTraitArgs is a type for building arg lists for functions defined in "SuperPathTrait".
pub struct SuperPathTraitArgs;
impl SuperPathTraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn super_path_method<'i>() -> () {
        ()
    }
}
impl SuperPathTraitSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_super_path_method: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"super_path_method",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_super_path_method(
    ) -> [u8; SuperPathTraitSpec::__SPEC_XDR_ENTRY_super_path_method.const_xdr_len()] {
        const { SuperPathTraitSpec::__SPEC_XDR_ENTRY_super_path_method.const_to_xdr() }
    }
}
pub mod submodule {
    use soroban_sdk::{contract, contractimpl};
    pub struct ContractSuperPath;
    ///ContractSuperPathArgs is a type for building arg lists for functions defined in "ContractSuperPath".
    pub struct ContractSuperPathArgs;
    ///ContractSuperPathClient is a client for calling the contract defined in "ContractSuperPath".
    pub struct ContractSuperPathClient<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> ContractSuperPathClient<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl super::SuperPathTrait for ContractSuperPath {}
    impl<'a> ContractSuperPathClient<'a> {}
    impl ContractSuperPathArgs {}
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __ContractSuperPath_SuperPathTrait_types {
        type Env;
        type u32;
    }
    impl<T: super::SuperPathTrait> __ContractSuperPath_SuperPathTrait_types for T {
        type Env = T::__SuperPathTrait_Env;
        type u32 = T::__SuperPathTrait_u32;
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(
        note = "use `ContractSuperPathClient::new(&env, &contract_id).super_path_method` instead"
    )]
    #[allow(deprecated)]
    pub fn __ContractSuperPath__super_path_method__invoke_raw(
        env: soroban_sdk::Env,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <ContractSuperPath as super::SuperPathTrait>::super_path_method(&env),
            &env,
        )
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(
        note = "use `ContractSuperPathClient::new(&env, &contract_id).super_path_method` instead"
    )]
    #[export_name = "super_path_method"]
    pub extern "C" fn __ContractSuperPath__super_path_method__invoke_raw_extern() -> soroban_sdk::Val
    {
        #[allow(deprecated)]
        __ContractSuperPath__super_path_method__invoke_raw(soroban_sdk::Env::default())
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    mod __ContractSuperPath__super_path_method__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #[allow(dead_code)]
        #[link_section = "contractspecv0"]
        static __SPEC_XDR_FN_SUPER_PATH_METHOD: [u8;
            super::ContractSuperPath::spec_xdr_super_path_method().len()] =
            super::ContractSuperPath::spec_xdr_super_path_method();
    }
    impl ContractSuperPath {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_super_path_method: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::ScSymbol(
                soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                    b"super_path_method",
                ),
            ),
            inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    <<ContractSuperPath as __ContractSuperPath_SuperPathTrait_types>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
        pub const fn spec_xdr_super_path_method(
        ) -> [u8; ContractSuperPath::__SPEC_XDR_ENTRY_super_path_method.const_xdr_len()] {
            const { ContractSuperPath::__SPEC_XDR_ENTRY_super_path_method.const_to_xdr() }
        }
    }
    impl<'a> ContractSuperPathClient<'a> {
        /// Call `super_path_method` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_super_path_method`] to receive errors instead of panicking.
        pub fn super_path_method(
            &self,
        ) -> <<ContractSuperPath as __ContractSuperPath_SuperPathTrait_types>::u32 as soroban_sdk::client::Output>::Ok{
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        /// Call `super_path_method` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_super_path_method(
            &self,
        ) -> Result<
            Result<
                <<ContractSuperPath as __ContractSuperPath_SuperPathTrait_types>::u32 as soroban_sdk::client::Output>::Ok,
                <<<ContractSuperPath as __ContractSuperPath_SuperPathTrait_types>::u32 as soroban_sdk::client::Output>::Ok as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::Error,
            >,
            Result<
                <<ContractSuperPath as __ContractSuperPath_SuperPathTrait_types>::u32 as soroban_sdk::client::Output>::Err,
                soroban_sdk::InvokeError,
            >,
        >{
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "super_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
    }
    impl ContractSuperPathArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn super_path_method<'i>() -> () {
            ()
        }
    }
}
