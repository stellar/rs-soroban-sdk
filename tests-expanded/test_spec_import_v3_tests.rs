#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use test_spec_lib::{EnumA, EnumIntA, ErrorA, EventA, EventD, StructA, StructTupleA};
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
    pub fn fn_struct_a(f1: u32, f2: bool) -> StructA {
        StructA { f1, f2 }
    }
    pub fn fn_struct_tuple_a(f1: i64, f2: i64) -> StructTupleA {
        StructTupleA(f1, f2)
    }
    pub fn fn_enum_a() -> EnumA {
        EnumA::V2
    }
    pub fn fn_enum_int_a() -> EnumIntA {
        EnumIntA::V3
    }
    pub fn fn_error_a(input: u32) -> Result<u32, ErrorA> {
        if input == 0 {
            Err(ErrorA::E2)
        } else {
            Ok(input)
        }
    }
    pub fn fn_event_a(env: Env, f1: Address, f2: String) {
        EventA { f1, f2 }.publish(&env);
    }
    pub fn fn_event_d(env: Env) {
        EventD.publish(&env);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_struct_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_STRUCT_A: [u8; super::Contract::spec_xdr_fn_struct_a().len()] =
        super::Contract::spec_xdr_fn_struct_a();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_struct_a: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"fn_struct_a"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"f1"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"f2"),
                        type_: <bool as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <StructA as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_fn_struct_a(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_fn_struct_a.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_fn_struct_a.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_struct_tuple_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_STRUCT_TUPLE_A: [u8; super::Contract::spec_xdr_fn_struct_tuple_a()
        .len()] = super::Contract::spec_xdr_fn_struct_tuple_a();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_struct_tuple_a: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"fn_struct_tuple_a",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"f1"),
                        type_: <i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"f2"),
                        type_: <i64 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <StructTupleA as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_fn_struct_tuple_a(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_fn_struct_tuple_a.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_fn_struct_tuple_a.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_enum_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_ENUM_A: [u8; super::Contract::spec_xdr_fn_enum_a().len()] =
        super::Contract::spec_xdr_fn_enum_a();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_enum_a: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"fn_enum_a"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <EnumA as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_fn_enum_a() -> [u8; Contract::__SPEC_XDR_ENTRY_fn_enum_a.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_fn_enum_a.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_enum_int_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_ENUM_INT_A: [u8; super::Contract::spec_xdr_fn_enum_int_a().len()] =
        super::Contract::spec_xdr_fn_enum_int_a();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_enum_int_a: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"fn_enum_int_a"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <EnumIntA as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_fn_enum_int_a(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_fn_enum_int_a.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_fn_enum_int_a.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_error_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_ERROR_A: [u8; super::Contract::spec_xdr_fn_error_a().len()] =
        super::Contract::spec_xdr_fn_error_a();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_error_a: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"fn_error_a"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"input"),
                        type_: <u32 as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    <Result<u32, ErrorA> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
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
    pub const fn spec_xdr_fn_error_a() -> [u8; Contract::__SPEC_XDR_ENTRY_fn_error_a.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_fn_error_a.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_event_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_EVENT_A: [u8; super::Contract::spec_xdr_fn_event_a().len()] =
        super::Contract::spec_xdr_fn_event_a();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_event_a: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"fn_event_a"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"f1"),
                        type_: <Address as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"f2"),
                        type_: <String as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
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
    pub const fn spec_xdr_fn_event_a() -> [u8; Contract::__SPEC_XDR_ENTRY_fn_event_a.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_fn_event_a.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__fn_event_d__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_FN_EVENT_D: [u8; super::Contract::spec_xdr_fn_event_d().len()] =
        super::Contract::spec_xdr_fn_event_d();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fn_event_d: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"fn_event_d"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
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
    pub const fn spec_xdr_fn_event_d() -> [u8; Contract::__SPEC_XDR_ENTRY_fn_event_d.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_fn_event_d.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    /// Call `fn_struct_a` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_struct_a`] to receive errors instead of panicking.
    pub fn fn_struct_a(
        &self,
        f1: impl soroban_sdk::ClientArg<u32, <u32 as soroban_sdk::ClientType>::Arg>,
        f2: impl soroban_sdk::ClientArg<bool, <bool as soroban_sdk::ClientType>::Arg>,
    ) -> <StructA as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::ClientArg::<u32, _>::into_val_for_client(f1, &self.env),
                    soroban_sdk::ClientArg::<bool, _>::into_val_for_client(f2, &self.env),
                ],
            ),
        );
        res
    }
    /// Call `fn_struct_a` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_struct_a(
        &self,
        f1: impl soroban_sdk::ClientArg<u32, <u32 as soroban_sdk::ClientType>::Arg>,
        f2: impl soroban_sdk::ClientArg<bool, <bool as soroban_sdk::ClientType>::Arg>,
    ) -> Result<
        Result<
            <StructA as soroban_sdk::ClientOutput>::Ok,
            <<StructA as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<StructA as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::ClientArg::<u32, _>::into_val_for_client(f1, &self.env),
                    soroban_sdk::ClientArg::<bool, _>::into_val_for_client(f2, &self.env),
                ],
            ),
        );
        res
    }
    /// Call `fn_struct_tuple_a` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_struct_tuple_a`] to receive errors instead of panicking.
    pub fn fn_struct_tuple_a(
        &self,
        f1: impl soroban_sdk::ClientArg<i64, <i64 as soroban_sdk::ClientType>::Arg>,
        f2: impl soroban_sdk::ClientArg<i64, <i64 as soroban_sdk::ClientType>::Arg>,
    ) -> <StructTupleA as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::ClientArg::<i64, _>::into_val_for_client(f1, &self.env),
                    soroban_sdk::ClientArg::<i64, _>::into_val_for_client(f2, &self.env),
                ],
            ),
        );
        res
    }
    /// Call `fn_struct_tuple_a` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_struct_tuple_a(
        &self,
        f1: impl soroban_sdk::ClientArg<i64, <i64 as soroban_sdk::ClientType>::Arg>,
        f2: impl soroban_sdk::ClientArg<i64, <i64 as soroban_sdk::ClientType>::Arg>,
    ) -> Result<
        Result<
            <StructTupleA as soroban_sdk::ClientOutput>::Ok,
            <<StructTupleA as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<StructTupleA as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::ClientArg::<i64, _>::into_val_for_client(f1, &self.env),
                    soroban_sdk::ClientArg::<i64, _>::into_val_for_client(f2, &self.env),
                ],
            ),
        );
        res
    }
    /// Call `fn_enum_a` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_enum_a`] to receive errors instead of panicking.
    pub fn fn_enum_a(&self) -> <EnumA as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn_enum_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `fn_enum_a` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_enum_a(
        &self,
    ) -> Result<
        Result<
            <EnumA as soroban_sdk::ClientOutput>::Ok,
            <<EnumA as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<EnumA as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn_enum_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `fn_enum_int_a` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_enum_int_a`] to receive errors instead of panicking.
    pub fn fn_enum_int_a(&self) -> <EnumIntA as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `fn_enum_int_a` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_enum_int_a(
        &self,
    ) -> Result<
        Result<
            <EnumIntA as soroban_sdk::ClientOutput>::Ok,
            <<EnumIntA as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<EnumIntA as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `fn_error_a` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_error_a`] to receive errors instead of panicking.
    pub fn fn_error_a(
        &self,
        input: impl soroban_sdk::ClientArg<u32, <u32 as soroban_sdk::ClientType>::Arg>,
    ) -> <Result<u32, ErrorA> as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_error_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::ClientArg::<u32, _>::into_val_for_client(
                    input, &self.env,
                )],
            ),
        );
        res
    }
    /// Call `fn_error_a` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_error_a(
        &self,
        input: impl soroban_sdk::ClientArg<u32, <u32 as soroban_sdk::ClientType>::Arg>,
    ) -> Result<
        Result<
            <Result<u32, ErrorA> as soroban_sdk::ClientOutput>::Ok,
            <<Result<u32, ErrorA> as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<Result<u32, ErrorA> as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_error_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [soroban_sdk::ClientArg::<u32, _>::into_val_for_client(
                    input, &self.env,
                )],
            ),
        );
        res
    }
    /// Call `fn_event_a` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_event_a`] to receive errors instead of panicking.
    pub fn fn_event_a(
        &self,
        f1: impl soroban_sdk::ClientArg<Address, <Address as soroban_sdk::ClientType>::Arg>,
        f2: impl soroban_sdk::ClientArg<String, <String as soroban_sdk::ClientType>::Arg>,
    ) -> <() as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::ClientArg::<Address, _>::into_val_for_client(f1, &self.env),
                    soroban_sdk::ClientArg::<String, _>::into_val_for_client(f2, &self.env),
                ],
            ),
        );
        res
    }
    /// Call `fn_event_a` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_event_a(
        &self,
        f1: impl soroban_sdk::ClientArg<Address, <Address as soroban_sdk::ClientType>::Arg>,
        f2: impl soroban_sdk::ClientArg<String, <String as soroban_sdk::ClientType>::Arg>,
    ) -> Result<
        Result<
            <() as soroban_sdk::ClientOutput>::Ok,
            <<() as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<() as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    soroban_sdk::ClientArg::<Address, _>::into_val_for_client(f1, &self.env),
                    soroban_sdk::ClientArg::<String, _>::into_val_for_client(f2, &self.env),
                ],
            ),
        );
        res
    }
    /// Call `fn_event_d` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_fn_event_d`] to receive errors instead of panicking.
    pub fn fn_event_d(&self) -> <() as soroban_sdk::ClientOutput>::Ok {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_event_d") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    /// Call `fn_event_d` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_fn_event_d(
        &self,
    ) -> Result<
        Result<
            <() as soroban_sdk::ClientOutput>::Ok,
            <<() as soroban_sdk::ClientOutput>::Ok as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<<() as soroban_sdk::ClientOutput>::Err, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fn_event_d") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_struct_a<'i>(f1: &'i u32, f2: &'i bool) -> (&'i u32, &'i bool) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_struct_tuple_a<'i>(f1: &'i i64, f2: &'i i64) -> (&'i i64, &'i i64) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_enum_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_enum_int_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_error_a<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_event_a<'i>(f1: &'i Address, f2: &'i String) -> (&'i Address, &'i String) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn_event_d<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_struct_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_struct_a(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_a` instead")]
pub extern "C" fn __Contract__fn_struct_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_struct_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_tuple_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_struct_tuple_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_struct_tuple_a(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_struct_tuple_a` instead")]
pub extern "C" fn __Contract__fn_struct_tuple_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_struct_tuple_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_enum_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::fn_enum_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_a` instead")]
pub extern "C" fn __Contract__fn_enum_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_enum_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_int_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_enum_int_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::fn_enum_int_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_enum_int_a` instead")]
pub extern "C" fn __Contract__fn_enum_int_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_enum_int_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_error_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_error_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_error_a(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_error_a` instead")]
pub extern "C" fn __Contract__fn_error_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_error_a__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_event_a` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_event_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_event_a(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_event_a` instead")]
pub extern "C" fn __Contract__fn_event_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_event_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_event_d` instead")]
#[allow(deprecated)]
pub fn __Contract__fn_event_d__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fn_event_d(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fn_event_d` instead")]
pub extern "C" fn __Contract__fn_event_d__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fn_event_d__invoke_raw(soroban_sdk::Env::default())
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
