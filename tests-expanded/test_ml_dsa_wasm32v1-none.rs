#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};
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
    pub fn verify_44(
        env: Env,
        public_key: BytesN<1312>,
        message: Bytes,
        signature: BytesN<2420>,
        context: Bytes,
    ) {
        env.crypto()
            .ml_dsa_44_verify(&public_key, &message, &signature, &context);
    }
    pub fn verify_65(
        env: Env,
        public_key: BytesN<1952>,
        message: Bytes,
        signature: BytesN<3309>,
        context: Bytes,
    ) {
        env.crypto()
            .ml_dsa_65_verify(&public_key, &message, &signature, &context);
    }
    pub fn verify_87(
        env: Env,
        public_key: BytesN<2592>,
        message: Bytes,
        signature: BytesN<4627>,
        context: Bytes,
    ) {
        env.crypto()
            .ml_dsa_87_verify(&public_key, &message, &signature, &context);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__verify_44__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_VERIFY_44: [u8; super::Contract::spec_xdr_verify_44().len()] =
        super::Contract::spec_xdr_verify_44();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_verify_44: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"verify_44"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"public_key",
                        ),
                        type_: <BytesN<1312> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"message",
                        ),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"signature",
                        ),
                        type_: <BytesN<2420> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"context",
                        ),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_verify_44() -> [u8; Contract::__SPEC_XDR_ENTRY_verify_44.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_verify_44.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__verify_65__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_VERIFY_65: [u8; super::Contract::spec_xdr_verify_65().len()] =
        super::Contract::spec_xdr_verify_65();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_verify_65: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"verify_65"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"public_key",
                        ),
                        type_: <BytesN<1952> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"message",
                        ),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"signature",
                        ),
                        type_: <BytesN<3309> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"context",
                        ),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_verify_65() -> [u8; Contract::__SPEC_XDR_ENTRY_verify_65.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_verify_65.const_to_xdr() }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__verify_87__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_VERIFY_87: [u8; super::Contract::spec_xdr_verify_87().len()] =
        super::Contract::spec_xdr_verify_87();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_verify_87: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"verify_87"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"public_key",
                        ),
                        type_: <BytesN<2592> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"message",
                        ),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"signature",
                        ),
                        type_: <BytesN<4627> as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"context",
                        ),
                        type_: <Bytes as soroban_sdk::SpecTypeDef>::SPEC_TYPE_DEF,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_verify_87() -> [u8; Contract::__SPEC_XDR_ENTRY_verify_87.const_xdr_len()]
    {
        const { Contract::__SPEC_XDR_ENTRY_verify_87.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    /// Call `verify_44` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_verify_44`] to receive errors instead of panicking.
    pub fn verify_44(
        &self,
        public_key: &BytesN<1312>,
        message: &Bytes,
        signature: &BytesN<2420>,
        context: &Bytes,
    ) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("verify_44");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    public_key.into_val(&self.env),
                    message.into_val(&self.env),
                    signature.into_val(&self.env),
                    context.into_val(&self.env),
                ],
            ),
        );
        res
    }
    /// Call `verify_44` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_verify_44(
        &self,
        public_key: &BytesN<1312>,
        message: &Bytes,
        signature: &BytesN<2420>,
        context: &Bytes,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("verify_44");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    public_key.into_val(&self.env),
                    message.into_val(&self.env),
                    signature.into_val(&self.env),
                    context.into_val(&self.env),
                ],
            ),
        );
        res
    }
    /// Call `verify_65` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_verify_65`] to receive errors instead of panicking.
    pub fn verify_65(
        &self,
        public_key: &BytesN<1952>,
        message: &Bytes,
        signature: &BytesN<3309>,
        context: &Bytes,
    ) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("verify_65");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    public_key.into_val(&self.env),
                    message.into_val(&self.env),
                    signature.into_val(&self.env),
                    context.into_val(&self.env),
                ],
            ),
        );
        res
    }
    /// Call `verify_65` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_verify_65(
        &self,
        public_key: &BytesN<1952>,
        message: &Bytes,
        signature: &BytesN<3309>,
        context: &Bytes,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("verify_65");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    public_key.into_val(&self.env),
                    message.into_val(&self.env),
                    signature.into_val(&self.env),
                    context.into_val(&self.env),
                ],
            ),
        );
        res
    }
    /// Call `verify_87` function on the contract.
    ///
    /// # Panics
    ///
    /// Panics if the invoked contract returns an error or panics.
    ///
    /// Use [`Self::try_verify_87`] to receive errors instead of panicking.
    pub fn verify_87(
        &self,
        public_key: &BytesN<2592>,
        message: &Bytes,
        signature: &BytesN<4627>,
        context: &Bytes,
    ) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("verify_87");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    public_key.into_val(&self.env),
                    message.into_val(&self.env),
                    signature.into_val(&self.env),
                    context.into_val(&self.env),
                ],
            ),
        );
        res
    }
    /// Call `verify_87` function on the contract.
    ///
    /// # Errors
    ///
    /// Returns an error if the invoked contract returns an error or panics. A panic in the invoked contract is caught and returned as an error rather than panicking the caller.
    pub fn try_verify_87(
        &self,
        public_key: &BytesN<2592>,
        message: &Bytes,
        signature: &BytesN<4627>,
        context: &Bytes,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("verify_87");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    public_key.into_val(&self.env),
                    message.into_val(&self.env),
                    signature.into_val(&self.env),
                    context.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn verify_44<'i>(
        public_key: &'i BytesN<1312>,
        message: &'i Bytes,
        signature: &'i BytesN<2420>,
        context: &'i Bytes,
    ) -> (&'i BytesN<1312>, &'i Bytes, &'i BytesN<2420>, &'i Bytes) {
        (public_key, message, signature, context)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn verify_65<'i>(
        public_key: &'i BytesN<1952>,
        message: &'i Bytes,
        signature: &'i BytesN<3309>,
        context: &'i Bytes,
    ) -> (&'i BytesN<1952>, &'i Bytes, &'i BytesN<3309>, &'i Bytes) {
        (public_key, message, signature, context)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn verify_87<'i>(
        public_key: &'i BytesN<2592>,
        message: &'i Bytes,
        signature: &'i BytesN<4627>,
        context: &'i Bytes,
    ) -> (&'i BytesN<2592>, &'i Bytes, &'i BytesN<4627>, &'i Bytes) {
        (public_key, message, signature, context)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_44` instead")]
#[allow(deprecated)]
pub fn __Contract__verify_44__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
    arg_3: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::verify_44(
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
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_3),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_44` instead")]
#[export_name = "verify_44"]
pub extern "C" fn __Contract__verify_44__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
    arg_3: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__verify_44__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2, arg_3)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_65` instead")]
#[allow(deprecated)]
pub fn __Contract__verify_65__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
    arg_3: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::verify_65(
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
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_3),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_65` instead")]
#[export_name = "verify_65"]
pub extern "C" fn __Contract__verify_65__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
    arg_3: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__verify_65__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2, arg_3)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_87` instead")]
#[allow(deprecated)]
pub fn __Contract__verify_87__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
    arg_3: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::verify_87(
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
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_2),
            ),
            <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                <_ as soroban_sdk::TryFromValForContractFn<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::try_from_val_for_contract_fn(&env, &arg_3),
            ),
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).verify_87` instead")]
#[export_name = "verify_87"]
pub extern "C" fn __Contract__verify_87__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
    arg_3: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__verify_87__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2, arg_3)
}
