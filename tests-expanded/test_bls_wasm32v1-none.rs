#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bls12_381::{Fp, Fp2, Fr, G1Affine, G2Affine},
    log, Env,
};
pub struct DummyProof {
    pub fp: Fp,
    pub fp2: Fp2,
    pub g1: G1Affine,
    pub g2: G2Affine,
    pub fr: Fr,
}
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_DUMMYPROOF: [u8; 128usize] = DummyProof::spec_xdr();
impl DummyProof {
    pub const fn spec_xdr() -> [u8; 128usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\nDummyProof\0\0\0\0\0\x05\0\0\0\0\0\0\0\x02fp\0\0\0\0\x03\xee\0\0\00\0\0\0\0\0\0\0\x03fp2\0\0\0\x03\xee\0\0\0`\0\0\0\0\0\0\0\x02fr\0\0\0\0\0\x0c\0\0\0\0\0\0\0\x02g1\0\0\0\0\x03\xee\0\0\0`\0\0\0\0\0\0\0\x02g2\0\0\0\0\x03\xee\0\0\0\xc0"
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
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
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
    pub fn g1_mul(env: Env, p: G1Affine, s: Fr) -> G1Affine {
        env.crypto().bls12_381().g1_mul(&p, &s)
    }
    pub fn g2_mul(env: Env, p: G2Affine, s: Fr) -> G2Affine {
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
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_mul__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_G1_MUL: [u8; 72usize] = super::Contract::spec_xdr_g1_mul();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g1_mul() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g1_mul\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01p\0\0\0\0\0\x03\xee\0\0\0`\0\0\0\0\0\0\0\x01s\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\x03\xee\0\0\0`"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g2_mul__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_G2_MUL: [u8; 72usize] = super::Contract::spec_xdr_g2_mul();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g2_mul() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g2_mul\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01p\0\0\0\0\0\x03\xee\0\0\0\xc0\0\0\0\0\0\0\0\x01s\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\x03\xee\0\0\0\xc0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__dummy_verify__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_DUMMY_VERIFY: [u8; 72usize] = super::Contract::spec_xdr_dummy_verify();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_dummy_verify() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cdummy_verify\0\0\0\x01\0\0\0\0\0\0\0\x05proof\0\0\0\0\0\x07\xd0\0\0\0\nDummyProof\0\0\0\0\0\x01\0\0\0\x01"
    }
}
impl<'a> ContractClient<'a> {
    pub fn g1_mul(&self, p: &G1Affine, s: &Fr) -> G1Affine {
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
        p: &G1Affine,
        s: &Fr,
    ) -> Result<
        Result<
            G1Affine,
            <G1Affine as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
    pub fn g2_mul(&self, p: &G2Affine, s: &Fr) -> G2Affine {
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
        p: &G2Affine,
        s: &Fr,
    ) -> Result<
        Result<
            G2Affine,
            <G2Affine as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
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
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g1_mul<'i>(p: &'i G1Affine, s: &'i Fr) -> (&'i G1Affine, &'i Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn g2_mul<'i>(p: &'i G2Affine, s: &'i Fr) -> (&'i G2Affine, &'i Fr) {
        (p, s)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn dummy_verify<'i>(proof: &'i DummyProof) -> (&'i DummyProof,) {
        (proof,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_mul {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::g1_mul(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g1_mul` instead")]
    #[export_name = "g1_mul"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g2_mul {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::g2_mul(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).g2_mul` instead")]
    #[export_name = "g2_mul"]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__dummy_verify {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::dummy_verify(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).dummy_verify` instead")]
    #[export_name = "dummy_verify"]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
