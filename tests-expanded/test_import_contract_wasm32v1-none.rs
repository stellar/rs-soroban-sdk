#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};
mod addcontract {
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01\x14\x04`\x01~\x01~`\x02\x7f~\x00`\x02~~\x01~`\x00\x00\x02\r\x02\x01i\x010\x00\x00\x01i\x01_\x00\x00\x03\x08\x07\x01\x01\x02\x03\x02\x02\x03\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x9c\x80\xc0\x00\x0b\x7f\x00A\x9c\x80\xc0\x00\x0b\x7f\x00A\xa0\x80\xc0\x00\x0b\x07I\x07\x06memory\x02\x00\x03add\x00\x04\x08safe_add\x00\x06\x0csafe_add_two\x00\x07\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xf3\x04\x07]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc0\x00F\r\x00\x02@ \x02A\x06F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x88!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x80\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0b;\x00\x02@\x02@ \x01B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x01B\x08\x86B\x06\x84!\x01\x0c\x01\x0b \x01\x10\x81\x80\x80\x80\x00!\x01\x0b \x00B\x007\x03\x00 \x00 \x017\x03\x08\x0b\x8e\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@\x02@\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08\"\x01 \x00|\"\x00 \x01T\r\x01 \x02 \x00\x10\x83\x80\x80\x80\x00 \x02(\x02\x00A\x01G\r\x02\x0b\x00\x0b\x10\x85\x80\x80\x80\x00\x00\x0b \x02)\x03\x08!\x00 \x02A\x10j$\x80\x80\x80\x80\x00 \x00\x0b\t\x00\x10\x88\x80\x80\x80\x00\x00\x0b\x9b\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x03 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00A\x00-\x00\x80\x80\xc0\x80\x00\x1aB\x83\x80\x80\x80\x10!\x01\x02@ \x00 \x03|\"\x03 \x00T\r\x00 \x02 \x03\x10\x83\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x01 \x02)\x03\x08!\x01\x0b \x02A\x10j$\x80\x80\x80\x80\x00 \x01\x0f\x0b\x00\x0b\x9b\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x03 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00A\x00-\x00\x8e\x80\xc0\x80\x00\x1aB\x83\x80\x80\x80\x10!\x01\x02@ \x00 \x03|\"\x03 \x00T\r\x00 \x02 \x03\x10\x83\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x01 \x02)\x03\x08!\x01\x0b \x02A\x10j$\x80\x80\x80\x80\x00 \x01\x0f\x0b\x00\x0b\x03\x00\x00\x0b\x0b%\x01\x00A\x80\x80\xc0\x00\x0b\x1cSpEcV1i\xf1\x99?P\x07u\xf4SpEcV1n\xe7\x85\xc3\x00\xabx\x12\x00\xb3\x1f\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03add\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x15::test_add_u64::Error\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x08Overflow\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x17::test_add_u64::MyError\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x08Overflow\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08safe_add\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x06\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0csafe_add_two\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x03\xe9\x00\x00\x00\x06\x00\x00\x07\xd0\x00\x00\x00\x17::test_add_u64::MyError\x00\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00xExecutable specified by the contract instance as a specific Wasm contract code entry identified by its Wasm sha256 hash.\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00_Executable reference via a persistent storage entry owned by this contract or another contract.\x00\x00\x00\x00\x0bExternalRef\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::ContractExecutableRef\x00\x00\x00\x01\x00\x00\x00\xc0Executable referenced via a persistent storage entry owned by a contract,\neither this contract or another contract.\n\nThe persistent storage entry owned by the `owner` has the `tag` as its key.\x00\x00\x00\x00\x00\x00\x00$::soroban_sdk::ContractExecutableRef\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05owner\x00\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x03tag\x00\x00\x00\x00\x10\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x1c::soroban_sdk::auth::Context\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00*::soroban_sdk::auth::SubContractInvocation\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00-::soroban_sdk::auth::InvokerContractAuthEntry\x00\x00\x00\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00-::soroban_sdk::auth::InvokerContractAuthEntry\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*::soroban_sdk::auth::SubContractInvocation\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\"::soroban_sdk::address::Executable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1d\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
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
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> Client<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
            }
        }
    }
    impl<'a> Client<'a> {
        pub fn add(&self, a: &u64, b: &u64) -> u64 {
            use core::ops::Not;
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
            res
        }
        pub fn safe_add(&self, a: &u64, b: &u64) -> u64 {
            use core::ops::Not;
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
            res
        }
        pub fn safe_add_two(&self, a: &u64, b: &u64) -> u64 {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "safe_add_two") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
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
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "safe_add_two") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [a.into_val(&self.env), b.into_val(&self.env)],
                ),
            );
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
    impl ContractExecutableRef {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::ContractExecutableRef"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLEREF: [u8; ContractExecutableRef::spec_xdr().len()] =
        ContractExecutableRef::spec_xdr();
    impl ContractExecutableRef {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        ContractExecutableRef::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"owner",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"tag",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::String,
                        },
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; ContractExecutableRef::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { ContractExecutableRef::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutableRef {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::String as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &ContractExecutableRef::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl ContractContext {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::ContractContext"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; ContractContext::spec_xdr().len()] =
        ContractContext::spec_xdr();
    impl ContractContext {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        ContractContext::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"args",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                                &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                                    element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Val,
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"contract",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"fn_name",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Symbol,
                        },
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; ContractContext::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { ContractContext::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &ContractContext::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl SubContractInvocation {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::SubContractInvocation"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_SUBCONTRACTINVOCATION: [u8; SubContractInvocation::spec_xdr().len()] =
        SubContractInvocation::spec_xdr();
    impl SubContractInvocation {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                SubContractInvocation::spec_name(),
            ),
            fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"context",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                <ContractContext>::spec_name(),
                            ),
                        }),
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"sub_invocations",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                            &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                                element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <InvokerContractAuthEntry>::spec_name(),
                                    ),
                                }),
                            },
                        ),
                    },
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; SubContractInvocation::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { SubContractInvocation::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &SubContractInvocation::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl CreateContractHostFnContext {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::CreateContractHostFnContext"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8;
        CreateContractHostFnContext::spec_xdr().len()] = CreateContractHostFnContext::spec_xdr();
    impl CreateContractHostFnContext {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        CreateContractHostFnContext::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"executable",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractExecutable>::spec_name(),
                                    ),
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"salt",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 },
                            ),
                        },
                    ]),
                },
            );
        pub const fn spec_xdr(
        ) -> [u8; CreateContractHostFnContext::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { CreateContractHostFnContext::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for CreateContractHostFnContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractExecutable as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &CreateContractHostFnContext::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl CreateContractWithConstructorHostFnContext {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::CreateContractWithConstructorHostFnContext"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8;
        CreateContractWithConstructorHostFnContext::spec_xdr().len()] =
        CreateContractWithConstructorHostFnContext::spec_xdr();
    impl CreateContractWithConstructorHostFnContext {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        CreateContractWithConstructorHostFnContext::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"constructor_args",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                                &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                                    element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Val,
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"executable",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractExecutable>::spec_name(),
                                    ),
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"salt",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 },
                            ),
                        },
                    ]),
                },
            );
        pub const fn spec_xdr(
        ) -> [u8; CreateContractWithConstructorHostFnContext::__SPEC_XDR_ENTRY.const_xdr_len()]
        {
            const { CreateContractWithConstructorHostFnContext::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &CreateContractWithConstructorHostFnContext::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl ContractExecutable {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::ContractExecutable"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; ContractExecutable::spec_xdr().len()] =
        ContractExecutable::spec_xdr();
    impl ContractExecutable {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                ContractExecutable::spec_name(),
            ),
            cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Wasm",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(soroban_sdk::xdr::r#const::ScSpecTypeBytesN {
                                    n: 32u32,
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"ExternalRef",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractExecutableRef>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; ContractExecutable::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { ContractExecutable::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <ContractExecutableRef as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &ContractExecutable::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl Context {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::Context"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTEXT: [u8; Context::spec_xdr().len()] = Context::spec_xdr();
    impl Context {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                Context::spec_name(),
            ),
            cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Contract",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractWithCtorHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractWithConstructorHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; Context::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Context::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for Context {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Context::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl InvokerContractAuthEntry {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::InvokerContractAuthEntry"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; InvokerContractAuthEntry::spec_xdr()
        .len()] = InvokerContractAuthEntry::spec_xdr();
    impl InvokerContractAuthEntry {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                InvokerContractAuthEntry::spec_name(),
            ),
            cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Contract",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <SubContractInvocation>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractWithCtorHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractWithConstructorHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; InvokerContractAuthEntry::__SPEC_XDR_ENTRY.const_xdr_len()]
        {
            const { InvokerContractAuthEntry::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for InvokerContractAuthEntry {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <SubContractInvocation as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &InvokerContractAuthEntry::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl Executable {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::Executable"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_EXECUTABLE: [u8; Executable::spec_xdr().len()] = Executable::spec_xdr();
    impl Executable {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(
                soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        Executable::spec_name(),
                    ),
                    cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                            soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"",
                                ),
                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"Wasm",
                                ),
                                type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                                    soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                        soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 },
                                    ),
                                ]),
                            },
                        ),
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::VoidV0(
                            soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseVoidV0 {
                                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"",
                                ),
                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"StellarAsset",
                                ),
                            },
                        ),
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::VoidV0(
                            soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseVoidV0 {
                                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"",
                                ),
                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"Account",
                                ),
                            },
                        ),
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; Executable::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Executable::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for Executable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Executable::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl Error {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::Error"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_ERROR: [u8; Error::spec_xdr().len()] = Error::spec_xdr();
    impl Error {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtErrorEnumV0(
                soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        Error::spec_name(),
                    ),
                    cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumCaseV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"Overflow",
                            ),
                            value: 1u32,
                        },
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; Error::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Error::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for Error {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Error::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
        }
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
    impl MyError {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::addcontract::MyError"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_MYERROR: [u8; MyError::spec_xdr().len()] = MyError::spec_xdr();
    impl MyError {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtErrorEnumV0(
                soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        MyError::spec_name(),
                    ),
                    cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumCaseV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"Overflow",
                            ),
                            value: 1u32,
                        },
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; MyError::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { MyError::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for MyError {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &MyError::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
        }
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
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x010\t`\x01~\x01~`\x02~~\x01~`\x03~~~\x01~`\x01\x7f\x00`\x01\x7f\x01~`\x02\x7f\x7f\x01~`\x02\x7f~\x01~`\x02\x7f~\x00`\x00\x00\x02C\x0b\x01i\x01_\x00\x00\x01x\x011\x00\x01\x01a\x014\x00\x00\x01a\x015\x00\x00\x01i\x010\x00\x00\x01v\x01g\x00\x01\x01i\x018\x00\x00\x01i\x017\x00\x00\x01b\x01j\x00\x01\x01i\x016\x00\x01\x01m\x01b\x00\x02\x03\x11\x10\x03\x04\x05\x01\x05\x06\x02\x07\x08\x02\x05\x01\x00\x02\x08\x02\x05\x03\x01\x00\x11\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\xb5\x81\xc0\x00\x0b\x7f\x00A\xb5\x81\xc0\x00\x0b\x7f\x00A\xc0\x81\xc0\x00\x0b\x07\x83\x01\n\x06memory\x02\x00\x0ffailed_transfer\x00\x11\nmap_values\x00\x14\x0csingle_value\x00\x16\x11single_value_void\x00\x17\x08transfer\x00\x18\nvec_values\x00\x1a\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xe2\r\x10\xb9\x02\x02\x02\x7f\x03~#\x80\x80\x80\x80\x00A0k\"\x01$\x80\x80\x80\x80\x00A\x00!\x02A\x00-\x00\x80\x80\xc0\x80\x00\x1a \x00)\x03 !\x03 \x00)\x03(!\x04A\xe8\x80\xc0\x80\x00\x10\x8c\x80\x80\x80\x00!\x05 \x01 \x047\x03\x10 \x01 \x037\x03\x08 \x01 \x057\x03\x00\x03@\x02@ \x02A\x18G\r\x00A\x00!\x02\x02@\x03@ \x02A\x18F\r\x01 \x01A\x18j \x02j \x01 \x02j)\x03\x007\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b \x01A\x18jA\x03\x10\x8d\x80\x80\x80\x00!\x04 \x00)\x03\x10 \x00)\x03\x18\x10\x8e\x80\x80\x80\x00!\x05B\x02!\x03\x02@ \x00(\x02\x00A\x01G\r\x00\x02@ \x00)\x03\x08\"\x03B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x03B\x08\x86B\x06\x84!\x03\x0c\x01\x0b \x03\x10\x80\x80\x80\x80\x00!\x03\x0b \x01 \x037\x03  \x01 \x057\x03\x18 \x04A\xd8\x80\xc0\x80\x00 \x01A\x18j\x10\x8f\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x01A0j$\x80\x80\x80\x80\x00\x0f\x0b \x01A\x18j \x02jB\x027\x03\x00 \x02A\x08j!\x02\x0c\x00\x0b\x0b\x07\x00 \x00)\x03\x00\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x85\x80\x80\x80\x00\x0bE\x00\x02@ \x00B\x80\x80\x80\x80\x80\x80\x80\xc0\x00|B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00 \x00\x85 \x01 \x00B?\x87\x85\x84B\x00R\r\x00 \x00B\x08\x86B\x0b\x84\x0f\x0b \x01 \x00\x10\x89\x80\x80\x80\x00\x0b \x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x8a\x80\x80\x80\x00\x0b\xa0\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00 \x00\x10\x8c\x80\x80\x80\x00!\x03 \x02 \x017\x03\x08 \x02 \x037\x03\x00A\x00!\x00\x03~\x02@ \x00A\x10G\r\x00A\x00!\x00\x02@\x03@ \x00A\x10F\r\x01 \x02A\x10j \x00j \x02 \x00j)\x03\x007\x03\x00 \x00A\x08j!\x00\x0c\x00\x0b\x0b \x02A\x10jA\x02\x10\x8d\x80\x80\x80\x00!\x01 \x02A j$\x80\x80\x80\x80\x00 \x01\x0f\x0b \x02A\x10j \x00jB\x027\x03\x00 \x00A\x08j!\x00\x0c\x00\x0b\x0b\x80\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A0k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\xcd\x00R\r\x00 \x03 \x02\x10\x92\x80\x80\x80\x00 \x03(\x02\x00A\x01F\r\x00 \x03)\x03\x10!\x02 \x03 \x03)\x03\x187\x03\x18 \x03 \x027\x03\x10 \x03 \x017\x03( \x03 \x007\x03  \x03B\x007\x03\x00 \x03\x10\x8b\x80\x80\x80\x00\x10\x93\x80\x80\x80\x00\x0b\x00\x0b}\x02\x01\x7f\x01~\x02@\x02@\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc5\x00F\r\x00 \x02A\x0bG\r\x02 \x00 \x01B?\x877\x03\x18 \x00 \x01B\x08\x877\x03\x10\x0c\x01\x0b \x01\x10\x86\x80\x80\x80\x00!\x03 \x01\x10\x87\x80\x80\x80\x00!\x01 \x00 \x037\x03\x18 \x00 \x017\x03\x10\x0bB\x00!\x01\x0c\x01\x0b \x00B\x83\x90\x80\x80\x80\x017\x03\x08B\x01!\x01\x0b \x00 \x017\x03\x00\x0b\x03\x00\x00\x0b\xa5\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\x04R\r\x00 \x02B\xff\x01\x83B\x04R\r\x00A\x00-\x00\x8e\x80\xc0\x80\x00\x1a \x03A\x84\x81\xc0\x80\x00A\n\x10\x95\x80\x80\x80\x007\x03\x00 \x03 \x00\x10\x90\x80\x80\x80\x00!\x00 \x03 \x02B\x84\x80\x80\x80p\x837\x03\x08 \x03 \x01B\x84\x80\x80\x80p\x837\x03\x00 \x00A\xf4\x80\xc0\x80\x00 \x03\x10\x8f\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x03A\x10j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0b\x1a\x00 \x00\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x88\x80\x80\x80\x00\x0b\x8f\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\x00A k\"\x02$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x02 \x01\x10\x92\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00A\x00-\x00\xaa\x80\xc0\x80\x00\x1a \x02)\x03\x18!\x01 \x02)\x03\x10!\x03 \x02A\x98\x81\xc0\x80\x00A\x0c\x10\x95\x80\x80\x80\x007\x03\x00 \x02 \x00\x10\x90\x80\x80\x80\x00 \x03 \x01\x10\x8e\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x02A j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0be\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x01$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00Q\r\x00\x00\x0bA\x00-\x00\xb8\x80\xc0\x80\x00\x1a \x01A\xa4\x81\xc0\x80\x00A\x11\x10\x95\x80\x80\x80\x007\x03\x08 \x01A\x08j \x00\x10\x90\x80\x80\x80\x00B\x02\x10\x81\x80\x80\x80\x00\x1a \x01A\x10j$\x80\x80\x80\x80\x00B\x02\x0b\x8e\x02\x02\x02\x7f\x04~#\x80\x80\x80\x80\x00A0k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00A\x01!\x04\x02@\x02@ \x01\xa7A\xff\x01qA\xb3\x7fj\x0e\x02\x00\x01\x02\x0bA\x00!\x04\x0b \x03 \x02\x10\x92\x80\x80\x80\x00 \x03(\x02\x00A\x01F\r\x00 \x03)\x03\x18!\x02 \x03)\x03\x10!\x05\x02@\x02@ \x04\r\x00B\x00!\x06\x0c\x01\x0b \x01\x10\x82\x80\x80\x80\x00!\x07\x02@\x02@ \x01\x10\x83\x80\x80\x80\x00\"\x01\xa7A\xff\x01q\"\x04A\x06F\r\x00\x02@ \x04A\xc0\x00G\r\x00B\x01!\x06 \x01\x10\x84\x80\x80\x80\x00!\x08\x0c\x02\x0b\x10\x99\x80\x80\x80\x00\x00\x0b \x01B\x08\x88!\x08B\x01!\x06\x0b \x07!\x01\x0b \x03 \x057\x03\x10 \x03 \x017\x03( \x03 \x007\x03  \x03 \x087\x03\x08 \x03 \x067\x03\x00 \x03 \x027\x03\x18 \x03\x10\x8b\x80\x80\x80\x00 \x03A0j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0b\t\x00\x10\x93\x80\x80\x80\x00\x00\x0b\xa1\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x03$\x80\x80\x80\x80\x00\x02@ \x00B\xff\x01\x83B\xcd\x00R\r\x00 \x01B\xff\x01\x83B\x04R\r\x00 \x02B\xff\x01\x83B\x04R\r\x00A\x00-\x00\x9c\x80\xc0\x80\x00\x1a \x03A\x8e\x81\xc0\x80\x00A\n\x10\x95\x80\x80\x80\x007\x03\x00 \x03 \x00\x10\x90\x80\x80\x80\x00!\x00 \x03 \x02B\x84\x80\x80\x80p\x837\x03\x08 \x03 \x01B\x84\x80\x80\x80p\x837\x03\x00 \x00 \x03A\x02\x10\x8d\x80\x80\x80\x00\x10\x81\x80\x80\x80\x00\x1a \x03A\x10j$\x80\x80\x80\x80\x00B\x02\x0f\x0b\x00\x0b\x0b\xbf\x01\x01\x00A\x80\x80\xc0\x00\x0b\xb5\x01SpEcV1\xdbTN\xd8\x83:a\xb6SpEcV1@\x04\xf7\xe9\xd8\xae\xab\xfdSpEcV1?\x81\x12\xc9\xc6\x10e\x9fSpEcV1\x03\xf9\xd2\xa7\x88\xc4\xae\x95SpEcV16\x00I`\x18\xd4K\x1aamountto_muxed_id\x00F\x00\x10\x00\x06\x00\x00\x00L\x00\x10\x00\x0b\x00\x00\x00\x0e\xb7\xba\xe2\xb3y\xe7\x00ab\x00\x00p\x00\x10\x00\x01\x00\x00\x00q\x00\x10\x00\x01\x00\x00\x00map_valuesvec_valuessingle_valuesingle_value_void\x00\x87\'\x0econtractspecv0\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x17::test_events::Transfer\x00\x00\x00\x00\x01\x00\x00\x00\x08transfer\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02to\x00\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0bto_muxed_id\x00\x00\x00\x03\xe8\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08transfer\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02to\x00\x00\x00\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x003An event whose data is a map, which is the default.\x00\x00\x00\x00\x00\x00\x00\x00\x18::test_events::MapValues\x00\x00\x00\x01\x00\x00\x00\nmap_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x05\x00\x00\x000An event whose data is a vec, rather than a map.\x00\x00\x00\x00\x00\x00\x00\x18::test_events::VecValues\x00\x00\x00\x01\x00\x00\x00\nvec_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nmap_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nvec_values\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x009An event whose data is a single value, rather than a map.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1a::test_events::SingleValue\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0csingle_value\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0csingle_value\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x00^An event whose data is a single value, and that has no data fields, and so\nwhose data is void.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1e::test_events::SingleValueVoid\x00\x00\x00\x00\x00\x01\x00\x00\x00\x11single_value_void\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0ffailed_transfer\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02to\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x06amount\x00\x00\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11single_value_void\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04from\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00xExecutable specified by the contract instance as a specific Wasm contract code entry identified by its Wasm sha256 hash.\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00_Executable reference via a persistent storage entry owned by this contract or another contract.\x00\x00\x00\x00\x0bExternalRef\x00\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::ContractExecutableRef\x00\x00\x00\x01\x00\x00\x00\xc0Executable referenced via a persistent storage entry owned by a contract,\neither this contract or another contract.\n\nThe persistent storage entry owned by the `owner` has the `tag` as its key.\x00\x00\x00\x00\x00\x00\x00$::soroban_sdk::ContractExecutableRef\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x05owner\x00\x00\x00\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x03tag\x00\x00\x00\x00\x10\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x1c::soroban_sdk::auth::Context\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00*::soroban_sdk::auth::SubContractInvocation\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00$::soroban_sdk::auth::ContractContext\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00-::soroban_sdk::auth::InvokerContractAuthEntry\x00\x00\x00\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00-::soroban_sdk::auth::InvokerContractAuthEntry\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*::soroban_sdk::auth::SubContractInvocation\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x000::soroban_sdk::auth::CreateContractHostFnContext\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00?::soroban_sdk::auth::CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00!::soroban_sdk::ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\"::soroban_sdk::address::Executable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1d\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
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
        _phantom: core::marker::PhantomData<&'a ()>,
    }
    impl<'a> Client<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                _phantom: core::marker::PhantomData,
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
            res
        }
        pub fn map_values(&self, from: &soroban_sdk::Address, a: &u32, b: &u32) -> () {
            use core::ops::Not;
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
            res
        }
        pub fn vec_values(&self, from: &soroban_sdk::Address, a: &u32, b: &u32) -> () {
            use core::ops::Not;
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
            res
        }
        pub fn single_value(&self, from: &soroban_sdk::Address, amount: &i128) -> () {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "single_value") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [from.into_val(&self.env), amount.into_val(&self.env)],
                ),
            );
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
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "single_value") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [from.into_val(&self.env), amount.into_val(&self.env)],
                ),
            );
            res
        }
        pub fn failed_transfer(
            &self,
            from: &soroban_sdk::Address,
            to: &soroban_sdk::Address,
            amount: &i128,
        ) -> () {
            use core::ops::Not;
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
            res
        }
        pub fn single_value_void(&self, from: &soroban_sdk::Address) -> () {
            use core::ops::Not;
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "single_value_void") },
                ::soroban_sdk::Vec::from_array(&self.env, [from.into_val(&self.env)]),
            );
            res
        }
        pub fn try_single_value_void(
            &self,
            from: &soroban_sdk::Address,
        ) -> Result<
            Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "single_value_void") },
                ::soroban_sdk::Vec::from_array(&self.env, [from.into_val(&self.env)]),
            );
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
    impl ContractExecutableRef {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::ContractExecutableRef"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLEREF: [u8; ContractExecutableRef::spec_xdr().len()] =
        ContractExecutableRef::spec_xdr();
    impl ContractExecutableRef {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        ContractExecutableRef::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"owner",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"tag",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::String,
                        },
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; ContractExecutableRef::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { ContractExecutableRef::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutableRef {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::String as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &ContractExecutableRef::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl ContractContext {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::ContractContext"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; ContractContext::spec_xdr().len()] =
        ContractContext::spec_xdr();
    impl ContractContext {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        ContractContext::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"args",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                                &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                                    element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Val,
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"contract",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"fn_name",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Symbol,
                        },
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; ContractContext::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { ContractContext::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &ContractContext::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl SubContractInvocation {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::SubContractInvocation"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_SUBCONTRACTINVOCATION: [u8; SubContractInvocation::spec_xdr().len()] =
        SubContractInvocation::spec_xdr();
    impl SubContractInvocation {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                SubContractInvocation::spec_name(),
            ),
            fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"context",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                <ContractContext>::spec_name(),
                            ),
                        }),
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"sub_invocations",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                            &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                                element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <InvokerContractAuthEntry>::spec_name(),
                                    ),
                                }),
                            },
                        ),
                    },
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; SubContractInvocation::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { SubContractInvocation::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &SubContractInvocation::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl CreateContractHostFnContext {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::CreateContractHostFnContext"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CREATECONTRACTHOSTFNCONTEXT: [u8;
        CreateContractHostFnContext::spec_xdr().len()] = CreateContractHostFnContext::spec_xdr();
    impl CreateContractHostFnContext {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        CreateContractHostFnContext::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"executable",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractExecutable>::spec_name(),
                                    ),
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"salt",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 },
                            ),
                        },
                    ]),
                },
            );
        pub const fn spec_xdr(
        ) -> [u8; CreateContractHostFnContext::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { CreateContractHostFnContext::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for CreateContractHostFnContext {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractExecutable as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &CreateContractHostFnContext::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl CreateContractWithConstructorHostFnContext {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::CreateContractWithConstructorHostFnContext"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CREATECONTRACTWITHCONSTRUCTORHOSTFNCONTEXT: [u8;
        CreateContractWithConstructorHostFnContext::spec_xdr().len()] =
        CreateContractWithConstructorHostFnContext::spec_xdr();
    impl CreateContractWithConstructorHostFnContext {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtStructV0(
                soroban_sdk::xdr::r#const::ScSpecUdtStructV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        CreateContractWithConstructorHostFnContext::spec_name(),
                    ),
                    fields: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"constructor_args",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                                &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                                    element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Val,
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"executable",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractExecutable>::spec_name(),
                                    ),
                                },
                            ),
                        },
                        soroban_sdk::xdr::r#const::ScSpecUdtStructFieldV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"salt",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 },
                            ),
                        },
                    ]),
                },
            );
        pub const fn spec_xdr(
        ) -> [u8; CreateContractWithConstructorHostFnContext::__SPEC_XDR_ENTRY.const_xdr_len()]
        {
            const { CreateContractWithConstructorHostFnContext::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &CreateContractWithConstructorHostFnContext::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl ContractExecutable {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::ContractExecutable"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; ContractExecutable::spec_xdr().len()] =
        ContractExecutable::spec_xdr();
    impl ContractExecutable {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                ContractExecutable::spec_name(),
            ),
            cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Wasm",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(soroban_sdk::xdr::r#const::ScSpecTypeBytesN {
                                    n: 32u32,
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"ExternalRef",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractExecutableRef>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; ContractExecutable::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { ContractExecutable::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for ContractExecutable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <ContractExecutableRef as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &ContractExecutable::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl Context {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::Context"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_CONTEXT: [u8; Context::spec_xdr().len()] = Context::spec_xdr();
    impl Context {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                Context::spec_name(),
            ),
            cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Contract",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <ContractContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractWithCtorHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractWithConstructorHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; Context::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Context::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for Context {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <ContractContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Context::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl InvokerContractAuthEntry {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::InvokerContractAuthEntry"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_INVOKERCONTRACTAUTHENTRY: [u8; InvokerContractAuthEntry::spec_xdr()
        .len()] = InvokerContractAuthEntry::spec_xdr();
    impl InvokerContractAuthEntry {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry = soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
            name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                InvokerContractAuthEntry::spec_name(),
            ),
            cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                &[
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Contract",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <SubContractInvocation>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                    soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"",
                        ),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"CreateContractWithCtorHostFn",
                        ),
                        type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                            &[
                                soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                                        <CreateContractWithConstructorHostFnContext>::spec_name(),
                                    ),
                                }),
                            ],
                        ),
                    }),
                ],
            ),
        });
        pub const fn spec_xdr() -> [u8; InvokerContractAuthEntry::__SPEC_XDR_ENTRY.const_xdr_len()]
        {
            const { InvokerContractAuthEntry::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for InvokerContractAuthEntry {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <SubContractInvocation as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <CreateContractWithConstructorHostFnContext as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &InvokerContractAuthEntry::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl Executable {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::Executable"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_TYPE_EXECUTABLE: [u8; Executable::spec_xdr().len()] = Executable::spec_xdr();
    impl Executable {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::UdtUnionV0(
                soroban_sdk::xdr::r#const::ScSpecUdtUnionV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        Executable::spec_name(),
                    ),
                    cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::TupleV0(
                            soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseTupleV0 {
                                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"",
                                ),
                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"Wasm",
                                ),
                                type_: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                                    soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                        soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 },
                                    ),
                                ]),
                            },
                        ),
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::VoidV0(
                            soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseVoidV0 {
                                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"",
                                ),
                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"StellarAsset",
                                ),
                            },
                        ),
                        soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseV0::VoidV0(
                            soroban_sdk::xdr::r#const::ScSpecUdtUnionCaseVoidV0 {
                                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"",
                                ),
                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                    b"Account",
                                ),
                            },
                        ),
                    ]),
                },
            );
        pub const fn spec_xdr() -> [u8; Executable::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Executable::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for Executable {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::BytesN<32> as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Executable::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl Transfer {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::Transfer"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_EVENT_TRANSFER: [u8; Transfer::spec_xdr().len()] = Transfer::spec_xdr();
    impl Transfer {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(
                soroban_sdk::xdr::r#const::ScSpecEventV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        Transfer::spec_name(),
                    ),
                    prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSymbol(
                            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"transfer",
                            ),
                        ),
                    ]),
                    params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"from",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"to",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"amount",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"to_muxed_id",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Option(
                                &soroban_sdk::xdr::r#const::ScSpecTypeOption {
                                    value_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                                },
                            ),
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                    ]),
                    data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
                },
            );
        pub const fn spec_xdr() -> [u8; Transfer::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { Transfer::__SPEC_XDR_ENTRY.const_to_xdr() }
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
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Transfer::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl MapValues {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::MapValues"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_EVENT_MAPVALUES: [u8; MapValues::spec_xdr().len()] = MapValues::spec_xdr();
    impl MapValues {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(
                soroban_sdk::xdr::r#const::ScSpecEventV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        MapValues::spec_name(),
                    ),
                    prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSymbol(
                            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"map_values",
                            ),
                        ),
                    ]),
                    params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"from",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                    ]),
                    data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Map,
                },
            );
        pub const fn spec_xdr() -> [u8; MapValues::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { MapValues::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for MapValues {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &MapValues::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl VecValues {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::VecValues"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_EVENT_VECVALUES: [u8; VecValues::spec_xdr().len()] = VecValues::spec_xdr();
    impl VecValues {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(
                soroban_sdk::xdr::r#const::ScSpecEventV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        VecValues::spec_name(),
                    ),
                    prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSymbol(
                            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"vec_values",
                            ),
                        ),
                    ]),
                    params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"from",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"a"),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"b"),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                    ]),
                    data_format: soroban_sdk::xdr::ScSpecEventDataFormat::Vec,
                },
            );
        pub const fn spec_xdr() -> [u8; VecValues::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { VecValues::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for VecValues {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <u32 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &VecValues::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl SingleValue {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::SingleValue"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_EVENT_SINGLEVALUE: [u8; SingleValue::spec_xdr().len()] =
        SingleValue::spec_xdr();
    impl SingleValue {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(
                soroban_sdk::xdr::r#const::ScSpecEventV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        SingleValue::spec_name(),
                    ),
                    prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSymbol(
                            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"single_value",
                            ),
                        ),
                    ]),
                    params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"from",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                        },
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"amount",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::I128,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::Data,
                        },
                    ]),
                    data_format: soroban_sdk::xdr::ScSpecEventDataFormat::SingleValue,
                },
            );
        pub const fn spec_xdr() -> [u8; SingleValue::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { SingleValue::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for SingleValue {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            <i128 as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &SingleValue::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
    impl SingleValueVoid {
        #[doc(hidden)]
        pub const fn spec_name() -> &'static str {
            "::test_import_contract::eventscontract::SingleValueVoid"
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_EVENT_SINGLEVALUEVOID: [u8; SingleValueVoid::spec_xdr().len()] =
        SingleValueVoid::spec_xdr();
    impl SingleValueVoid {
        const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
            soroban_sdk::xdr::r#const::ScSpecEntry::EventV0(
                soroban_sdk::xdr::r#const::ScSpecEventV0 {
                    doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                    name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(
                        SingleValueVoid::spec_name(),
                    ),
                    prefix_topics: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSymbol(
                            soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"single_value_void",
                            ),
                        ),
                    ]),
                    params: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecEventParamV0 {
                            doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                            name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                                b"from",
                            ),
                            type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                            location: soroban_sdk::xdr::ScSpecEventParamLocationV0::TopicList,
                        },
                    ]),
                    data_format: soroban_sdk::xdr::ScSpecEventDataFormat::SingleValue,
                },
            );
        pub const fn spec_xdr() -> [u8; SingleValueVoid::__SPEC_XDR_ENTRY.const_xdr_len()] {
            const { SingleValueVoid::__SPEC_XDR_ENTRY.const_to_xdr() }
        }
    }
    impl soroban_sdk::SpecShakingMarker for SingleValueVoid {
        #[doc(hidden)]
        #[inline(always)]
        fn spec_shaking_marker() {
            <soroban_sdk::Address as soroban_sdk::SpecShakingMarker>::spec_shaking_marker();
            {
                static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker = soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &SingleValueVoid::spec_xdr(),
                );
                let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
            }
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
impl Error {
    #[doc(hidden)]
    pub const fn spec_name() -> &'static str {
        "::test_import_contract::Error"
    }
}
#[doc(hidden)]
#[allow(dead_code)]
#[link_section = "contractspecv0"]
static __SPEC_XDR_TYPE_ERROR: [u8; Error::spec_xdr().len()] = Error::spec_xdr();
impl Error {
    const __SPEC_XDR_ENTRY: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::UdtErrorEnumV0(
            soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                lib: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::StringM::try_from_str_or_panic(Error::spec_name()),
                cases: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"Abort"),
                        value: 0u32,
                    },
                    soroban_sdk::xdr::r#const::ScSpecUdtErrorEnumCaseV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"Overflow",
                        ),
                        value: 1u32,
                    },
                ]),
            },
        );
    pub const fn spec_xdr() -> [u8; Error::__SPEC_XDR_ENTRY.const_xdr_len()] {
        const { Error::__SPEC_XDR_ENTRY.const_to_xdr() }
    }
}
impl soroban_sdk::SpecShakingMarker for Error {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        {
            static MARKER: soroban_sdk::reexports_for_macros::soroban_spec::shaking::Marker =
                soroban_sdk::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(
                    &Error::spec_xdr(),
                );
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
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
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_ADD_WITH: [u8; super::Contract::spec_xdr_add_with().len()] =
        super::Contract::spec_xdr_add_with();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_add_with: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"add_with"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"contract_id",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"x"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"y"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add_with() -> [u8; Contract::__SPEC_XDR_ENTRY_add_with.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_add_with.const_to_xdr() }
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
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_SAFE_ADD_WITH: [u8; super::Contract::spec_xdr_safe_add_with().len()] =
        super::Contract::spec_xdr_safe_add_with();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_safe_add_with: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"safe_add_with"),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"contract_id",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"x"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"y"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::Result(
                        &soroban_sdk::xdr::r#const::ScSpecTypeResult {
                            ok_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                            error_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Error,
                        },
                    ),
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_safe_add_with(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_safe_add_with.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_safe_add_with.const_to_xdr() }
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
    #[link_section = "contractspecv0"]
    static __SPEC_XDR_FN_SAFE_ADD_WITH_TWO: [u8; super::Contract::spec_xdr_safe_add_with_two()
        .len()] = super::Contract::spec_xdr_safe_add_with_two();
}
impl Contract {
    #[allow(non_upper_case_globals)]
    const __SPEC_XDR_ENTRY_safe_add_with_two: soroban_sdk::xdr::r#const::ScSpecEntry =
        soroban_sdk::xdr::r#const::ScSpecEntry::FunctionV0(
            soroban_sdk::xdr::r#const::ScSpecFunctionV0 {
                doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                name: soroban_sdk::xdr::r#const::ScSymbol(
                    soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"safe_add_with_two",
                    ),
                ),
                inputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                            b"contract_id",
                        ),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::Address,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"x"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                    },
                    soroban_sdk::xdr::r#const::ScSpecFunctionInputV0 {
                        doc: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b""),
                        name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"y"),
                        type_: soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                    },
                ]),
                outputs: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                    soroban_sdk::xdr::r#const::ScSpecTypeDef::Result(
                        &soroban_sdk::xdr::r#const::ScSpecTypeResult {
                            ok_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::U64,
                            error_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Error,
                        },
                    ),
                ]),
            },
        );
    #[allow(non_snake_case)]
    pub const fn spec_xdr_safe_add_with_two(
    ) -> [u8; Contract::__SPEC_XDR_ENTRY_safe_add_with_two.const_xdr_len()] {
        const { Contract::__SPEC_XDR_ENTRY_safe_add_with_two.const_to_xdr() }
    }
}
impl<'a> ContractClient<'a> {
    pub fn add_with(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
        use core::ops::Not;
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
        res
    }
    pub fn safe_add_with(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
        use core::ops::Not;
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
        res
    }
    pub fn safe_add_with_two(&self, contract_id: &Address, x: &u64, y: &u64) -> u64 {
        use core::ops::Not;
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
#[export_name = "add_with"]
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
#[export_name = "safe_add_with"]
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
#[export_name = "safe_add_with_two"]
pub extern "C" fn __Contract__safe_add_with_two__invoke_raw_extern(
    arg_0: soroban_sdk::Val,
    arg_1: soroban_sdk::Val,
    arg_2: soroban_sdk::Val,
) -> soroban_sdk::Val {
    #[allow(deprecated)]
    __Contract__safe_add_with_two__invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
}
