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
    set_auths: Option<&'a [soroban_sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [soroban_sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
mod __contract_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = soroban_sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(
        name: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl soroban_sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for Contract {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
    }
}
impl Contract {
    pub fn hello(env: Env) {
        if true {
            (&env).logs().add("none", &[]);
        }
        if true {
            (&env).logs().add("none", &[]);
        }
        if true {
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
        if true {
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
        if true {
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
        if true {
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
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_hello(
        &self,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
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
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
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
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 0usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    0usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env)
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).hello` instead")]
    pub extern "C" fn invoke_raw_extern() -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default())
    }
    use super::*;
}
mod test {
    extern crate std;
    use crate::{Contract, ContractClient};
    use soroban_sdk::{testutils::Logs, Env};
    extern crate test;
    #[rustc_test_marker = "test::test_logging"]
    #[doc(hidden)]
    pub const test_logging: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_logging"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/logging/src/lib.rs",
            start_line: 38usize,
            start_col: 8usize,
            end_line: 38usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_logging()),
        ),
    };
    fn test_logging() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        client.hello();
        env.logs().print();
        if true {
            let pats = <[_]>::into_vec(::alloc::boxed::box_new([
                "\"none\"",
                "\"none\"",
                "[\"one:\", one]",
                "[\"one:\", one]",
                "[\"one and two:\", one, two]",
                "[\"one and two:\", one, two]",
            ]));
            for (msg, pat) in env.logs().all().iter().zip(pats.iter()) {
                if !msg.contains(pat) {
                    ::core::panicking::panic("assertion failed: msg.contains(pat)")
                }
            }
        } else {
            match (&env.logs().all(), &::alloc::vec::from_elem("", 0)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_logging])
}
