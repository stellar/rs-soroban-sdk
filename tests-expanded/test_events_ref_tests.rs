#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, MuxedAddress};
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
pub struct Transfer<'a> {
    from: &'a Address,
    to: &'a Address,
    amount: &'a i128,
    to_muxed_id: Option<&'a u64>,
}
pub static __SPEC_XDR_EVENT_TRANSFER: [u8; 144usize] = Transfer::spec_xdr();
impl<'a> Transfer<'a> {
    pub const fn spec_xdr() -> [u8; 144usize] {
        *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x08Transfer\0\0\0\x01\0\0\0\x08transfer\0\0\0\x04\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\0\0\0\0\x0bto_muxed_id\0\0\0\x03\xe8\0\0\0\x06\0\0\0\0\0\0\0\x02"
    }
}
impl<'a> soroban_sdk::IncludeSpecMarker for Transfer<'a> {
    #[doc(hidden)]
    #[inline(always)]
    fn include_spec_marker() {
        <&'a Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <&'a Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <&'a i128 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <Option<&'a u64> as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
    }
}
impl<'a> soroban_sdk::Event for Transfer<'a> {
    fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
        use soroban_sdk::IntoVal;
        (
            &{
                #[allow(deprecated)]
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            },
            {
                let v: soroban_sdk::Val = self.to.into_val(env);
                v
            },
        )
            .into_val(env)
    }
    fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
        use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
        const KEYS: [&'static str; 2usize] = ["amount", "to_muxed_id"];
        let vals: [soroban_sdk::Val; 2usize] =
            [self.amount.into_val(env), self.to_muxed_id.into_val(env)];
        env.map_new_from_slices(&KEYS, &vals)
            .unwrap_infallible()
            .into()
    }
}
impl<'a> Transfer<'a> {
    pub fn publish(&self, env: &soroban_sdk::Env) {
        <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        <_ as soroban_sdk::Event>::publish(self, env);
    }
}
impl Contract {
    pub fn transfer(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from: &from,
            to: &to.address(),
            amount: &amount,
            to_muxed_id: to.id().as_ref(),
        }
        .publish(&env);
    }
    pub fn failed_transfer(env: Env, from: Address, to: Address, amount: i128) {
        Transfer {
            from: &from,
            to: &to,
            amount: &amount,
            to_muxed_id: None,
        }
        .publish(&env);
        {
            ::core::panicking::panic_fmt(format_args!("fail"));
        };
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_TRANSFER: [u8; 80usize] = super::Contract::spec_xdr_transfer();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_transfer() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08transfer\0\0\0\x03\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x14\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__failed_transfer__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FAILED_TRANSFER: [u8; 88usize] =
        super::Contract::spec_xdr_failed_transfer();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_failed_transfer() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0ffailed_transfer\0\0\0\0\x03\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x13\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn transfer(&self, from: &Address, to: impl Into<MuxedAddress>, amount: &i128) -> () {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into().into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_transfer(
        &self,
        from: &Address,
        to: impl Into<MuxedAddress>,
        amount: &i128,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("transfer");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into().into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn failed_transfer(&self, from: &Address, to: &Address, amount: &i128) -> () {
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
            &{ soroban_sdk::Symbol::new(&self.env, "failed_transfer") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into_val(&self.env),
                    amount.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_failed_transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: &i128,
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
            &{ soroban_sdk::Symbol::new(&self.env, "failed_transfer") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    from.into_val(&self.env),
                    to.into_val(&self.env),
                    amount.into_val(&self.env),
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
    pub fn transfer<'i>(
        from: &'i Address,
        to: &'i MuxedAddress,
        amount: &'i i128,
    ) -> (&'i Address, &'i MuxedAddress, &'i i128) {
        (from, to, amount)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn failed_transfer<'i>(
        from: &'i Address,
        to: &'i Address,
        amount: &'i i128,
    ) -> (&'i Address, &'i Address, &'i i128) {
        (from, to, amount)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__transfer {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::transfer(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
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
        invoke_raw(env, args[0usize], args[1usize], args[2usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).transfer` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__failed_transfer {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::failed_transfer(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
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
        invoke_raw(env, args[0usize], args[1usize], args[2usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).failed_transfer` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__a60968eb9ff75bf813738a9007ab5bbea9f174011ab4092819ed57e87eb6b301_ctor() {
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
                    __Contract__a60968eb9ff75bf813738a9007ab5bbea9f174011ab4092819ed57e87eb6b301_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "transfer",
            #[allow(deprecated)]
            &__Contract__transfer::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "failed_transfer",
            #[allow(deprecated)]
            &__Contract__failed_transfer::invoke_raw_slice,
        );
    }
}
#[cfg(test)]
mod test {
    extern crate alloc;
    use crate::{Contract, ContractClient};
    use soroban_sdk::{
        map, symbol_short,
        testutils::{Address as _, Events, MuxedAddress as _},
        vec, Address, Env, IntoVal, MuxedAddress, Symbol, Val,
    };
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_event"]
    #[doc(hidden)]
    pub const test_event: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_event"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/events_ref/src/lib.rs",
            start_line: 53usize,
            start_col: 8usize,
            end_line: 53usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_event()),
        ),
    };
    fn test_event() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let from = Address::generate(&env);
        let to = MuxedAddress::generate(&env);
        let amount = 1i128;
        client.transfer(&from, &to, &amount);
        match (
            &env.events().all(),
            &::soroban_sdk::Vec::from_array(
                &env,
                [(
                    contract_id.clone(),
                    (Symbol::new(&env, "transfer"), &from, to.address()).into_val(&env),
                    ::soroban_sdk::Map::from_array(
                        &env,
                        [
                            (
                                {
                                    #[allow(deprecated)]
                                    const SYMBOL: soroban_sdk::Symbol =
                                        soroban_sdk::Symbol::short("amount");
                                    SYMBOL
                                },
                                <_ as IntoVal<Env, Val>>::into_val(&1i128, &env),
                            ),
                            (
                                Symbol::new(&env, "to_muxed_id"),
                                <_ as IntoVal<Env, Val>>::into_val(&to.id().unwrap(), &env),
                            ),
                        ],
                    )
                    .to_val(),
                )],
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
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_event_with_option_none"]
    #[doc(hidden)]
    pub const test_event_with_option_none: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_event_with_option_none"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/events_ref/src/lib.rs",
            start_line: 91usize,
            start_col: 8usize,
            end_line: 91usize,
            end_col: 35usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_event_with_option_none()),
        ),
    };
    fn test_event_with_option_none() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let amount = 1i128;
        client.transfer(&from, &to, &amount);
        match (
            &env.events().all(),
            &::soroban_sdk::Vec::from_array(
                &env,
                [(
                    contract_id.clone(),
                    (Symbol::new(&env, "transfer"), &from, &to).into_val(&env),
                    ::soroban_sdk::Map::from_array(
                        &env,
                        [
                            (
                                {
                                    #[allow(deprecated)]
                                    const SYMBOL: soroban_sdk::Symbol =
                                        soroban_sdk::Symbol::short("amount");
                                    SYMBOL
                                },
                                <_ as IntoVal<Env, Val>>::into_val(&1i128, &env),
                            ),
                            (Symbol::new(&env, "to_muxed_id"), ().into_val(&env)),
                        ],
                    )
                    .to_val(),
                )],
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
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_no_events_recorded_for_failed_call"]
    #[doc(hidden)]
    pub const test_no_events_recorded_for_failed_call: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_no_events_recorded_for_failed_call"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/events_ref/src/lib.rs",
            start_line: 126usize,
            start_col: 8usize,
            end_line: 126usize,
            end_col: 47usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_no_events_recorded_for_failed_call()),
        ),
    };
    fn test_no_events_recorded_for_failed_call() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let _ = client.try_failed_transfer(&from, &to, &1);
        match (&env.events().all(), &::soroban_sdk::Vec::new(&env)) {
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
        &test_event,
        &test_event_with_option_none,
        &test_no_events_recorded_for_failed_call,
    ])
}
