#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractevent, contractimpl, contracttrait, Address, Env};
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
pub struct Exec<'a, 'b>
where
    'a: 'b,
{
    from: &'a Address,
    result: &'b u32,
}
impl<'a, 'b> Exec<'a, 'b>
where
    'a: 'b,
{
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_generics::Exec"
    }
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_EVENT_EXEC: [u8; Exec::spec_xdr().len()] = Exec::spec_xdr();
impl<'a, 'b> Exec<'a, 'b>
where
    'a: 'b,
{
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(soroban_sdk::xdr::r#const::ScSpecEventV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(Exec::spec_name()),
            prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"exec"),
                ),
            ]),
            params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                },
                soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"result"),
                    type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                },
            ]),
            data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
        });
    pub const fn spec_xdr() -> [u8; Exec::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Exec::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl<'a, 'b> soroban_sdk::SpecShakingMarker for Exec<'a, 'b>
where
    'a: 'b,
{
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Exec::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl<'a, 'b> soroban_sdk::Event for Exec<'a, 'b>
where
    'a: 'b,
{
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 1usize] = ["result"];
        let vals: [soroban_sdk::Val; 1usize] = [self.result.into_val(env)];
        env.sparse_map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl<'a, 'b> Exec<'a, 'b>
where
    'a: 'b,
{
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
impl<'a, 'b> Contract
where
    'a: 'b,
{
    pub fn exec(i1: u32, i2: &u32, i3: &'b u32) -> u32 {
        i1 + i2 + *i3
    }
    pub fn exec_fn_lifetime<'c>(env: Env, from: Address, i1: &'c u32) -> u32 {
        Exec {
            from: &from,
            result: i1,
        }
        .publish(&env);
        *i1
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__exec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_EXEC: [u8; super::Contract::spec_xdr_exec().len()] =
        super::Contract::spec_xdr_exec();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_exec: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"exec"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"i1"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"i2"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"i3"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec() -> [u8; Contract::__SPEC_XDR_ENTRY_exec.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_exec.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__exec_fn_lifetime__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_EXEC_FN_LIFETIME: [u8; super::Contract::spec_xdr_exec_fn_lifetime()
        .len()] = super::Contract::spec_xdr_exec_fn_lifetime();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_exec_fn_lifetime: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"exec_fn_lifetime",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"from"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"i1"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec_fn_lifetime(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_exec_fn_lifetime.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_exec_fn_lifetime.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn exec(&self, i1: &u32, i2: &u32, i3: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    i1.into_val(&self.env),
                    i2.into_val(&self.env),
                    i3.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_exec(
        &self,
        i1: &u32,
        i2: &u32,
        i3: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    i1.into_val(&self.env),
                    i2.into_val(&self.env),
                    i3.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn exec_fn_lifetime(&self, from: &Address, i1: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "exec_fn_lifetime") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [from.into_val(&self.env), i1.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_exec_fn_lifetime(
        &self,
        from: &Address,
        i1: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "exec_fn_lifetime") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [from.into_val(&self.env), i1.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec<'i>(i1: &'i u32, i2: &'i u32, i3: &'i u32) -> (&'i u32, &'i u32, &'i u32) {
        (i1, i2, i3)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec_fn_lifetime<'i>(from: &'i Address, i1: &'i u32) -> (&'i Address, &'i u32) {
        (from, i1)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
#[allow(deprecated)]
pub fn __Contract__exec__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::exec(
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_1),
            ),
            &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
#[export_name = "exec"]
pub extern "C" fn __Contract__exec__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__exec__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec_fn_lifetime` instead")]
#[allow(deprecated)]
pub fn __Contract__exec_fn_lifetime__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::exec_fn_lifetime(
            env.clone(),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_0),
            ),
            &<_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec_fn_lifetime` instead")]
#[export_name = "exec_fn_lifetime"]
pub extern "C" fn __Contract__exec_fn_lifetime__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__exec_fn_lifetime__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
pub struct TraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_trait as Trait;
pub trait Trait {
    fn exec_trait_lifetime<'c>(_env: Env, i1: &'c u32) -> u32 {
        *i1
    }
}
///TraitClient is a client for calling the contract defined in "Trait".
pub struct TraitClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> TraitClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl<'a> TraitClient<'a> {
    pub fn exec_trait_lifetime(&self, i1: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "exec_trait_lifetime") },
            ::soroban_sdk::Vec::from_array(&self.env, [i1.into_val(&self.env)]),
        );
        res
    }
    pub fn try_exec_trait_lifetime(
        &self,
        i1: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "exec_trait_lifetime") },
            ::soroban_sdk::Vec::from_array(&self.env, [i1.into_val(&self.env)]),
        );
        res
    }
}
///TraitArgs is a type for building arg lists for functions defined in "Trait".
pub struct TraitArgs;
impl TraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec_trait_lifetime<'i>(i1: &'i u32) -> (&'i u32,) {
        (i1,)
    }
}
impl TraitSpec {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_exec_trait_lifetime: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"exec_trait_lifetime",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"i1"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec_trait_lifetime(
    ) -> [u8; TraitSpec::__SPEC_XDR_ENTRY_exec_trait_lifetime.const_xdr_len()] {
        const { TraitSpec::__SPEC_XDR_ENTRY_exec_trait_lifetime.const_to_xdr() }
    }
}
impl Trait for Contract {}
impl<'a> ContractClient<'a> {}
impl ContractArgs {}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec_trait_lifetime` instead")]
#[allow(deprecated)]
pub fn __Contract__exec_trait_lifetime__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract as Trait>::exec_trait_lifetime(
            env.clone(),
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec_trait_lifetime` instead")]
#[export_name = "exec_trait_lifetime"]
pub extern "C" fn __Contract__exec_trait_lifetime__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__exec_trait_lifetime__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__exec_trait_lifetime__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_EXEC_TRAIT_LIFETIME: [u8; super::Contract::spec_xdr_exec_trait_lifetime(
    )
    .len()] = super::Contract::spec_xdr_exec_trait_lifetime();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_exec_trait_lifetime: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"exec_trait_lifetime",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"i1"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec_trait_lifetime(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_exec_trait_lifetime.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_exec_trait_lifetime.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn exec_trait_lifetime(&self, i1: &u32) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "exec_trait_lifetime") },
            ::soroban_sdk::Vec::from_array(&self.env, [i1.into_val(&self.env)]),
        );
        res
    }
    pub fn try_exec_trait_lifetime(
        &self,
        i1: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "exec_trait_lifetime") },
            ::soroban_sdk::Vec::from_array(&self.env, [i1.into_val(&self.env)]),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec_trait_lifetime<'i>(i1: &'i u32) -> (&'i u32,) {
        (i1,)
    }
}
