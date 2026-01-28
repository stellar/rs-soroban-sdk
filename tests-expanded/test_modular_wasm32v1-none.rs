#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl};
mod feat1 {
    use crate::Contract;
    use crate::ContractArgs;
    use crate::ContractClient;
    use soroban_sdk::contractimpl;
    impl Contract {
        pub fn one() -> u32 {
            1
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __Contract__one__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #[link_section = "contractspecv0"]
        pub static __SPEC_XDR_FN_ONE: [u8; 28usize] = super::Contract::spec_xdr_one();
    }
    impl Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_one() -> [u8; 28usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x03one\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
        }
    }
    impl<'a> ContractClient<'a> {
        pub fn one(&self) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("one");
                    SYMBOL
                },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_one(
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
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("one");
                    SYMBOL
                },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
    }
    impl ContractArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn one<'i>() -> () {
            ()
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).one` instead")]
    pub fn __Contract__one__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<Contract>::one(),
            &env,
        )
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).one` instead")]
    #[export_name = "one"]
    pub extern "C" fn __Contract__one__invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        __Contract__one__invoke_raw(soroban_sdk::Env::default())
    }
}
mod feat2 {
    use crate::ContractArgs;
    use crate::ContractClient;
    use soroban_sdk::contractimpl;
    impl super::Contract {
        pub fn two() -> u32 {
            2
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    pub mod __super__Contract__two__spec {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #[link_section = "contractspecv0"]
        pub static __SPEC_XDR_FN_TWO: [u8; 28usize] = super::super::Contract::spec_xdr_two();
    }
    impl super::Contract {
        #[allow(non_snake_case)]
        pub const fn spec_xdr_two() -> [u8; 28usize] {
            *b"\0\0\0\0\0\0\0\0\0\0\0\x03two\0\0\0\0\0\0\0\0\x01\0\0\0\x04"
        }
    }
    impl<'a> ContractClient<'a> {
        pub fn two(&self) -> u32 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("two");
                    SYMBOL
                },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
        pub fn try_two(
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
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("two");
                    SYMBOL
                },
                ::soroban_sdk::Vec::new(&self.env),
            );
            res
        }
    }
    impl ContractArgs {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn two<'i>() -> () {
            ()
        }
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).two` instead")]
    pub fn __super__Contract__two__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::two(),
            &env,
        )
    }
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).two` instead")]
    #[export_name = "two"]
    pub extern "C" fn __super__Contract__two__invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        __super__Contract__two__invoke_raw(soroban_sdk::Env::default())
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
    pub fn zero() -> u32 {
        0
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__zero__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_ZERO: [u8; 28usize] = super::Contract::spec_xdr_zero();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_zero() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x04zero\0\0\0\0\0\0\0\x01\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn zero(&self) -> u32 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("zero");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_zero(
        &self,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("zero");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn zero<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).zero` instead")]
pub fn __Contract__zero__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
        #[allow(deprecated)]
        &<Contract>::zero(),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).zero` instead")]
#[export_name = "zero"]
pub extern "C" fn __Contract__zero__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__zero__invoke_raw(soroban_sdk::Env::default())
}
