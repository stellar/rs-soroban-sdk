#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
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
                self.env.mock_all_auths();
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
                self.env.mock_all_auths();
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
                self.env.mock_all_auths();
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
pub fn __Contract__void_fn__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::void_fn(
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
pub fn __Contract__tuple1__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::tuple1(
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
pub fn __Contract__tuple2__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    <_ as soroban_sdk::IntoVal<
        soroban_sdk::Env,
        soroban_sdk::Val,
    >>::into_val(
        #[allow(deprecated)]
        &<Contract>::tuple2(
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
#[cfg(test)]
mod test {
    use crate::{Contract, ContractClient};
    use soroban_sdk::Env;
    mod wasm {
        pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01\x1f\x05`\x01~\x01~`\x02~~\x01~`\x03~~~\x01~`\x03~\x7f\x7f\x00`\x02\x7f\x7f\x01~\x02\x19\x04\x01i\x012\x00\x00\x01i\x011\x00\x00\x01v\x01g\x00\x01\x01v\x01h\x00\x02\x03\x06\x05\x00\x00\x03\x04\x00\x05\x03\x01\x00\x10\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x07E\x07\x06memory\x02\x00\x07void_fn\x00\x04\x06tuple1\x00\x05\x06tuple2\x00\x08\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xc8\x03\x05\x13\x00\x02@ \x00B\xff\x01\x83B\x02Q\r\x00\x00\x0bB\x02\x0bx\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00\x02@\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00 \x01B\x027\x03\x08 \x00 \x01A\x08jA\x01\x10\x86\x80\x80\x80\x00 \x01)\x03\x08\"\x00B\xff\x01\x83B\x04Q\r\x01\x0b\x00\x0b \x01 \x00B\x84\x80\x80\x80p\x837\x03\x08 \x01A\x08jA\x01\x10\x87\x80\x80\x80\x00!\x00 \x01A\x10j$\x80\x80\x80\x80\x00 \x00\x0b\x1d\x00 \x00 \x01\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84\x10\x83\x80\x80\x80\x00\x1a\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x82\x80\x80\x80\x00\x0b\xff\x01\x02\x02\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00\x02@\x02@\x02@ \x00B\xff\x01\x83B\xcb\x00R\r\x00A\x00!\x02\x02@\x03@ \x02A\x10F\r\x01 \x01 \x02jB\x027\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b \x00 \x01A\x02\x10\x86\x80\x80\x80\x00 \x01)\x03\x00\"\x03B\xff\x01\x83B\x04R\r\x00 \x01)\x03\x08\"\x00\xa7A\xff\x01q\"\x02A\xc1\x00F\r\x01 \x02A\x07G\r\x00 \x00B\x08\x87!\x00\x0c\x02\x0b\x00\x0b \x00\x10\x80\x80\x80\x80\x00!\x00\x0b\x02@\x02@ \x00B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00B\x08\x86B\x07\x84!\x00\x0c\x01\x0b \x00\x10\x81\x80\x80\x80\x00!\x00\x0b \x01 \x007\x03\x08 \x01 \x03B\x84\x80\x80\x80p\x837\x03\x00 \x01A\x02\x10\x87\x80\x80\x80\x00!\x00 \x01A\x10j$\x80\x80\x80\x80\x00 \x00\x0b\x0b\t\x01\x00A\x80\x80\xc0\x00\x0b\x00\x00\xcb\x01\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07void_fn\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x08void_arg\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06tuple1\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x03arg\x00\x00\x00\x03\xed\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x03\xed\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06tuple2\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x03arg\x00\x00\x00\x03\xed\x00\x00\x00\x02\x00\x00\x00\x04\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x03\xed\x00\x00\x00\x02\x00\x00\x00\x04\x00\x00\x00\x07\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x19\x00\x00\x00\x00\x00+\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.84.0\x00\x00";
        pub trait Contract {
            fn void_fn(env: soroban_sdk::Env, void_arg: ()) -> ();
            fn tuple1(env: soroban_sdk::Env, arg: (u32,)) -> (u32,);
            fn tuple2(env: soroban_sdk::Env, arg: (u32, i64)) -> (u32, i64);
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
                        self.env.mock_all_auths();
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
                        self.env.mock_all_auths();
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
                        self.env.mock_all_auths();
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
        ///Args is a type for building arg lists for functions defined in "Contract".
        pub struct Args;
        impl Args {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn void_fn<'i>(void_arg: &'i ()) -> (&'i (),) {
                (void_arg,)
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
    }
    extern crate test;
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
