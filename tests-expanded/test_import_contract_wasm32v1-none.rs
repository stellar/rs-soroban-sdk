#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, Address, Env};
mod addcontract {
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01\x14\x04`\x01~\x01~`\x02\x7f~\x00`\x02~~\x01~`\x00\x00\x02\r\x02\x01i\x010\x00\x00\x01i\x01_\x00\x00\x03\x05\x04\x01\x02\x03\x03\x05\x03\x01\x00\x10\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x07/\x05\x06memory\x02\x00\x03add\x00\x03\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\x89\x02\x04]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc0\x00F\r\x00\x02@ \x02A\x06F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x88!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x80\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0b\x9a\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x00 \x02)\x03\x08|\"\x01 \x00T\r\x01\x02@\x02@ \x01B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x01B\x08\x86B\x06\x84!\x00\x0c\x01\x0b \x01\x10\x81\x80\x80\x80\x00!\x00\x0b \x02A\x10j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\x10\x84\x80\x80\x80\x00\x00\x0b\t\x00\x10\x85\x80\x80\x80\x00\x00\x0b\x03\x00\x00\x0b\x0b\t\x01\x00A\x80\x80\xc0\x00\x0b\x00\x00K\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03add\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x06\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x19\x00\x00\x00\x00\x00+\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.84.0\x00\x00";
    pub trait Contract {
        fn add(env: soroban_sdk::Env, a: u64, b: u64) -> u64;
    }
    ///Client is a client for calling the contract defined in "Contract".
    pub struct Client<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> Client<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> Client<'a> {
        pub fn add(&self, a: &u64, b: &u64) -> u64 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn try_add(
            &self,
            a: &u64,
            b: &u64,
        ) -> Result<
            Result<
                u64,
                <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            res
        }
    }
    ///Args is a type for building arg lists for functions defined in "Contract".
    pub struct Args;
    impl Args {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn add<'i>(a: &'i u64, b: &'i u64) -> (&'i u64, &'i u64) {
            (a, b)
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
    pub fn add_with(env: Env, contract_id: Address, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add_with__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_ADD_WITH: [u8; 88usize] = super::Contract::spec_xdr_add_with();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add_with() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08add_with\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractClient<'a> {
    pub fn add_with(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add_with");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        res
    }
    pub fn try_add_with(
        &self,
        contract_id: &Address,
        x: &u64,
        y: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add_with");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn add_with<'i>(
        contract_id: &'i Address,
        x: &'i u64,
        y: &'i u64,
    ) -> (&'i Address, &'i u64, &'i u64) {
        (contract_id, x, y)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
#[allow(deprecated)]
pub fn __Contract__add_with__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::add_with(
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
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
#[export_name = "add_with"]
pub extern "C" fn __Contract__add_with__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__add_with__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
