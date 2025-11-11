#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Env};
extern crate alloc;
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
    pub fn num_list(env: Env, count: u32) -> soroban_sdk::Vec<u32> {
        let mut v1 = ::alloc::vec::Vec::new();
        (0..count).for_each(|i| v1.push(i));
        let mut v2 = ::soroban_sdk::Vec::new(&env);
        for i in v1 {
            v2.push_back(i);
        }
        v2
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__num_list__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_NUM_LIST: [u8; 56usize] = super::Contract::spec_xdr_num_list();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_num_list() -> [u8; 56usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08num_list\0\0\0\x01\0\0\0\0\0\0\0\x05count\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xea\0\0\0\x04"
    }
}
impl<'a> ContractClient<'a> {
    pub fn num_list(&self, count: &u32) -> soroban_sdk::Vec<u32> {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("num_list");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [count.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_num_list(
        &self,
        count: &u32,
    ) -> Result<
        Result<
            soroban_sdk::Vec<u32>,
            <soroban_sdk::Vec<u32> as soroban_sdk::TryFromVal<
                soroban_sdk::Env,
                soroban_sdk::Val,
            >>::Error,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("num_list");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [count.into_val(&self.env)]),
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
    pub fn num_list<'i>(count: &'i u32) -> (&'i u32,) {
        (count,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__num_list {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).num_list` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::num_list(
                env.clone(),
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).num_list` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
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
        invoke_raw(env, args[0usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).num_list` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__526e07cb9219443e6b3ef052e7872bf9e5bfee9b91633ed921601b5e93014bb7_ctor() {
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
                    __Contract__526e07cb9219443e6b3ef052e7872bf9e5bfee9b91633ed921601b5e93014bb7_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "num_list",
            #[allow(deprecated)]
            &__Contract__num_list::invoke_raw_slice,
        );
    }
}
mod test {
    use crate::{Contract, ContractClient};
    use soroban_sdk::{vec, Env};
    mod imported {
        pub const WASM: &[u8] = b"\0asm\x01\0\0\0\x01.\t`\0\x01~`\x02~~\x01~`\x02\x7f\x7f\0`\0\0`\x01\x7f\x01\x7f`\x03\x7f\x7f\x7f\0`\x01\x7f\0`\x01~\x01~`\x03\x7f\x7f\x7f\x01\x7f\x02\r\x02\x01v\x01_\0\0\x01v\x016\0\x01\x03\x10\x0f\x02\x03\x04\x03\x05\x06\x02\x07\x06\x06\x03\x06\x03\x03\x08\x05\x03\x01\0\x11\x06!\x04\x7f\x01A\x80\x80\xc0\0\x0b\x7f\0A\x80\x80\xc0\0\x0b\x7f\0A\x8c\x80\xc0\0\x0b\x7f\0A\x90\x80\xc0\0\x0b\x074\x05\x06memory\x02\0\x08num_list\0\t\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xd8\x0e\x0f\x85\x01\x01\x02\x7f\x02@\x02@ \x01\r\0A\x04!\x02\x0c\x01\x0bA\0-\0\x80\x80\xc0\x80\0\x1a\x10\x83\x80\x80\x80\0\x02@A\0(\x02\x84\x80\xc0\x80\0\"\x02A\x03j\"\x03 \x02I\r\0\x02@ \x03A|q\"\x02 \x01j\"\x03A\0(\x02\x88\x80\xc0\x80\0M\r\0 \x01\x10\x84\x80\x80\x80\0!\x02\x0c\x02\x0bA\0 \x036\x02\x84\x80\xc0\x80\0\x0c\x01\x0b\x10\x85\x80\x80\x80\0\0\x0b \0 \x016\x02\x04 \0 \x026\x02\0\x0bC\x01\x01\x7f\x02@\x02@A\0(\x02\x88\x80\xc0\x80\0\r\0?\0\"\0A\xff\xff\x03K\r\x01A\0 \0A\x10t\"\06\x02\x88\x80\xc0\x80\0A\0 \06\x02\x84\x80\xc0\x80\0\x0b\x0f\x0b\x10\x85\x80\x80\x80\0\0\x0b\x91\x01\x01\x04\x7f \0A\xff\xff\x03j\"\x01A\x80\x80|q!\x02 \x01A\x10v!\x03\x02@\x02@\x03@ \x03@\0A\x7fF\r\x01A\0A\0(\x02\x88\x80\xc0\x80\0 \x02j6\x02\x88\x80\xc0\x80\0\x10\x83\x80\x80\x80\0A\0(\x02\x84\x80\xc0\x80\0\"\x01A\x03j\"\x04 \x01I\r\x02 \x04A|q\"\x01 \0j\"\x04A\0(\x02\x88\x80\xc0\x80\0K\r\0\x0bA\0 \x046\x02\x84\x80\xc0\x80\0 \x01\x0f\x0b\x10\x8f\x80\x80\x80\0\0\x0b\x10\x85\x80\x80\x80\0\0\x0b\t\0\x10\x8e\x80\x80\x80\0\0\x0b\x94\x02\x01\x04\x7f#\x80\x80\x80\x80\0A\x10k\"\x03$\x80\x80\x80\x80\0\x02@\x02@\x02@ \x02(\x02\x04E\r\0\x02@ \x02(\x02\x08\"\x04\r\0 \x03A\x08j \x01\x10\x82\x80\x80\x80\0 \x03(\x02\x0c!\x04 \x03(\x02\x08!\x02\x0c\x02\x0b \x02(\x02\0!\x05\x10\x83\x80\x80\x80\0A\0(\x02\x84\x80\xc0\x80\0\"\x02A\x03j\"\x06 \x02I\r\x02\x02@\x02@ \x06A|q\"\x02 \x01j\"\x06A\0(\x02\x88\x80\xc0\x80\0M\r\0 \x01\x10\x84\x80\x80\x80\0!\x02\x0c\x01\x0bA\0 \x066\x02\x84\x80\xc0\x80\0\x0b\x02@\x02@ \x02\r\0A\0!\x02\x0c\x01\x0b \x02 \x05 \x04\x10\x90\x80\x80\x80\0\x1a\x0b \x01!\x04\x0c\x01\x0b \x03 \x01\x10\x82\x80\x80\x80\0 \x03(\x02\x04!\x04 \x03(\x02\0!\x02\x0b \0 \x02A\x04 \x02\x1b6\x02\x04 \0 \x02E6\x02\0 \0 \x04 \x01 \x02\x1b6\x02\x08 \x03A\x10j$\x80\x80\x80\x80\0\x0f\x0b\x10\x85\x80\x80\x80\0\0\x0b\xec\x01\x01\x06\x7f#\x80\x80\x80\x80\0A k\"\x01$\x80\x80\x80\x80\0A\0!\x02\x02@\x02@\x02@\x02@ \0(\x02\0\"\x03A\x7fF\r\0 \x03A\x01t \x03A\x01j \x03A\0J\x1b\"\x04A\xff\xff\xff\xff\x03M\r\x01\x0b\x0c\x01\x0b \x04A\x04 \x04A\x04K\x1b\"\x05A\x02t\"\x04A\xfc\xff\xff\xff\x07K\r\0\x02@\x02@ \x03\r\0A\0!\x03\x0c\x01\x0b \x01 \x03A\x02t6\x02\x1c \x01 \0(\x02\x046\x02\x14A\x04!\x03\x0b \x01 \x036\x02\x18 \x01A\x08j \x04 \x01A\x14j\x10\x86\x80\x80\x80\0 \x01(\x02\x08A\x01G\r\x01 \x01(\x02\x10!\x06 \x01(\x02\x0c!\x02\x0b \x02 \x06\x10\x88\x80\x80\x80\0\0\x0b \x01(\x02\x0c!\x03 \0 \x056\x02\0 \0 \x036\x02\x04 \x01A j$\x80\x80\x80\x80\0\x0b\x19\0\x02@ \0\r\0\x10\x8c\x80\x80\x80\0\0\x0b \x01\x10\x8d\x80\x80\x80\0\0\x0b\xee\x01\x01\x06\x7f#\x80\x80\x80\x80\0A\x10k\"\x01$\x80\x80\x80\x80\0\x02@\x02@ \0B\xff\x01\x83B\x04R\r\0 \0B \x88\xa7!\x02A\0!\x03 \x01A\06\x02\x0c \x01B\x80\x80\x80\x80\xc0\07\x02\x04A\x01!\x04A\x04!\x05A\0!\x06\x02@\x03@ \x06 \x02O\r\x01\x02@ \x04A\x7fj \x01(\x02\x04G\r\0 \x01A\x04j\x10\x87\x80\x80\x80\0 \x01(\x02\x08!\x05\x0b \x05 \x03j \x066\x02\0 \x03A\x04j!\x03 \x01 \x046\x02\x0c \x04A\x01j!\x04 \x06 \x06 \x02Ij!\x06\x0c\0\x0b\x0b\x10\x80\x80\x80\x80\0!\0\x03@ \x03E\r\x02 \x03A|j!\x03 \0 \x055\x02\0B \x86B\x04\x84\x10\x81\x80\x80\x80\0!\0 \x05A\x04j!\x05\x0c\0\x0b\x0b\0\x0b \x01A\x10j$\x80\x80\x80\x80\0 \0\x0b\x0b\0 \0\x10\x8b\x80\x80\x80\0\0\x0b\t\0\x10\x8e\x80\x80\x80\0\0\x0b\t\0\x10\x8e\x80\x80\x80\0\0\x0b\x0b\0 \0\x10\x8a\x80\x80\x80\0\0\x0b\x03\0\0\x0b\t\0\x10\x85\x80\x80\x80\0\0\x0b\xa5\x05\x01\x08\x7f\x02@\x02@ \x02A\x10O\r\0 \0!\x03\x0c\x01\x0b\x02@ \0A\0 \0kA\x03q\"\x04j\"\x05 \0M\r\0 \x04A\x7fj!\x06 \0!\x03 \x01!\x07\x02@ \x04E\r\0 \x04!\x08 \0!\x03 \x01!\x07\x03@ \x03 \x07-\0\0:\0\0 \x07A\x01j!\x07 \x03A\x01j!\x03 \x08A\x7fj\"\x08\r\0\x0b\x0b \x06A\x07I\r\0\x03@ \x03 \x07-\0\0:\0\0 \x03A\x01j \x07A\x01j-\0\0:\0\0 \x03A\x02j \x07A\x02j-\0\0:\0\0 \x03A\x03j \x07A\x03j-\0\0:\0\0 \x03A\x04j \x07A\x04j-\0\0:\0\0 \x03A\x05j \x07A\x05j-\0\0:\0\0 \x03A\x06j \x07A\x06j-\0\0:\0\0 \x03A\x07j \x07A\x07j-\0\0:\0\0 \x07A\x08j!\x07 \x03A\x08j\"\x03 \x05G\r\0\x0b\x0b \x05 \x02 \x04k\"\x08A|q\"\x06j!\x03\x02@\x02@ \x01 \x04j\"\x07A\x03q\r\0 \x05 \x03O\r\x01 \x07!\x01\x03@ \x05 \x01(\x02\06\x02\0 \x01A\x04j!\x01 \x05A\x04j\"\x05 \x03I\r\0\x0c\x02\x0b\x0b \x05 \x03O\r\0 \x07A\x03t\"\x02A\x18q!\x04 \x07A|q\"\tA\x04j!\x01A\0 \x02kA\x18q!\n \t(\x02\0!\x02\x03@ \x05 \x02 \x04v \x01(\x02\0\"\x02 \ntr6\x02\0 \x01A\x04j!\x01 \x05A\x04j\"\x05 \x03I\r\0\x0b\x0b \x08A\x03q!\x02 \x07 \x06j!\x01\x0b\x02@ \x03 \x03 \x02j\"\x05O\r\0 \x02A\x7fj!\x08\x02@ \x02A\x07q\"\x07E\r\0\x03@ \x03 \x01-\0\0:\0\0 \x01A\x01j!\x01 \x03A\x01j!\x03 \x07A\x7fj\"\x07\r\0\x0b\x0b \x08A\x07I\r\0\x03@ \x03 \x01-\0\0:\0\0 \x03A\x01j \x01A\x01j-\0\0:\0\0 \x03A\x02j \x01A\x02j-\0\0:\0\0 \x03A\x03j \x01A\x03j-\0\0:\0\0 \x03A\x04j \x01A\x04j-\0\0:\0\0 \x03A\x05j \x01A\x05j-\0\0:\0\0 \x03A\x06j \x01A\x06j-\0\0:\0\0 \x03A\x07j \x01A\x07j-\0\0:\0\0 \x01A\x08j!\x01 \x03A\x08j\"\x03 \x05G\r\0\x0b\x0b \0\x0b\x0b\t\x01\0A\x80\x80\xc0\0\x0b\0\0G\x0econtractspecv0\0\0\0\0\0\0\0\0\0\0\0\x08num_list\0\0\0\x01\0\0\0\0\0\0\0\x05count\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xea\0\0\0\x04\0\x1e\x11contractenvmetav0\0\0\0\0\0\0\0\x17\0\0\0\0\0+\x0econtractmetav0\0\0\0\0\0\0\0\x05rsver\0\0\0\0\0\0\x061.84.0\0\0";
        pub trait Contract {
            fn num_list(env: soroban_sdk::Env, count: u32) -> soroban_sdk::Vec<u32>;
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
            pub fn num_list(&self, count: &u32) -> soroban_sdk::Vec<u32> {
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("num_list");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [count.into_val(&self.env)]),
                );
                if let Some(old_auth_manager) = old_auth_manager {
                    self.env.host().set_auth_manager(old_auth_manager).unwrap();
                }
                res
            }
            pub fn try_num_list(
                &self,
                count: &u32,
            ) -> Result<
                Result<
                    soroban_sdk::Vec<u32>,
                    <soroban_sdk::Vec<u32> as soroban_sdk::TryFromVal<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::Error,
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
                        const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("num_list");
                        SYMBOL
                    },
                    ::soroban_sdk::Vec::from_array(&self.env, [count.into_val(&self.env)]),
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
            pub fn num_list<'i>(count: &'i u32) -> (&'i u32,) {
                (count,)
            }
        }
    }
    mod native {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "test::native::test"]
        #[doc(hidden)]
        pub const test: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("test::native::test"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/alloc/src/test.rs",
                start_line: 16usize,
                start_col: 16usize,
                end_line: 16usize,
                end_col: 20usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test()),
            ),
        };
        fn test() {
            let e = Env::default();
            let contract_id = e.register(Contract, ());
            let client = ContractClient::new(&e, &contract_id);
            let list = client.num_list(&50);
            match (
                &list,
                &::soroban_sdk::Vec::from_array(
                    &e,
                    [
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                        40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                    ],
                ),
            ) {
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
            }
        }
    }
    mod wasm {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "test::wasm::test"]
        #[doc(hidden)]
        pub const test: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("test::wasm::test"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/alloc/src/test.rs",
                start_line: 16usize,
                start_col: 16usize,
                end_line: 16usize,
                end_col: 20usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test()),
            ),
        };
        fn test() {
            let e = Env::default();
            let contract_id = e.register(imported::WASM, ());
            let client = ContractClient::new(&e, &contract_id);
            let list = client.num_list(&50);
            match (
                &list,
                &::soroban_sdk::Vec::from_array(
                    &e,
                    [
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                        40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                    ],
                ),
            ) {
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
            }
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test, &test])
}
