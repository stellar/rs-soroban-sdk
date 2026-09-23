#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};
mod addcontract {
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01\x14\x04`\x01~\x01~`\x02\x7f~\x00`\x02~~\x01~`\x00\x00\x02\r\x02\x01i\x010\x00\x00\x01i\x01_\x00\x00\x03\x08\x07\x01\x01\x02\x03\x02\x02\x03\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x9c\x80\xc0\x00\x0b\x7f\x00A\x9c\x80\xc0\x00\x0b\x7f\x00A\xa0\x80\xc0\x00\x0b\x07I\x07\x06memory\x02\x00\x03add\x00\x04\x08safe_add\x00\x06\x0csafe_add_two\x00\x07\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xf3\x04\x07]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc0\x00F\r\x00\x02@ \x02A\x06F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x88!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x80\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0b;\x00\x02@\x02@ \x01B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x01B\x08\x86B\x06\x84!\x01\x0c\x01\x0b \x01\x10\x81\x80\x80\x80\x00!\x01\x0b \x00B\x007\x03\x00 \x00 \x017\x03\x08\x0b\x8e\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@\x02@\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08\"\x01 \x00|\"\x00 \x01T\r\x01 \x02 \x00\x10\x83\x80\x80\x80\x00 \x02(\x02\x00A\x01G\r\x02\x0b\x00\x0b\x10\x85\x80\x80\x80\x00\x00\x0b \x02)\x03\x08!\x00 \x02A\x10j$\x80\x80\x80\x80\x00 \x00\x0b\t\x00\x10\x88\x80\x80\x80\x00\x00\x0b\x9b\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x03 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00A\x00-\x00\x80\x80\xc0\x80\x00\x1aB\x83\x80\x80\x80\x10!\x01\x02@ \x00 \x03|\"\x03 \x00T\r\x00 \x02 \x03\x10\x83\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x01 \x02)\x03\x08!\x01\x0b \x02A\x10j$\x80\x80\x80\x80\x00 \x01\x0f\x0b\x00\x0b\x9b\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x03 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00A\x00-\x00\x8e\x80\xc0\x80\x00\x1aB\x83\x80\x80\x80\x10!\x01\x02@ \x00 \x03|\"\x03 \x00T\r\x00 \x02 \x03\x10\x83\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x01 \x02)\x03\x08!\x01\x0b \x02A\x10j$\x80\x80\x80\x80\x00 \x01\x0f\x0b\x00\x0b\x03\x00\x00\x0b\x0b%\x01\x00A\x80\x80\xc0\x00\x0b\x1cSpEcV2W\xa6\x99\xdd\xee\xdb\xf9\x82SpEcV2^y\xe5\xfc\x15\x10\x82\x93\x00\x83\x1c\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03add\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05Error\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x08Overflow\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07MyError\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x08Overflow\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08safe_add\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x06\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0csafe_add_two\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x06\x00\x00\x07\xd0\x00\x00\x00\x07MyError\x00\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00xExecutable specified by the contract instance as a specific Wasm contract code entry identified by its Wasm sha256 hash.\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00_Executable reference via a persistent storage entry owned by this contract or another contract.\x00\x00\x00\x00\x0bExternalRef\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15ContractExecutableRef\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\xc0Executable referenced via a persistent storage entry owned by a contract,\neither this contract or another contract.\n\nThe persistent storage entry owned by the `owner` has the `tag` as its key.\x00\x00\x00\x00\x00\x00\x00\x15ContractExecutableRef\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05owner\x00\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x03tag\x00\x00\x00\x00\x10\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x07Context\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nExecutable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1d\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
    pub trait Contract {
        fn add(env: soroban_sdk::Env, a: u64, b: u64) -> u64;
        fn safe_add(env: soroban_sdk::Env, a: u64, b: u64) -> Result<u64, Error>;
        fn safe_add_two(env: soroban_sdk::Env, a: u64, b: u64) -> Result<u64, MyError>;
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
    impl<'a> Client<'a> {
        pub fn add(&self, a: &u64, b: &u64) -> u64 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_add(
            &self,
            a: &u64,
            b: &u64,
        ) -> Result<
            Result<
                u64,
                <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn safe_add(&self, a: &u64, b: &u64) -> u64 {
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("safe_add");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_safe_add(
            &self,
            a: &u64,
            b: &u64,
        ) -> Result<
            Result<
                u64,
                <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<Error, soroban_sdk::InvokeError>,
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
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("safe_add");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn safe_add_two(&self, a: &u64, b: &u64) -> u64 {
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
                &{ soroban_sdk::Symbol::new(&self.env, "safe_add_two") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_safe_add_two(
            &self,
            a: &u64,
            b: &u64,
        ) -> Result<
            Result<
                u64,
                <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<MyError, soroban_sdk::InvokeError>,
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
                &{ soroban_sdk::Symbol::new(&self.env, "safe_add_two") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
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
        pub fn add<'i>(a: &'i u64, b: &'i u64) -> (&'i u64, &'i u64) {
            (a, b)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn safe_add<'i>(a: &'i u64, b: &'i u64) -> (&'i u64, &'i u64) {
            (a, b)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn safe_add_two<'i>(a: &'i u64, b: &'i u64) -> (&'i u64, &'i u64) {
            (a, b)
        }
    }
    pub struct ContractExecutableRef {
        pub owner: soroban_sdk::Address,
        pub tag: soroban_sdk::String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ContractExecutableRef {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ContractExecutableRef",
                "owner",
                &self.owner,
                "tag",
                &&self.tag,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ContractExecutableRef {
        #[inline]
        fn clone(&self) -> ContractExecutableRef {
            ContractExecutableRef {
                owner: ::core::clone::Clone::clone(&self.owner),
                tag: ::core::clone::Clone::clone(&self.tag),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ContractExecutableRef {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ContractExecutableRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ContractExecutableRef {
        #[inline]
        fn eq(&self, other: &ContractExecutableRef) -> bool {
            self.owner == other.owner && self.tag == other.tag
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ContractExecutableRef {
        #[inline]
        fn cmp(&self, other: &ContractExecutableRef) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.owner, &other.owner) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.tag, &other.tag),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ContractExecutableRef {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ContractExecutableRef,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.owner, &other.owner) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.tag, &other.tag)
                }
                cmp => cmp,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLEREF: [u8; 80usize] = ContractExecutableRef::spec_xdr();
    impl ContractExecutableRef {
        pub const fn spec_xdr() -> [u8; 80usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x15ContractExecutableRef\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x05owner\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x03tag\0\0\0\0\x10"
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutableRef {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::String as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractExecutableRef {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["owner", "tag"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                owner: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                tag: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutableRef> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ContractExecutableRef,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["owner", "tag"];
            let vals: [Val; 2usize] = [
                (&val.owner)
                    .try_into_val(env)
                    .map_err(|_| ConversionError)?,
                (&val.tag).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ContractExecutableRef> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ContractExecutableRef,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutableRef>>::try_from_val(
                env, *val,
            )
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for ContractExecutableRef {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScMap,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let map = val;
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            map.validate()?;
            Ok(Self {
                owner: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "owner"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                tag: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "tag"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for ContractExecutableRef {
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
    impl TryFrom<&ContractExecutableRef> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "owner"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.owner)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "tag"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.tag)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
            ])))
        }
    }
    impl TryFrom<ContractExecutableRef> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&ContractExecutableRef> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
        }
    }
    impl TryFrom<ContractExecutableRef> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryContractExecutableRef {
            owner: <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            tag: <soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryContractExecutableRef {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArbitraryContractExecutableRef",
                    "owner",
                    &self.owner,
                    "tag",
                    &&self.tag,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryContractExecutableRef {
            #[inline]
            fn clone(&self) -> ArbitraryContractExecutableRef {
                ArbitraryContractExecutableRef {
                    owner: ::core::clone::Clone::clone(&self.owner),
                    tag: ::core::clone::Clone::clone(&self.tag),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryContractExecutableRef {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryContractExecutableRef {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryContractExecutableRef {
            #[inline]
            fn eq(&self, other: &ArbitraryContractExecutableRef) -> bool {
                self.owner == other.owner && self.tag == other.tag
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryContractExecutableRef {
            #[inline]
            fn cmp(&self, other: &ArbitraryContractExecutableRef) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.owner, &other.owner) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.tag, &other.tag),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryContractExecutableRef {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryContractExecutableRef,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.owner, &other.owner) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.tag, &other.tag)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryContractExecutableRef: ::std::thread::LocalKey<
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
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContractExecutableRef {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryContractExecutableRef {
                            owner: arbitrary::Arbitrary::arbitrary(u)?,
                            tag: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
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
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryContractExecutableRef {
                            owner: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            tag: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
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
                                <<soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for ContractExecutableRef {
            type Prototype = ArbitraryContractExecutableRef;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContractExecutableRef>
            for ContractExecutableRef
        {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryContractExecutableRef,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(ContractExecutableRef {
                    owner: soroban_sdk::IntoVal::into_val(&v.owner, env),
                    tag: soroban_sdk::IntoVal::into_val(&v.tag, env),
                })
            }
        }
    };
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
                    match ::core::cmp::PartialOrd::partial_cmp(&self.contract, &other.contract) {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; 96usize] = ContractContext::spec_xdr();
    impl ContractContext {
        pub const fn spec_xdr() -> [u8; 96usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fContractContext\0\0\0\0\x03\0\0\0\0\0\0\0\x04args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\x08contract\0\0\0\x13\0\0\0\0\0\0\0\x07fn_name\0\0\0\0\x11"
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Vec<
                soroban_sdk::Val,
            > as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Symbol as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_SUBCONTRACTINVOCATION: [u8; 144usize] =
        SubContractInvocation::spec_xdr();
    impl SubContractInvocation {
        pub const fn spec_xdr() -> [u8; 144usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x07context\0\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\0\0\0\0\x0fsub_invocations\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x18InvokerContractAuthEntry"
        }
    }
    impl soroban_sdk::SpecShakingMarker for SubContractInvocation {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Vec<
                InvokerContractAuthEntry,
            > as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, SubContractInvocation>>::try_from_val(
                env, *val,
            )
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8; 116usize] =
        CreateContractHostFnContext::spec_xdr();
    impl CreateContractHostFnContext {
        pub const fn spec_xdr() -> [u8; 116usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x02\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
        }
    }
    impl soroban_sdk::SpecShakingMarker for CreateContractHostFnContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractExecutable as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
        fn try_from(val: &CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
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
        fn try_from(val: &CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
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
            fn cmp(&self, other: &ArbitraryCreateContractHostFnContext) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.salt, &other.salt),
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
                match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext: ::std::thread::LocalKey<
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
        fn cmp(&self, other: &CreateContractWithConstructorHostFnContext) -> ::core::cmp::Ordering {
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
                    match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable)
                    {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8; 164usize] =
        CreateContractWithConstructorHostFnContext::spec_xdr();
    impl CreateContractWithConstructorHostFnContext {
        pub const fn spec_xdr() -> [u8; 164usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0*CreateContractWithConstructorHostFnContext\0\0\0\0\0\x03\0\0\0\0\0\0\0\x10constructor_args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
        }
    }
    impl soroban_sdk::SpecShakingMarker for CreateContractWithConstructorHostFnContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Vec<
                soroban_sdk::Val,
            > as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <ContractExecutable as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            },
                        )?;
                    }
                    let result = (|| {
                        Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                            constructor_args: arbitrary::Arbitrary::arbitrary(u)?,
                            executable: arbitrary::Arbitrary::arbitrary(u)?,
                            salt: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                count.set(count.get() - 1);
                            },
                        );
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            },
                        )?;
                    }
                    let result = (|| {
                        Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                            constructor_args: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            executable: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            salt: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                count.set(count.get() - 1);
                            },
                        );
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
    pub enum ContractExecutable {
        Wasm(soroban_sdk::BytesN<32>),
        ExternalRef(ContractExecutableRef),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ContractExecutable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ContractExecutable::Wasm(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                }
                ContractExecutable::ExternalRef(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ExternalRef", &__self_0)
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
                ContractExecutable::ExternalRef(__self_0) => {
                    ContractExecutable::ExternalRef(::core::clone::Clone::clone(__self_0))
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
            let _: ::core::cmp::AssertParamIsEq<ContractExecutableRef>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ContractExecutable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ContractExecutable {
        #[inline]
        fn eq(&self, other: &ContractExecutable) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (
                        ContractExecutable::ExternalRef(__self_0),
                        ContractExecutable::ExternalRef(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ContractExecutable {
        #[inline]
        fn cmp(&self, other: &ContractExecutable) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (
                        ContractExecutable::ExternalRef(__self_0),
                        ContractExecutable::ExternalRef(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                },
                cmp => cmp,
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
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (
                    ContractExecutable::ExternalRef(__self_0),
                    ContractExecutable::ExternalRef(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; 128usize] = ContractExecutable::spec_xdr();
    impl ContractExecutable {
        pub const fn spec_xdr() -> [u8; 128usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x12ContractExecutable\0\0\0\0\0\x02\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\x01\0\0\0\0\0\0\0\x0bExternalRef\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x15ContractExecutableRef\0\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <ContractExecutableRef as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractExecutable {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["Wasm", "ExternalRef"];
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
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::ExternalRef(
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
                ContractExecutable::ExternalRef(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"ExternalRef")?.to_val(),
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
                "ExternalRef" => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    let rv0: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    Self::ExternalRef(
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
                ContractExecutable::ExternalRef(value0) => (
                    soroban_sdk::xdr::ScSymbol(
                        "ExternalRef"
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
            ExternalRef(
                <ContractExecutableRef as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
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
                    ArbitraryContractExecutable::ExternalRef(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ExternalRef",
                            &__self_0,
                        )
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
                    ArbitraryContractExecutable::ExternalRef(__self_0) => {
                        ArbitraryContractExecutable::ExternalRef(::core::clone::Clone::clone(
                            __self_0,
                        ))
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
                let _: ::core::cmp::AssertParamIsEq<
                    <ContractExecutableRef as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryContractExecutable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryContractExecutable {
            #[inline]
            fn eq(&self, other: &ArbitraryContractExecutable) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            ArbitraryContractExecutable::ExternalRef(__self_0),
                            ArbitraryContractExecutable::ExternalRef(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryContractExecutable {
            #[inline]
            fn cmp(&self, other: &ArbitraryContractExecutable) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            ArbitraryContractExecutable::ExternalRef(__self_0),
                            ArbitraryContractExecutable::ExternalRef(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
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
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (
                        ArbitraryContractExecutable::Wasm(__self_0),
                        ArbitraryContractExecutable::Wasm(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        ArbitraryContractExecutable::ExternalRef(__self_0),
                        ArbitraryContractExecutable::ExternalRef(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 2u64)
                                >> 32
                            {
                                0u64 => ArbitraryContractExecutable::Wasm(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                1u64 => ArbitraryContractExecutable::ExternalRef(
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
                                * 2u64)
                                >> 32
                            {
                                0u64 => ArbitraryContractExecutable::Wasm(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                1u64 => ArbitraryContractExecutable::ExternalRef(
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
                                        arbitrary::size_hint::and_all(
                                            &[
                                                <<ContractExecutableRef as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
                    ArbitraryContractExecutable::ExternalRef(field_0) => {
                        ContractExecutable::ExternalRef(soroban_sdk::IntoVal::into_val(
                            field_0, env,
                        ))
                    }
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
        fn partial_cmp(&self, other: &Context) -> ::core::option::Option<::core::cmp::Ordering> {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTEXT: [u8; 244usize] = Context::spec_xdr();
    impl Context {
        pub const fn spec_xdr() -> [u8; 244usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07Context\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for Context {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                Context::CreateContractWithCtorHostFn(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractWithCtorHostFn")?
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
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
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
                        ArbitraryContext::CreateContractWithCtorHostFn(::core::clone::Clone::clone(
                            __self_0,
                        ))
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
            const RECURSIVE_COUNT_ArbitraryContext: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => {
                                    ArbitraryContext::Contract(arbitrary::Arbitrary::arbitrary(u)?)
                                }
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
                        Context::CreateContractHostFn(soroban_sdk::IntoVal::into_val(field_0, env))
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; 268usize] =
        InvokerContractAuthEntry::spec_xdr();
    impl InvokerContractAuthEntry {
        pub const fn spec_xdr() -> [u8; 268usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x18InvokerContractAuthEntry\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for InvokerContractAuthEntry {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <SubContractInvocation as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                InvokerContractAuthEntry::CreateContractWithCtorHostFn(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractWithCtorHostFn")?
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, InvokerContractAuthEntry>>::try_from_val(
                env, *val,
            )
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
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractHostFn",
                            &__self_0,
                        )
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
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
        impl ::core::clone::Clone for ArbitraryInvokerContractAuthEntry {
            #[inline]
            fn clone(&self) -> ArbitraryInvokerContractAuthEntry {
                match self {
                    ArbitraryInvokerContractAuthEntry::Contract(__self_0) => {
                        ArbitraryInvokerContractAuthEntry::Contract(::core::clone::Clone::clone(
                            __self_0,
                        ))
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
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
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryInvokerContractAuthEntry::Contract(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                1u64 => ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                2u64 => {
                                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    )
                                }
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryInvokerContractAuthEntry::Contract(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                1u64 => ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                2u64 => {
                                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    )
                                }
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
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
                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(field_0) => {
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            soroban_sdk::IntoVal::into_val(field_0, env),
                        )
                    }
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
                Executable::StellarAsset => ::core::fmt::Formatter::write_str(f, "StellarAsset"),
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
        fn partial_cmp(&self, other: &Executable) -> ::core::option::Option<::core::cmp::Ordering> {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_EXECUTABLE: [u8; 104usize] = Executable::spec_xdr();
    impl Executable {
        pub const fn spec_xdr() -> [u8; 104usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\nExecutable\0\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\0\0\0\0\0\0\0\0\x0cStellarAsset\0\0\0\0\0\0\0\0\0\0\0\x07Account\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for Executable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Executable>>::try_from_val(env, *val)
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
                    ArbitraryExecutable::Account => ::core::fmt::Formatter::write_str(f, "Account"),
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
                    (ArbitraryExecutable::Wasm(__self_0), ArbitraryExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => {
                                    ArbitraryExecutable::Wasm(arbitrary::Arbitrary::arbitrary(u)?)
                                }
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
    pub enum Error {
        Overflow = 1,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Overflow")
        }
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
    impl ::core::cmp::Ord for Error {
        #[inline]
        fn cmp(&self, other: &Error) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Error {
        #[inline]
        fn partial_cmp(&self, other: &Error) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_ERROR: [u8; 48usize] = Error::spec_xdr();
    impl Error {
        pub const fn spec_xdr() -> [u8; 48usize] {
            *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x05Error\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x08Overflow\0\0\0\x01"
        }
    }
    impl soroban_sdk::SpecShakingMarker for Error {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {}
    }
    impl TryFrom<soroban_sdk::Error> for Error {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    1u32 => Self::Overflow,
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
                Error::Overflow => soroban_sdk::Error::from_contract_error(1u32),
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
                    1u32 => Self::Overflow,
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
                Error::Overflow => soroban_sdk::InvokeError::Contract(1u32),
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
    pub enum MyError {
        Overflow = 1,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MyError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Overflow")
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MyError {}
    #[automatically_derived]
    impl ::core::clone::Clone for MyError {
        #[inline]
        fn clone(&self) -> MyError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MyError {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyError {
        #[inline]
        fn eq(&self, other: &MyError) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for MyError {
        #[inline]
        fn cmp(&self, other: &MyError) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for MyError {
        #[inline]
        fn partial_cmp(&self, other: &MyError) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_MYERROR: [u8; 48usize] = MyError::spec_xdr();
    impl MyError {
        pub const fn spec_xdr() -> [u8; 48usize] {
            *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x07MyError\0\0\0\0\x01\0\0\0\0\0\0\0\x08Overflow\0\0\0\x01"
        }
    }
    impl soroban_sdk::SpecShakingMarker for MyError {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {}
    }
    impl TryFrom<soroban_sdk::Error> for MyError {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    1u32 => Self::Overflow,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for MyError {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<MyError> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: MyError) -> soroban_sdk::Error {
            <_ as From<&MyError>>::from(&val)
        }
    }
    impl From<&MyError> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &MyError) -> soroban_sdk::Error {
            match val {
                MyError::Overflow => soroban_sdk::Error::from_contract_error(1u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for MyError {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    1u32 => Self::Overflow,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for MyError {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<MyError> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: MyError) -> soroban_sdk::InvokeError {
            <_ as From<&MyError>>::from(&val)
        }
    }
    impl From<&MyError> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &MyError) -> soroban_sdk::InvokeError {
            match val {
                MyError::Overflow => soroban_sdk::InvokeError::Contract(1u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for MyError {
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
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, MyError> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &MyError,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &MyError> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&MyError,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, MyError>>::try_from_val(env, *val)
        }
    }
}
mod eventscontract {
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x010\t`\x01~\x01~`\x02~~\x01~`\x03~~~\x01~`\x01\x7f\x00`\x01\x7f\x01~`\x02\x7f\x7f\x01~`\x02\x7f~\x01~`\x02\x7f~\x00`\x00\x00\x02C\x0b\x01i\x01_\x00\x00\x01x\x011\x00\x01\x01a\x014\x00\x00\x01a\x015\x00\x00\x01i\x010\x00\x00\x01v\x01g\x00\x01\x01i\x018\x00\x00\x01i\x017\x00\x00\x01b\x01j\x00\x01\x01i\x016\x00\x01\x01m\x01b\x00\x02\x03\x11\x10\x03\x04\x05\x01\x05\x06\x02\x07\x08\x02\x05\x01\x00\x02\x08\x02\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\xb5\x81\xc0\x00\x0b\x7f\x00A\xb5\x81\xc0\x00\x0b\x7f\x00A\xc0\x81\xc0\x00\x0b\x07\x83\x01\n\x06memory\x02\x00\x0ffailed_transfer\x00\x11\nmap_values\x00\x14\x0csingle_value\x00\x16\x11single_value_void\x00\x17\x08transfer\x00\x18\nvec_values\x00\x1a\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xe2\r\x10\xb9\x02\x02\x02\x7f\x03~#\x80\x80\x80\x80\x00A0k\"\x01$\x80\x80\x80\x80\x00A\x00!\x02A\x00-\x00\x80\x80\xc0\x80\x00\x1a \x00)\x03 !\x03 \x00)\x03(!\x04A\xe8\x80\xc0\x80\x00\x10\x8c\x80\x80\x80\x00!\x05 \x01 \x047\x03\x10 \x01 \x037\x03\x08 \x01 \x057\x03\x00\x03@\x02@ \x02A\x18G\r\x00A\x00!\x02\x02@\x03@ \x02A\x18F\r\x01 \x01A\x18j \x02j \x01 \x02j)\x03\x007\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b \x01A\x18jA\x03\x10\x8d\x80\x80\x80\x00!\x04 \x00)\x03\x10 \x00)\x03\x18\x10\x8e\x80\x80\x80\x00!\x05B\x02!\x03\x02@ \x00(\x02\x00A\x01G\r\x00\x02@ \x00)\x03\x08\"\x03B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x03B\x08\x86B\x06\x84!\x03\x0c\x01\x0b \x03\x10\x80\x80\x80\x80\x00!\x03\x0b \x01 \x037\x03  \x01 \x057\x03\x18 \x04A\xd8\x80\xc0\x80\x00 \x01A\x18j\x10\x8f\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x01A0j$\x80\x80\x80\x80\x00\x0f\x0b \x01A\x18j \x02jB\x027\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b\x07\x00 \x00)\x03\x00\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x85\x80\x80\x80\x00\x0bE\x00\x02@ \x00B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00 \x00\x85 \x01 \x00B?\x87\x85\x84B\x00R\r\x00 \x00B\x08\x86B\x0b\x84\x0f\x0b \x01 \x00\x10\x89\x80\x80\x80\x00\x0b \x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x8a\x80\x80\x80\x00\x0b\xa0\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00 \x00\x10\x8c\x80\x80\x80\x00!\x03 \x02 \x017\x03\x08 \x02 \x037\x03\x00A\x00!\x00\x03~\x02@ \x00A\x10G\r\x00A\x00!\x00\x02@\x03@ \x00A\x10F\r\x01 \x02A\x10j \x00j \x02 \x00j)\x03\x007\x03\x00 \x00A\x08j!\x00\x0c\x00\x0b\x0b \x02A\x10jA\x02\x10\x8d\x80\x80\x80\x00!\x01 \x02A j$\x80\x80\x80\x80\x00 \x01\x0f\x0b \x02A\x10j \x00jB\x027\x03\x00 \x00A\x08j!\x00\x0c\x00\x0b\x0b\x80\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A0k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\xcd\x00R\r\x00 \x03 \x02\x10\x92\x80\x80\x80\x00 \x03(\x02\x00A\x01F\r\x00 \x03)\x03\x10!\x02 \x03 \x03)\x03\x187\x03\x18 \x03 \x027\x03\x10 \x03 \x017\x03( \x03 \x007\x03  \x03B\x007\x03\x00 \x03\x10\x8b\x80\x80\x80\x00\x10\x93\x80\x80\x80\x00\x0b\x00\x0b}\x02\x01\x7f\x01~\x02@\x02@\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc5\x00F\r\x00 \x02A\x0bG\r\x02 \x00 \x01B?\x877\x03\x18 \x00 \x01B\x08\x877\x03\x10\x0c\x01\x0b \x01\x10\x86\x80\x80\x80\x00!\x03 \x01\x10\x87\x80\x80\x80\x00!\x01 \x00 \x037\x03\x18 \x00 \x017\x03\x10\x0bB\x00!\x01\x0c\x01\x0b \x00B\x83\x90\x80\x80\x80\x017\x03\x08B\x01!\x01\x0b \x00 \x017\x03\x00\x0b\x03\x00\x00\x0b\xa5\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\x04R\r\x00 \x02B\xff\x01\x83B\x04R\r\x00A\x00-\x00\x8e\x80\xc0\x80\x00\x1a \x03A\x84\x81\xc0\x80\x00A\n\x10\x95\x80\x80\x80\x007\x03\x00 \x03 \x00\x10\x90\x80\x80\x80\x00!\x00 \x03 \x02B\x84\x80\x80\x80p\x837\x03\x08 \x03 \x01B\x84\x80\x80\x80p\x837\x03\x00 \x00A\xf4\x80\xc0\x80\x00 \x03\x10\x8f\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x03A\x10j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x88\x80\x80\x80\x00\x0b\x8f\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x02 \x01\x10\x92\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00A\x00-\x00\xaa\x80\xc0\x80\x00\x1a \x02)\x03\x18!\x01 \x02)\x03\x10!\x03 \x02A\x98\x81\xc0\x80\x00A\x0c\x10\x95\x80\x80\x80\x007\x03\x00 \x02 \x00\x10\x90\x80\x80\x80\x00 \x03 \x01\x10\x8e\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x02A j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0be\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00Q\r\x00\x00\x0bA\x00-\x00\xb8\x80\xc0\x80\x00\x1a \x01A\xa4\x81\xc0\x80\x00A\x11\x10\x95\x80\x80\x80\x007\x03\x08 \x01A\x08j \x00\x10\x90\x80\x80\x80\x00B\x02\x10\x81\x80\x80\x80\x00\x1a \x01A\x10j$\x80\x80\x80\x80\x00B\x02\x0b\x8e\x02\x02\x02\x7f\x04~#\x80\x80\x80\x80\x00A0k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00A\x01!\x04\x02@\x02@ \x01\xa7A\xff\x01qA\xb3\x7fj\x0e\x02\x00\x01\x02\x0bA\x00!\x04\x0b \x03 \x02\x10\x92\x80\x80\x80\x00 \x03(\x02\x00A\x01F\r\x00 \x03)\x03\x18!\x02 \x03)\x03\x10!\x05\x02@\x02@ \x04\r\x00B\x00!\x06\x0c\x01\x0b \x01\x10\x82\x80\x80\x80\x00!\x07\x02@\x02@ \x01\x10\x83\x80\x80\x80\x00\"\x01\xa7A\xff\x01q\"\x04A\x06F\r\x00\x02@ \x04A\xc0\x00G\r\x00B\x01!\x06 \x01\x10\x84\x80\x80\x80\x00!\x08\x0c\x02\x0b\x10\x99\x80\x80\x80\x00\x00\x0b \x01B\x08\x88!\x08B\x01!\x06\x0b \x07!\x01\x0b \x03 \x057\x03\x10 \x03 \x017\x03( \x03 \x007\x03  \x03 \x087\x03\x08 \x03 \x067\x03\x00 \x03 \x027\x03\x18 \x03\x10\x8b\x80\x80\x80\x00 \x03A0j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0b\t\x00\x10\x93\x80\x80\x80\x00\x00\x0b\xa1\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\x04R\r\x00 \x02B\xff\x01\x83B\x04R\r\x00A\x00-\x00\x9c\x80\xc0\x80\x00\x1a \x03A\x8e\x81\xc0\x80\x00A\n\x10\x95\x80\x80\x80\x007\x03\x00 \x03 \x00\x10\x90\x80\x80\x80\x00!\x00 \x03 \x02B\x84\x80\x80\x80p\x837\x03\x08 \x03 \x01B\x84\x80\x80\x80p\x837\x03\x00 \x00 \x03A\x02\x10\x8d\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x03A\x10j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0b\x0b\xbf\x01\x01\x00A\x80\x80\xc0\x00\x0b\xb5\x01SpEcV2\x00R\xe4\xd1\xf1dRTSpEcV2\xe5\xa9\x03\xf2?\xa3}XSpEcV2_\x99\xa9h9\xd5*\xd1SpEcV2&uo\xfe&\x9f\xca\x14SpEcV2Z\xae\xe7w\xae\xff\xa0\x89amountto_muxed_id\x00F\x00\x10\x00\x06\x00\x00\x00L\x00\x10\x00\x0b\x00\x00\x00\x0e\xb7\xba\xe2\xb3y\xe7\x00ab\x00\x00p\x00\x10\x00\x01\x00\x00\x00q\x00\x10\x00\x01\x00\x00\x00map_valuesvec_valuessingle_valuesingle_value_void\x00\xbf#\x0econtractspecv0\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08Transfer\x00\x00\x00\x01\x00\x00\x00\x08transfer\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02to\x00\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0bto_muxed_id\x00\x00\x00\x03\xe8\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08transfer\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02to\x00\x00\x00\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x003An event whose data is a map, which is the default.\x00\x00\x00\x00\x00\x00\x00\x00\tMapValues\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\nmap_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x05\x00\x00\x000An event whose data is a vec, rather than a map.\x00\x00\x00\x00\x00\x00\x00\tVecValues\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\nvec_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nmap_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nvec_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x009An event whose data is a single value, rather than a map.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0bSingleValue\x00\x00\x00\x00\x01\x00\x00\x00\x0csingle_value\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0csingle_value\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x00^An event whose data is a single value, and that has no data fields, and so\nwhose data is void.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fSingleValueVoid\x00\x00\x00\x00\x01\x00\x00\x00\x11single_value_void\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0ffailed_transfer\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02to\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11single_value_void\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00xExecutable specified by the contract instance as a specific Wasm contract code entry identified by its Wasm sha256 hash.\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00_Executable reference via a persistent storage entry owned by this contract or another contract.\x00\x00\x00\x00\x0bExternalRef\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15ContractExecutableRef\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\xc0Executable referenced via a persistent storage entry owned by a contract,\neither this contract or another contract.\n\nThe persistent storage entry owned by the `owner` has the `tag` as its key.\x00\x00\x00\x00\x00\x00\x00\x15ContractExecutableRef\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05owner\x00\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x03tag\x00\x00\x00\x00\x10\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x07Context\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nExecutable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1d\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
    pub trait Contract {
        fn transfer(
            env: soroban_sdk::Env,
            from: soroban_sdk::Address,
            to: soroban_sdk::MuxedAddress,
            amount: i128,
        );
        fn map_values(env: soroban_sdk::Env, from: soroban_sdk::Address, a: u32, b: u32);
        fn vec_values(env: soroban_sdk::Env, from: soroban_sdk::Address, a: u32, b: u32);
        fn single_value(env: soroban_sdk::Env, from: soroban_sdk::Address, amount: i128);
        fn failed_transfer(
            env: soroban_sdk::Env,
            from: soroban_sdk::Address,
            to: soroban_sdk::Address,
            amount: i128,
        );
        fn single_value_void(env: soroban_sdk::Env, from: soroban_sdk::Address);
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
    impl<'a> Client<'a> {
        pub fn transfer(
            &self,
            from: &soroban_sdk::Address,
            to: impl Into<soroban_sdk::MuxedAddress>,
            amount: &i128,
        ) -> () {
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
            from: &soroban_sdk::Address,
            to: impl Into<soroban_sdk::MuxedAddress>,
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
        pub fn map_values(&self, from: &soroban_sdk::Address, a: &u32, b: &u32) -> () {
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
                &{ soroban_sdk::Symbol::new(&self.env, "map_values") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        from.into_val(&self.env),
                        a.into_val(&self.env),
                        b.into_val(&self.env),
                    ],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_map_values(
            &self,
            from: &soroban_sdk::Address,
            a: &u32,
            b: &u32,
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
                &{ soroban_sdk::Symbol::new(&self.env, "map_values") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        from.into_val(&self.env),
                        a.into_val(&self.env),
                        b.into_val(&self.env),
                    ],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn vec_values(&self, from: &soroban_sdk::Address, a: &u32, b: &u32) -> () {
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
                &{ soroban_sdk::Symbol::new(&self.env, "vec_values") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        from.into_val(&self.env),
                        a.into_val(&self.env),
                        b.into_val(&self.env),
                    ],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_vec_values(
            &self,
            from: &soroban_sdk::Address,
            a: &u32,
            b: &u32,
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
                &{ soroban_sdk::Symbol::new(&self.env, "vec_values") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        from.into_val(&self.env),
                        a.into_val(&self.env),
                        b.into_val(&self.env),
                    ],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn single_value(&self, from: &soroban_sdk::Address, amount: &i128) -> () {
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
                &{ soroban_sdk::Symbol::new(&self.env, "single_value") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [from.into_val(&self.env), amount.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_single_value(
            &self,
            from: &soroban_sdk::Address,
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
                &{ soroban_sdk::Symbol::new(&self.env, "single_value") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [from.into_val(&self.env), amount.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn failed_transfer(
            &self,
            from: &soroban_sdk::Address,
            to: &soroban_sdk::Address,
            amount: &i128,
        ) -> () {
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
            from: &soroban_sdk::Address,
            to: &soroban_sdk::Address,
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
        pub fn single_value_void(&self, from: &soroban_sdk::Address) -> () {
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
                &{ soroban_sdk::Symbol::new(&self.env, "single_value_void") },
                ::soroban_sdk::Vec::from_array(&self.env, [from.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_single_value_void(
            &self,
            from: &soroban_sdk::Address,
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
                &{ soroban_sdk::Symbol::new(&self.env, "single_value_void") },
                ::soroban_sdk::Vec::from_array(&self.env, [from.into_val(&self.env)]),
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
        pub fn transfer<'i>(
            from: &'i soroban_sdk::Address,
            to: &'i soroban_sdk::MuxedAddress,
            amount: &'i i128,
        ) -> (
            &'i soroban_sdk::Address,
            &'i soroban_sdk::MuxedAddress,
            &'i i128,
        ) {
            (from, to, amount)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn map_values<'i>(
            from: &'i soroban_sdk::Address,
            a: &'i u32,
            b: &'i u32,
        ) -> (&'i soroban_sdk::Address, &'i u32, &'i u32) {
            (from, a, b)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn vec_values<'i>(
            from: &'i soroban_sdk::Address,
            a: &'i u32,
            b: &'i u32,
        ) -> (&'i soroban_sdk::Address, &'i u32, &'i u32) {
            (from, a, b)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn single_value<'i>(
            from: &'i soroban_sdk::Address,
            amount: &'i i128,
        ) -> (&'i soroban_sdk::Address, &'i i128) {
            (from, amount)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn failed_transfer<'i>(
            from: &'i soroban_sdk::Address,
            to: &'i soroban_sdk::Address,
            amount: &'i i128,
        ) -> (&'i soroban_sdk::Address, &'i soroban_sdk::Address, &'i i128) {
            (from, to, amount)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn single_value_void<'i>(
            from: &'i soroban_sdk::Address,
        ) -> (&'i soroban_sdk::Address,) {
            (from,)
        }
    }
    pub struct ContractExecutableRef {
        pub owner: soroban_sdk::Address,
        pub tag: soroban_sdk::String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ContractExecutableRef {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ContractExecutableRef",
                "owner",
                &self.owner,
                "tag",
                &&self.tag,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ContractExecutableRef {
        #[inline]
        fn clone(&self) -> ContractExecutableRef {
            ContractExecutableRef {
                owner: ::core::clone::Clone::clone(&self.owner),
                tag: ::core::clone::Clone::clone(&self.tag),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ContractExecutableRef {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ContractExecutableRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ContractExecutableRef {
        #[inline]
        fn eq(&self, other: &ContractExecutableRef) -> bool {
            self.owner == other.owner && self.tag == other.tag
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ContractExecutableRef {
        #[inline]
        fn cmp(&self, other: &ContractExecutableRef) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.owner, &other.owner) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.tag, &other.tag),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ContractExecutableRef {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ContractExecutableRef,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.owner, &other.owner) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.tag, &other.tag)
                }
                cmp => cmp,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLEREF: [u8; 80usize] = ContractExecutableRef::spec_xdr();
    impl ContractExecutableRef {
        pub const fn spec_xdr() -> [u8; 80usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x15ContractExecutableRef\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x05owner\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x03tag\0\0\0\0\x10"
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutableRef {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::String as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractExecutableRef {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["owner", "tag"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                owner: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                tag: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutableRef> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ContractExecutableRef,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["owner", "tag"];
            let vals: [Val; 2usize] = [
                (&val.owner)
                    .try_into_val(env)
                    .map_err(|_| ConversionError)?,
                (&val.tag).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ContractExecutableRef> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ContractExecutableRef,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ContractExecutableRef>>::try_from_val(
                env, *val,
            )
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for ContractExecutableRef {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScMap,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let map = val;
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            map.validate()?;
            Ok(Self {
                owner: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "owner"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                tag: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "tag"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for ContractExecutableRef {
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
    impl TryFrom<&ContractExecutableRef> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(::alloc::boxed::box_new([
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "owner"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.owner)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
                soroban_sdk::xdr::ScMapEntry {
                    key: soroban_sdk::xdr::ScSymbol(
                        "tag"
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into(),
                    val: (&val.tag)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                },
            ])))
        }
    }
    impl TryFrom<ContractExecutableRef> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&ContractExecutableRef> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
        }
    }
    impl TryFrom<ContractExecutableRef> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: ContractExecutableRef) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryContractExecutableRef {
            owner: <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            tag: <soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryContractExecutableRef {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArbitraryContractExecutableRef",
                    "owner",
                    &self.owner,
                    "tag",
                    &&self.tag,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryContractExecutableRef {
            #[inline]
            fn clone(&self) -> ArbitraryContractExecutableRef {
                ArbitraryContractExecutableRef {
                    owner: ::core::clone::Clone::clone(&self.owner),
                    tag: ::core::clone::Clone::clone(&self.tag),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryContractExecutableRef {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryContractExecutableRef {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryContractExecutableRef {
            #[inline]
            fn eq(&self, other: &ArbitraryContractExecutableRef) -> bool {
                self.owner == other.owner && self.tag == other.tag
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryContractExecutableRef {
            #[inline]
            fn cmp(&self, other: &ArbitraryContractExecutableRef) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.owner, &other.owner) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.tag, &other.tag),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryContractExecutableRef {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryContractExecutableRef,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.owner, &other.owner) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.tag, &other.tag)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryContractExecutableRef: ::std::thread::LocalKey<
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
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryContractExecutableRef {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryContractExecutableRef {
                            owner: arbitrary::Arbitrary::arbitrary(u)?,
                            tag: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
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
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryContractExecutableRef {
                            owner: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            tag: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryContractExecutableRef.with(|count| {
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
                                <<soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for ContractExecutableRef {
            type Prototype = ArbitraryContractExecutableRef;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryContractExecutableRef>
            for ContractExecutableRef
        {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryContractExecutableRef,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(ContractExecutableRef {
                    owner: soroban_sdk::IntoVal::into_val(&v.owner, env),
                    tag: soroban_sdk::IntoVal::into_val(&v.tag, env),
                })
            }
        }
    };
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
                    match ::core::cmp::PartialOrd::partial_cmp(&self.contract, &other.contract) {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; 96usize] = ContractContext::spec_xdr();
    impl ContractContext {
        pub const fn spec_xdr() -> [u8; 96usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fContractContext\0\0\0\0\x03\0\0\0\0\0\0\0\x04args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\x08contract\0\0\0\x13\0\0\0\0\0\0\0\x07fn_name\0\0\0\0\x11"
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Vec<
                soroban_sdk::Val,
            > as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Symbol as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_SUBCONTRACTINVOCATION: [u8; 144usize] =
        SubContractInvocation::spec_xdr();
    impl SubContractInvocation {
        pub const fn spec_xdr() -> [u8; 144usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x07context\0\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\0\0\0\0\x0fsub_invocations\0\0\0\x03\xea\0\0\x07\xd0\0\0\0\x18InvokerContractAuthEntry"
        }
    }
    impl soroban_sdk::SpecShakingMarker for SubContractInvocation {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Vec<
                InvokerContractAuthEntry,
            > as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, SubContractInvocation>>::try_from_val(
                env, *val,
            )
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8; 116usize] =
        CreateContractHostFnContext::spec_xdr();
    impl CreateContractHostFnContext {
        pub const fn spec_xdr() -> [u8; 116usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x02\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
        }
    }
    impl soroban_sdk::SpecShakingMarker for CreateContractHostFnContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractExecutable as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
        fn try_from(val: &CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
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
        fn try_from(val: &CreateContractHostFnContext) -> Result<Self, soroban_sdk::xdr::Error> {
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
            fn cmp(&self, other: &ArbitraryCreateContractHostFnContext) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.executable, &other.executable) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.salt, &other.salt),
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
                match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.salt, &other.salt)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryCreateContractHostFnContext: ::std::thread::LocalKey<
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
        fn cmp(&self, other: &CreateContractWithConstructorHostFnContext) -> ::core::cmp::Ordering {
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
                    match ::core::cmp::PartialOrd::partial_cmp(&self.executable, &other.executable)
                    {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8; 164usize] =
        CreateContractWithConstructorHostFnContext::spec_xdr();
    impl CreateContractWithConstructorHostFnContext {
        pub const fn spec_xdr() -> [u8; 164usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0*CreateContractWithConstructorHostFnContext\0\0\0\0\0\x03\0\0\0\0\0\0\0\x10constructor_args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\nexecutable\0\0\0\0\x07\xd0\0\0\0\x12ContractExecutable\0\0\0\0\0\0\0\0\0\x04salt\0\0\x03\xee\0\0\0 "
        }
    }
    impl soroban_sdk::SpecShakingMarker for CreateContractWithConstructorHostFnContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Vec<
                soroban_sdk::Val,
            > as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <ContractExecutable as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals)
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
            if map
                .iter()
                .any(|entry| !#[allow(non_exhaustive_omitted_patterns)]
                match entry.key {
                    soroban_sdk::xdr::ScVal::Symbol(_) => true,
                    _ => false,
                })
            {
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => soroban_sdk::xdr::ScVal::Void,
                    };
                    let rv: soroban_sdk::Val = (&val)
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
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            },
                        )?;
                    }
                    let result = (|| {
                        Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                            constructor_args: arbitrary::Arbitrary::arbitrary(u)?,
                            executable: arbitrary::Arbitrary::arbitrary(u)?,
                            salt: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                count.set(count.get() - 1);
                            },
                        );
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                if count.get() > 0 {
                                    return Err(arbitrary::Error::NotEnoughData);
                                }
                                count.set(count.get() + 1);
                                Ok(())
                            },
                        )?;
                    }
                    let result = (|| {
                        Ok(ArbitraryCreateContractWithConstructorHostFnContext {
                            constructor_args: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            executable: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            salt: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryCreateContractWithConstructorHostFnContext.with(
                            |count| {
                                count.set(count.get() - 1);
                            },
                        );
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
    pub enum ContractExecutable {
        Wasm(soroban_sdk::BytesN<32>),
        ExternalRef(ContractExecutableRef),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ContractExecutable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ContractExecutable::Wasm(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wasm", &__self_0)
                }
                ContractExecutable::ExternalRef(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ExternalRef", &__self_0)
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
                ContractExecutable::ExternalRef(__self_0) => {
                    ContractExecutable::ExternalRef(::core::clone::Clone::clone(__self_0))
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
            let _: ::core::cmp::AssertParamIsEq<ContractExecutableRef>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ContractExecutable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ContractExecutable {
        #[inline]
        fn eq(&self, other: &ContractExecutable) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (
                        ContractExecutable::ExternalRef(__self_0),
                        ContractExecutable::ExternalRef(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ContractExecutable {
        #[inline]
        fn cmp(&self, other: &ContractExecutable) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (
                        ContractExecutable::ExternalRef(__self_0),
                        ContractExecutable::ExternalRef(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                },
                cmp => cmp,
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
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (ContractExecutable::Wasm(__self_0), ContractExecutable::Wasm(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (
                    ContractExecutable::ExternalRef(__self_0),
                    ContractExecutable::ExternalRef(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; 128usize] = ContractExecutable::spec_xdr();
    impl ContractExecutable {
        pub const fn spec_xdr() -> [u8; 128usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x12ContractExecutable\0\0\0\0\0\x02\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\x01\0\0\0\0\0\0\0\x0bExternalRef\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x15ContractExecutableRef\0\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <ContractExecutableRef as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ContractExecutable {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["Wasm", "ExternalRef"];
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
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::ExternalRef(
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
                ContractExecutable::ExternalRef(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"ExternalRef")?.to_val(),
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
                "ExternalRef" => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    let rv0: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    Self::ExternalRef(
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
                ContractExecutable::ExternalRef(value0) => (
                    soroban_sdk::xdr::ScSymbol(
                        "ExternalRef"
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
            ExternalRef(
                <ContractExecutableRef as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
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
                    ArbitraryContractExecutable::ExternalRef(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ExternalRef",
                            &__self_0,
                        )
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
                    ArbitraryContractExecutable::ExternalRef(__self_0) => {
                        ArbitraryContractExecutable::ExternalRef(::core::clone::Clone::clone(
                            __self_0,
                        ))
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
                let _: ::core::cmp::AssertParamIsEq<
                    <ContractExecutableRef as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryContractExecutable {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryContractExecutable {
            #[inline]
            fn eq(&self, other: &ArbitraryContractExecutable) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            ArbitraryContractExecutable::ExternalRef(__self_0),
                            ArbitraryContractExecutable::ExternalRef(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryContractExecutable {
            #[inline]
            fn cmp(&self, other: &ArbitraryContractExecutable) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (
                            ArbitraryContractExecutable::Wasm(__self_0),
                            ArbitraryContractExecutable::Wasm(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        (
                            ArbitraryContractExecutable::ExternalRef(__self_0),
                            ArbitraryContractExecutable::ExternalRef(__arg1_0),
                        ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
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
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (
                        ArbitraryContractExecutable::Wasm(__self_0),
                        ArbitraryContractExecutable::Wasm(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    (
                        ArbitraryContractExecutable::ExternalRef(__self_0),
                        ArbitraryContractExecutable::ExternalRef(__arg1_0),
                    ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 2u64)
                                >> 32
                            {
                                0u64 => ArbitraryContractExecutable::Wasm(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                1u64 => ArbitraryContractExecutable::ExternalRef(
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
                                * 2u64)
                                >> 32
                            {
                                0u64 => ArbitraryContractExecutable::Wasm(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                1u64 => ArbitraryContractExecutable::ExternalRef(
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
                                        arbitrary::size_hint::and_all(
                                            &[
                                                <<ContractExecutableRef as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
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
                    ArbitraryContractExecutable::ExternalRef(field_0) => {
                        ContractExecutable::ExternalRef(soroban_sdk::IntoVal::into_val(
                            field_0, env,
                        ))
                    }
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
        fn partial_cmp(&self, other: &Context) -> ::core::option::Option<::core::cmp::Ordering> {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_CONTEXT: [u8; 244usize] = Context::spec_xdr();
    impl Context {
        pub const fn spec_xdr() -> [u8; 244usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07Context\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for Context {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                Context::CreateContractWithCtorHostFn(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractWithCtorHostFn")?
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
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
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
                        ArbitraryContext::CreateContractWithCtorHostFn(::core::clone::Clone::clone(
                            __self_0,
                        ))
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
            const RECURSIVE_COUNT_ArbitraryContext: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => {
                                    ArbitraryContext::Contract(arbitrary::Arbitrary::arbitrary(u)?)
                                }
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
                        Context::CreateContractHostFn(soroban_sdk::IntoVal::into_val(field_0, env))
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; 268usize] =
        InvokerContractAuthEntry::spec_xdr();
    impl InvokerContractAuthEntry {
        pub const fn spec_xdr() -> [u8; 268usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x18InvokerContractAuthEntry\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x15SubContractInvocation\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for InvokerContractAuthEntry {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <SubContractInvocation as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractHostFn")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                InvokerContractAuthEntry::CreateContractWithCtorHostFn(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"CreateContractWithCtorHostFn")?
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, InvokerContractAuthEntry>>::try_from_val(
                env, *val,
            )
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
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Contract", &__self_0)
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CreateContractHostFn",
                            &__self_0,
                        )
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
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
        impl ::core::clone::Clone for ArbitraryInvokerContractAuthEntry {
            #[inline]
            fn clone(&self) -> ArbitraryInvokerContractAuthEntry {
                match self {
                    ArbitraryInvokerContractAuthEntry::Contract(__self_0) => {
                        ArbitraryInvokerContractAuthEntry::Contract(::core::clone::Clone::clone(
                            __self_0,
                        ))
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractHostFn(__self_0) => {
                        ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0) => {
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
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
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__self_0),
                        ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(__arg1_0),
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryInvokerContractAuthEntry::Contract(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                1u64 => ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                2u64 => {
                                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary(u)?,
                                    )
                                }
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryInvokerContractAuthEntry::Contract(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                1u64 => ArbitraryInvokerContractAuthEntry::CreateContractHostFn(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                2u64 => {
                                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(
                                        arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                    )
                                }
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
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
                    ArbitraryInvokerContractAuthEntry::CreateContractWithCtorHostFn(field_0) => {
                        InvokerContractAuthEntry::CreateContractWithCtorHostFn(
                            soroban_sdk::IntoVal::into_val(field_0, env),
                        )
                    }
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
                Executable::StellarAsset => ::core::fmt::Formatter::write_str(f, "StellarAsset"),
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
        fn partial_cmp(&self, other: &Executable) -> ::core::option::Option<::core::cmp::Ordering> {
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
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_TYPE_EXECUTABLE: [u8; 104usize] = Executable::spec_xdr();
    impl Executable {
        pub const fn spec_xdr() -> [u8; 104usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\nExecutable\0\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\0\0\0\0\0\0\0\0\x0cStellarAsset\0\0\0\0\0\0\0\0\0\0\0\x07Account\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for Executable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Executable>>::try_from_val(env, *val)
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
                    ArbitraryExecutable::Account => ::core::fmt::Formatter::write_str(f, "Account"),
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
                    (ArbitraryExecutable::Wasm(__self_0), ArbitraryExecutable::Wasm(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
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
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => {
                                    ArbitraryExecutable::Wasm(arbitrary::Arbitrary::arbitrary(u)?)
                                }
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
    pub struct Transfer {
        pub from: soroban_sdk::Address,
        pub to: soroban_sdk::Address,
        pub amount: i128,
        pub to_muxed_id: Option<u64>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Transfer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Transfer",
                "from",
                &self.from,
                "to",
                &self.to,
                "amount",
                &self.amount,
                "to_muxed_id",
                &&self.to_muxed_id,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Transfer {
        #[inline]
        fn clone(&self) -> Transfer {
            Transfer {
                from: ::core::clone::Clone::clone(&self.from),
                to: ::core::clone::Clone::clone(&self.to),
                amount: ::core::clone::Clone::clone(&self.amount),
                to_muxed_id: ::core::clone::Clone::clone(&self.to_muxed_id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Transfer {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<i128>;
            let _: ::core::cmp::AssertParamIsEq<Option<u64>>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Transfer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Transfer {
        #[inline]
        fn eq(&self, other: &Transfer) -> bool {
            self.amount == other.amount
                && self.from == other.from
                && self.to == other.to
                && self.to_muxed_id == other.to_muxed_id
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Transfer {
        #[inline]
        fn cmp(&self, other: &Transfer) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.from, &other.from) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.to, &other.to) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.amount, &other.amount) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.to_muxed_id, &other.to_muxed_id)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Transfer {
        #[inline]
        fn partial_cmp(&self, other: &Transfer) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.from, &other.from) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.to, &other.to) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.amount, &other.amount)
                            {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &self.to_muxed_id,
                                        &other.to_muxed_id,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_EVENT_TRANSFER: [u8; 144usize] = Transfer::spec_xdr();
    impl Transfer {
        pub const fn spec_xdr() -> [u8; 144usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x08Transfer\0\0\0\x01\0\0\0\x08transfer\0\0\0\x04\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02to\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\0\0\0\0\x0bto_muxed_id\0\0\0\x03\xe8\0\0\0\x06\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::SpecShakingMarker for Transfer {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <Option<u64> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::Event for Transfer {
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
            env.sparse_map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl Transfer {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct MapValues {
        pub from: soroban_sdk::Address,
        pub a: u32,
        pub b: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MapValues {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "MapValues",
                "from",
                &self.from,
                "a",
                &self.a,
                "b",
                &&self.b,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MapValues {
        #[inline]
        fn clone(&self) -> MapValues {
            MapValues {
                from: ::core::clone::Clone::clone(&self.from),
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MapValues {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MapValues {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MapValues {
        #[inline]
        fn eq(&self, other: &MapValues) -> bool {
            self.a == other.a && self.b == other.b && self.from == other.from
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for MapValues {
        #[inline]
        fn cmp(&self, other: &MapValues) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.from, &other.from) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for MapValues {
        #[inline]
        fn partial_cmp(&self, other: &MapValues) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.from, &other.from) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_EVENT_MAPVALUES: [u8; 116usize] = MapValues::spec_xdr();
    impl MapValues {
        pub const fn spec_xdr() -> [u8; 116usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\tMapValues\0\0\0\0\0\0\x01\0\0\0\nmap_values\0\0\0\0\0\x03\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::SpecShakingMarker for MapValues {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::Event for MapValues {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (&{ soroban_sdk::Symbol::new(env, "map_values") }, {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            })
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 2usize] = ["a", "b"];
            let vals: [soroban_sdk::Val; 2usize] = [self.a.into_val(env), self.b.into_val(env)];
            env.sparse_map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl MapValues {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct VecValues {
        pub from: soroban_sdk::Address,
        pub a: u32,
        pub b: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VecValues {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "VecValues",
                "from",
                &self.from,
                "a",
                &self.a,
                "b",
                &&self.b,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VecValues {
        #[inline]
        fn clone(&self) -> VecValues {
            VecValues {
                from: ::core::clone::Clone::clone(&self.from),
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for VecValues {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for VecValues {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for VecValues {
        #[inline]
        fn eq(&self, other: &VecValues) -> bool {
            self.a == other.a && self.b == other.b && self.from == other.from
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for VecValues {
        #[inline]
        fn cmp(&self, other: &VecValues) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.from, &other.from) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.a, &other.a) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.b, &other.b),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for VecValues {
        #[inline]
        fn partial_cmp(&self, other: &VecValues) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.from, &other.from) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.b, &other.b)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_EVENT_VECVALUES: [u8; 116usize] = VecValues::spec_xdr();
    impl VecValues {
        pub const fn spec_xdr() -> [u8; 116usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\tVecValues\0\0\0\0\0\0\x01\0\0\0\nvec_values\0\0\0\0\0\x03\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x01a\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x01b\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\x01"
        }
    }
    impl soroban_sdk::SpecShakingMarker for VecValues {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::Event for VecValues {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (&{ soroban_sdk::Symbol::new(env, "vec_values") }, {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            })
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::IntoVal;
            (
                {
                    let v: soroban_sdk::Val = self.a.into_val(env);
                    v
                },
                {
                    let v: soroban_sdk::Val = self.b.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
    }
    impl VecValues {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct SingleValue {
        pub from: soroban_sdk::Address,
        pub amount: i128,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SingleValue {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SingleValue",
                "from",
                &self.from,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SingleValue {
        #[inline]
        fn clone(&self) -> SingleValue {
            SingleValue {
                from: ::core::clone::Clone::clone(&self.from),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SingleValue {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<i128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SingleValue {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SingleValue {
        #[inline]
        fn eq(&self, other: &SingleValue) -> bool {
            self.amount == other.amount && self.from == other.from
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for SingleValue {
        #[inline]
        fn cmp(&self, other: &SingleValue) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.from, &other.from) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.amount, &other.amount),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for SingleValue {
        #[inline]
        fn partial_cmp(
            &self,
            other: &SingleValue,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.from, &other.from) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.amount, &other.amount)
                }
                cmp => cmp,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_EVENT_SINGLEVALUE: [u8; 100usize] = SingleValue::spec_xdr();
    impl SingleValue {
        pub const fn spec_xdr() -> [u8; 100usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x0bSingleValue\0\0\0\0\x01\0\0\0\x0csingle_value\0\0\0\x02\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x06amount\0\0\0\0\0\x0b\0\0\0\0\0\0\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for SingleValue {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::Event for SingleValue {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (&{ soroban_sdk::Symbol::new(env, "single_value") }, {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            })
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::IntoVal;
            self.amount.into_val(env)
        }
    }
    impl SingleValue {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct SingleValueVoid {
        pub from: soroban_sdk::Address,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SingleValueVoid {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "SingleValueVoid",
                "from",
                &&self.from,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SingleValueVoid {
        #[inline]
        fn clone(&self) -> SingleValueVoid {
            SingleValueVoid {
                from: ::core::clone::Clone::clone(&self.from),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SingleValueVoid {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SingleValueVoid {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SingleValueVoid {
        #[inline]
        fn eq(&self, other: &SingleValueVoid) -> bool {
            self.from == other.from
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for SingleValueVoid {
        #[inline]
        fn cmp(&self, other: &SingleValueVoid) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.from, &other.from)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for SingleValueVoid {
        #[inline]
        fn partial_cmp(
            &self,
            other: &SingleValueVoid,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.from, &other.from)
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    static __SPEC_XDR_EVENT_SINGLEVALUEVOID: [u8; 88usize] = SingleValueVoid::spec_xdr();
    impl SingleValueVoid {
        pub const fn spec_xdr() -> [u8; 88usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x0fSingleValueVoid\0\0\0\0\x01\0\0\0\x11single_value_void\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x04from\0\0\0\x13\0\0\0\x01\0\0\0\0"
        }
    }
    impl soroban_sdk::SpecShakingMarker for SingleValueVoid {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
        }
    }
    impl soroban_sdk::Event for SingleValueVoid {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (&{ soroban_sdk::Symbol::new(env, "single_value_void") }, {
                let v: soroban_sdk::Val = self.from.into_val(env);
                v
            })
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            soroban_sdk::Val::VOID.to_val()
        }
    }
    impl SingleValueVoid {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
}
pub enum Error {
    Abort = 0,
    Overflow = 1,
}
#[automatically_derived]
impl ::core::fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Error::Abort => "Abort",
                Error::Overflow => "Overflow",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Error {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[doc(hidden)]
#[allow(dead_code)]
static __SPEC_XDR_TYPE_ERROR: [u8; 68usize] = Error::spec_xdr();
impl Error {
    pub const fn spec_xdr() -> [u8; 68usize] {
        *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x05Error\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x05Abort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x08Overflow\0\0\0\x01"
    }
}
impl soroban_sdk::SpecShakingMarker for Error {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {}
}
impl TryFrom<soroban_sdk::Error> for Error {
    type Error = soroban_sdk::Error;
    #[inline(always)]
    fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
        if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
            let discriminant = error.get_code();
            Ok(match discriminant {
                0u32 => Self::Abort,
                1u32 => Self::Overflow,
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
            Error::Abort => soroban_sdk::Error::from_contract_error(0u32),
            Error::Overflow => soroban_sdk::Error::from_contract_error(1u32),
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
                0u32 => Self::Abort,
                1u32 => Self::Overflow,
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
            Error::Abort => soroban_sdk::InvokeError::Contract(0u32),
            Error::Overflow => soroban_sdk::InvokeError::Contract(1u32),
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
impl Contract {
    pub fn add_with(env: Env, contract_id: Address, x: u64, y: u64) -> u64 {
        addcontract::Client::new(&env, &contract_id).add(&x, &y)
    }
    pub fn safe_add_with(env: Env, contract_id: Address, x: u64, y: u64) -> Result<u64, Error> {
        match addcontract::Client::new(&env, &contract_id).try_safe_add(&x, &y) {
            Ok(Ok(i)) => Ok(i),
            Err(Ok(addcontract::Error::Overflow)) => Err(Error::Overflow),
            _ => Err(Error::Abort),
        }
    }
    pub fn safe_add_with_two(env: Env, contract_id: Address, x: u64, y: u64) -> Result<u64, Error> {
        match addcontract::Client::new(&env, &contract_id).try_safe_add_two(&x, &y) {
            Ok(Ok(i)) => Ok(i),
            Err(Ok(addcontract::MyError::Overflow)) => Err(Error::Overflow),
            _ => Err(Error::Abort),
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__add_with__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_ADD_WITH: [u8; 88usize] = super::Contract::spec_xdr_add_with();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add_with() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08add_with\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__safe_add_with__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_SAFE_ADD_WITH: [u8; 104usize] = super::Contract::spec_xdr_safe_add_with();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_safe_add_with() -> [u8; 104usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rsafe_add_with\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x06\0\0\0\x01\0\0\x03\xe9\0\0\0\x06\0\0\0\x03"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod __Contract__safe_add_with_two__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    static __SPEC_XDR_FN_SAFE_ADD_WITH_TWO: [u8; 108usize] =
        super::Contract::spec_xdr_safe_add_with_two();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_safe_add_with_two() -> [u8; 108usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11safe_add_with_two\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x06\0\0\0\x01\0\0\x03\xe9\0\0\0\x06\0\0\0\x03"
    }
}
impl<'a> ContractClient<'a> {
    pub fn add_with(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add_with");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_add_with(
        &self,
        contract_id: &Address,
        x: &u64,
        y: &u64,
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
                const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("add_with");
                SYMBOL
            },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn safe_add_with(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "safe_add_with") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_safe_add_with(
        &self,
        contract_id: &Address,
        x: &u64,
        y: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<Error, soroban_sdk::InvokeError>,
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
            &{ soroban_sdk::Symbol::new(&self.env, "safe_add_with") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn safe_add_with_two(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
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
            &{ soroban_sdk::Symbol::new(&self.env, "safe_add_with_two") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_safe_add_with_two(
        &self,
        contract_id: &Address,
        x: &u64,
        y: &u64,
    ) -> Result<
        Result<u64, <u64 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<Error, soroban_sdk::InvokeError>,
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
            &{ soroban_sdk::Symbol::new(&self.env, "safe_add_with_two") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    x.into_val(&self.env),
                    y.into_val(&self.env),
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
    pub fn add_with<'i>(
        contract_id: &'i Address,
        x: &'i u64,
        y: &'i u64,
    ) -> (&'i Address, &'i u64, &'i u64) {
        (contract_id, x, y)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn safe_add_with<'i>(
        contract_id: &'i Address,
        x: &'i u64,
        y: &'i u64,
    ) -> (&'i Address, &'i u64, &'i u64) {
        (contract_id, x, y)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn safe_add_with_two<'i>(
        contract_id: &'i Address,
        x: &'i u64,
        y: &'i u64,
    ) -> (&'i Address, &'i u64, &'i u64) {
        (contract_id, x, y)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
#[allow(deprecated)]
pub fn __Contract__add_with__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::add_with(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
pub fn __Contract__add_with__invoke_raw_slice(
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
    __Contract__add_with__invoke_raw(env, args[0usize], args[1usize], args[2usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).add_with` instead")]
pub extern "C" fn __Contract__add_with__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__add_with__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).safe_add_with` instead")]
#[allow(deprecated)]
pub fn __Contract__safe_add_with__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::safe_add_with(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).safe_add_with` instead")]
pub fn __Contract__safe_add_with__invoke_raw_slice(
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
    __Contract__safe_add_with__invoke_raw(env, args[0usize], args[1usize], args[2usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).safe_add_with` instead")]
pub extern "C" fn __Contract__safe_add_with__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__safe_add_with__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).safe_add_with_two` instead")]
#[allow(deprecated)]
pub fn __Contract__safe_add_with_two__invoke_raw(
    env: soroban_sdk::Env,
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
        <Contract>::safe_add_with_two(
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
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).safe_add_with_two` instead")]
pub fn __Contract__safe_add_with_two__invoke_raw_slice(
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
    __Contract__safe_add_with_two__invoke_raw(env, args[0usize], args[1usize], args[2usize])
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use `ContractClient::new(&env, &contract_id).safe_add_with_two` instead")]
pub extern "C" fn __Contract__safe_add_with_two__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__safe_add_with_two__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract____7f4ee4d361d09b9f0b349441853c9c2507e71fb15e4ddfe088d43fd41f0e1d27_ctor() {
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
                    __Contract____7f4ee4d361d09b9f0b349441853c9c2507e71fb15e4ddfe088d43fd41f0e1d27_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "add_with",
            #[allow(deprecated)]
            &__Contract__add_with__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "safe_add_with",
            #[allow(deprecated)]
            &__Contract__safe_add_with__invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "safe_add_with_two",
            #[allow(deprecated)]
            &__Contract__safe_add_with_two__invoke_raw_slice,
        );
    }
}
mod test {
    use crate::{addcontract, eventscontract, Contract, ContractClient, Error};
    use soroban_sdk::{
        testutils::{Address as _, Events},
        Address, Env, Event,
    };
    extern crate test;
    #[rustc_test_marker = "test::test_add"]
    #[doc(hidden)]
    pub const test_add: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_add"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/import_contract/src/lib.rs",
            start_line: 53usize,
            start_col: 8usize,
            end_line: 53usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_add()),
        ),
    };
    fn test_add() {
        let e = Env::default();
        let add_contract_id = e.register(addcontract::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let x = 10u64;
        let y = 12u64;
        let z = client.add_with(&add_contract_id, &x, &y);
        if !(z == 22) {
            ::core::panicking::panic("assertion failed: z == 22")
        }
    }
    extern crate test;
    #[rustc_test_marker = "test::test_safe_add"]
    #[doc(hidden)]
    pub const test_safe_add: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_safe_add"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/import_contract/src/lib.rs",
            start_line: 67usize,
            start_col: 8usize,
            end_line: 67usize,
            end_col: 21usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_safe_add()),
        ),
    };
    fn test_safe_add() {
        let e = Env::default();
        let add_contract_id = e.register(addcontract::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let x = u64::MAX;
        let y = 1;
        let z = client.try_safe_add_with(&add_contract_id, &x, &y);
        match (&z, &Err(Ok(Error::Overflow))) {
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
        let z = client.try_safe_add_with_two(&add_contract_id, &x, &y);
        match (&z, &Err(Ok(Error::Overflow))) {
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
    #[rustc_test_marker = "test::test_imported_event_data_format_single_value"]
    #[doc(hidden)]
    pub const test_imported_event_data_format_single_value: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("test::test_imported_event_data_format_single_value"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/import_contract/src/lib.rs",
                start_line: 87usize,
                start_col: 8usize,
                end_line: 87usize,
                end_col: 52usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test_imported_event_data_format_single_value()),
            ),
        };
    fn test_imported_event_data_format_single_value() {
        let e = Env::default();
        let contract_id = e.register(eventscontract::WASM, ());
        let client = eventscontract::Client::new(&e, &contract_id);
        let from = Address::generate(&e);
        client.single_value(&from, &7i128);
        match (
            &e.events().all(),
            &[eventscontract::SingleValue { from, amount: 7 }.to_xdr(&e, &contract_id)],
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
    #[rustc_test_marker = "test::test_imported_event_data_format_single_value_void"]
    #[doc(hidden)]
    pub const test_imported_event_data_format_single_value_void: test::TestDescAndFn =
        test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "test::test_imported_event_data_format_single_value_void",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/import_contract/src/lib.rs",
                start_line: 102usize,
                start_col: 8usize,
                end_line: 102usize,
                end_col: 57usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test_imported_event_data_format_single_value_void()),
            ),
        };
    fn test_imported_event_data_format_single_value_void() {
        let e = Env::default();
        let contract_id = e.register(eventscontract::WASM, ());
        let client = eventscontract::Client::new(&e, &contract_id);
        let from = Address::generate(&e);
        client.single_value_void(&from);
        match (
            &e.events().all(),
            &[eventscontract::SingleValueVoid { from }.to_xdr(&e, &contract_id)],
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
    #[rustc_test_marker = "test::test_imported_event_data_format_vec"]
    #[doc(hidden)]
    pub const test_imported_event_data_format_vec: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_imported_event_data_format_vec"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/import_contract/src/lib.rs",
            start_line: 117usize,
            start_col: 8usize,
            end_line: 117usize,
            end_col: 43usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_imported_event_data_format_vec()),
        ),
    };
    fn test_imported_event_data_format_vec() {
        let e = Env::default();
        let contract_id = e.register(eventscontract::WASM, ());
        let client = eventscontract::Client::new(&e, &contract_id);
        let from = Address::generate(&e);
        client.vec_values(&from, &1, &2);
        match (
            &e.events().all(),
            &[eventscontract::VecValues { from, a: 1, b: 2 }.to_xdr(&e, &contract_id)],
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
    #[rustc_test_marker = "test::test_imported_event_data_format_map"]
    #[doc(hidden)]
    pub const test_imported_event_data_format_map: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_imported_event_data_format_map"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/import_contract/src/lib.rs",
            start_line: 132usize,
            start_col: 8usize,
            end_line: 132usize,
            end_col: 43usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_imported_event_data_format_map()),
        ),
    };
    fn test_imported_event_data_format_map() {
        let e = Env::default();
        let contract_id = e.register(eventscontract::WASM, ());
        let client = eventscontract::Client::new(&e, &contract_id);
        let from = Address::generate(&e);
        client.map_values(&from, &1, &2);
        match (
            &e.events().all(),
            &[eventscontract::MapValues { from, a: 1, b: 2 }.to_xdr(&e, &contract_id)],
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_add,
        &test_imported_event_data_format_map,
        &test_imported_event_data_format_single_value,
        &test_imported_event_data_format_single_value_void,
        &test_imported_event_data_format_vec,
        &test_safe_add,
    ])
}
