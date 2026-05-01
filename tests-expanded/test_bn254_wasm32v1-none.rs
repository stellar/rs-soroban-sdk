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
#[link_section = "contractspecv0"]
pub static __SPEC_XDR_TYPE_MOCKPROOF: [u8; 80usize] = MockProof::spec_xdr();
impl MockProof {
    pub const fn spec_xdr() -> [u8; 80usize] {
        *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\tMockProof\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02g1\0\0\0\0\x03\xea\0\0\x03\xee\0\0\0@\0\0\0\0\0\0\0\x02g2\0\0\0\0\x03\xea\0\0\x03\xee\0\0\0\x80"
    }
}
impl soroban_sdk::spec_shaking::SpecTypeId for MockProof {
    const SPEC_TYPE_ID: [u8; 32] =
        *b":\x81\xa6\xa0\x9e\xe7\xa7\x1f\xb2\xd7\xc9\x1b)/\x02d\xafvb+Mi[n13\xa4\x87\xd6\x1dT\x08";
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_TYPE_MOCKPROOF: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        2,
        *b":\x81\xa6\xa0\x9e\xe7\xa7\x1f\xb2\xd7\xc9\x1b)/\x02d\xafvb+Mi[n13\xa4\x87\xd6\x1dT\x08",
        [],
    );
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
        env.map_unpack_to_slice(map, &KEYS, &mut vals)
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
    pub static __SPEC_XDR_FN_VERIFY_PAIRING: [u8; 76usize] =
        super::Contract::spec_xdr_verify_pairing();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_verify_pairing() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0everify_pairing\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05proof\0\0\0\0\0\x07\xd0\0\0\0\tMockProof\0\0\0\0\0\0\x01\0\0\0\x01"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_VERIFY_PAIRING: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    74usize,
    1usize,
>(
    0,
    *b"(\xf5\xcfQ\xad\x8eE\xd6x\xb8\xe8\xb3\x81\xe7\xa4\x18w\xfa\xe4\x0e{a\xf9\x05\xdb\xb0\xa6\xaf\x06\xbe\x1aw",
    [<MockProof as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__g1_add__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_G1_ADD: [u8; 76usize] = super::Contract::spec_xdr_g1_add();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_g1_add() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g1_add\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\x03\xee\0\0\0@\0\0\0\0\0\0\0\x01b\0\0\0\0\0\x03\xee\0\0\0@\0\0\0\x01\0\0\x03\xee\0\0\0@"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_G1_ADD: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\xe0\xaf\xf3\xacg\xce\x8d2\x14\xbai\x82\xa7q\xf3\xf5g\xd2*\xdb\xc8\xae\x04\xbaC\xe1\xc1\xed\x95l\xccv",
    [],
);
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
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06g1_mul\0\0\0\0\0\x02\0\0\0\0\0\0\0\x01p\0\0\0\0\0\x03\xee\0\0\0@\0\0\0\0\0\0\0\x01s\0\0\0\0\0\0\x0c\0\0\0\x01\0\0\x03\xee\0\0\0@"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_G1_MUL: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"86PV\x83\x14\xc9\xec+\x07\xc3\x91\x12\x10\x9c\xae\xd5:XB\x80\x95\xf8\xd4s\x901_\xb0r\xa2^",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__fr_vec_get__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_FR_VEC_GET: [u8; 80usize] = super::Contract::spec_xdr_fr_vec_get();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fr_vec_get() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nfr_vec_get\0\0\0\0\0\x02\0\0\0\0\0\0\0\x06values\0\0\0\0\x03\xea\0\0\0\x0c\0\0\0\0\0\0\0\x05index\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x0c"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_FR_VEC_GET: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\xefy\x1dH\xc0j@N\xca\x03J\x07yA&=\x12\x9ff\xaa\\:\xb8HF\x93[\x06\xeef]\x84",
        [],
    );
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
