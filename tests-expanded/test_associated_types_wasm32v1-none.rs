#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, Env, String};
pub struct DefaultImpl;
impl Trait for DefaultImpl {
    type Impl = Self;
    fn exec(env: &Env) -> String {
        String::from_str(env, "default")
    }
}
pub trait Trait {
    type Impl: Trait;
    fn exec(env: &Env) -> String {
        Self::Impl::exec(env)
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
impl Trait for Contract {
    type Impl = DefaultImpl;
    fn exec(env: &Env) -> String {
        Self::Impl::exec(env)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__exec__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EXEC: [u8; 28usize] = super::Contract::spec_xdr_exec();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x04exec\0\0\0\0\0\0\0\x01\0\0\0\x10"
    }
}
impl<'a> ContractClient<'a> {
    pub fn exec(&self) -> String {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_exec(
        &self,
    ) -> Result<
        Result<
            String,
            <String as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
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
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__exec {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        use super::Trait;
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::exec(&env),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
    #[export_name = "exec"]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
