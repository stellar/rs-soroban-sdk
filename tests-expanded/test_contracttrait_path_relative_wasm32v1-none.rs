#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
pub mod traits {
    use soroban_sdk::{contracttrait, Env};
    pub struct RelativePathTraitSpec;
    /// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
    pub use __contractimpl_for_relative_path_trait as RelativePathTrait;
    #[doc(hidden)]
    #[allow(non_camel_case_types)]
    pub trait __RelativePathTraitFnTypes {
        type __relative_path_method__arg_0;
        type __relative_path_method__ret;
    }
    impl<T: RelativePathTrait + ?Sized> __RelativePathTraitFnTypes for T {
        type __relative_path_method__arg_0 = Env;
        type __relative_path_method__ret = u32;
    }
    pub trait RelativePathTrait: __RelativePathTraitFnTypes {
        fn relative_path_method(env: &Env) -> u32 {
            let _ = env;
            400
        }
    }
    ///RelativePathTraitClient is a client for calling the contract defined in "RelativePathTrait".
    pub struct RelativePathTraitClient<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> RelativePathTraitClient<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> RelativePathTraitClient<'a> {
        /// Call `relative_path_method` function on the contract.
        ///
        /// # Panics
        ///
        /// Panics if the invoked contract returns an error or panics.
        ///
        /// Use [`Self::try_relative_path_method`] to receive errors instead of panicking.
        pub fn relative_path_method(&self) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        /// Call `relative_path_method` function on the contract.
        ///
        /// # Errors
        ///
        /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
        pub fn try_relative_path_method(
            &self,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
    }
    ///RelativePathTraitArgs is a type for building arg lists for functions defined in "RelativePathTrait".
    pub struct RelativePathTraitArgs;
    impl RelativePathTraitArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn relative_path_method<'i>() -> () {
            ()
        }
    }
    impl RelativePathTraitSpec {
        #[allow(non_upper_case_globals)]
        const __SPEC_XDR_ENTRY_relative_path_method: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
                soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::ScSymbol(
                        soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"relative_path_method",
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
        pub const fn spec_xdr_relative_path_method(
        ) -> [u8; RelativePathTraitSpec::__SPEC_XDR_ENTRY_relative_path_method.const_xdr_len()]
        {
            const { RelativePathTraitSpec::__SPEC_XDR_ENTRY_relative_path_method.const_to_xdr() }
        }
    }
}
pub struct ContractRelativePath;
///ContractRelativePathArgs is a type for building arg lists for functions defined in "ContractRelativePath".
pub struct ContractRelativePathArgs;
///ContractRelativePathClient is a client for calling the contract defined in "ContractRelativePath".
pub struct ContractRelativePathClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> ContractRelativePathClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl traits::RelativePathTrait for ContractRelativePath {}
impl<'a> ContractRelativePathClient<'a> {}
impl ContractRelativePathArgs {}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __ContractRelativePath__traits_RelativePathTrait__relative_path_method__arg_0 {
    type Env;
}
impl<T: traits::RelativePathTrait + ?Sized>
    __ContractRelativePath__traits_RelativePathTrait__relative_path_method__arg_0 for T
{
    type Env = T::__relative_path_method__arg_0;
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait __ContractRelativePath__traits_RelativePathTrait__relative_path_method__ret {
    type u32;
}
impl<T: traits::RelativePathTrait + ?Sized>
    __ContractRelativePath__traits_RelativePathTrait__relative_path_method__ret for T
{
    type u32 = T::__relative_path_method__ret;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractRelativePathClient::new(&env, &contract_id).relative_path_method` instead"
)]
#[allow(deprecated)]
pub fn __ContractRelativePath__relative_path_method__invoke_raw(
    env: soroban_sdk::Env,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <ContractRelativePath as traits::RelativePathTrait>::relative_path_method(&env),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(
    note = "use `ContractRelativePathClient::new(&env, &contract_id).relative_path_method` instead"
)]
#[export_name = "relative_path_method"]
pub extern "C" fn __ContractRelativePath__relative_path_method__invoke_raw_extern(
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __ContractRelativePath__relative_path_method__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __ContractRelativePath__relative_path_method__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_RELATIVE_PATH_METHOD: [u8;
        super::ContractRelativePath::spec_xdr_relative_path_method().len()] =
        super::ContractRelativePath::spec_xdr_relative_path_method();
}
impl ContractRelativePath {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_relative_path_method: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
        name: soroban_sdk::xdr::r#const::ScSymbol(
            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"relative_path_method",
            ),
        ),
        inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
        outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
            &[
                <<ContractRelativePath as __ContractRelativePath__traits_RelativePathTrait__relative_path_method__ret>::u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_relative_path_method(
    ) -> [u8; ContractRelativePath::__SPEC_XDR_ENTRY_relative_path_method.const_xdr_len()] {
        const { ContractRelativePath::__SPEC_XDR_ENTRY_relative_path_method.const_to_xdr() }
    }
}
impl<'a> ContractRelativePathClient<'a> {
    /// Call `relative_path_method` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_relative_path_method`] to receive errors instead of panicking.
    pub fn relative_path_method(
        &self,
    ) -> <ContractRelativePath as __ContractRelativePath__traits_RelativePathTrait__relative_path_method__ret>::u32{
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `relative_path_method` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_relative_path_method(
        &self,
    ) -> Result<
        Result<
            <ContractRelativePath as __ContractRelativePath__traits_RelativePathTrait__relative_path_method__ret>::u32,
            <<ContractRelativePath as __ContractRelativePath__traits_RelativePathTrait__relative_path_method__ret>::u32 as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "relative_path_method") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractRelativePathArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn relative_path_method<'i>() -> () {
        ()
    }
}
