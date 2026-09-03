#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bls12_381::{Bls12381Fp, Bls12381Fp2, Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine},
    log, Env, Vec,
};
pub struct DummyProof {
    pub fp: Bls12381Fp,
    pub fp2: Bls12381Fp2,
    pub g1: Bls12381G1Affine,
    pub g2: Bls12381G2Affine,
    pub fr: Bls12381Fr,
}
impl DummyProof {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_bls::DummyProof"
    }
}
#[doc(hidden)]
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_DUMMYPROOF: [u8; DummyProof::spec_xdr_len()] = DummyProof::spec_xdr();
impl DummyProof {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::UdtStructV0(soroban_sdk::xdr::ScSpecUdtStructV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(DummyProof::spec_name()),
            fields: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"fp"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 48u32 },
                    ),
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"fp2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 96u32 },
                    ),
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"fr"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::U256,
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g1"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 96u32 },
                    ),
                },
                soroban_sdk::xdr::ScSpecUdtStructFieldV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g2"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 192u32 },
                    ),
                },
            ]),
        });
    pub const fn spec_xdr_len() -> usize {
        const { DummyProof::__SPEC_XDR_ENTRY.const_xdr_len() }
    }
    pub const fn spec_xdr() -> [u8; DummyProof::spec_xdr_len()] {
        const { DummyProof::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for DummyProof {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Bls12381Fp as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Bls12381Fp2 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Bls12381Fr as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Bls12381G1Affine as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        <Bls12381G2Affine as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        {
            static MARKER: [u8; 14] =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &DummyProof::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for DummyProof {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &soroban_sdk::Val,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
        const KEYS: [&'static str; 5usize] = ["fp", "fp2", "fr", "g1", "g2"];
        let mut vals: [Val; 5usize] = [Val::VOID.to_val(); 5usize];
        let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
        env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
            .map_err(|_| ConversionError)?;
        Ok(Self {
            fp: vals[0]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            fp2: vals[1]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            fr: vals[2]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            g1: vals[3]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
            g2: vals[4]
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?,
        })
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, DummyProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &DummyProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
        const KEYS: [&'static str; 5usize] = ["fp", "fp2", "fr", "g1", "g2"];
        let vals: [Val; 5usize] = [
            (&val.fp).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.fp2).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.fr).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.g1).try_into_val(env).map_err(|_| ConversionError)?,
            (&val.g2).try_into_val(env).map_err(|_| ConversionError)?,
        ];
        Ok(env
            .map_new_from_slices(&KEYS, &vals)
            .map_err(|_| ConversionError)?
            .into())
    }
}
impl soroban_sdk::TryFromVal<soroban_sdk::Env, &DummyProof> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    #[inline(always)]
    fn try_from_val(
        env: &soroban_sdk::Env,
        val: &&DummyProof,
    ) -> Result<Self, soroban_sdk::ConversionError> {
        <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, DummyProof>>::try_from_val(env, *val)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for DummyProof {
    #[inline]
    fn clone(&self) -> DummyProof {
        DummyProof {
            fp: ::core::clone::Clone::clone(&self.fp),
            fp2: ::core::clone::Clone::clone(&self.fp2),
            g1: ::core::clone::Clone::clone(&self.g1),
            g2: ::core::clone::Clone::clone(&self.g2),
            fr: ::core::clone::Clone::clone(&self.fr),
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
    pub fn g1_mul(env: Env, p: Bls12381G1Affine, s: Bls12381Fr) -> Bls12381G1Affine {
        env.crypto().bls12_381().g1_mul(&p, &s)
    }
    pub fn g2_mul(env: Env, p: Bls12381G2Affine, s: Bls12381Fr) -> Bls12381G2Affine {
        env.crypto().bls12_381().g2_mul(&p, &s)
    }
    pub fn dummy_verify(env: Env, proof: DummyProof) -> bool {
        let g1 = env.crypto().bls12_381().map_fp_to_g1(&proof.fp);
        let in1 = env.crypto().bls12_381().g1_is_in_subgroup(&g1);
        if false {
            (&env).logs().add(
                "`map_fp_to_g1` result is in subgroup: ",
                &[<_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(&in1, &env)],
            );
        }
        let g2 = env.crypto().bls12_381().map_fp2_to_g2(&proof.fp2);
        let in2 = env.crypto().bls12_381().g2_is_in_subgroup(&g2);
        if false {
            (&env).logs().add(
                "`map_fp2_to_g2` result is in subgroup: ",
                &[<_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(&in2, &env)],
            );
        }
        let g1_mul = env.crypto().bls12_381().g1_mul(&proof.g1, &proof.fr);
        let g2_mul = env.crypto().bls12_381().g2_mul(&proof.g2, &proof.fr);
        let vp1 = soroban_sdk::Vec::from_array(&env, [g1_mul]);
        let vp2 = soroban_sdk::Vec::from_array(&env, [g2_mul]);
        env.crypto().bls12_381().pairing_check(vp1, vp2)
    }
    pub fn fr_vec_get(_env: Env, values: Vec<Bls12381Fr>, index: u32) -> Bls12381Fr {
        values.get(index).unwrap()
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
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 96u32 },
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
                    n: 96u32,
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
pub mod __Contract__g2_mul__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_G2_MUL: [u8; super::Contract::spec_xdr_len_g2_mul()] =
        super::Contract::spec_xdr_g2_mul();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_g2_mul: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"g2_mul"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"p"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::BytesN(
                        soroban_sdk::xdr::ScSpecTypeBytesN { n: 192u32 },
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
                    n: 192u32,
                }),
            ]),
        });
    #[allow(non_snake_case)]
    pub const fn spec_xdr_len_g2_mul() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_g2_mul.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g2_mul() -> [u8; Contract::spec_xdr_len_g2_mul()] {
        const { Contract::__SPEC_XDR_ENTRY_g2_mul.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__dummy_verify__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DUMMY_VERIFY: [u8; super::Contract::spec_xdr_len_dummy_verify()] =
        super::Contract::spec_xdr_dummy_verify();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_dummy_verify: soroban_sdk::xdr::ScSpecEntryView<'static> =
        soroban_sdk::xdr::ScSpecEntryView::FunctionV0(soroban_sdk::xdr::ScSpecFunctionV0View {
            doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::ScSymbolView(
                soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"dummy_verify"),
            ),
            inputs: soroban_sdk::xdr::VecMView::try_from_slice_or_panic(&[
                soroban_sdk::xdr::ScSpecFunctionInputV0View {
                    doc: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::StringMView::try_from_slice_or_panic(b"proof"),
                    type_: soroban_sdk::xdr::ScSpecTypeDefView::Udt(
                        soroban_sdk::xdr::ScSpecTypeUdtView {
                            name: soroban_sdk::xdr::StringMView::try_from_str_or_panic(
                                <DummyProof>::spec_name(),
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
    pub const fn spec_xdr_len_dummy_verify() -> usize {
        const { Contract::__SPEC_XDR_ENTRY_dummy_verify.const_xdr_len() }
    }
    #[allow(non_snake_case)]
    pub const fn spec_xdr_dummy_verify() -> [u8; Contract::spec_xdr_len_dummy_verify()] {
        const { Contract::__SPEC_XDR_ENTRY_dummy_verify.const_to_xdr() }
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
    pub fn g1_mul(&self, p: &Bls12381G1Affine, s: &Bls12381Fr) -> Bls12381G1Affine {
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
        p: &Bls12381G1Affine,
        s: &Bls12381Fr,
    ) -> Result<
        Result<
            Bls12381G1Affine,
            <Bls12381G1Affine as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
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
    pub fn g2_mul(&self, p: &Bls12381G2Affine, s: &Bls12381Fr) -> Bls12381G2Affine {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g2_mul");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [p.into_val(&self.env), s.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn try_g2_mul(
        &self,
        p: &Bls12381G2Affine,
        s: &Bls12381Fr,
    ) -> Result<
        Result<
            Bls12381G2Affine,
            <Bls12381G2Affine as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    >{
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("g2_mul");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [p.into_val(&self.env), s.into_val(&self.env)],
            ),
        );
        res
    }
    pub fn dummy_verify(&self, proof: &DummyProof) -> bool {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "dummy_verify") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        res
    }
    pub fn try_dummy_verify(
        &self,
        proof: &DummyProof,
    ) -> Result<
        Result<bool, <bool as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "dummy_verify") },
            ::soroban_sdk::Vec::from_array(&self.env, [proof.into_val(&self.env)]),
        );
        res
    }
    pub fn fr_vec_get(&self, values: &Vec<Bls12381Fr>, index: &u32) -> Bls12381Fr {
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
        values: &Vec<Bls12381Fr>,
        index: &u32,
    ) -> Result<
        Result<
            Bls12381Fr,
            <Bls12381Fr as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
    pub fn g1_mul<'i>(
        p: &'i Bls12381G1Affine,
        s: &'i Bls12381Fr,
    ) -> (&'i Bls12381G1Affine, &'i Bls12381Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g2_mul<'i>(
        p: &'i Bls12381G2Affine,
        s: &'i Bls12381Fr,
    ) -> (&'i Bls12381G2Affine, &'i Bls12381Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn dummy_verify<'i>(proof: &'i DummyProof) -> (&'i DummyProof,) {
        (proof,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fr_vec_get<'i>(
        values: &'i Vec<Bls12381Fr>,
        index: &'i u32,
    ) -> (&'i Vec<Bls12381Fr>, &'i u32) {
        (values, index)
    }
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
#[allow(deprecated)]
pub fn __Contract__g2_mul__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::g2_mul(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
#[export_name = "g2_mul"]
pub extern "C" fn __Contract__g2_mul__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__g2_mul__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
#[allow(deprecated)]
pub fn __Contract__dummy_verify__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::dummy_verify(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
#[export_name = "dummy_verify"]
pub extern "C" fn __Contract__dummy_verify__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__dummy_verify__invoke_raw(soroban_sdk::Env::default(), arg_0)
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
