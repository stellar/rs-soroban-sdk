#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, contracttrait, Env, String};
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
pub struct DefaultImpl;
pub struct TraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
pub use __contractimpl_for_trait as Trait;
pub trait Trait {
    type Impl: Trait;
    fn exec(env: &Env) -> String {
        Self::Impl::exec(env)
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
///TraitArgs is a type for building arg lists for functions defined in "Trait".
pub struct TraitArgs;
impl TraitArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn exec<'i>() -> () {
        ()
    }
}
impl TraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x04exec\0\0\0\0\0\0\0\x01\0\0\0\x10"
    }
}
impl Trait for DefaultImpl {
    type Impl = Self;
    fn exec(env: &Env) -> String {
        String::from_str(env, "default")
    }
}
impl Trait for Contract {
    type Impl = DefaultImpl;
}
impl<'a> ContractClient<'a> {}
impl ContractArgs {}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
#[allow(deprecated)]
pub fn __Contract__exec__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    use Trait;
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::exec(&env), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec` instead")]
#[export_name = "exec"]
pub extern "C" fn __Contract__exec__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__exec__invoke_raw(soroban_sdk::Env::default())
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
pub trait TraitWithoutContractTrait {
    type Impl: Trait;
    fn exec2(env: &Env) -> String {
        Self::Impl::exec(env)
    }
}
impl TraitWithoutContractTrait for DefaultImpl {
    type Impl = Self;
    fn exec2(env: &Env) -> String {
        String::from_str(env, "default2")
    }
}
impl TraitWithoutContractTrait for Contract {
    type Impl = DefaultImpl;
    fn exec2(env: &Env) -> String {
        Self::Impl::exec2(env)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__exec2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EXEC2: [u8; 32usize] = super::Contract::spec_xdr_exec2();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_exec2() -> [u8; 32usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05exec2\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x10"
    }
}
impl<'a> ContractClient<'a> {
    pub fn exec2(&self) -> String {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec2");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_exec2(
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("exec2");
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
    pub fn exec2<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec2` instead")]
#[allow(deprecated)]
pub fn __Contract__exec2__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    use TraitWithoutContractTrait;
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::exec2(&env), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).exec2` instead")]
#[export_name = "exec2"]
pub extern "C" fn __Contract__exec2__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__exec2__invoke_raw(soroban_sdk::Env::default())
}
