#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Error, Vec};
pub trait AssociatedType {
    type Val;
    type ValVal;
    fn set_val(env: Env, input: Self::Val);
    fn get_val(env: Env) -> Self::Val;
    fn both(input: Self::Val) -> Self::Val;
    fn wrapped(input: Vec<Self::Val>) -> Result<Self::Val, Error>;
    fn double_wrapped(input: Option<Vec<Self::Val>>) -> Result<Vec<Self::Val>, Error>;
    fn valval(input: Self::ValVal) -> Option<Self::ValVal>;
    fn tuple(input1: Self::Val) -> (Self::Val, Self::ValVal);
    fn valref(input: &Self::Val) -> Self::Val;
}
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
impl AssociatedType for Contract {
    type Val = u64;
    type ValVal = Self::Val;
    fn set_val(env: Env, input: Self::Val) {
        env.storage().instance().set(
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("val");
                SYMBOL
            },
            &input,
        );
    }
    fn get_val(env: Env) -> Self::Val {
        env.storage()
            .instance()
            .get(&{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("val");
                SYMBOL
            })
            .unwrap()
    }
    fn both(input: Self::Val) -> Self::Val {
        input + 1
    }
    fn wrapped(input: Vec<Self::Val>) -> Result<Self::Val, Error> {
        if input.is_empty() {
            Err(Error::from_contract_error(0))
        } else {
            let mut sum = 0;
            for val in input {
                sum += val;
            }
            Ok(sum)
        }
    }
    fn double_wrapped(input: Option<Vec<Self::Val>>) -> Result<Vec<Self::Val>, Error> {
        match input {
            Some(v) => Ok(v),
            None => Err(Error::from_contract_error(1)),
        }
    }
    fn valval(input: Self::ValVal) -> Option<Self::ValVal> {
        Some(input)
    }
    fn tuple(input: Self::Val) -> (Self::Val, Self::Val) {
        (input, input)
    }
    fn valref(input: &Self::Val) -> Self::Val {
        input.clone()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__set_val__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_SET_VAL: [u8; super::Contract::spec_xdr_len_set_val()] =
        super::Contract::spec_xdr_set_val();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_set_val: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"set_val"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U64,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_set_val() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_set_val.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_set_val() -> [u8; Contract::spec_xdr_len_set_val()] {
        const { Contract::__SPEC_XDR_ENTRY_set_val.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_val__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_GET_VAL: [u8; super::Contract::spec_xdr_len_get_val()] =
        super::Contract::spec_xdr_get_val();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_get_val: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"get_val"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::U64,
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_get_val() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_get_val.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_val() -> [u8; Contract::spec_xdr_len_get_val()] {
        const { Contract::__SPEC_XDR_ENTRY_get_val.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__both__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_BOTH: [u8; super::Contract::spec_xdr_len_both()] =
        super::Contract::spec_xdr_both();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_both: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"both"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U64,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::U64,
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_both() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_both.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_both() -> [u8; Contract::spec_xdr_len_both()] {
        const { Contract::__SPEC_XDR_ENTRY_both.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrapped__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_WRAPPED: [u8; super::Contract::spec_xdr_len_wrapped()] =
        super::Contract::spec_xdr_wrapped();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_wrapped: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"wrapped"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::U64,
                        },
                    ),
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Result(
                    &soroban_sdk::xdr::ScSpecTypeResultView {
                        ok_type: &soroban_sdk::xdr::ScSpecTypeDefView::U64,
                        error_type: &soroban_sdk::xdr::ScSpecTypeDefView::Error,
                    },
                ),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_wrapped() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_wrapped.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrapped() -> [u8; Contract::spec_xdr_len_wrapped()] {
        const { Contract::__SPEC_XDR_ENTRY_wrapped.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__double_wrapped__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DOUBLE_WRAPPED: [u8; super::Contract::spec_xdr_len_double_wrapped()] =
        super::Contract::spec_xdr_double_wrapped();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_double_wrapped: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"double_wrapped"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Option(
                        &soroban_sdk::xdr::ScSpecTypeOptionView {
                            value_type: &soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                                &soroban_sdk::xdr::ScSpecTypeVecView {
                                    element_type: &soroban_sdk::xdr::ScSpecTypeDefView::U64,
                                },
                            ),
                        },
                    ),
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Result(
                    &soroban_sdk::xdr::ScSpecTypeResultView {
                        ok_type: &soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                            &soroban_sdk::xdr::ScSpecTypeVecView {
                                element_type: &soroban_sdk::xdr::ScSpecTypeDefView::U64,
                            },
                        ),
                        error_type: &soroban_sdk::xdr::ScSpecTypeDefView::Error,
                    },
                ),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_double_wrapped() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_double_wrapped.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_double_wrapped() -> [u8; Contract::spec_xdr_len_double_wrapped()] {
        const { Contract::__SPEC_XDR_ENTRY_double_wrapped.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__valval__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VALVAL: [u8; super::Contract::spec_xdr_len_valval()] =
        super::Contract::spec_xdr_valval();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_valval: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"valval"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U64,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Option(
                    &soroban_sdk::xdr::ScSpecTypeOptionView {
                        value_type: &soroban_sdk::xdr::ScSpecTypeDefView::U64,
                    },
                ),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_valval() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_valval.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_valval() -> [u8; Contract::spec_xdr_len_valval()] {
        const { Contract::__SPEC_XDR_ENTRY_valval.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__tuple__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_TUPLE: [u8; super::Contract::spec_xdr_len_tuple()] =
        super::Contract::spec_xdr_tuple();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_tuple: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"tuple"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U64,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Tuple(
                    &soroban_sdk::xdr::ScSpecTypeTupleView {
                        value_types: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                            soroban_sdk::xdr::ScSpecTypeDefView::U64,
                            soroban_sdk::xdr::ScSpecTypeDefView::U64,
                        ]),
                    },
                ),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_tuple() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_tuple.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple() -> [u8; Contract::spec_xdr_len_tuple()] {
        const { Contract::__SPEC_XDR_ENTRY_tuple.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__valref__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VALREF: [u8; super::Contract::spec_xdr_len_valref()] =
        super::Contract::spec_xdr_valref();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_valref: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"valref"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"input"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U64,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::U64,
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_valref() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_valref.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_valref() -> [u8; Contract::spec_xdr_len_valref()] {
        const { Contract::__SPEC_XDR_ENTRY_valref.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn set_val(&self, input: &u64) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("set_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_set_val(
        &self,
        input: &u64,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("set_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn get_val(&self) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_get_val(
        &self,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("get_val");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn both(&self, input: &u64) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("both");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_both(
        &self,
        input: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("both");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn wrapped(&self, input: &Vec<u64>) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("wrapped");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_wrapped(
        &self,
        input: &Vec<u64>,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("wrapped");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn double_wrapped(&self, input: &Option<Vec<u64>>) -> Vec<u64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "double_wrapped") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_double_wrapped(
        &self,
        input: &Option<Vec<u64>>,
    ) -> Result<
        Result<
            Vec<u64>,
            <Vec<u64> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "double_wrapped") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn valval(&self, input: &u64) -> Option<u64> {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valval");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_valval(
        &self,
        input: &u64,
    ) -> Result<
        Result<
            Option<u64>,
            <Option<u64> as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valval");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn tuple(&self, input: &u64) -> (u64, u64) {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_tuple(
        &self,
        input: &u64,
    ) -> Result<
        Result<
            (u64, u64),
            <(u64, u64) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn valref(&self, input: &u64) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valref");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
    pub fn try_valref(
        &self,
        input: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("valref");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn set_val<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_val<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn both<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrapped<'i>(input: &'i Vec<u64>) -> (&'i Vec<u64>,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn double_wrapped<'i>(input: &'i Option<Vec<u64>>) -> (&'i Option<Vec<u64>>,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn valval<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn valref<'i>(input: &'i u64) -> (&'i u64,) {
        (input,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).set_val` instead")]
#[allow(deprecated)]
pub fn __Contract__set_val__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::set_val(
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).set_val` instead")]
#[export_name = "set_val"]
pub extern "C" fn __Contract__set_val__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__set_val__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_val` instead")]
#[allow(deprecated)]
pub fn __Contract__get_val__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::get_val(env.clone()),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_val` instead")]
#[export_name = "get_val"]
pub extern "C" fn __Contract__get_val__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_val__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).both` instead")]
#[allow(deprecated)]
pub fn __Contract__both__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::both(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).both` instead")]
#[export_name = "both"]
pub extern "C" fn __Contract__both__invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__both__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapped` instead")]
#[allow(deprecated)]
pub fn __Contract__wrapped__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::wrapped(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapped` instead")]
#[export_name = "wrapped"]
pub extern "C" fn __Contract__wrapped__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__wrapped__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).double_wrapped` instead")]
#[allow(deprecated)]
pub fn __Contract__double_wrapped__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::double_wrapped(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).double_wrapped` instead")]
#[export_name = "double_wrapped"]
pub extern "C" fn __Contract__double_wrapped__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__double_wrapped__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valval` instead")]
#[allow(deprecated)]
pub fn __Contract__valval__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::valval(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valval` instead")]
#[export_name = "valval"]
pub extern "C" fn __Contract__valval__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__valval__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::tuple(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
#[export_name = "tuple"]
pub extern "C" fn __Contract__tuple__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valref` instead")]
#[allow(deprecated)]
pub fn __Contract__valref__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as AssociatedType>::valref(
            &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valref` instead")]
#[export_name = "valref"]
pub extern "C" fn __Contract__valref__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__valref__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
