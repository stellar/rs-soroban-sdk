#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, contracttrait};
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
    pub fn empty() {}
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMPTY: [u8; 28usize] = super::Contract::spec_xdr_empty();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05empty\0\0\0\0\0\0\0\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn empty(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_empty(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty");
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
    pub fn empty<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::empty(),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty` instead")]
    #[export_name = "empty"]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
impl Contract {
    pub fn empty2() {}
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMPTY2: [u8; 28usize] = super::Contract::spec_xdr_empty2();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty2() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06empty2\0\0\0\0\0\0\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn empty2(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty2");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_empty2(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty2");
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
    pub fn empty2<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty2 {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty2` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::empty2(),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty2` instead")]
    #[export_name = "empty2"]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
pub struct TraitSpec;
/// Macro for `contractimpl`ing the default functions of the trait that are not overriden.
pub use __contractimpl_for_trait as Trait;
trait Trait {
    fn empty3() {}
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
    pub fn empty3(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty3");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_empty3(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty3");
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
    pub fn empty3<'i>() -> () {
        ()
    }
}
impl TraitSpec {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty3() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06empty3\0\0\0\0\0\0\0\0\0\0"
    }
}
impl Trait for Contract {
    fn empty3() {}
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty3__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_EMPTY3: [u8; 28usize] = super::Contract::spec_xdr_empty3();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_empty3() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06empty3\0\0\0\0\0\0\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn empty3(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty3");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_empty3(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("empty3");
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
    pub fn empty3<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__empty3 {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty3` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        use super::Trait;
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::empty3(),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).empty3` instead")]
    #[export_name = "empty3"]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
