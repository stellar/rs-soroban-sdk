#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use test_spec_lib::{
    EnumA, EnumB, EnumIntA, EnumIntB, ErrorA, ErrorB, ErrorC, EventA, EventB, StructA, StructB,
    StructTupleA, StructTupleB,
};
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
    pub fn create_struct_a(f1: u32, f2: bool) -> StructA {
        StructA { f1, f2 }
    }
    pub fn create_struct_b(f1: i64, f2: String) -> StructB {
        StructB { f1, f2 }
    }
    pub fn create_struct_tuple_a(f1: i64, f2: i64) -> StructTupleA {
        StructTupleA(f1, f2)
    }
    pub fn create_struct_tuple_b(f1: u128, f2: u128) -> StructTupleB {
        StructTupleB(f1, f2)
    }
    pub fn get_enum_a() -> EnumA {
        EnumA::V2
    }
    pub fn get_enum_b(value: i64) -> EnumB {
        EnumB::V2(value)
    }
    pub fn get_enum_int_a() -> EnumIntA {
        EnumIntA::V3
    }
    pub fn get_enum_int_b() -> EnumIntB {
        EnumIntB::V2
    }
    pub fn check_a(input: u32) -> Result<u32, ErrorA> {
        if input == 0 {
            Err(ErrorA::E2)
        } else {
            Ok(input)
        }
    }
    pub fn check_b(input: u32) -> Result<u32, ErrorB> {
        if input > 1000 {
            Err(ErrorB::E3)
        } else {
            Ok(input)
        }
    }
    pub fn check_c(input: u32) -> Result<u32, ErrorC> {
        if input < 10 {
            Err(ErrorC::E1)
        } else {
            Ok(input)
        }
    }
    pub fn emit_event_a(env: Env, f1: Address, f2: String) {
        EventA { f1, f2 }.publish(&env);
    }
    pub fn emit_event_b(env: Env, f1: Address, f2: Address, f3: i128) {
        EventB { f1, f2, f3 }.publish(&env);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_A: [u8; 84usize] =
        super::Contract::spec_xdr_create_struct_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_a() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0fcreate_struct_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_B: [u8; 84usize] =
        super::Contract::spec_xdr_create_struct_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_b() -> [u8; 84usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0fcreate_struct_b\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructB\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_tuple_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_TUPLE_A: [u8; 96usize] =
        super::Contract::spec_xdr_create_struct_tuple_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_tuple_a() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x15create_struct_tuple_a\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__create_struct_tuple_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CREATE_STRUCT_TUPLE_B: [u8; 96usize] =
        super::Contract::spec_xdr_create_struct_tuple_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_create_struct_tuple_b() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x15create_struct_tuple_b\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\n\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleB"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_GET_ENUM_A: [u8; 48usize] = super::Contract::spec_xdr_get_enum_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_a() -> [u8; 48usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nget_enum_a\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumA\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_GET_ENUM_B: [u8; 68usize] = super::Contract::spec_xdr_get_enum_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_b() -> [u8; 68usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\nget_enum_b\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumB\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_int_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_GET_ENUM_INT_A: [u8; 52usize] =
        super::Contract::spec_xdr_get_enum_int_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_int_a() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0eget_enum_int_a\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__get_enum_int_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_GET_ENUM_INT_B: [u8; 52usize] =
        super::Contract::spec_xdr_get_enum_int_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_get_enum_int_b() -> [u8; 52usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0eget_enum_int_b\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntB"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__check_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CHECK_A: [u8; 72usize] = super::Contract::spec_xdr_check_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_check_a() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07check_a\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorA\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__check_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CHECK_B: [u8; 72usize] = super::Contract::spec_xdr_check_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_check_b() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07check_b\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorB\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__check_c__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_CHECK_C: [u8; 72usize] = super::Contract::spec_xdr_check_c();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_check_c() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x07check_c\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorC\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__emit_event_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_EMIT_EVENT_A: [u8; 64usize] = super::Contract::spec_xdr_emit_event_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_emit_event_a() -> [u8; 64usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cemit_event_a\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__emit_event_b__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_EMIT_EVENT_B: [u8; 80usize] = super::Contract::spec_xdr_emit_event_b();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_emit_event_b() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cemit_event_b\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn create_struct_a(&self, f1: &u32, f2: &bool) -> StructA {
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_create_struct_a(
        &self,
        f1: &u32,
        f2: &bool,
    ) -> Result<
        Result<
            StructA,
            <StructA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn create_struct_b(&self, f1: &i64, f2: &String) -> StructB {
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_create_struct_b(
        &self,
        f1: &i64,
        f2: &String,
    ) -> Result<
        Result<
            StructB,
            <StructB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn create_struct_tuple_a(&self, f1: &i64, f2: &i64) -> StructTupleA {
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_create_struct_tuple_a(
        &self,
        f1: &i64,
        f2: &i64,
    ) -> Result<
        Result<
            StructTupleA,
            <StructTupleA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn create_struct_tuple_b(&self, f1: &u128, f2: &u128) -> StructTupleB {
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_create_struct_tuple_b(
        &self,
        f1: &u128,
        f2: &u128,
    ) -> Result<
        Result<
            StructTupleB,
            <StructTupleB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn get_enum_a(&self) -> EnumA {
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_get_enum_a(
        &self,
    ) -> Result<
        Result<
            EnumA,
            <EnumA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn get_enum_b(&self, value: &i64) -> EnumB {
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_get_enum_b(
        &self,
        value: &i64,
    ) -> Result<
        Result<
            EnumB,
            <EnumB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
            ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn get_enum_int_a(&self) -> EnumIntA {
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_get_enum_int_a(
        &self,
    ) -> Result<
        Result<
            EnumIntA,
            <EnumIntA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn get_enum_int_b(&self) -> EnumIntB {
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_get_enum_int_b(
        &self,
    ) -> Result<
        Result<
            EnumIntB,
            <EnumIntB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
            &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
            ::soroban_sdk::Vec::new(&self.env),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn check_a(&self, input: &u32) -> u32 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_check_a(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorA, soroban_sdk::InvokeError>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn check_b(&self, input: &u32) -> u32 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_check_b(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorB, soroban_sdk::InvokeError>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn check_c(&self, input: &u32) -> u32 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_check_c(
        &self,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<ErrorC, soroban_sdk::InvokeError>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn emit_event_a(&self, f1: &Address, f2: &String) -> () {
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
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_emit_event_a(
        &self,
        f1: &Address,
        f2: &String,
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
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [f1.into_val(&self.env), f2.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn emit_event_b(&self, f1: &Address, f2: &Address, f3: &i128) -> () {
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
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                    f3.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_emit_event_b(
        &self,
        f1: &Address,
        f2: &Address,
        f3: &i128,
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
            &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                    f3.into_val(&self.env),
                ],
            ),
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
    pub fn create_struct_a<'i>(f1: &'i u32, f2: &'i bool) -> (&'i u32, &'i bool) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_b<'i>(f1: &'i i64, f2: &'i String) -> (&'i i64, &'i String) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_tuple_a<'i>(f1: &'i i64, f2: &'i i64) -> (&'i i64, &'i i64) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn create_struct_tuple_b<'i>(f1: &'i u128, f2: &'i u128) -> (&'i u128, &'i u128) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_b<'i>(value: &'i i64) -> (&'i i64,) {
        (value,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_int_a<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn get_enum_int_b<'i>() -> () {
        ()
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn check_a<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn check_b<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn check_c<'i>(input: &'i u32) -> (&'i u32,) {
        (input,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn emit_event_a<'i>(f1: &'i Address, f2: &'i String) -> (&'i Address, &'i String) {
        (f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn emit_event_b<'i>(
        f1: &'i Address,
        f2: &'i Address,
        f3: &'i i128,
    ) -> (&'i Address, &'i Address, &'i i128) {
        (f1, f2, f3)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_a` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_a(
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
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_a` instead")]
pub fn __Contract__create_struct_a__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 2usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                2usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__create_struct_a__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_a` instead")]
pub extern "C" fn __Contract__create_struct_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_b` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_b(
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
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_b` instead")]
pub fn __Contract__create_struct_b__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 2usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                2usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__create_struct_b__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_b` instead")]
pub extern "C" fn __Contract__create_struct_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_b__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_a` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_tuple_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_tuple_a(
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
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_a` instead")]
pub fn __Contract__create_struct_tuple_a__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 2usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                2usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__create_struct_tuple_a__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_a` instead")]
pub extern "C" fn __Contract__create_struct_tuple_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_tuple_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_b` instead")]
#[allow(deprecated)]
pub fn __Contract__create_struct_tuple_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::create_struct_tuple_b(
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
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_b` instead")]
pub fn __Contract__create_struct_tuple_b__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 2usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                2usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__create_struct_tuple_b__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).create_struct_tuple_b` instead")]
pub extern "C" fn __Contract__create_struct_tuple_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__create_struct_tuple_b__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_a` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::get_enum_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_a` instead")]
pub fn __Contract__get_enum_a__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__get_enum_a__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_a` instead")]
pub extern "C" fn __Contract__get_enum_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_b` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::get_enum_b(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_b` instead")]
pub fn __Contract__get_enum_b__invoke_raw_slice(
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
    __Contract__get_enum_b__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_b` instead")]
pub extern "C" fn __Contract__get_enum_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_b__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_a` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_int_a__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::get_enum_int_a(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_a` instead")]
pub fn __Contract__get_enum_int_a__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__get_enum_int_a__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_a` instead")]
pub extern "C" fn __Contract__get_enum_int_a__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_int_a__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_b` instead")]
#[allow(deprecated)]
pub fn __Contract__get_enum_int_b__invoke_raw(env: soroban_sdk::Env) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(<Contract>::get_enum_int_b(), &env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_b` instead")]
pub fn __Contract__get_enum_int_b__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
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
    __Contract__get_enum_int_b__invoke_raw(env)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).get_enum_int_b` instead")]
pub extern "C" fn __Contract__get_enum_int_b__invoke_raw_extern() -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__get_enum_int_b__invoke_raw(soroban_sdk::Env::default())
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_a` instead")]
#[allow(deprecated)]
pub fn __Contract__check_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::check_a(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_a` instead")]
pub fn __Contract__check_a__invoke_raw_slice(
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
    __Contract__check_a__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_a` instead")]
pub extern "C" fn __Contract__check_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__check_a__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_b` instead")]
#[allow(deprecated)]
pub fn __Contract__check_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::check_b(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_b` instead")]
pub fn __Contract__check_b__invoke_raw_slice(
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
    __Contract__check_b__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_b` instead")]
pub extern "C" fn __Contract__check_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__check_b__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_c` instead")]
#[allow(deprecated)]
pub fn __Contract__check_c__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::check_c(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_c` instead")]
pub fn __Contract__check_c__invoke_raw_slice(
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
    __Contract__check_c__invoke_raw(env, args[0usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).check_c` instead")]
pub extern "C" fn __Contract__check_c__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__check_c__invoke_raw(soroban_sdk::Env::default(), arg_0)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_a` instead")]
#[allow(deprecated)]
pub fn __Contract__emit_event_a__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::emit_event_a(
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
        ),
        &env,
    )
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_a` instead")]
pub fn __Contract__emit_event_a__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 2usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                2usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__emit_event_a__invoke_raw(env, args[0usize], args[1usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_a` instead")]
pub extern "C" fn __Contract__emit_event_a__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__emit_event_a__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_b` instead")]
#[allow(deprecated)]
pub fn __Contract__emit_event_b__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::emit_event_b(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_b` instead")]
pub fn __Contract__emit_event_b__invoke_raw_slice(
    env: soroban_sdk::Env,
    args: &[soroban_sdk::Val],
) -> soroban_sdk::Val {
    if args.len() != 3usize {
        {
            ::core::panicking::panic_fmt(format_args!(
                "invalid number of input arguments: {0} expected, got {1}",
                3usize,
                args.len(),
            ));
        };
    }
    #[allow(deprecated)]
    __Contract__emit_event_b__invoke_raw(env, args[0usize], args[1usize], args[2usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).emit_event_b` instead")]
pub extern "C" fn __Contract__emit_event_b__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__emit_event_b__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____8727ab2bacd61e6ccd141d16ef9059af42832a1950ff16a75bd5abdb342be149_ctor() {
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
                    __Contract____8727ab2bacd61e6ccd141d16ef9059af42832a1950ff16a75bd5abdb342be149_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "create_struct_a",
            #[allow(deprecated)]
            &__Contract__create_struct_a__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "create_struct_b",
            #[allow(deprecated)]
            &__Contract__create_struct_b__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "create_struct_tuple_a",
            #[allow(deprecated)]
            &__Contract__create_struct_tuple_a__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "create_struct_tuple_b",
            #[allow(deprecated)]
            &__Contract__create_struct_tuple_b__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "get_enum_a",
            #[allow(deprecated)]
            &__Contract__get_enum_a__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "get_enum_b",
            #[allow(deprecated)]
            &__Contract__get_enum_b__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "get_enum_int_a",
            #[allow(deprecated)]
            &__Contract__get_enum_int_a__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "get_enum_int_b",
            #[allow(deprecated)]
            &__Contract__get_enum_int_b__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "check_a",
            #[allow(deprecated)]
            &__Contract__check_a__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "check_b",
            #[allow(deprecated)]
            &__Contract__check_b__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "check_c",
            #[allow(deprecated)]
            &__Contract__check_c__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "emit_event_a",
            #[allow(deprecated)]
            &__Contract__emit_event_a__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "emit_event_b",
            #[allow(deprecated)]
            &__Contract__emit_event_b__invoke_raw_slice,
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_struct_a"]
    #[doc(hidden)]
    pub const test_struct_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_struct_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_lib/src/lib.rs",
            start_line: 96usize,
            start_col: 8usize,
            end_line: 96usize,
            end_col: 21usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_struct_a()),
        ),
    };
    fn test_struct_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let s = client.create_struct_a(&10, &true);
        match (&s.f1, &10) {
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
        match (&s.f2, &true) {
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
    #[rustc_test_marker = "test::test_struct_tuple_a"]
    #[doc(hidden)]
    pub const test_struct_tuple_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_struct_tuple_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_lib/src/lib.rs",
            start_line: 107usize,
            start_col: 8usize,
            end_line: 107usize,
            end_col: 27usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_struct_tuple_a()),
        ),
    };
    fn test_struct_tuple_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let s = client.create_struct_tuple_a(&5, &10);
        match (&s.0, &5) {
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
        match (&s.1, &10) {
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
    #[rustc_test_marker = "test::test_enum_a"]
    #[doc(hidden)]
    pub const test_enum_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_enum_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_lib/src/lib.rs",
            start_line: 118usize,
            start_col: 8usize,
            end_line: 118usize,
            end_col: 19usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_enum_a()),
        ),
    };
    fn test_enum_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let v = client.get_enum_a();
        match (&v, &EnumA::V2) {
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
    #[rustc_test_marker = "test::test_enum_int_a"]
    #[doc(hidden)]
    pub const test_enum_int_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_enum_int_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_lib/src/lib.rs",
            start_line: 128usize,
            start_col: 8usize,
            end_line: 128usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_enum_int_a()),
        ),
    };
    fn test_enum_int_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let v = client.get_enum_int_a();
        match (&v, &EnumIntA::V3) {
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
    #[rustc_test_marker = "test::test_check_a"]
    #[doc(hidden)]
    pub const test_check_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_check_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_lib/src/lib.rs",
            start_line: 138usize,
            start_col: 8usize,
            end_line: 138usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_check_a()),
        ),
    };
    fn test_check_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let result = client.try_check_a(&10);
        match (&result, &Ok(Ok(10))) {
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
        let result = client.try_check_a(&0);
        match (&result, &Err(Ok(ErrorA::E2))) {
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
    #[rustc_test_marker = "test::test_emit_event_a"]
    #[doc(hidden)]
    pub const test_emit_event_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_emit_event_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_lib/src/lib.rs",
            start_line: 151usize,
            start_col: 8usize,
            end_line: 151usize,
            end_col: 25usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_emit_event_a()),
        ),
    };
    fn test_emit_event_a() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let f1 = Address::generate(&e);
        let f2 = String::from_str(&e, "test");
        client.emit_event_a(&f1, &f2);
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_check_a,
        &test_emit_event_a,
        &test_enum_a,
        &test_enum_int_a,
        &test_struct_a,
        &test_struct_tuple_a,
    ])
}
