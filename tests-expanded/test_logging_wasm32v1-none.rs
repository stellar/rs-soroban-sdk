#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env};
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
    pub fn hello(env: Env) {
        if false {
            (&env).logs().add("none", &[]);
        }
        if false {
            (&env).logs().add("none", &[]);
        }
        if false {
            (&env).logs().add(
                "one:",
                &[
                    <_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(
                        &{
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("one");
                            SYMBOL
                        },
                        &env,
                    ),
                ],
            );
        }
        if false {
            (&env).logs().add(
                "one:",
                &[
                    <_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(
                        &{
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("one");
                            SYMBOL
                        },
                        &env,
                    ),
                ],
            );
        }
        if false {
            (&env).logs().add(
                "one and two:",
                &[
                    <_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(
                        &{
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("one");
                            SYMBOL
                        },
                        &env,
                    ),
                    <_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(
                        &{
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("two");
                            SYMBOL
                        },
                        &env,
                    ),
                ],
            );
        }
        if false {
            (&env).logs().add(
                "one and two:",
                &[
                    <_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(
                        &{
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("one");
                            SYMBOL
                        },
                        &env,
                    ),
                    <_ as ::soroban_sdk::IntoVal<Env, ::soroban_sdk::Val>>::into_val(
                        &{
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("two");
                            SYMBOL
                        },
                        &env,
                    ),
                ],
            );
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__hello__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_HELLO: [u8; 28usize] = super::Contract::spec_xdr_hello();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_hello() -> [u8; 28usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x05hello\0\0\0\0\0\0\0\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn hello(&self) -> () {
        use core::ops::Not;
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
                SYMBOL
            },
            ::soroban_sdk::Vec::new(&self.env),
        );
        res
    }
    pub fn try_hello(
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("hello");
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
    pub fn hello<'i>() -> () {
        ()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__hello {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::hello(env.clone()),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
    #[export_name = "hello"]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
