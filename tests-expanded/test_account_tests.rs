#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{
    auth::Context, auth::CustomAccountInterface, contract, contracterror, contractimpl,
    crypto::Hash, Env, Vec,
};
pub enum Error {
    Fail = 1,
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
impl ::core::marker::StructuralPartialEq for Error {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        true
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
pub static __SPEC_XDR_TYPE_ERROR: [u8; 44usize] = Error::spec_xdr();
impl Error {
    pub const fn spec_xdr() -> [u8; 44usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x05Error\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04Fail\0\0\0\x01"
    }
}
impl TryFrom<soroban_sdk::Error> for Error {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                1u32 => Self::Fail,
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
            Error::Fail => soroban_sdk::Error::from_contract_error(1u32),
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
                1u32 => Self::Fail,
                _ => return Err(error),
            }),
        }
    }
}
impl TryFrom<&soroban_sdk::InvokeError> for Error {
    type Error = soroban_sdk::InvokeError;
    #[inline(always)]
    fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
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
            Error::Fail => soroban_sdk::InvokeError::Contract(1u32),
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
impl CustomAccountInterface for Contract {
    type Error = Error;
    type Signature = ();
    #[allow(non_snake_case)]
    fn __check_auth(
        _env: Env,
        _signature_payload: Hash<32>,
        _signatures: Self::Signature,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        Ok(())
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
    pub static __SPEC_XDR_FN___CHECK_AUTH: [u8; 156usize] =
        super::Contract::spec_xdr___check_auth();
}
impl Contract {
    #[allow(non_snake_case)]
    #[allow(non_snake_case)]
    pub const fn spec_xdr___check_auth() -> [u8; 156usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0c__check_auth\0\0\0\x03\0\0\0\0\0\0\0\x11signature_payload\0\0\0\0\0\x03\xee\0\0\0 \0\0\0\0\0\0\0\nsignatures\0\0\0\0\x03\xed\0\0\0\0\0\0\0\0\0\0\0\rauth_contexts\0\0\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x07Context\0\0\0\0\x01\0\0\x03\xe9\0\0\x03\xed\0\0\0\0\0\0\0\x03"
    }
}
impl<'a> ContractClient<'a> {}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn __check_auth<'i>(
        _signature_payload: &'i Hash<32>,
        _signatures: &'i (),
        _auth_contexts: &'i Vec<Context>,
    ) -> (&'i Hash<32>, &'i (), &'i Vec<Context>) {
        (_signature_payload, _signatures, _auth_contexts)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(non_snake_case)]
pub mod __Contract____check_auth {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead")]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        use super::CustomAccountInterface;
        <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>>::into_val(
            #[allow(deprecated)]
            &<super::Contract>::__check_auth(
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead")]
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
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).__check_auth` instead")]
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
extern "C" fn __Contract_CustomAccountInterface_d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor(
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
#[link_section = "__DATA,__mod_init_func"]
static __Contract_CustomAccountInterface_d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
    #[allow(non_snake_case)]
    unsafe extern "C" fn __Contract_CustomAccountInterface_d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor() -> usize {
        __Contract_CustomAccountInterface_d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor();
        0
    }
    __Contract_CustomAccountInterface_d465b6861ce11142d9f64c1622e1ad88ae003d910de0a8493889a96a23449736_ctor___rust_ctor___ctor
};
mod test {
    use crate::Contract;
    use soroban_sdk::{
        contract,
        testutils::{MockAuth, MockAuthInvoke},
        Env, IntoVal,
    };
    struct TestContract;
    ///TestContractArgs is a type for building arg lists for functions defined in "TestContract".
    pub struct TestContractArgs;
    ///TestContractClient is a client for calling the contract defined in "TestContract".
    pub struct TestContractClient<'a> {
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
    impl<'a> TestContractClient<'a> {
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
    mod __testcontract_fn_set_registry {
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
    impl soroban_sdk::testutils::ContractFunctionRegister for TestContract {
        fn register(name: &'static str, func: &'static __testcontract_fn_set_registry::F) {
            __testcontract_fn_set_registry::register(name, func);
        }
    }
    #[doc(hidden)]
    impl soroban_sdk::testutils::ContractFunctionSet for TestContract {
        fn call(
            &self,
            func: &str,
            env: soroban_sdk::Env,
            args: &[soroban_sdk::Val],
        ) -> Option<soroban_sdk::Val> {
            __testcontract_fn_set_registry::call(func, env, args)
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/account/src/lib.rs",
            start_line: 45usize,
            start_col: 8usize,
            end_line: 45usize,
            end_col: 12usize,
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
        let test_contract_id = e.register(TestContract, ());
        let contract_id = e.register(Contract, ());
        e.set_auths(&[MockAuth {
            address: &contract_id,
            invoke: &MockAuthInvoke {
                contract: &test_contract_id,
                fn_name: "",
                args: ().into_val(&e),
                sub_invokes: &[],
            },
        }
        .into()]);
        e.as_contract(&test_contract_id, || contract_id.require_auth());
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test])
}
