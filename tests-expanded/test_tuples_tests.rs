#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl};
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
    pub fn void_fn(_void_arg: ()) -> () {}
    pub fn tuple1(arg: (u32,)) -> (u32,) {
        arg
    }
    pub fn tuple2(arg: (u32, i64)) -> (u32, i64) {
        arg
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__void_fn__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_VOID_FN: [u8; 52usize] = super::Contract::spec_xdr_void_fn();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_void_fn() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07void_fn\0\0\0\0\x01\0\0\0\0\0\0\0\x08void_arg\0\0\0\x02\0\0\0\x01\0\0\0\x02"
    }
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_VOID_FN: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"\x94\x83*7S\xb5C\x1dJ\xfdD\x96\xff>j.#\x8b\xe3\xd8\xd8@\x07V\xf5\x931\xc6\xe7\xa1\xdcC",
        [],
    );
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__tuple1__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_TUPLE1: [u8; 64usize] = super::Contract::spec_xdr_tuple1();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple1() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06tuple1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03arg\0\0\0\x03\xed\0\0\0\x01\0\0\0\x04\0\0\0\x01\0\0\x03\xed\0\0\0\x01\0\0\0\x04"
    }
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TUPLE1: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\xb0\xef<f\xb1\x98h\x93m\x8f\xed\xf3o\xe1\x1a\xca\xd3\xeb\xa6w\xe1\x03\x0b\x95\xcdB\x85<\xa1Sa!",
    [],
);
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__tuple2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_TUPLE2: [u8; 72usize] = super::Contract::spec_xdr_tuple2();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_tuple2() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x06tuple2\0\0\0\0\0\x01\0\0\0\0\0\0\0\x03arg\0\0\0\x03\xed\0\0\0\x02\0\0\0\x04\0\0\0\x07\0\0\0\x01\0\0\x03\xed\0\0\0\x02\0\0\0\x04\0\0\0\x07"
    }
}
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_TUPLE2: [u8; 42usize] =
    soroban_sdk::spec_shaking::encode_graph_record::<42usize, 0usize>(
        0,
        *b"w\x9e\xa1:\xedN\x8a)\xca\x95\xd4e\x99\xfd\x95\x94Ff0w+N\xef\x85\x87k\xe58Se\xa3l",
        [],
    );
impl<'a> ContractClient<'a> {
    pub fn void_fn(&self, _void_arg: &()) -> () {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("void_fn");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_void_arg.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_void_fn(
        &self,
        _void_arg: &(),
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("void_fn");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [_void_arg.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn tuple1(&self, arg: &(u32,)) -> (u32,) {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_tuple1(
        &self,
        arg: &(u32,),
    ) -> Result<
        Result<
            (u32,),
            <(u32,) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn tuple2(&self, arg: &(u32, i64)) -> (u32, i64) {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_tuple2(
        &self,
        arg: &(u32, i64),
    ) -> Result<
        Result<
            (u32, i64),
            <(u32, i64) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
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
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
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
    pub fn void_fn<'i>(_void_arg: &'i ()) -> (&'i (),) {
        (_void_arg,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple1<'i>(arg: &'i (u32,)) -> (&'i (u32,),) {
        (arg,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn tuple2<'i>(arg: &'i (u32, i64)) -> (&'i (u32, i64),) {
        (arg,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).void_fn` instead")]
#[allow(deprecated)]
pub fn __Contract__void_fn__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::void_fn(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).void_fn` instead")]
pub fn __Contract__void_fn__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__void_fn__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).void_fn` instead")]
pub extern "C" fn __Contract__void_fn__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__void_fn__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple1` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple1__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::tuple1(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple1` instead")]
pub fn __Contract__tuple1__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__tuple1__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple1` instead")]
pub extern "C" fn __Contract__tuple1__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple1__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple2` instead")]
#[allow(deprecated)]
pub fn __Contract__tuple2__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::tuple2(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple2` instead")]
pub fn __Contract__tuple2__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 1usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                1usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__tuple2__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).tuple2` instead")]
pub extern "C" fn __Contract__tuple2__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__tuple2__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____69e94e814d1599c21b8ac3d759295183311eaabe224b3ad8865aaa5d01729db0_ctor() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::ctor::__support::CtorRetType {
                unsafe {
                    __Contract____69e94e814d1599c21b8ac3d759295183311eaabe224b3ad8865aaa5d01729db0_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "void_fn",
            #[allow(deprecated)]
            &__Contract__void_fn__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "tuple1",
            #[allow(deprecated)]
            &__Contract__tuple1__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "tuple2",
            #[allow(deprecated)]
            &__Contract__tuple2__invoke_raw_slice,
        );
    }
}
mod test {
    use crate::{Contract, ContractClient};
    use soroban_sdk::Env;
    mod wasm {
        pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01\x1f\x05`\x01~\x01~`\x02~~\x01~`\x03~~~\x01~`\x03~\x7f\x7f\x00`\x02\x7f\x7f\x01~\x02\x19\x04\x01i\x012\x00\x00\x01i\x011\x00\x00\x01v\x01g\x00\x01\x01v\x01h\x00\x02\x03\x06\x05\x00\x03\x04\x00\x00\x05\x03\x01\x00\x10\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x07E\x07\x06memory\x02\x00\x06tuple1\x00\x04\x06tuple2\x00\x07\x07void_fn\x00\x08\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xc8\x03\x05x\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00\x02@\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00 \x01B\x027\x03\x08 \x00 \x01A\x08jA\x01\x10\x85\x80\x80\x80\x00 \x01)\x03\x08\"\x00B\xff\x01\x83B\x04Q\r\x01\x0b\x00\x0b \x01 \x00B\x84\x80\x80\x80p\x837\x03\x08 \x01A\x08jA\x01\x10\x86\x80\x80\x80\x00!\x00 \x01A\x10j$\x80\x80\x80\x80\x00 \x00\x0b\x1d\x00 \x00 \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x00\x1a\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x82\x80\x80\x80\x00\x0b\xff\x01\x02\x02\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00\x02@\x02@\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00A\x00!\x02\x02@\x03@ \x02A\x10F\r\x01 \x01 \x02jB\x027\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b \x00 \x01A\x02\x10\x85\x80\x80\x80\x00 \x01)\x03\x00\"\x03B\xff\x01\x83B\x04R\r\x00 \x01)\x03\x08\"\x00\xa7A\xff\x01q\"\x02A\xc1\x00F\r\x01 \x02A\x07G\r\x00 \x00B\x08\x87!\x00\x0c\x02\x0b\x00\x0b \x00\x10\x80\x80\x80\x80\x00!\x00\x0b\x02@\x02@ \x00B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00B\x08\x86B\x07\x84!\x00\x0c\x01\x0b \x00\x10\x81\x80\x80\x80\x00!\x00\x0b \x01 \x007\x03\x08 \x01 \x03B\x84\x80\x80\x80p\x837\x03\x00 \x01A\x02\x10\x86\x80\x80\x80\x00!\x00 \x01A\x10j$\x80\x80\x80\x80\x00 \x00\x0b\x13\x00\x02@ \x00B\xff\x01\x83B\x02Q\r\x00\x00\x0bB\x02\x0b\x0b\t\x01\x00A\x80\x80\xc0\x00\x0b\x00\x00\xdf\x16\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06tuple1\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x03arg\x00\x00\x00\x03\xed\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xed\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06tuple2\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x03arg\x00\x00\x00\x03\xed\x00\x00\x00\x02\x00\x00\x00\x04\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x03\xed\x00\x00\x00\x02\x00\x00\x00\x04\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07void_fn\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x08void_arg\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x07Context\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nExecutable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\xab\x06\x1ccontractspecv0.rssdk.graphv0SpGrV\x01\x00\x00\xb0\xef<f\xb1\x98h\x93m\x8f\xed\xf3o\xe1\x1a\xca\xd3\xeb\xa6w\xe1\x03\x0b\x95\xcdB\x85<\xa1Sa!\x00\x00SpGrV\x01\x00\x00w\x9e\xa1:\xedN\x8a)\xca\x95\xd4e\x99\xfd\x95\x94Ff0w+N\xef\x85\x87k\xe58Se\xa3l\x00\x00SpGrV\x01\x00\x00\x94\x83*7S\xb5C\x1dJ\xfdD\x96\xff>j.#\x8b\xe3\xd8\xd8@\x07V\xf5\x931\xc6\xe7\xa1\xdcC\x00\x00SpGrV\x01\x00\x02\xa3J\xcf\xf7D\x93\x0bB]\x95\xeb\xfe\x03y\x83e5\\\x16\xeb\x94Ne\xe6Xw\x1f&\xf7\xc0pT\x00\x03\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xec\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9ns\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xafSpGrV\x01\x00\x02\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xec\x00\x00SpGrV\x01\x00\x02\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xcc\x00\x00SpGrV\x01\x00\x02\x9e)H\x8e\xf0\x01{{\xce\x9fdO\x0eD\xc0,\x0f\xe8\xee\'\x845r\x9f\xeb`\xd0\x12H\x17\x96g\x00\x02\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xecULqD\xd3\xfa:\x1f\x0c\xa5\xb7\x04\xe5H\x8b\x91J\x9e\x0fe\x7f\x9f[\xdbG#\xc7o\xb0\xf4\xcf\xe6SpGrV\x01\x00\x02ULqD\xd3\xfa:\x1f\x0c\xa5\xb7\x04\xe5H\x8b\x91J\x9e\x0fe\x7f\x9f[\xdbG#\xc7o\xb0\xf4\xcf\xe6\x00\x03\x9e)H\x8e\xf0\x01{{\xce\x9fdO\x0eD\xc0,\x0f\xe8\xee\'\x845r\x9f\xeb`\xd0\x12H\x17\x96g\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9ns\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xafSpGrV\x01\x00\x02\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9n\x00\x01\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xccSpGrV\x01\x00\x02s\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xaf\x00\x01\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xccSpGrV\x01\x00\x02L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e\x00\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
        pub trait Contract {
            fn tuple1(env: soroban_sdk::Env, arg: (u32,)) -> (u32,);
            fn tuple2(env: soroban_sdk::Env, arg: (u32, i64)) -> (u32, i64);
            fn void_fn(env: soroban_sdk::Env, void_arg: ()) -> ();
        }
        ///Client is a client for calling the contract defined in "Contract".
        pub struct Client<'a> {
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
        impl<'a> Client<'a> {
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
            pub fn set_auths(
                &self,
                auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry],
            ) -> Self {
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
            pub fn mock_auths(
                &self,
                mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>],
            ) -> Self {
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
        impl<'a> Client<'a> {
            pub fn tuple1(&self, arg: &(u32,)) -> (u32,) {
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple1");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn try_tuple1(
                &self,
                arg: &(u32,),
            ) -> Result<
                Result<
                    (u32,),
                    <(u32,) as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
                >,
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
                        if self.allow_non_root_auth {
                            self.env.mock_all_auths_allowing_non_root_auth();
                        } else {
                            self.env.mock_all_auths();
                        }
                    }
                }
                use soroban_sdk::{FromVal, IntoVal};
                let res = self.env.try_invoke_contract(
                    &self.address,
                    &{
                        #[allow(deprecated)]
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple1");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn tuple2(&self, arg: &(u32, i64)) -> (u32, i64) {
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple2");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn try_tuple2(
                &self,
                arg: &(u32, i64),
            ) -> Result<
                Result<
                    (u32, i64),
                    <(
                        u32,
                        i64,
                    ) as soroban_sdk::TryFromVal<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::Error,
                >,
                Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
            >{
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
                let res = self.env.try_invoke_contract(
                    &self.address,
                    &{
                        #[allow(deprecated)]
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("tuple2");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [arg.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn void_fn(&self, void_arg: &()) -> () {
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("void_fn");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [void_arg.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn try_void_fn(
                &self,
                void_arg: &(),
            ) -> Result<
                Result<
                    (),
                    <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
                >,
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
                        if self.allow_non_root_auth {
                            self.env.mock_all_auths_allowing_non_root_auth();
                        } else {
                            self.env.mock_all_auths();
                        }
                    }
                }
                use soroban_sdk::{FromVal, IntoVal};
                let res = self.env.try_invoke_contract(
                    &self.address,
                    &{
                        #[allow(deprecated)]
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("void_fn");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [void_arg.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
        }
        ///Args is a type for building arg lists for functions defined in "Contract".
        pub struct Args;
        impl Args {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn tuple1<'i>(arg: &'i (u32,)) -> (&'i (u32,),) {
                (arg,)
            }
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn tuple2<'i>(arg: &'i (u32, i64)) -> (&'i (u32, i64),) {
                (arg,)
            }
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn void_fn<'i>(void_arg: &'i ()) -> (&'i (),) {
                (void_arg,)
            }
        }
        pub struct ContractContext {
            pub args: soroban_sdk::Vec<soroban_sdk::Val>,
            pub contract: soroban_sdk::Address,
            pub fn_name: soroban_sdk::Symbol,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ContractContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ContractContext",
                    "args",
                    &self.args,
                    "contract",
                    &self.contract,
                    "fn_name",
                    &&self.fn_name,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ContractContext {
            #[inline]
            fn clone(&self) -> ContractContext {
                ContractContext {
                    args: ::core::clone::Clone::clone(&self.args),
                    contract: ::core::clone::Clone::clone(&self.contract),
                    fn_name: ::core::clone::Clone::clone(&self.fn_name),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ContractContext {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<soroban_sdk::Val>>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ContractContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ContractContext {
            #[inline]
            fn eq(&self, other: &ContractContext) -> bool {
                self.args == other.args
                    && self.contract == other.contract
                    && self.fn_name == other.fn_name
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ContractContext {
            #[inline]
            fn cmp(&self, other: &ContractContext) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.args, &other.args) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.contract, &other.contract) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.fn_name, &other.fn_name)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ContractContext {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ContractContext,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.args, &other.args) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&self.contract, &other.contract)
                        {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(&self.fn_name, &other.fn_name)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; 96usize] = ContractContext::spec_xdr();
        impl ContractContext {
            pub const fn spec_xdr() -> [u8; 96usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fContractContext\0\0\0\0\x03\0\0\0\0\0\0\0\x04args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\x08contract\0\0\0\x13\0\0\0\0\0\0\0\x07fn_name\0\0\0\0\x11"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for ContractContext {
            const SPEC_TYPE_ID: [u8; 32] = *b"\x03\x04uN\xea\xd7[\x13V\x9f\xd4\xbd\xc1\x8a\xd6\x7f\xd8iD\xa5B\x89qT\x0b'\xad(\xb8\x9f\x8f\x19";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CONTRACTCONTEXT: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"\x03\x04uN\xea\xd7[\x13V\x9f\xd4\xbd\xc1\x8a\xd6\x7f\xd8iD\xa5B\x89qT\x0b'\xad(\xb8\x9f\x8f\x19",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractContext {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["args", "contract", "fn_name"];
                let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    args: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    contract: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    fn_name: vals[2]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ContractContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &ContractContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["args", "contract", "fn_name"];
                let vals: [Val; 3usize] = [
                    (&val.args).try_into_val(env).map_err(|_| ConversionError)?,
                    (&val.contract)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.fn_name)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ContractContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&ContractContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ContractContext>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for ContractContext {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 3usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self {
                    args: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "args"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                    contract: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                    fn_name: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "fn_name"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for ContractContext {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&ContractContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "args"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.args)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.contract)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "fn_name"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.fn_name)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<ContractContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&ContractContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<ContractContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryContractContext {
                args: <soroban_sdk::Vec<
                    soroban_sdk::Val,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                contract: <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                fn_name: <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryContractContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ArbitraryContractContext",
                        "args",
                        &self.args,
                        "contract",
                        &self.contract,
                        "fn_name",
                        &&self.fn_name,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryContractContext {
                #[inline]
                fn clone(&self) -> ArbitraryContractContext {
                    ArbitraryContractContext {
                        args: ::core::clone::Clone::clone(&self.args),
                        contract: ::core::clone::Clone::clone(&self.contract),
                        fn_name: ::core::clone::Clone::clone(&self.fn_name),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryContractContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            soroban_sdk::Val,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryContractContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryContractContext {
                #[inline]
                fn eq(&self, other: &ArbitraryContractContext) -> bool {
                    self.args == other.args
                        && self.contract == other.contract
                        && self.fn_name == other.fn_name
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryContractContext {
                #[inline]
                fn cmp(&self, other: &ArbitraryContractContext) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.args, &other.args) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.contract, &other.contract) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(&self.fn_name, &other.fn_name)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryContractContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryContractContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.args, &other.args) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.contract,
                                &other.contract,
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &self.fn_name,
                                        &other.fn_name,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryContractContext: ::std::thread::LocalKey<
                    std::cell::Cell<u32>,
                > = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContractContext {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryContractContext {
                                args: arbitrary::Arbitrary::arbitrary(u)?,
                                contract: arbitrary::Arbitrary::arbitrary(u)?,
                                fn_name: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryContractContext {
                                args: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                contract: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                fn_name: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractContext.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::and_all(
                                &[
                                    <<soroban_sdk::Vec<
                                        soroban_sdk::Val,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Symbol as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for ContractContext {
                type Prototype = ArbitraryContractContext;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContractContext> for ContractContext {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryContractContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(ContractContext {
                        args: soroban_sdk::IntoVal::into_val(&v.args, env),
                        contract: soroban_sdk::IntoVal::into_val(&v.contract, env),
                        fn_name: soroban_sdk::IntoVal::into_val(&v.fn_name, env),
                    })
                }
            }
        };
        pub struct SubContractInvocation {
            pub context: ContractContext,
            pub sub_invocations: soroban_sdk::Vec<InvokerContractAuthEntry>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SubContractInvocation {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SubContractInvocation",
                    "context",
                    &self.context,
                    "sub_invocations",
                    &&self.sub_invocations,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SubContractInvocation {
            #[inline]
            fn clone(&self) -> SubContractInvocation {
                SubContractInvocation {
                    context: ::core::clone::Clone::clone(&self.context),
                    sub_invocations: ::core::clone::Clone::clone(&self.sub_invocations),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SubContractInvocation {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ContractContext>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<InvokerContractAuthEntry>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SubContractInvocation {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SubContractInvocation {
            #[inline]
            fn eq(&self, other: &SubContractInvocation) -> bool {
                self.context == other.context && self.sub_invocations == other.sub_invocations
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for SubContractInvocation {
            #[inline]
            fn cmp(&self, other: &SubContractInvocation) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.context, &other.context) {
                    ::core::cmp::Ordering::Equal => {
                        ::core::cmp::Ord::cmp(&self.sub_invocations, &other.sub_invocations)
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for SubContractInvocation {
            #[inline]
            fn partial_cmp(
                &self,
                other: &SubContractInvocation,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.context, &other.context) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &self.sub_invocations,
                            &other.sub_invocations,
                        )
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_SUBCONTRACTINVOCATION: [u8; 144usize] =
            SubContractInvocation::spec_xdr();
        impl SubContractInvocation {
            pub const fn spec_xdr() -> [u8; 144usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x07context\0\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\0\0\0\0\x0fsub_invocations\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x18InvokerContractAuthEntry"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for SubContractInvocation {
            const SPEC_TYPE_ID: [u8; 32] = *b" \x9d\xc5_\xba\x8fv\x18\x95\x02\xbdJ}\x97\x01KN\xd6\0\xf8\xb6\xefq\xa8j\x11\\\xc7\xd7\xd4\xcf\xf0";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_SUBCONTRACTINVOCATION: [u8; 106usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            106usize,
            2usize,
        >(
            2,
            *b" \x9d\xc5_\xba\x8fv\x18\x95\x02\xbdJ}\x97\x01KN\xd6\0\xf8\xb6\xefq\xa8j\x11\\\xc7\xd7\xd4\xcf\xf0",
            [
                <ContractContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <InvokerContractAuthEntry as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for SubContractInvocation {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["context", "sub_invocations"];
                let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    context: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    sub_invocations: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, SubContractInvocation> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &SubContractInvocation,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["context", "sub_invocations"];
                let vals: [Val; 2usize] = [
                    (&val.context)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.sub_invocations)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &SubContractInvocation> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&SubContractInvocation,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    SubContractInvocation,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for SubContractInvocation {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 2usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self {
                    context: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "context"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                    sub_invocations: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "sub_invocations"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for SubContractInvocation {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&SubContractInvocation> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "context"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.context)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "sub_invocations"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.sub_invocations)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<SubContractInvocation> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&SubContractInvocation> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<SubContractInvocation> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: SubContractInvocation) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitrarySubContractInvocation {
                context: <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                sub_invocations: <soroban_sdk::Vec<
                    InvokerContractAuthEntry,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitrarySubContractInvocation {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ArbitrarySubContractInvocation",
                        "context",
                        &self.context,
                        "sub_invocations",
                        &&self.sub_invocations,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitrarySubContractInvocation {
                #[inline]
                fn clone(&self) -> ArbitrarySubContractInvocation {
                    ArbitrarySubContractInvocation {
                        context: ::core::clone::Clone::clone(&self.context),
                        sub_invocations: ::core::clone::Clone::clone(&self.sub_invocations),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitrarySubContractInvocation {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            InvokerContractAuthEntry,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitrarySubContractInvocation {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitrarySubContractInvocation {
                #[inline]
                fn eq(&self, other: &ArbitrarySubContractInvocation) -> bool {
                    self.context == other.context && self.sub_invocations == other.sub_invocations
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitrarySubContractInvocation {
                #[inline]
                fn cmp(&self, other: &ArbitrarySubContractInvocation) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.context, &other.context) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(&self.sub_invocations, &other.sub_invocations)
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitrarySubContractInvocation {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitrarySubContractInvocation,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.context, &other.context) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(
                                &self.sub_invocations,
                                &other.sub_invocations,
                            )
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitrarySubContractInvocation: ::std::thread::LocalKey<
                    std::cell::Cell<u32>,
                > = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitrarySubContractInvocation {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitrarySubContractInvocation {
                                context: arbitrary::Arbitrary::arbitrary(u)?,
                                sub_invocations: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitrarySubContractInvocation {
                                context: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                sub_invocations: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitrarySubContractInvocation.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::and_all(
                                &[
                                    <<ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::Vec<
                                        InvokerContractAuthEntry,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for SubContractInvocation {
                type Prototype = ArbitrarySubContractInvocation;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitrarySubContractInvocation>
                for SubContractInvocation
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitrarySubContractInvocation,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(SubContractInvocation {
                        context: soroban_sdk::IntoVal::into_val(&v.context, env),
                        sub_invocations: soroban_sdk::IntoVal::into_val(&v.sub_invocations, env),
                    })
                }
            }
        };
        pub struct CreateContractHostFnContext {
            pub executable: ContractExecutable,
            pub salt: soroban_sdk::BytesN<32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateContractHostFnContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CreateContractHostFnContext",
                    "executable",
                    &self.executable,
                    "salt",
                    &&self.salt,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateContractHostFnContext {
            #[inline]
            fn clone(&self) -> CreateContractHostFnContext {
                CreateContractHostFnContext {
                    executable: ::core::clone::Clone::clone(&self.executable),
                    salt: ::core::clone::Clone::clone(&self.salt),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CreateContractHostFnContext {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ContractExecutable>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CreateContractHostFnContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CreateContractHostFnContext {
            #[inline]
            fn eq(&self, other: &CreateContractHostFnContext) -> bool {
                self.executable == other.executable && self.salt == other.salt
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for CreateContractHostFnContext {
            #[inline]
            fn cmp(&self, other: &CreateContractHostFnContext) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.salt, &other.salt),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for CreateContractHostFnContext {
            #[inline]
            fn partial_cmp(
                &self,
                other: &CreateContractHostFnContext,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8; 116usize] =
            CreateContractHostFnContext::spec_xdr();
        impl CreateContractHostFnContext {
            pub const fn spec_xdr() -> [u8; 116usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x02\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for CreateContractHostFnContext {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xe1\"T\xf0&\x19?P\xad\xa3\xa0\xd2\xf1\xea\xf8~\xde\xe7\x12\xe5_&\xb62Cl\xc8x\xcc.\xd4\xcd";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xe1\"T\xf0&\x19?P\xad\xa3\xa0\xd2\xf1\xea\xf8~\xde\xe7\x12\xe5_&\xb62Cl\xc8x\xcc.\xd4\xcd",
            [<ContractExecutable as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for CreateContractHostFnContext {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["executable", "salt"];
                let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    executable: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    salt: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, CreateContractHostFnContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 2usize] = ["executable", "salt"];
                let vals: [Val; 2usize] = [
                    (&val.executable)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.salt).try_into_val(env).map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &CreateContractHostFnContext> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    CreateContractHostFnContext,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap>
            for CreateContractHostFnContext
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 2usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self {
                    executable: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "executable"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                    salt: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "salt"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal>
            for CreateContractHostFnContext
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&CreateContractHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "executable"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.executable)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "salt"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.salt)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<CreateContractHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&CreateContractHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<CreateContractHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryCreateContractHostFnContext {
                executable: <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                salt: <soroban_sdk::BytesN<
                    32,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ArbitraryCreateContractHostFnContext",
                        "executable",
                        &self.executable,
                        "salt",
                        &&self.salt,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn clone(&self) -> ArbitraryCreateContractHostFnContext {
                    ArbitraryCreateContractHostFnContext {
                        executable: ::core::clone::Clone::clone(&self.executable),
                        salt: ::core::clone::Clone::clone(&self.salt),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryCreateContractHostFnContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryCreateContractHostFnContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn eq(&self, other: &ArbitraryCreateContractHostFnContext) -> bool {
                    self.executable == other.executable && self.salt == other.salt
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn cmp(
                    &self,
                    other: &ArbitraryCreateContractHostFnContext,
                ) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(&self.salt, &other.salt)
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryCreateContractHostFnContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryCreateContractHostFnContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable)
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext:
                    ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryCreateContractHostFnContext {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractHostFnContext {
                                executable: arbitrary::Arbitrary::arbitrary(u)?,
                                salt: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractHostFnContext {
                                executable: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                salt: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::and_all(
                                &[
                                    <<ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::BytesN<
                                        32,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for CreateContractHostFnContext {
                type Prototype = ArbitraryCreateContractHostFnContext;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryCreateContractHostFnContext>
                for CreateContractHostFnContext
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryCreateContractHostFnContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(CreateContractHostFnContext {
                        executable: soroban_sdk::IntoVal::into_val(&v.executable, env),
                        salt: soroban_sdk::IntoVal::into_val(&v.salt, env),
                    })
                }
            }
        };
        pub struct CreateContractWithConstructorHostFnContext {
            pub constructor_args: soroban_sdk::Vec<soroban_sdk::Val>,
            pub executable: ContractExecutable,
            pub salt: soroban_sdk::BytesN<32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CreateContractWithConstructorHostFnContext",
                    "constructor_args",
                    &self.constructor_args,
                    "executable",
                    &self.executable,
                    "salt",
                    &&self.salt,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn clone(&self) -> CreateContractWithConstructorHostFnContext {
                CreateContractWithConstructorHostFnContext {
                    constructor_args: ::core::clone::Clone::clone(&self.constructor_args),
                    executable: ::core::clone::Clone::clone(&self.executable),
                    salt: ::core::clone::Clone::clone(&self.salt),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CreateContractWithConstructorHostFnContext {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<soroban_sdk::Val>>;
                let _: ::core::cmp::AssertParamIsEq<ContractExecutable>;
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CreateContractWithConstructorHostFnContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn eq(&self, other: &CreateContractWithConstructorHostFnContext) -> bool {
                self.constructor_args == other.constructor_args
                    && self.executable == other.executable
                    && self.salt == other.salt
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn cmp(
                &self,
                other: &CreateContractWithConstructorHostFnContext,
            ) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.constructor_args, &other.constructor_args) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.salt, &other.salt)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for CreateContractWithConstructorHostFnContext {
            #[inline]
            fn partial_cmp(
                &self,
                other: &CreateContractWithConstructorHostFnContext,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(
                    &self.constructor_args,
                    &other.constructor_args,
                ) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(
                            &self.executable,
                            &other.executable,
                        ) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8; 164usize] =
            CreateContractWithConstructorHostFnContext::spec_xdr();
        impl CreateContractWithConstructorHostFnContext {
            pub const fn spec_xdr() -> [u8; 164usize] {
                *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0*CreateContractWithConstructorHostFnContext\0\0\0\0\0\x03\0\0\0\0\0\0\0\x10constructor_args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for CreateContractWithConstructorHostFnContext {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xd2;\xff\xe6\x97\xda;\x83c$F\x15Z\xf1r\xf4\xc18\xfda!\x0b\r\x87\x88\xa0\x9a\x08Yu\xccS";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8; 74usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            74usize,
            1usize,
        >(
            2,
            *b"\xd2;\xff\xe6\x97\xda;\x83c$F\x15Z\xf1r\xf4\xc18\xfda!\x0b\r\x87\x88\xa0\x9a\x08Yu\xccS",
            [<ContractExecutable as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>
            for CreateContractWithConstructorHostFnContext
        {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["constructor_args", "executable", "salt"];
                let mut vals: [Val; 3usize] = [Val::VOID.to_val(); 3usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals)
                    .map_err(|_| ConversionError)?;
                Ok(Self {
                    constructor_args: vals[0]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    executable: vals[1]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                    salt: vals[2]
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::ConversionError)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, CreateContractWithConstructorHostFnContext>
            for soroban_sdk::Val
        {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
                const KEYS: [&'static str; 3usize] = ["constructor_args", "executable", "salt"];
                let vals: [Val; 3usize] = [
                    (&val.constructor_args)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.executable)
                        .try_into_val(env)
                        .map_err(|_| ConversionError)?,
                    (&val.salt).try_into_val(env).map_err(|_| ConversionError)?,
                ];
                Ok(env
                    .map_new_from_slices(&KEYS, &vals)
                    .map_err(|_| ConversionError)?
                    .into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &CreateContractWithConstructorHostFnContext>
            for soroban_sdk::Val
        {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    CreateContractWithConstructorHostFnContext,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap>
            for CreateContractWithConstructorHostFnContext
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScMap,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != 3usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self {
                    constructor_args: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "constructor_args"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                    executable: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "executable"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                    salt: {
                        let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                            "salt"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into();
                        let idx = map
                            .binary_search_by_key(&key, |entry| entry.key.clone())
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        let rv: soroban_sdk::Val = (&map[idx].val.clone())
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        rv.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    },
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal>
            for CreateContractWithConstructorHostFnContext
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "constructor_args"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.constructor_args)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "executable"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.executable)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "salt"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.salt)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ])))
            }
        }
        impl TryFrom<CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: &CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }
        impl TryFrom<CreateContractWithConstructorHostFnContext> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(
                val: CreateContractWithConstructorHostFnContext,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub struct ArbitraryCreateContractWithConstructorHostFnContext {
                constructor_args: <soroban_sdk::Vec<
                    soroban_sdk::Val,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                executable: <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                salt: <soroban_sdk::BytesN<
                    32,
                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ArbitraryCreateContractWithConstructorHostFnContext",
                        "constructor_args",
                        &self.constructor_args,
                        "executable",
                        &self.executable,
                        "salt",
                        &&self.salt,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn clone(&self) -> ArbitraryCreateContractWithConstructorHostFnContext {
                    ArbitraryCreateContractWithConstructorHostFnContext {
                        constructor_args: ::core::clone::Clone::clone(&self.constructor_args),
                        executable: ::core::clone::Clone::clone(&self.executable),
                        salt: ::core::clone::Clone::clone(&self.salt),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::Vec<
                            soroban_sdk::Val,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryCreateContractWithConstructorHostFnContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn eq(&self, other: &ArbitraryCreateContractWithConstructorHostFnContext) -> bool {
                    self.constructor_args == other.constructor_args
                        && self.executable == other.executable
                        && self.salt == other.salt
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn cmp(
                    &self,
                    other: &ArbitraryCreateContractWithConstructorHostFnContext,
                ) -> ::core::cmp::Ordering {
                    match ::core::cmp::Ord::cmp(&self.constructor_args, &other.constructor_args) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(&self.salt, &other.salt)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryCreateContractWithConstructorHostFnContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryCreateContractWithConstructorHostFnContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.constructor_args,
                        &other.constructor_args,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.executable,
                                &other.executable,
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext:
                    ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary>
                    for ArbitraryCreateContractWithConstructorHostFnContext
                {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
                                    if count.get() > 0 {
                                        return Err(arbitrary::Error::NotEnoughData);
                                    }
                                    count.set(count.get() + 1);
                                    Ok(())
                                })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                                constructor_args: arbitrary::Arbitrary::arbitrary(u)?,
                                executable: arbitrary::Arbitrary::arbitrary(u)?,
                                salt: arbitrary::Arbitrary::arbitrary(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
                                    count.set(count.get() - 1);
                                });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
                                    if count.get() > 0 {
                                        return Err(arbitrary::Error::NotEnoughData);
                                    }
                                    count.set(count.get() + 1);
                                    Ok(())
                                })?;
                        }
                        let result = (|| {
                            Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                                constructor_args: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                executable: arbitrary::Arbitrary::arbitrary(&mut u)?,
                                salt: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                            })
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext
                                .with(|count| {
                                    count.set(count.get() - 1);
                                });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::and_all(
                                &[
                                    <<soroban_sdk::Vec<
                                        soroban_sdk::Val,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<ContractExecutable as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                    <<soroban_sdk::BytesN<
                                        32,
                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                        depth,
                                    ),
                                ],
                            )
                        })
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary
                for CreateContractWithConstructorHostFnContext
            {
                type Prototype = ArbitraryCreateContractWithConstructorHostFnContext;
            }
            impl
                soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    ArbitraryCreateContractWithConstructorHostFnContext,
                > for CreateContractWithConstructorHostFnContext
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryCreateContractWithConstructorHostFnContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(CreateContractWithConstructorHostFnContext {
                        constructor_args: soroban_sdk::IntoVal::into_val(&v.constructor_args, env),
                        executable: soroban_sdk::IntoVal::into_val(&v.executable, env),
                        salt: soroban_sdk::IntoVal::into_val(&v.salt, env),
                    })
                }
            }
        };
        pub enum Context {
            Contract(ContractContext),
            CreateContractHostFn(CreateContractHostFnContext),
            CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Context {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Context::Contract(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
                    }
                    Context::CreateContractHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractHostFn",
                            &__self_0,
                        )
                    }
                    Context::CreateContractWithCtorHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractWithCtorHostFn",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Context {
            #[inline]
            fn clone(&self) -> Context {
                match self {
                    Context::Contract(__self_0) => {
                        Context::Contract(::core::clone::Clone::clone(__self_0))
                    }
                    Context::CreateContractHostFn(__self_0) => {
                        Context::CreateContractHostFn(::core::clone::Clone::clone(__self_0))
                    }
                    Context::CreateContractWithCtorHostFn(__self_0) => {
                        Context::CreateContractWithCtorHostFn(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Context {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ContractContext>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractHostFnContext>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractWithConstructorHostFnContext>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Context {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Context {
            #[inline]
            fn eq(&self, other: &Context) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Context::Contract(__self_0), Context::Contract(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Context::CreateContractHostFn(__self_0),
                            Context::CreateContractHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Context::CreateContractWithCtorHostFn(__self_0),
                            Context::CreateContractWithCtorHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Context {
            #[inline]
            fn cmp(&self, other: &Context) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (Context::Contract(__self_0), Context::Contract(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (
                            Context::CreateContractHostFn(__self_0),
                            Context::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            Context::CreateContractWithCtorHostFn(__self_0),
                            Context::CreateContractWithCtorHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Context {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Context,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (Context::Contract(__self_0), Context::Contract(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (
                        Context::CreateContractHostFn(__self_0),
                        Context::CreateContractHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        Context::CreateContractWithCtorHostFn(__self_0),
                        Context::CreateContractWithCtorHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CONTEXT: [u8; 244usize] = Context::spec_xdr();
        impl Context {
            pub const fn spec_xdr() -> [u8; 244usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07Context\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for Context {
            const SPEC_TYPE_ID: [u8; 32] = *b"\r\xb6\x0b\xec\x8f\xd04l1\xb3-\xa0{\x90\xa3\xc2\xab\x93\xd4\x82x\xe1_\x8a\xa8N?.\xcd\xc1\xfc\x08";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CONTEXT: [u8; 138usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            138usize,
            3usize,
        >(
            2,
            *b"\r\xb6\x0b\xec\x8f\xd04l1\xb3-\xa0{\x90\xa3\xc2\xab\x93\xd4\x82x\xe1_\x8a\xa8N?.\xcd\xc1\xfc\x08",
            [
                <ContractContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractWithConstructorHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Context {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &[
                    "Contract",
                    "CreateContractHostFn",
                    "CreateContractWithCtorHostFn",
                ];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Contract(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        1 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractHostFn(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        2 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractWithCtorHostFn(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        _ => Err(soroban_sdk::ConversionError {})?,
                    },
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, Context> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &Context,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    Context::Contract(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Contract")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Context::CreateContractHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?
                                .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Context::CreateContractWithCtorHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(
                                env,
                                &"CreateContractWithCtorHostFn",
                            )?
                            .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Context> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&Context,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Context>>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for Context {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVec,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let vec = val;
                let mut iter = vec.iter();
                let discriminant: soroban_sdk::xdr::ScSymbol = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .clone()
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_utf8_string()?;
                Ok(match discriminant_name {
                    "Contract" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Contract(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractWithCtorHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractWithCtorHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Context {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&Context> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Context) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    Context::Contract(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    Context::CreateContractHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractHostFn"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    Context::CreateContractWithCtorHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractWithCtorHostFn"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl TryFrom<Context> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Context) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&Context> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Context) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<Context> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Context) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryContext {
                Contract(
                    <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractHostFn(
                    <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractWithCtorHostFn(
                    <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryContext {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryContext::Contract(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Contract", &__self_0,
                            )
                        }
                        ArbitraryContext::CreateContractHostFn(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "CreateContractHostFn",
                                &__self_0,
                            )
                        }
                        ArbitraryContext::CreateContractWithCtorHostFn(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "CreateContractWithCtorHostFn",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryContext {
                #[inline]
                fn clone(&self) -> ArbitraryContext {
                    match self {
                        ArbitraryContext::Contract(__self_0) => {
                            ArbitraryContext::Contract(::core::clone::Clone::clone(__self_0))
                        }
                        ArbitraryContext::CreateContractHostFn(__self_0) => {
                            ArbitraryContext::CreateContractHostFn(::core::clone::Clone::clone(
                                __self_0,
                            ))
                        }
                        ArbitraryContext::CreateContractWithCtorHostFn(__self_0) => {
                            ArbitraryContext::CreateContractWithCtorHostFn(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryContext {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryContext {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryContext {
                #[inline]
                fn eq(&self, other: &ArbitraryContext) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryContext::Contract(__self_0),
                                ArbitraryContext::Contract(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryContext::CreateContractHostFn(__self_0),
                                ArbitraryContext::CreateContractHostFn(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryContext::CreateContractWithCtorHostFn(__self_0),
                                ArbitraryContext::CreateContractWithCtorHostFn(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryContext {
                #[inline]
                fn cmp(&self, other: &ArbitraryContext) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryContext::Contract(__self_0),
                                ArbitraryContext::Contract(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryContext::CreateContractHostFn(__self_0),
                                ArbitraryContext::CreateContractHostFn(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryContext::CreateContractWithCtorHostFn(__self_0),
                                ArbitraryContext::CreateContractWithCtorHostFn(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryContext {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryContext,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryContext::Contract(__self_0),
                            ArbitraryContext::Contract(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryContext::CreateContractHostFn(__self_0),
                            ArbitraryContext::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryContext::CreateContractWithCtorHostFn(__self_0),
                            ArbitraryContext::CreateContractWithCtorHostFn(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryContext: ::std::thread::LocalKey<
                    std::cell::Cell<u32>,
                > = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContext {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContext::Contract(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    1u64 => ArbitraryContext::CreateContractHostFn(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    2u64 => ArbitraryContext::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContext::Contract(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    1u64 => ArbitraryContext::CreateContractHostFn(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    2u64 => ArbitraryContext::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContext.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::and(
                            <u32 as arbitrary::Arbitrary>::size_hint(depth),
                            arbitrary::size_hint::recursion_guard(depth, |depth| {
                                arbitrary::size_hint::or_all(
                                        &[
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<ContractContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                        ],
                                    )
                            }),
                        )
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Context {
                type Prototype = ArbitraryContext;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContext> for Context {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryContext,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryContext::Contract(field_0) => {
                            Context::Contract(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                        ArbitraryContext::CreateContractHostFn(field_0) => {
                            Context::CreateContractHostFn(soroban_sdk::IntoVal::into_val(
                                field_0, env,
                            ))
                        }
                        ArbitraryContext::CreateContractWithCtorHostFn(field_0) => {
                            Context::CreateContractWithCtorHostFn(soroban_sdk::IntoVal::into_val(
                                field_0, env,
                            ))
                        }
                    })
                }
            }
        };
        pub enum ContractExecutable {
            Wasm(soroban_sdk::BytesN<32>),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ContractExecutable {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ContractExecutable::Wasm(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ContractExecutable {
            #[inline]
            fn clone(&self) -> ContractExecutable {
                match self {
                    ContractExecutable::Wasm(__self_0) => {
                        ContractExecutable::Wasm(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ContractExecutable {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ContractExecutable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ContractExecutable {
            #[inline]
            fn eq(&self, other: &ContractExecutable) -> bool {
                match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ContractExecutable {
            #[inline]
            fn cmp(&self, other: &ContractExecutable) -> ::core::cmp::Ordering {
                match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ContractExecutable {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ContractExecutable,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                }
            }
        }
        pub static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; 68usize] =
            ContractExecutable::spec_xdr();
        impl ContractExecutable {
            pub const fn spec_xdr() -> [u8; 68usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x12ContractExecutable\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 "
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for ContractExecutable {
            const SPEC_TYPE_ID: [u8; 32] = *b"^\xbe34\xd8\x99\x84\x91\x81\x9fu\x9fu\x05\xb8\xb4\x14\x95\xb7\x9d|\x06$\x04y\xe9\"\xb9\x14\xfc\xf9\x85";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_CONTRACTEXECUTABLE: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"^\xbe34\xd8\x99\x84\x91\x81\x9fu\x9fu\x05\xb8\xb4\x14\x95\xb7\x9d|\x06$\x04y\xe9\"\xb9\x14\xfc\xf9\x85",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractExecutable {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &["Wasm"];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Wasm(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        _ => Err(soroban_sdk::ConversionError {})?,
                    },
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &ContractExecutable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    ContractExecutable::Wasm(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Wasm")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ContractExecutable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&ContractExecutable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutable>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for ContractExecutable {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVec,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let vec = val;
                let mut iter = vec.iter();
                let discriminant: soroban_sdk::xdr::ScSymbol = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .clone()
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_utf8_string()?;
                Ok(match discriminant_name {
                    "Wasm" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Wasm(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for ContractExecutable {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&ContractExecutable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    ContractExecutable::Wasm(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Wasm"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl TryFrom<ContractExecutable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&ContractExecutable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<ContractExecutable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: ContractExecutable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryContractExecutable {
                Wasm(
                    <soroban_sdk::BytesN<
                        32,
                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryContractExecutable {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryContractExecutable::Wasm(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryContractExecutable {
                #[inline]
                fn clone(&self) -> ArbitraryContractExecutable {
                    match self {
                        ArbitraryContractExecutable::Wasm(__self_0) => {
                            ArbitraryContractExecutable::Wasm(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryContractExecutable {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryContractExecutable {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryContractExecutable {
                #[inline]
                fn eq(&self, other: &ArbitraryContractExecutable) -> bool {
                    match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => __self_0 == __arg1_0,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryContractExecutable {
                #[inline]
                fn cmp(&self, other: &ArbitraryContractExecutable) -> ::core::cmp::Ordering {
                    match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryContractExecutable {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryContractExecutable,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryContractExecutable: ::std::thread::LocalKey<
                    std::cell::Cell<u32>,
                > = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContractExecutable {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 1u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContractExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 1u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryContractExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryContractExecutable.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::and(
                            <u32 as arbitrary::Arbitrary>::size_hint(depth),
                            arbitrary::size_hint::recursion_guard(depth, |depth| {
                                arbitrary::size_hint::or_all(
                                        &[
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<soroban_sdk::BytesN<
                                                        32,
                                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                        ],
                                    )
                            }),
                        )
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for ContractExecutable {
                type Prototype = ArbitraryContractExecutable;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContractExecutable> for ContractExecutable {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryContractExecutable,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryContractExecutable::Wasm(field_0) => {
                            ContractExecutable::Wasm(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                    })
                }
            }
        };
        pub enum InvokerContractAuthEntry {
            Contract(SubContractInvocation),
            CreateContractHostFn(CreateContractHostFnContext),
            CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InvokerContractAuthEntry {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    InvokerContractAuthEntry::Contract(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
                    }
                    InvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractHostFn",
                            &__self_0,
                        )
                    }
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractWithCtorHostFn",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for InvokerContractAuthEntry {
            #[inline]
            fn clone(&self) -> InvokerContractAuthEntry {
                match self {
                    InvokerContractAuthEntry::Contract(__self_0) => {
                        InvokerContractAuthEntry::Contract(::core::clone::Clone::clone(__self_0))
                    }
                    InvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        InvokerContractAuthEntry::CreateContractHostFn(::core::clone::Clone::clone(
                            __self_0,
                        ))
                    }
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InvokerContractAuthEntry {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<SubContractInvocation>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractHostFnContext>;
                let _: ::core::cmp::AssertParamIsEq<CreateContractWithConstructorHostFnContext>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InvokerContractAuthEntry {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InvokerContractAuthEntry {
            #[inline]
            fn eq(&self, other: &InvokerContractAuthEntry) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            InvokerContractAuthEntry::Contract(__self_0),
                            InvokerContractAuthEntry::Contract(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            InvokerContractAuthEntry::CreateContractHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InvokerContractAuthEntry {
            #[inline]
            fn cmp(&self, other: &InvokerContractAuthEntry) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (
                            InvokerContractAuthEntry::Contract(__self_0),
                            InvokerContractAuthEntry::Contract(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            InvokerContractAuthEntry::CreateContractHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                            InvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InvokerContractAuthEntry {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InvokerContractAuthEntry,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (
                        InvokerContractAuthEntry::Contract(__self_0),
                        InvokerContractAuthEntry::Contract(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        InvokerContractAuthEntry::CreateContractHostFn(__self_0),
                        InvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; 268usize] =
            InvokerContractAuthEntry::spec_xdr();
        impl InvokerContractAuthEntry {
            pub const fn spec_xdr() -> [u8; 268usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x18InvokerContractAuthEntry\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for InvokerContractAuthEntry {
            const SPEC_TYPE_ID: [u8; 32] = *b"\xf0{\xa6\xe9r\xf3\x10\xf6\x0b)\xdb\x8e\r\xea\xe0\xa0\x89\xca\x1a\x1c\x12\xf8\x8f'K\xda\x9b\x87\xab\xaa\xf8=";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; 138usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            138usize,
            3usize,
        >(
            2,
            *b"\xf0{\xa6\xe9r\xf3\x10\xf6\x0b)\xdb\x8e\r\xea\xe0\xa0\x89\xca\x1a\x1c\x12\xf8\x8f'K\xda\x9b\x87\xab\xaa\xf8=",
            [
                <SubContractInvocation as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
                <CreateContractWithConstructorHostFnContext as soroban_sdk::spec_shaking::SpecTypeId>::SPEC_TYPE_ID,
            ],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for InvokerContractAuthEntry {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &[
                    "Contract",
                    "CreateContractHostFn",
                    "CreateContractWithCtorHostFn",
                ];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Contract(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        1 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractHostFn(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        2 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::CreateContractWithCtorHostFn(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        _ => Err(soroban_sdk::ConversionError {})?,
                    },
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, InvokerContractAuthEntry> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &InvokerContractAuthEntry,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    InvokerContractAuthEntry::Contract(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Contract")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    InvokerContractAuthEntry::CreateContractHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?
                                .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(
                                env,
                                &"CreateContractWithCtorHostFn",
                            )?
                            .to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &InvokerContractAuthEntry> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&InvokerContractAuthEntry,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    InvokerContractAuthEntry,
                >>::try_from_val(env, *val)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec>
            for InvokerContractAuthEntry
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVec,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let vec = val;
                let mut iter = vec.iter();
                let discriminant: soroban_sdk::xdr::ScSymbol = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .clone()
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_utf8_string()?;
                Ok(match discriminant_name {
                    "Contract" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Contract(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "CreateContractWithCtorHostFn" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::CreateContractWithCtorHostFn(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal>
            for InvokerContractAuthEntry
        {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&InvokerContractAuthEntry> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    InvokerContractAuthEntry::Contract(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Contract"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    InvokerContractAuthEntry::CreateContractHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractHostFn"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    InvokerContractAuthEntry::CreateContractWithCtorHostFn(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "CreateContractWithCtorHostFn"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl TryFrom<InvokerContractAuthEntry> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&InvokerContractAuthEntry> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<InvokerContractAuthEntry> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: InvokerContractAuthEntry) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryInvokerContractAuthEntry {
                Contract(
                    <SubContractInvocation as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractHostFn(
                    <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                CreateContractWithCtorHostFn(
                    <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryInvokerContractAuthEntry::Contract(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Contract", &__self_0,
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "CreateContractHostFn",
                                &__self_0,
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            __self_0,
                        ) => ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractWithCtorHostFn",
                            &__self_0,
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn clone(&self) -> ArbitraryInvokerContractAuthEntry {
                    match self {
                        ArbitraryInvokerContractAuthEntry::Contract(__self_0) => {
                            ArbitraryInvokerContractAuthEntry::Contract(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                            ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            __self_0,
                        ) => ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryInvokerContractAuthEntry {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <SubContractInvocation as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                    let _: ::core::cmp::AssertParamIsEq<
                        <CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryInvokerContractAuthEntry {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn eq(&self, other: &ArbitraryInvokerContractAuthEntry) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryInvokerContractAuthEntry::Contract(__self_0),
                                ArbitraryInvokerContractAuthEntry::Contract(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0),
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __self_0,
                                ),
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __arg1_0,
                                ),
                            ) => __self_0 == __arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn cmp(&self, other: &ArbitraryInvokerContractAuthEntry) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryInvokerContractAuthEntry::Contract(__self_0),
                                ArbitraryInvokerContractAuthEntry::Contract(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0),
                                ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            (
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __self_0,
                                ),
                                ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                    __arg1_0,
                                ),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryInvokerContractAuthEntry {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryInvokerContractAuthEntry,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryInvokerContractAuthEntry::Contract(__self_0),
                            ArbitraryInvokerContractAuthEntry::Contract(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0),
                            ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                __self_0,
                            ),
                            ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                __arg1_0,
                            ),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry: ::std::thread::LocalKey<
                    std::cell::Cell<u32>,
                > = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryInvokerContractAuthEntry {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(
                                    <u32 as arbitrary::Arbitrary>::arbitrary(u)?,
                                ) * 3u64) >> 32
                                {
                                    0u64 => {
                                        ArbitraryInvokerContractAuthEntry::Contract(
                                            arbitrary::Arbitrary::arbitrary(u)?,
                                        )
                                    }
                                    1u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                            arbitrary::Arbitrary::arbitrary(u)?,
                                        )
                                    }
                                    2u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                            arbitrary::Arbitrary::arbitrary(u)?,
                                        )
                                    }
                                    _ => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(
                                    <u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?,
                                ) * 3u64) >> 32
                                {
                                    0u64 => {
                                        ArbitraryInvokerContractAuthEntry::Contract(
                                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                        )
                                    }
                                    1u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                        )
                                    }
                                    2u64 => {
                                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                        )
                                    }
                                    _ => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryInvokerContractAuthEntry.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::and(
                            <u32 as arbitrary::Arbitrary>::size_hint(depth),
                            arbitrary::size_hint::recursion_guard(depth, |depth| {
                                arbitrary::size_hint::or_all(
                                        &[
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<SubContractInvocation as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<CreateContractWithConstructorHostFnContext as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                        ],
                                    )
                            }),
                        )
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for InvokerContractAuthEntry {
                type Prototype = ArbitraryInvokerContractAuthEntry;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryInvokerContractAuthEntry>
                for InvokerContractAuthEntry
            {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryInvokerContractAuthEntry,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryInvokerContractAuthEntry::Contract(field_0) => {
                            InvokerContractAuthEntry::Contract(soroban_sdk::IntoVal::into_val(
                                field_0, env,
                            ))
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(field_0) => {
                            InvokerContractAuthEntry::CreateContractHostFn(
                                soroban_sdk::IntoVal::into_val(field_0, env),
                            )
                        }
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            field_0,
                        ) => InvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            soroban_sdk::IntoVal::into_val(field_0, env),
                        ),
                    })
                }
            }
        };
        pub enum Executable {
            Wasm(soroban_sdk::BytesN<32>),
            StellarAsset,
            Account,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Executable {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Executable::Wasm(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                    }
                    Executable::StellarAsset => {
                        ::core::fmt::Formatter::write_str(f, "StellarAsset")
                    }
                    Executable::Account => ::core::fmt::Formatter::write_str(f, "Account"),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Executable {
            #[inline]
            fn clone(&self) -> Executable {
                match self {
                    Executable::Wasm(__self_0) => {
                        Executable::Wasm(::core::clone::Clone::clone(__self_0))
                    }
                    Executable::StellarAsset => Executable::StellarAsset,
                    Executable::Account => Executable::Account,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Executable {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<soroban_sdk::BytesN<32>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Executable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Executable {
            #[inline]
            fn eq(&self, other: &Executable) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Executable::Wasm(__self_0), Executable::Wasm(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Executable {
            #[inline]
            fn cmp(&self, other: &Executable) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (Executable::Wasm(__self_0), Executable::Wasm(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Executable {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Executable,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (Executable::Wasm(__self_0), Executable::Wasm(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        pub static __SPEC_XDR_TYPE_EXECUTABLE: [u8; 104usize] = Executable::spec_xdr();
        impl Executable {
            pub const fn spec_xdr() -> [u8; 104usize] {
                *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\nExecutable\0\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\0\0\0\0\0\0\0\0\x0cStellarAsset\0\0\0\0\0\0\0\0\0\0\0\x07Account\0"
            }
        }
        impl soroban_sdk::spec_shaking::SpecTypeId for Executable {
            const SPEC_TYPE_ID: [u8; 32] = *b"L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e";
        }
        #[allow(non_upper_case_globals)]
        pub static __SPEC_GRAPH_TYPE_EXECUTABLE: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
            42usize,
            0usize,
        >(
            2,
            *b"L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e",
            [],
        );
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Executable {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
                const CASES: &'static [&'static str] = &["Wasm", "StellarAsset", "Account"];
                let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
                let mut iter = vec.try_iter();
                let discriminant: soroban_sdk::Symbol = iter
                    .next()
                    .ok_or(soroban_sdk::ConversionError)??
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?;
                Ok(
                    match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                        as usize
                    {
                        0 => {
                            if iter.len() > 1usize {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Wasm(
                                iter.next()
                                    .ok_or(soroban_sdk::ConversionError)??
                                    .try_into_val(env)?,
                            )
                        }
                        1 => {
                            if iter.len() > 0 {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::StellarAsset
                        }
                        2 => {
                            if iter.len() > 0 {
                                return Err(soroban_sdk::ConversionError);
                            }
                            Self::Account
                        }
                        _ => Err(soroban_sdk::ConversionError {})?,
                    },
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, Executable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &Executable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::{TryFromVal, TryIntoVal};
                match val {
                    Executable::Wasm(ref value0) => {
                        let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                            soroban_sdk::Symbol::try_from_val(env, &"Wasm")?.to_val(),
                            value0.try_into_val(env)?,
                        );
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Executable::StellarAsset => {
                        let tup: (soroban_sdk::Val,) =
                            (soroban_sdk::Symbol::try_from_val(env, &"StellarAsset")?.to_val(),);
                        tup.try_into_val(env).map_err(Into::into)
                    }
                    Executable::Account => {
                        let tup: (soroban_sdk::Val,) =
                            (soroban_sdk::Symbol::try_from_val(env, &"Account")?.to_val(),);
                        tup.try_into_val(env).map_err(Into::into)
                    }
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Executable> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&Executable,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Executable>>::try_from_val(
                    env, *val,
                )
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for Executable {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVec,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let vec = val;
                let mut iter = vec.iter();
                let discriminant: soroban_sdk::xdr::ScSymbol = iter
                    .next()
                    .ok_or(soroban_sdk::xdr::Error::Invalid)?
                    .clone()
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_utf8_string()?;
                Ok(match discriminant_name {
                    "Wasm" => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        let rv0: soroban_sdk::Val = iter
                            .next()
                            .ok_or(soroban_sdk::xdr::Error::Invalid)?
                            .try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                        Self::Wasm(
                            rv0.try_into_val(env)
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                    }
                    "StellarAsset" => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::StellarAsset
                    }
                    "Account" => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::xdr::Error::Invalid);
                        }
                        Self::Account
                    }
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for Executable {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::xdr::ScVal,
            ) -> Result<Self, soroban_sdk::xdr::Error> {
                if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }
        impl TryFrom<&Executable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                extern crate alloc;
                Ok(match val {
                    Executable::Wasm(value0) => (
                        soroban_sdk::xdr::ScSymbol(
                            "Wasm"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        ),
                        value0,
                    )
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    Executable::StellarAsset => {
                        let symbol = soroban_sdk::xdr::ScSymbol(
                            "StellarAsset"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        );
                        let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                        (val,)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    }
                    Executable::Account => {
                        let symbol = soroban_sdk::xdr::ScSymbol(
                            "Account"
                                .try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        );
                        let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                        (val,)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                    }
                })
            }
        }
        impl TryFrom<Executable> for soroban_sdk::xdr::ScVec {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        impl TryFrom<&Executable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: &Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
            }
        }
        impl TryFrom<Executable> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(val: Executable) -> Result<Self, soroban_sdk::xdr::Error> {
                (&val).try_into()
            }
        }
        const _: () = {
            use soroban_sdk::testutils::arbitrary::arbitrary;
            use soroban_sdk::testutils::arbitrary::std;
            pub enum ArbitraryExecutable {
                Wasm(
                    <soroban_sdk::BytesN<
                        32,
                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                ),
                StellarAsset,
                Account,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ArbitraryExecutable {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ArbitraryExecutable::Wasm(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                        }
                        ArbitraryExecutable::StellarAsset => {
                            ::core::fmt::Formatter::write_str(f, "StellarAsset")
                        }
                        ArbitraryExecutable::Account => {
                            ::core::fmt::Formatter::write_str(f, "Account")
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ArbitraryExecutable {
                #[inline]
                fn clone(&self) -> ArbitraryExecutable {
                    match self {
                        ArbitraryExecutable::Wasm(__self_0) => {
                            ArbitraryExecutable::Wasm(::core::clone::Clone::clone(__self_0))
                        }
                        ArbitraryExecutable::StellarAsset => ArbitraryExecutable::StellarAsset,
                        ArbitraryExecutable::Account => ArbitraryExecutable::Account,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ArbitraryExecutable {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<
                        <soroban_sdk::BytesN<
                            32,
                        > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                    >;
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ArbitraryExecutable {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ArbitraryExecutable {
                #[inline]
                fn eq(&self, other: &ArbitraryExecutable) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ArbitraryExecutable::Wasm(__self_0),
                                ArbitraryExecutable::Wasm(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for ArbitraryExecutable {
                #[inline]
                fn cmp(&self, other: &ArbitraryExecutable) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                        ::core::cmp::Ordering::Equal => match (self, other) {
                            (
                                ArbitraryExecutable::Wasm(__self_0),
                                ArbitraryExecutable::Wasm(__arg1_0),
                            ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                            _ => ::core::cmp::Ordering::Equal,
                        },
                        cmp => cmp,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for ArbitraryExecutable {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &ArbitraryExecutable,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    match (self, other) {
                        (
                            ArbitraryExecutable::Wasm(__self_0),
                            ArbitraryExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                    }
                }
            }
            const _: () = {
                #[allow(non_upper_case_globals)]
                const RECURSIVE_COUNT_ArbitraryExecutable: ::std::thread::LocalKey<
                    std::cell::Cell<u32>,
                > = {
                    #[inline]
                    fn __init() -> std::cell::Cell<u32> {
                        std::cell::Cell::new(0)
                    }
                    unsafe {
                        ::std::thread::LocalKey::new(
                            const {
                                if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            (),
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                } else {
                                    |init| {
                                        #[thread_local]
                                        static VAL: ::std::thread::local_impl::LazyStorage<
                                            std::cell::Cell<u32>,
                                            !,
                                        > = ::std::thread::local_impl::LazyStorage::new();
                                        VAL.get_or_init(init, __init)
                                    }
                                }
                            },
                        )
                    }
                };
                #[automatically_derived]
                impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryExecutable {
                    fn arbitrary(
                        u: &mut arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    ),
                                    1u64 => ArbitraryExecutable::StellarAsset,
                                    2u64 => ArbitraryExecutable::Account,
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    fn arbitrary_take_rest(
                        mut u: arbitrary::Unstructured<'arbitrary>,
                    ) -> arbitrary::Result<Self> {
                        let guard_against_recursion = u.is_empty();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            })?;
                        }
                        let result = (|| {
                            Ok(
                                match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                    * 3u64)
                                    >> 32
                                {
                                    0u64 => ArbitraryExecutable::Wasm(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    ),
                                    1u64 => ArbitraryExecutable::StellarAsset,
                                    2u64 => ArbitraryExecutable::Account,
                                    _ => ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    ),
                                },
                            )
                        })();
                        if guard_against_recursion {
                            RECURSIVE_COUNT_ArbitraryExecutable.with(|count| {
                                count.set(count.get() - 1);
                            });
                        }
                        result
                    }
                    #[inline]
                    fn size_hint(depth: usize) -> (usize, Option<usize>) {
                        arbitrary::size_hint::and(
                            <u32 as arbitrary::Arbitrary>::size_hint(depth),
                            arbitrary::size_hint::recursion_guard(depth, |depth| {
                                arbitrary::size_hint::or_all(
                                        &[
                                            arbitrary::size_hint::and_all(
                                                &[
                                                    <<soroban_sdk::BytesN<
                                                        32,
                                                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                        depth,
                                                    ),
                                                ],
                                            ),
                                            arbitrary::size_hint::and_all(&[]),
                                            arbitrary::size_hint::and_all(&[]),
                                        ],
                                    )
                            }),
                        )
                    }
                }
            };
            impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for Executable {
                type Prototype = ArbitraryExecutable;
            }
            impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryExecutable> for Executable {
                type Error = soroban_sdk::ConversionError;
                fn try_from_val(
                    env: &soroban_sdk::Env,
                    v: &ArbitraryExecutable,
                ) -> std::result::Result<Self, Self::Error> {
                    Ok(match v {
                        ArbitraryExecutable::Wasm(field_0) => {
                            Executable::Wasm(soroban_sdk::IntoVal::into_val(field_0, env))
                        }
                        ArbitraryExecutable::StellarAsset => Executable::StellarAsset,
                        ArbitraryExecutable::Account => Executable::Account,
                    })
                }
            }
        };
    }
    extern crate test;
    #[rustc_test_marker = "test::test_native_void"]
    #[doc(hidden)]
    pub const test_native_void: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_native_void"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/tuples/src/lib.rs",
            start_line: 31usize,
            start_col: 8usize,
            end_line: 31usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_native_void()),
        ),
    };
    fn test_native_void() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        client.void_fn(&());
    }
    extern crate test;
    #[rustc_test_marker = "test::test_native_tuple1"]
    #[doc(hidden)]
    pub const test_native_tuple1: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_native_tuple1"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/tuples/src/lib.rs",
            start_line: 39usize,
            start_col: 8usize,
            end_line: 39usize,
            end_col: 26usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_native_tuple1()),
        ),
    };
    fn test_native_tuple1() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let result = client.tuple1(&(42u32,));
        match (&result, &(42u32,)) {
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
    extern crate test;
    #[rustc_test_marker = "test::test_native_tuple2"]
    #[doc(hidden)]
    pub const test_native_tuple2: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_native_tuple2"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/tuples/src/lib.rs",
            start_line: 48usize,
            start_col: 8usize,
            end_line: 48usize,
            end_col: 26usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_native_tuple2()),
        ),
    };
    fn test_native_tuple2() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let result = client.tuple2(&(42u32, -100i64));
        match (&result, &(42u32, -100i64)) {
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
    extern crate test;
    #[rustc_test_marker = "test::test_wasm_void"]
    #[doc(hidden)]
    pub const test_wasm_void: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wasm_void"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/tuples/src/lib.rs",
            start_line: 57usize,
            start_col: 8usize,
            end_line: 57usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wasm_void()),
        ),
    };
    fn test_wasm_void() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);
        client.void_fn(&());
    }
    extern crate test;
    #[rustc_test_marker = "test::test_wasm_tuple1"]
    #[doc(hidden)]
    pub const test_wasm_tuple1: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wasm_tuple1"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/tuples/src/lib.rs",
            start_line: 65usize,
            start_col: 8usize,
            end_line: 65usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wasm_tuple1()),
        ),
    };
    fn test_wasm_tuple1() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);
        let result = client.tuple1(&(42u32,));
        match (&result, &(42u32,)) {
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
    extern crate test;
    #[rustc_test_marker = "test::test_wasm_tuple2"]
    #[doc(hidden)]
    pub const test_wasm_tuple2: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wasm_tuple2"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/tuples/src/lib.rs",
            start_line: 74usize,
            start_col: 8usize,
            end_line: 74usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wasm_tuple2()),
        ),
    };
    fn test_wasm_tuple2() {
        let e = Env::default();
        let contract_id = e.register(wasm::WASM, ());
        let client = wasm::Client::new(&e, &contract_id);
        let result = client.tuple2(&(42u32, -100i64));
        match (&result, &(42u32, -100i64)) {
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_native_tuple1,
        &test_native_tuple2,
        &test_native_void,
        &test_wasm_tuple1,
        &test_wasm_tuple2,
        &test_wasm_void,
    ])
}
