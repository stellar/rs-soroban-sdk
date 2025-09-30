#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal};
pub struct ContractA;
///ContractAArgs is a type for building arg lists for functions defined in "ContractA".
pub struct ContractAArgs;
///ContractAClient is a client for calling the contract defined in "ContractA".
pub struct ContractAClient<'a> {
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
impl<'a> ContractAClient<'a> {
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
mod __contracta_fn_set_registry {
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
impl soroban_sdk::testutils::ContractFunctionRegister for ContractA {
    fn register(name: &'static str, func: &'static __contracta_fn_set_registry::F) {
        __contracta_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for ContractA {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contracta_fn_set_registry::call(func, env, args)
    }
}
impl ContractA {
    pub fn fn1(a: Address) -> u64 {
        a.require_auth();
        2
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractA__fn1__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN1: [u8; 44usize] = super::ContractA::spec_xdr_fn1();
}
impl ContractA {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn1() -> [u8; 44usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03fn1\0\0\0\0\x01\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x13\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractAClient<'a> {
    pub fn fn1(&self, a: &Address) -> u64 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_fn1(
        &self,
        a: &Address,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn1");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(&self.env, [a.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
}
impl ContractAArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn1<'i>(a: &'i Address) -> (&'i Address,) {
        (a,)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractA__fn1 {
    use super::*;
    #[deprecated(note = "use `ContractAClient::new(&env, &contract_id).fn1` instead")]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::ContractA>::fn1(
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
    #[deprecated(note = "use `ContractAClient::new(&env, &contract_id).fn1` instead")]
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
    #[deprecated(note = "use `ContractAClient::new(&env, &contract_id).fn1` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
extern "C" fn __ContractA__7c3764b58a7ababbe8a6b452f6a400d8ae3704b80f8c5ea1b251eebbc8698020_ctor() {
    <ContractA as soroban_sdk::testutils::ContractFunctionRegister>::register(
        "fn1",
        #[allow(deprecated)]
        &__ContractA__fn1::invoke_raw_slice,
    );
}
#[used]
#[allow(non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
#[link_section = ".init_array"]
static __ContractA__7c3764b58a7ababbe8a6b452f6a400d8ae3704b80f8c5ea1b251eebbc8698020_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
    #[allow(non_snake_case)]
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ContractA__7c3764b58a7ababbe8a6b452f6a400d8ae3704b80f8c5ea1b251eebbc8698020_ctor___rust_ctor___ctor() -> usize {
        __ContractA__7c3764b58a7ababbe8a6b452f6a400d8ae3704b80f8c5ea1b251eebbc8698020_ctor();
        0
    }
    __ContractA__7c3764b58a7ababbe8a6b452f6a400d8ae3704b80f8c5ea1b251eebbc8698020_ctor___rust_ctor___ctor
};
#[cfg(test)]
mod test_a {
    use super::*;
    use soroban_sdk::{
        contracterror,
        testutils::{
            Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke,
        },
        xdr::{
            InvokeContractArgs, ScAddress, ScError, ScErrorCode, ScVal, SorobanAddressCredentials,
            SorobanAuthorizationEntry, SorobanAuthorizedFunction, SorobanAuthorizedInvocation,
            SorobanCredentials, StringM, VecM,
        },
        Address, Env, Error, Symbol, Val,
    };
    extern crate std;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test_a::test_with_mock_all_auth"]
    #[doc(hidden)]
    pub const test_with_mock_all_auth: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_a::test_with_mock_all_auth"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 33usize,
            start_col: 8usize,
            end_line: 33usize,
            end_col: 31usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_mock_all_auth()),
        ),
    };
    fn test_with_mock_all_auth() {
        let e = Env::default();
        let contract_id = e.register(ContractA, ());
        let client = ContractAClient::new(&e, &contract_id);
        let a = Address::generate(&e);
        let r = client.mock_all_auths().fn1(&a);
        match (&r, &2) {
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
        match (
            &e.auths(),
            &<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(
                    a.clone(),
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            contract_id.clone(),
                            Symbol::new(&e, "fn1"),
                            (&a,).into_val(&e),
                        )),
                        sub_invocations: ::alloc::vec::Vec::new(),
                    },
                )]),
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
    #[rustc_test_marker = "test_a::test_with_mock_auth"]
    #[doc(hidden)]
    pub const test_with_mock_auth: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_a::test_with_mock_auth"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 60usize,
            start_col: 8usize,
            end_line: 60usize,
            end_col: 27usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_mock_auth()),
        ),
    };
    fn test_with_mock_auth() {
        let e = Env::default();
        let contract_id = e.register(ContractA, ());
        let client = ContractAClient::new(&e, &contract_id);
        let a = Address::generate(&e);
        let r = client
            .mock_auths(&[MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_id,
                    fn_name: "fn1",
                    args: (&a,).into_val(&e),
                    sub_invokes: &[],
                },
            }])
            .fn1(&a);
        match (&r, &2) {
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
        match (
            &e.auths(),
            &<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(
                    a.clone(),
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            contract_id.clone(),
                            Symbol::new(&e, "fn1"),
                            (&a,).into_val(&e),
                        )),
                        sub_invocations: ::alloc::vec::Vec::new(),
                    },
                )]),
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
    #[rustc_test_marker = "test_a::test_with_real_contract_auth_approve"]
    #[doc(hidden)]
    pub const test_with_real_contract_auth_approve: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_a::test_with_real_contract_auth_approve"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 97usize,
            start_col: 8usize,
            end_line: 97usize,
            end_col: 44usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_real_contract_auth_approve()),
        ),
    };
    fn test_with_real_contract_auth_approve() {
        let e = Env::default();
        let contract_id = e.register(ContractA, ());
        let client = ContractAClient::new(&e, &contract_id);
        let a = e.register(auth_approve::Contract, ());
        let a_xdr: ScAddress = (&a).try_into().unwrap();
        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 123,
                    signature_expiration_ledger: 100,
                    signature: ScVal::Void,
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                        contract_address: contract_id.clone().try_into().unwrap(),
                        function_name: StringM::try_from("fn1").unwrap().into(),
                        args: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([ScVal::Address(a_xdr.clone())]),
                        )
                        .try_into()
                        .unwrap(),
                    }),
                    sub_invocations: VecM::default(),
                },
            }])
            .fn1(&a);
        match (&r, &2) {
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
        match (
            &e.auths(),
            &<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(
                    a.clone(),
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            contract_id.clone(),
                            Symbol::new(&e, "fn1"),
                            (&a,).into_val(&e),
                        )),
                        sub_invocations: ::alloc::vec::Vec::new(),
                    },
                )]),
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
    #[rustc_test_marker = "test_a::test_with_real_contract_auth_decline"]
    #[doc(hidden)]
    pub const test_with_real_contract_auth_decline: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_a::test_with_real_contract_auth_decline"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 143usize,
            start_col: 8usize,
            end_line: 143usize,
            end_col: 44usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_real_contract_auth_decline()),
        ),
    };
    fn test_with_real_contract_auth_decline() {
        let e = Env::default();
        let contract_id = e.register(ContractA, ());
        let client = ContractAClient::new(&e, &contract_id);
        let a = e.register(auth_decline::Contract, ());
        let a_xdr: ScAddress = (&a).try_into().unwrap();
        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 456,
                    signature_expiration_ledger: u32::MAX,
                    signature: ScVal::Void,
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                        contract_address: contract_id.try_into().unwrap(),
                        function_name: StringM::try_from("fn1").unwrap().into(),
                        args: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([ScVal::Address(a_xdr.clone())]),
                        )
                        .try_into()
                        .unwrap(),
                    }),
                    sub_invocations: VecM::default(),
                },
            }])
            .try_fn1(&a);
        match (
            &r,
            &Err(Ok(Error::from_scerror(ScError::Context(
                ScErrorCode::InvalidAction,
            )))),
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
        match (&e.auths(), &[]) {
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
    mod auth_approve {
        use super::*;
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
            #[allow(non_snake_case)]
            pub fn __check_auth(_signature_payload: Val, _signatures: Val, _auth_context: Val) {}
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth__spec {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            #[allow(non_snake_case)]
            pub static __SPEC_XDR_FN___CHECK_AUTH: [u8; 112usize] =
                super::Contract::spec_xdr___check_auth();
        }
        impl Contract {
            #[allow(non_snake_case)]
            #[allow(non_snake_case)]
            pub const fn spec_xdr___check_auth() -> [u8; 112usize] {
                *b"\0\0\0\0\0\0\0\0\0\0\0\x0c__check_auth\0\0\0\x03\0\0\0\0\0\0\0\x11signature_payload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\nsignatures\0\0\0\0\0\0\0\0\0\0\0\0\0\x0cauth_context\0\0\0\0\0\0\0\0"
            }
        }
        impl<'a> ContractClient<'a> {}
        impl ContractArgs {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn __check_auth<'i>(
                _signature_payload: &'i Val,
                _signatures: &'i Val,
                _auth_context: &'i Val,
            ) -> (&'i Val, &'i Val, &'i Val) {
                (_signature_payload, _signatures, _auth_context)
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth {
            use super::*;
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw(
                env: soroban_sdk::Env,
                arg_0: soroban_sdk::Val,
                arg_1: soroban_sdk::Val,
                arg_2: soroban_sdk::Val,
            ) -> soroban_sdk::Val {
                <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
                    #[allow(deprecated)]
                    &<super::Contract>::__check_auth(
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_0
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_1
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_2
                            ),
                        ),
                    ),
                    &env,
                )
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw_slice(
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
                invoke_raw(env, args[0usize], args[1usize], args[2usize])
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
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
        extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor(
        ) {
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "__check_auth",
                #[allow(deprecated)]
                &__Contract____check_auth::invoke_raw_slice,
            );
        }
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        #[link_section = ".init_array"]
        static __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor() -> usize {
                __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor();
                0
            }
            __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor
        };
    }
    mod auth_decline {
        use super::*;
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
        #[repr(u32)]
        pub enum Error {
            Decline = 1,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Error {}
        #[automatically_derived]
        impl ::core::clone::Clone for Error {
            #[inline]
            fn clone(&self) -> Error {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Decline")
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Error {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Error {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Error {
            #[inline]
            fn eq(&self, other: &Error) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Error {
            #[inline]
            fn partial_cmp(&self, other: &Error) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Error {
            #[inline]
            fn cmp(&self, other: &Error) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        pub static __SPEC_XDR_TYPE_ERROR: [u8; 48usize] = Error::spec_xdr();
        impl Error {
            pub const fn spec_xdr() -> [u8; 48usize] {
                *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x05Error\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x07Decline\0\0\0\0\x01"
            }
        }
        impl TryFrom<soroban_sdk::Error> for Error {
            type Error = soroban_sdk::Error;
            #[inline(always)]
            fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
                if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                    let discriminant = error.get_code();
                    Ok(match discriminant {
                        1u32 => Self::Decline,
                        _ => return Err(error),
                    })
                } else {
                    Err(error)
                }
            }
        }
        impl TryFrom<&soroban_sdk::Error> for Error {
            type Error = soroban_sdk::Error;
            #[inline(always)]
            fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
                <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
            }
        }
        impl From<Error> for soroban_sdk::Error {
            #[inline(always)]
            fn from(val: Error) -> soroban_sdk::Error {
                <_ as From<&Error>>::from(&val)
            }
        }
        impl From<&Error> for soroban_sdk::Error {
            #[inline(always)]
            fn from(val: &Error) -> soroban_sdk::Error {
                match val {
                    Error::Decline => soroban_sdk::Error::from_contract_error(1u32),
                }
            }
        }
        impl TryFrom<soroban_sdk::InvokeError> for Error {
            type Error = soroban_sdk::InvokeError;
            #[inline(always)]
            fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
                match error {
                    soroban_sdk::InvokeError::Abort => Err(error),
                    soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                        1u32 => Self::Decline,
                        _ => return Err(error),
                    }),
                }
            }
        }
        impl TryFrom<&soroban_sdk::InvokeError> for Error {
            type Error = soroban_sdk::InvokeError;
            #[inline(always)]
            fn try_from(
                error: &soroban_sdk::InvokeError,
            ) -> Result<Self, soroban_sdk::InvokeError> {
                <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
            }
        }
        impl From<Error> for soroban_sdk::InvokeError {
            #[inline(always)]
            fn from(val: Error) -> soroban_sdk::InvokeError {
                <_ as From<&Error>>::from(&val)
            }
        }
        impl From<&Error> for soroban_sdk::InvokeError {
            #[inline(always)]
            fn from(val: &Error) -> soroban_sdk::InvokeError {
                match val {
                    Error::Decline => soroban_sdk::InvokeError::Contract(1u32),
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Error {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::TryIntoVal;
                let error: soroban_sdk::Error = val.try_into_val(env)?;
                error.try_into().map_err(|_| soroban_sdk::ConversionError)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, Error> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &Error,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                let error: soroban_sdk::Error = val.into();
                Ok(error.into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Error> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&Error,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Error>>::try_from_val(env, *val)
            }
        }
        impl Contract {
            #[allow(non_snake_case)]
            pub fn __check_auth(
                _signature_payload: Val,
                _signatures: Val,
                _auth_context: Val,
            ) -> Result<(), Error> {
                Err(Error::Decline)
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth__spec {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            #[allow(non_snake_case)]
            pub static __SPEC_XDR_FN___CHECK_AUTH: [u8; 128usize] =
                super::Contract::spec_xdr___check_auth();
        }
        impl Contract {
            #[allow(non_snake_case)]
            #[allow(non_snake_case)]
            pub const fn spec_xdr___check_auth() -> [u8; 128usize] {
                *b"\0\0\0\0\0\0\0\0\0\0\0\x0c__check_auth\0\0\0\x03\0\0\0\0\0\0\0\x11signature_payload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\nsignatures\0\0\0\0\0\0\0\0\0\0\0\0\0\x0cauth_context\0\0\0\0\0\0\0\x01\0\0\x03\xe9\0\0\x03\xed\0\0\0\0\0\0\0\x03"
            }
        }
        impl<'a> ContractClient<'a> {}
        impl ContractArgs {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn __check_auth<'i>(
                _signature_payload: &'i Val,
                _signatures: &'i Val,
                _auth_context: &'i Val,
            ) -> (&'i Val, &'i Val, &'i Val) {
                (_signature_payload, _signatures, _auth_context)
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth {
            use super::*;
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw(
                env: soroban_sdk::Env,
                arg_0: soroban_sdk::Val,
                arg_1: soroban_sdk::Val,
                arg_2: soroban_sdk::Val,
            ) -> soroban_sdk::Val {
                <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
                    #[allow(deprecated)]
                    &<super::Contract>::__check_auth(
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_0
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_1
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_2
                            ),
                        ),
                    ),
                    &env,
                )
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw_slice(
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
                invoke_raw(env, args[0usize], args[1usize], args[2usize])
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
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
        extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor(
        ) {
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "__check_auth",
                #[allow(deprecated)]
                &__Contract____check_auth::invoke_raw_slice,
            );
        }
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        #[link_section = ".init_array"]
        static __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor() -> usize {
                __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor();
                0
            }
            __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor
        };
    }
}
pub struct ContractB;
///ContractBArgs is a type for building arg lists for functions defined in "ContractB".
pub struct ContractBArgs;
///ContractBClient is a client for calling the contract defined in "ContractB".
pub struct ContractBClient<'a> {
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
impl<'a> ContractBClient<'a> {
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
mod __contractb_fn_set_registry {
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
impl soroban_sdk::testutils::ContractFunctionRegister for ContractB {
    fn register(name: &'static str, func: &'static __contractb_fn_set_registry::F) {
        __contractb_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for ContractB {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contractb_fn_set_registry::call(func, env, args)
    }
}
impl ContractB {
    pub fn fn2(e: Env, a: Address, sub: Address) -> u64 {
        a.require_auth_for_args((1, 2).into_val(&e));
        let client = ContractAClient::new(&e, &sub);
        client.fn1(&a)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractB__fn2__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_FN2: [u8; 60usize] = super::ContractB::spec_xdr_fn2();
}
impl ContractB {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_fn2() -> [u8; 60usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x03fn2\0\0\0\0\x02\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x03sub\0\0\0\0\x13\0\0\0\x01\0\0\0\x06"
    }
}
impl<'a> ContractBClient<'a> {
    pub fn fn2(&self, a: &Address, sub: &Address) -> u64 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), sub.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_fn2(
        &self,
        a: &Address,
        sub: &Address,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [a.into_val(&self.env), sub.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
}
impl ContractBArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn fn2<'i>(a: &'i Address, sub: &'i Address) -> (&'i Address, &'i Address) {
        (a, sub)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __ContractB__fn2 {
    use super::*;
    #[deprecated(note = "use `ContractBClient::new(&env, &contract_id).fn2` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::ContractB>::fn2(
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
    #[deprecated(note = "use `ContractBClient::new(&env, &contract_id).fn2` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
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
        invoke_raw(env, args[0usize], args[1usize])
    }
    #[deprecated(note = "use `ContractBClient::new(&env, &contract_id).fn2` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
extern "C" fn __ContractB__389cfcb1cb10680376b4cd5cf632e6b11c3e59494c10e1d42514faf6c4c21b84_ctor() {
    <ContractB as soroban_sdk::testutils::ContractFunctionRegister>::register(
        "fn2",
        #[allow(deprecated)]
        &__ContractB__fn2::invoke_raw_slice,
    );
}
#[used]
#[allow(non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
#[link_section = ".init_array"]
static __ContractB__389cfcb1cb10680376b4cd5cf632e6b11c3e59494c10e1d42514faf6c4c21b84_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
    #[allow(non_snake_case)]
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ContractB__389cfcb1cb10680376b4cd5cf632e6b11c3e59494c10e1d42514faf6c4c21b84_ctor___rust_ctor___ctor() -> usize {
        __ContractB__389cfcb1cb10680376b4cd5cf632e6b11c3e59494c10e1d42514faf6c4c21b84_ctor();
        0
    }
    __ContractB__389cfcb1cb10680376b4cd5cf632e6b11c3e59494c10e1d42514faf6c4c21b84_ctor___rust_ctor___ctor
};
#[cfg(test)]
mod test_b {
    use super::*;
    use soroban_sdk::{
        contracterror, symbol_short,
        testutils::{
            Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke,
        },
        xdr::{
            InvokeContractArgs, ScAddress, ScError, ScErrorCode, ScVal, SorobanAddressCredentials,
            SorobanAuthorizationEntry, SorobanAuthorizedFunction, SorobanAuthorizedInvocation,
            SorobanCredentials, StringM,
        },
        Address, Env, Error, Val,
    };
    extern crate std;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test_b::test_with_mock_all_auth"]
    #[doc(hidden)]
    pub const test_with_mock_all_auth: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_b::test_with_mock_all_auth"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 251usize,
            start_col: 8usize,
            end_line: 251usize,
            end_col: 31usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_mock_all_auth()),
        ),
    };
    fn test_with_mock_all_auth() {
        let e = Env::default();
        let contract_a_id = e.register(ContractA, ());
        let contract_b_id = e.register(ContractB, ());
        let client = ContractBClient::new(&e, &contract_b_id);
        let a = Address::generate(&e);
        let r = client.mock_all_auths().fn2(&a, &contract_a_id);
        match (&r, &2) {
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
        match (
            &e.auths(),
            &[(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_b_id.clone(),
                        {
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                            SYMBOL
                        },
                        (1, 2).into_val(&e),
                    )),
                    sub_invocations: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([AuthorizedInvocation {
                            function: AuthorizedFunction::Contract((
                                contract_a_id.clone(),
                                {
                                    #[allow(deprecated)]
                                    const SYMBOL: soroban_sdk::Symbol =
                                        soroban_sdk::Symbol::short("fn1");
                                    SYMBOL
                                },
                                (&a,).into_val(&e),
                            )),
                            sub_invocations: ::alloc::vec::Vec::new(),
                        }]),
                    ),
                },
            )],
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
    #[rustc_test_marker = "test_b::test_with_mock_auth"]
    #[doc(hidden)]
    pub const test_with_mock_auth: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_b::test_with_mock_auth"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 286usize,
            start_col: 8usize,
            end_line: 286usize,
            end_col: 27usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_mock_auth()),
        ),
    };
    fn test_with_mock_auth() {
        let e = Env::default();
        let contract_a_id = e.register(ContractA, ());
        let contract_b_id = e.register(ContractB, ());
        let client = ContractBClient::new(&e, &contract_b_id);
        let a = Address::generate(&e);
        let r = client
            .mock_auths(&[MockAuth {
                address: &a,
                invoke: &MockAuthInvoke {
                    contract: &contract_b_id,
                    fn_name: "fn2",
                    args: (1, 2).into_val(&e),
                    sub_invokes: &[MockAuthInvoke {
                        contract: &contract_a_id,
                        fn_name: "fn1",
                        args: (&a,).into_val(&e),
                        sub_invokes: &[],
                    }],
                },
            }])
            .fn2(&a, &contract_a_id);
        match (&r, &2) {
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
        match (
            &e.auths(),
            &[(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_b_id.clone(),
                        {
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                            SYMBOL
                        },
                        (1, 2).into_val(&e),
                    )),
                    sub_invocations: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([AuthorizedInvocation {
                            function: AuthorizedFunction::Contract((
                                contract_a_id.clone(),
                                {
                                    #[allow(deprecated)]
                                    const SYMBOL: soroban_sdk::Symbol =
                                        soroban_sdk::Symbol::short("fn1");
                                    SYMBOL
                                },
                                (&a,).into_val(&e),
                            )),
                            sub_invocations: ::alloc::vec::Vec::new(),
                        }]),
                    ),
                },
            )],
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
    #[rustc_test_marker = "test_b::test_with_real_contract_auth_approve"]
    #[doc(hidden)]
    pub const test_with_real_contract_auth_approve: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_b::test_with_real_contract_auth_approve"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 336usize,
            start_col: 8usize,
            end_line: 336usize,
            end_col: 44usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_real_contract_auth_approve()),
        ),
    };
    fn test_with_real_contract_auth_approve() {
        let e = Env::default();
        let contract_a_id = e.register(ContractA, ());
        let contract_b_id = e.register(ContractB, ());
        let client = ContractBClient::new(&e, &contract_b_id);
        let a = e.register(auth_approve::Contract, ());
        let a_xdr: ScAddress = (&a).try_into().unwrap();
        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 543,
                    signature_expiration_ledger: 100,
                    signature: ScVal::Void,
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                        contract_address: contract_b_id.clone().try_into().unwrap(),
                        function_name: StringM::try_from("fn2").unwrap().into(),
                        args: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([ScVal::I32(1), ScVal::I32(2)]),
                        )
                        .try_into()
                        .unwrap(),
                    }),
                    sub_invocations: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([SorobanAuthorizedInvocation {
                            function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                                contract_address: contract_a_id.clone().try_into().unwrap(),
                                function_name: StringM::try_from("fn1").unwrap().into(),
                                args: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([ScVal::Address(a_xdr.clone())]),
                                )
                                .try_into()
                                .unwrap(),
                            }),
                            sub_invocations: Default::default(),
                        }]),
                    )
                    .try_into()
                    .unwrap(),
                },
            }])
            .fn2(&a, &contract_a_id);
        match (&r, &2) {
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
        match (
            &e.auths(),
            &[(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_b_id.clone(),
                        {
                            #[allow(deprecated)]
                            const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("fn2");
                            SYMBOL
                        },
                        (1, 2).into_val(&e),
                    )),
                    sub_invocations: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([AuthorizedInvocation {
                            function: AuthorizedFunction::Contract((
                                contract_a_id.clone(),
                                {
                                    #[allow(deprecated)]
                                    const SYMBOL: soroban_sdk::Symbol =
                                        soroban_sdk::Symbol::short("fn1");
                                    SYMBOL
                                },
                                (&a,).into_val(&e),
                            )),
                            sub_invocations: ::alloc::vec::Vec::new(),
                        }]),
                    ),
                },
            )],
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
    #[rustc_test_marker = "test_b::test_with_real_contract_auth_decline"]
    #[doc(hidden)]
    pub const test_with_real_contract_auth_decline: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_b::test_with_real_contract_auth_decline"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/auth/src/lib.rs",
            start_line: 399usize,
            start_col: 8usize,
            end_line: 399usize,
            end_col: 44usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_with_real_contract_auth_decline()),
        ),
    };
    fn test_with_real_contract_auth_decline() {
        let e = Env::default();
        let contract_a_id = e.register(ContractA, ());
        let contract_b_id = e.register(ContractB, ());
        let client = ContractBClient::new(&e, &contract_b_id);
        let a = e.register(auth_decline::Contract, ());
        let a_xdr: ScAddress = (&a).try_into().unwrap();
        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 789,
                    signature_expiration_ledger: 150,
                    signature: ScVal::Void,
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                        contract_address: contract_b_id.try_into().unwrap(),
                        function_name: StringM::try_from("fn2").unwrap().into(),
                        args: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([ScVal::I32(1), ScVal::I32(2)]),
                        )
                        .try_into()
                        .unwrap(),
                    }),
                    sub_invocations: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([SorobanAuthorizedInvocation {
                            function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                                contract_address: contract_a_id.clone().try_into().unwrap(),
                                function_name: StringM::try_from("fn1").unwrap().into(),
                                args: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([ScVal::Address(a_xdr.clone())]),
                                )
                                .try_into()
                                .unwrap(),
                            }),
                            sub_invocations: Default::default(),
                        }]),
                    )
                    .try_into()
                    .unwrap(),
                },
            }])
            .try_fn2(&a, &contract_a_id);
        match (
            &r,
            &Err(Ok(Error::from_scerror(ScError::Context(
                ScErrorCode::InvalidAction,
            )))),
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
        match (&e.auths(), &[]) {
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
    mod auth_approve {
        use super::*;
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
            #[allow(non_snake_case)]
            pub fn __check_auth(_signature_payload: Val, _signatures: Val, _auth_context: Val) {}
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth__spec {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            #[allow(non_snake_case)]
            pub static __SPEC_XDR_FN___CHECK_AUTH: [u8; 112usize] =
                super::Contract::spec_xdr___check_auth();
        }
        impl Contract {
            #[allow(non_snake_case)]
            #[allow(non_snake_case)]
            pub const fn spec_xdr___check_auth() -> [u8; 112usize] {
                *b"\0\0\0\0\0\0\0\0\0\0\0\x0c__check_auth\0\0\0\x03\0\0\0\0\0\0\0\x11signature_payload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\nsignatures\0\0\0\0\0\0\0\0\0\0\0\0\0\x0cauth_context\0\0\0\0\0\0\0\0"
            }
        }
        impl<'a> ContractClient<'a> {}
        impl ContractArgs {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn __check_auth<'i>(
                _signature_payload: &'i Val,
                _signatures: &'i Val,
                _auth_context: &'i Val,
            ) -> (&'i Val, &'i Val, &'i Val) {
                (_signature_payload, _signatures, _auth_context)
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth {
            use super::*;
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw(
                env: soroban_sdk::Env,
                arg_0: soroban_sdk::Val,
                arg_1: soroban_sdk::Val,
                arg_2: soroban_sdk::Val,
            ) -> soroban_sdk::Val {
                <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
                    #[allow(deprecated)]
                    &<super::Contract>::__check_auth(
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_0
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_1
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_2
                            ),
                        ),
                    ),
                    &env,
                )
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw_slice(
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
                invoke_raw(env, args[0usize], args[1usize], args[2usize])
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
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
        extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor(
        ) {
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "__check_auth",
                #[allow(deprecated)]
                &__Contract____check_auth::invoke_raw_slice,
            );
        }
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        #[link_section = ".init_array"]
        static __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor() -> usize {
                __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor();
                0
            }
            __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor
        };
    }
    mod auth_decline {
        use super::*;
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
        #[repr(u32)]
        pub enum Error {
            Decline = 1,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Error {}
        #[automatically_derived]
        impl ::core::clone::Clone for Error {
            #[inline]
            fn clone(&self) -> Error {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Decline")
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Error {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Error {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Error {
            #[inline]
            fn eq(&self, other: &Error) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Error {
            #[inline]
            fn partial_cmp(&self, other: &Error) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Error {
            #[inline]
            fn cmp(&self, other: &Error) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
        pub static __SPEC_XDR_TYPE_ERROR: [u8; 48usize] = Error::spec_xdr();
        impl Error {
            pub const fn spec_xdr() -> [u8; 48usize] {
                *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x05Error\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x07Decline\0\0\0\0\x01"
            }
        }
        impl TryFrom<soroban_sdk::Error> for Error {
            type Error = soroban_sdk::Error;
            #[inline(always)]
            fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
                if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                    let discriminant = error.get_code();
                    Ok(match discriminant {
                        1u32 => Self::Decline,
                        _ => return Err(error),
                    })
                } else {
                    Err(error)
                }
            }
        }
        impl TryFrom<&soroban_sdk::Error> for Error {
            type Error = soroban_sdk::Error;
            #[inline(always)]
            fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
                <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
            }
        }
        impl From<Error> for soroban_sdk::Error {
            #[inline(always)]
            fn from(val: Error) -> soroban_sdk::Error {
                <_ as From<&Error>>::from(&val)
            }
        }
        impl From<&Error> for soroban_sdk::Error {
            #[inline(always)]
            fn from(val: &Error) -> soroban_sdk::Error {
                match val {
                    Error::Decline => soroban_sdk::Error::from_contract_error(1u32),
                }
            }
        }
        impl TryFrom<soroban_sdk::InvokeError> for Error {
            type Error = soroban_sdk::InvokeError;
            #[inline(always)]
            fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
                match error {
                    soroban_sdk::InvokeError::Abort => Err(error),
                    soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                        1u32 => Self::Decline,
                        _ => return Err(error),
                    }),
                }
            }
        }
        impl TryFrom<&soroban_sdk::InvokeError> for Error {
            type Error = soroban_sdk::InvokeError;
            #[inline(always)]
            fn try_from(
                error: &soroban_sdk::InvokeError,
            ) -> Result<Self, soroban_sdk::InvokeError> {
                <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
            }
        }
        impl From<Error> for soroban_sdk::InvokeError {
            #[inline(always)]
            fn from(val: Error) -> soroban_sdk::InvokeError {
                <_ as From<&Error>>::from(&val)
            }
        }
        impl From<&Error> for soroban_sdk::InvokeError {
            #[inline(always)]
            fn from(val: &Error) -> soroban_sdk::InvokeError {
                match val {
                    Error::Decline => soroban_sdk::InvokeError::Contract(1u32),
                }
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for Error {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &soroban_sdk::Val,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                use soroban_sdk::TryIntoVal;
                let error: soroban_sdk::Error = val.try_into_val(env)?;
                error.try_into().map_err(|_| soroban_sdk::ConversionError)
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, Error> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &Error,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                let error: soroban_sdk::Error = val.into();
                Ok(error.into())
            }
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, &Error> for soroban_sdk::Val {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(
                env: &soroban_sdk::Env,
                val: &&Error,
            ) -> Result<Self, soroban_sdk::ConversionError> {
                <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Error>>::try_from_val(env, *val)
            }
        }
        impl Contract {
            #[allow(non_snake_case)]
            pub fn __check_auth(
                _signature_payload: Val,
                _signatures: Val,
                _auth_context: Val,
            ) -> Result<(), Error> {
                Err(Error::Decline)
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth__spec {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            #[allow(non_snake_case)]
            pub static __SPEC_XDR_FN___CHECK_AUTH: [u8; 128usize] =
                super::Contract::spec_xdr___check_auth();
        }
        impl Contract {
            #[allow(non_snake_case)]
            #[allow(non_snake_case)]
            pub const fn spec_xdr___check_auth() -> [u8; 128usize] {
                *b"\0\0\0\0\0\0\0\0\0\0\0\x0c__check_auth\0\0\0\x03\0\0\0\0\0\0\0\x11signature_payload\0\0\0\0\0\0\0\0\0\0\0\0\0\0\nsignatures\0\0\0\0\0\0\0\0\0\0\0\0\0\x0cauth_context\0\0\0\0\0\0\0\x01\0\0\x03\xe9\0\0\x03\xed\0\0\0\0\0\0\0\x03"
            }
        }
        impl<'a> ContractClient<'a> {}
        impl ContractArgs {
            #[inline(always)]
            #[allow(clippy::unused_unit)]
            pub fn __check_auth<'i>(
                _signature_payload: &'i Val,
                _signatures: &'i Val,
                _auth_context: &'i Val,
            ) -> (&'i Val, &'i Val, &'i Val) {
                (_signature_payload, _signatures, _auth_context)
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        pub mod __Contract____check_auth {
            use super::*;
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw(
                env: soroban_sdk::Env,
                arg_0: soroban_sdk::Val,
                arg_1: soroban_sdk::Val,
                arg_2: soroban_sdk::Val,
            ) -> soroban_sdk::Val {
                <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
                    #[allow(deprecated)]
                    &<super::Contract>::__check_auth(
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_0
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_1
                            ),
                        ),
                        <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                            <_ as soroban_sdk::TryFromValForContractFn<
                                soroban_sdk::Env,
                                soroban_sdk::Val,
                            >>::try_from_val_for_contract_fn(
                                &env, &arg_2
                            ),
                        ),
                    ),
                    &env,
                )
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
            pub fn invoke_raw_slice(
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
                invoke_raw(env, args[0usize], args[1usize], args[2usize])
            }
            #[deprecated(
                note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead"
            )]
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
        extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor(
        ) {
            <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
                "__check_auth",
                #[allow(deprecated)]
                &__Contract____check_auth::invoke_raw_slice,
            );
        }
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        #[link_section = ".init_array"]
        static __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor() -> usize {
                __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor();
                0
            }
            __Contract__d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_with_mock_all_auth,
        &test_with_mock_auth,
        &test_with_real_contract_auth_approve,
        &test_with_real_contract_auth_decline,
        &test_with_mock_all_auth,
        &test_with_mock_auth,
        &test_with_real_contract_auth_approve,
        &test_with_real_contract_auth_decline,
    ])
}
