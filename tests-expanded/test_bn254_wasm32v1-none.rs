#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bn254::{Bn254Fr, Bn254G1Affine, Bn254G2Affine},
    Env, Vec,
};
pub struct MockProof {
    pub g1: Vec<Bn254G1Affine>,
    pub g2: Vec<Bn254G2Affine>,
}
impl MockProof {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_bn254::MockProof"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_MOCKPROOF: [u8; MockProof::spec_xdr_len()] = MockProof::spec_xdr();
impl MockProof {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(MockProof::spec_name()),
            fields: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                                soroban_sdk::xdr::ScSpecTypeBytesN { n: 64u32 },
                            ),
                        },
                    ),
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                                soroban_sdk::xdr::ScSpecTypeBytesN { n: 128u32 },
                            ),
                        },
                    ),
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { MockProof::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; MockProof::spec_xdr_len()] {
        const { MockProof::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for MockProof {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["g1", "g2"];
        let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            g1: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            g2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, MockProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &MockProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 2usize] = ["g1", "g2"];
        let vals: [Val; 2usize] = [
            (&val.g1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.g2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &MockProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&MockProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, MockProof>>::try_from_val(env, *val)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MockProof {
    #[inline]
    fn clone(&self) -> MockProof {
        MockProof {
            g1: ::core::clone::Clone::clone(&self.g1),
            g2: ::core::clone::Clone::clone(&self.g2),
        }
    }
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
impl Contract {
    pub fn verify_pairing(env: Env, proof: MockProof) -> bool {
        env.crypto().bn254().pairing_check(proof.g1, proof.g2)
    }
    pub fn g1_add(a: Bn254G1Affine, b: Bn254G1Affine) -> Bn254G1Affine {
        a + b
    }
    pub fn g1_mul(p: Bn254G1Affine, s: Bn254Fr) -> Bn254G1Affine {
        p * s
    }
    pub fn fr_vec_get(_env: Env, values: Vec<Bn254Fr>, index: u32) -> Bn254Fr {
        values.get(index).unwrap()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__verify_pairing__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_VERIFY_PAIRING: [u8; super::Contract::spec_xdr_len_verify_pairing()] =
        super::Contract::spec_xdr_verify_pairing();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_verify_pairing: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"verify_pairing"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"proof"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                        soroban_sdk::xdr::ScSpecTypeUdtView {
                            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                <MockProof>::spec_name(),
                            ),
                        },
                    ),
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::Bool,
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_verify_pairing() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_verify_pairing.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_verify_pairing() -> [u8; Contract::spec_xdr_len_verify_pairing()] {
        const { Contract::__SPEC_XDR_ENTRY_verify_pairing.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_add__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_G1_ADD: [u8; super::Contract::spec_xdr_len_g1_add()] =
        super::Contract::spec_xdr_g1_add();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_g1_add: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g1_add"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"a"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 64u32 },
                    ),
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"b"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 64u32 },
                    ),
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::BytesN(soroban_sdk::xdr::ScSpecTypeBytesN {
                    n: 64u32,
                }),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_g1_add() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_g1_add.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g1_add() -> [u8; Contract::spec_xdr_len_g1_add()] {
        const { Contract::__SPEC_XDR_ENTRY_g1_add.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_mul__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_G1_MUL: [u8; super::Contract::spec_xdr_len_g1_mul()] =
        super::Contract::spec_xdr_g1_mul();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_g1_mul: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g1_mul"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"p"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 64u32 },
                    ),
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"s"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U256,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::BytesN(soroban_sdk::xdr::ScSpecTypeBytesN {
                    n: 64u32,
                }),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_g1_mul() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_g1_mul.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g1_mul() -> [u8; Contract::spec_xdr_len_g1_mul()] {
        const { Contract::__SPEC_XDR_ENTRY_g1_mul.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fr_vec_get__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FR_VEC_GET: [u8; super::Contract::spec_xdr_len_fr_vec_get()] =
        super::Contract::spec_xdr_fr_vec_get();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_fr_vec_get: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"fr_vec_get"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"values"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Vec(
                        &soroban_sdk::xdr::ScSpecTypeVecView {
                            element_type: &soroban_sdk::xdr::ScSpecTypeDefView::U256,
                        },
                    ),
                },
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"index"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U32,
                },
            ]),
            outputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecTypeDefView::U256,
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_fr_vec_get() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_fr_vec_get.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fr_vec_get() -> [u8; Contract::spec_xdr_len_fr_vec_get()] {
        const { Contract::__SPEC_XDR_ENTRY_fr_vec_get.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn verify_pairing(&self, proof: &MockProof) -> bool {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "verify_pairing") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        res
    }
    pub fn try_verify_pairing(
        &self,
        proof: &MockProof,
    ) -> Result<
        Result<bool, <bool as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "verify_pairing") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        res
    }
    pub fn g1_add(&self, a: &Bn254G1Affine, b: &Bn254G1Affine) -> Bn254G1Affine {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_add");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_g1_add(
        &self,
        a: &Bn254G1Affine,
        b: &Bn254G1Affine,
    ) -> Result<
        Result<
            Bn254G1Affine,
            <Bn254G1Affine as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_add");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), b.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn g1_mul(&self, p: &Bn254G1Affine, s: &Bn254Fr) -> Bn254G1Affine {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_mul");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [p.into_val(&self.env), s.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_g1_mul(
        &self,
        p: &Bn254G1Affine,
        s: &Bn254Fr,
    ) -> Result<
        Result<
            Bn254G1Affine,
            <Bn254G1Affine as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g1_mul");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [p.into_val(&self.env), s.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn fr_vec_get(&self, values: &Vec<Bn254Fr>, index: &u32) -> Bn254Fr {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fr_vec_get") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [values.into_val(&self.env), index.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_fr_vec_get(
        &self,
        values: &Vec<Bn254Fr>,
        index: &u32,
    ) -> Result<
        Result<
            Bn254Fr,
            <Bn254Fr as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "fr_vec_get") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [values.into_val(&self.env), index.into_val(&self.env)],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn verify_pairing<'i>(proof: &'i MockProof) -> (&'i MockProof,) {
        (proof,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g1_add<'i>(
        a: &'i Bn254G1Affine,
        b: &'i Bn254G1Affine,
    ) -> (&'i Bn254G1Affine, &'i Bn254G1Affine) {
        (a, b)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g1_mul<'i>(p: &'i Bn254G1Affine, s: &'i Bn254Fr) -> (&'i Bn254G1Affine, &'i Bn254Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fr_vec_get<'i>(values: &'i Vec<Bn254Fr>, index: &'i u32) -> (&'i Vec<Bn254Fr>, &'i u32) {
        (values, index)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_pairing` instead")]
#[allow(deprecated)]
pub fn __Contract__verify_pairing__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::verify_pairing(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_pairing` instead")]
#[export_name = "verify_pairing"]
pub extern "C" fn __Contract__verify_pairing__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__verify_pairing__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_add` instead")]
#[allow(deprecated)]
pub fn __Contract__g1_add__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::g1_add(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_add` instead")]
#[export_name = "g1_add"]
pub extern "C" fn __Contract__g1_add__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__g1_add__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
#[allow(deprecated)]
pub fn __Contract__g1_mul__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::g1_mul(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
#[export_name = "g1_mul"]
pub extern "C" fn __Contract__g1_mul__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__g1_mul__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fr_vec_get` instead")]
#[allow(deprecated)]
pub fn __Contract__fr_vec_get__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::fr_vec_get(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).fr_vec_get` instead")]
#[export_name = "fr_vec_get"]
pub extern "C" fn __Contract__fr_vec_get__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__fr_vec_get__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
