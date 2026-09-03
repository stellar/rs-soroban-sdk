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
    set_auths: Option<&'a [soroban_sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [soroban_sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
mod __contract_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = soroban_sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(
        name: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl soroban_sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for Contract {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
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
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_set_val(
        &self,
        input: &u64,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn get_val(&self) -> u64 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_get_val(
        &self,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn both(&self, input: &u64) -> u64 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_both(
        &self,
        input: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn wrapped(&self, input: &Vec<u64>) -> u64 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrapped(
        &self,
        input: &Vec<u64>,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn double_wrapped(&self, input: &Option<Vec<u64>>) -> Vec<u64> {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "double_wrapped") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "double_wrapped") },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn valval(&self, input: &u64) -> Option<u64> {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn tuple(&self, input: &u64) -> (u64, u64) {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn valref(&self, input: &u64) -> u64 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_valref(
        &self,
        input: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
pub fn __Contract__set_val__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__set_val__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).set_val` instead")]
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
pub fn __Contract__get_val__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 0usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                0usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__get_val__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_val` instead")]
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
pub fn __Contract__both__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__both__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).both` instead")]
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
pub fn __Contract__wrapped__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__wrapped__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrapped` instead")]
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
pub fn __Contract__double_wrapped__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__double_wrapped__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).double_wrapped` instead")]
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
pub fn __Contract__valval__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__valval__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valval` instead")]
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
pub fn __Contract__tuple__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__tuple__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple` instead")]
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
pub fn __Contract__valref__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__valref__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).valref` instead")]
pub extern "C" fn __Contract__valref__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__valref__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__AssociatedType__43ee96751c899ff0e1a286858793b710aeacd1f5d72c118904d14312012a0923_ctor(
) {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::ctor::__support::CtorRetType {
                unsafe {
                    __Contract__AssociatedType__43ee96751c899ff0e1a286858793b710aeacd1f5d72c118904d14312012a0923_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "set_val",
            #[allow(deprecated)]
            &__Contract__set_val__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "get_val",
            #[allow(deprecated)]
            &__Contract__get_val__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "both",
            #[allow(deprecated)]
            &__Contract__both__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrapped",
            #[allow(deprecated)]
            &__Contract__wrapped__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "double_wrapped",
            #[allow(deprecated)]
            &__Contract__double_wrapped__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "valval",
            #[allow(deprecated)]
            &__Contract__valval__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "tuple",
            #[allow(deprecated)]
            &__Contract__tuple__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "valref",
            #[allow(deprecated)]
            &__Contract__valref__invoke_raw_slice,
        );
    }
}
mod test {
    use crate::{Contract, ContractClient};
    use soroban_sdk::{vec, Env};
    extern crate test;
    #[rustc_test_marker = "test::test_associated_type_chained"]
    #[doc(hidden)]
    pub const test_associated_type_chained: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_associated_type_chained"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/associated_type_chained/src/lib.rs",
            start_line: 80usize,
            start_col: 8usize,
            end_line: 80usize,
            end_col: 36usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_associated_type_chained()),
        ),
    };
    fn test_associated_type_chained() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        client.set_val(&42u64);
        match (&client.get_val(), &42u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&client.both(&42u64), &43u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &client.wrapped(&::soroban_sdk::Vec::from_array(&e, [1u64, 2u64, 3u64])),
            &6u64,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &client.try_wrapped(&::soroban_sdk::Vec::new(&e)).err(),
            &Some(Ok(soroban_sdk::Error::from_contract_error(0))),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &client.double_wrapped(&Some(::soroban_sdk::Vec::from_array(&e, [4u64, 5u64]))),
            &::soroban_sdk::Vec::from_array(&e, [4u64, 5u64]),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &client.try_double_wrapped(&None).err(),
            &Some(Ok(soroban_sdk::Error::from_contract_error(1))),
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&client.valval(&42u64), &Some(42u64)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&client.tuple(&42u64), &(42u64, 42u64)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&client.valref(&42u64), &42u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_associated_type_chained])
}
