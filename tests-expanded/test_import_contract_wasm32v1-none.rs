#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::{contract, contractimpl, Address, Env};
mod addcontract {
    pub const WASM: &[u8] = b"\x00asm\x01\x00\x00\x00\x01\x14\x04`\x01~\x01~`\x02\x7f~\x00`\x02~~\x01~`\x00\x00\x02\r\x02\x01i\x010\x00\x00\x01i\x01_\x00\x00\x03\x05\x04\x01\x02\x03\x03\x05\x03\x01\x00\x10\x06!\x04\x7f\x01A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x7f\x00A\x80\x80\xc0\x00\x0b\x07/\x05\x06memory\x02\x00\x03add\x00\x03\x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\x8b\x02\x04]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc0\x00F\r\x00\x02@ \x02A\x06F\r\x00B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x88!\x01B\x00!\x03\x0c\x01\x0bB\x00!\x03 \x01\x10\x80\x80\x80\x80\x00!\x01\x0b \x00 \x037\x03\x00 \x00 \x017\x03\x08\x0b\x9c\x01\x01\x01\x7f#\x80\x80\x80\x80\x00A\x10k\"\x02$\x80\x80\x80\x80\x00 \x02 \x00\x10\x82\x80\x80\x80\x00\x02@\x02@ \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08!\x00 \x02 \x01\x10\x82\x80\x80\x80\x00 \x02(\x02\x00A\x01F\r\x00 \x02)\x03\x08\"\x01 \x00|\"\x00 \x01T\r\x01\x02@\x02@ \x00B\xff\xff\xff\xff\xff\xff\xff\xff\x00V\r\x00 \x00B\x08\x86B\x06\x84!\x00\x0c\x01\x0b \x00\x10\x81\x80\x80\x80\x00!\x00\x0b \x02A\x10j$\x80\x80\x80\x80\x00 \x00\x0f\x0b\x00\x0b\x10\x84\x80\x80\x80\x00\x00\x0b\t\x00\x10\x85\x80\x80\x80\x00\x00\x0b\x03\x00\x00\x0b\x0b\t\x01\x00A\x80\x80\xc0\x00\x0b\x00\x00\xdf\x15\x0econtractspecv0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03add\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x01a\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x00\x00\x00\x00\x01b\x00\x00\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x02\x00\x00\x00\xe3Context of a single authorized call performed by an address.\n\nCustom account contracts that implement `__check_auth` special function\nreceive a list of `Context` values corresponding to all the calls that\nneed to be authorized.\x00\x00\x00\x00\x00\x00\x00\x00\x07Context\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14Contract invocation.\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x01\x00\x00\x00=Contract that has a constructor with no arguments is created.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00DContract that has a constructor with 1 or more arguments is created.\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00\xbdAuthorization context of a single contract call.\n\nThis struct corresponds to a `require_auth_for_args` call for an address\nfrom `contract` function with `fn_name` name and `args` arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x04args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08contract\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x07fn_name\x00\x00\x00\x00\x11\x00\x00\x00\x02\x00\x00\x00_Contract executable used for creating a new contract and used in\n`CreateContractHostFnContext`.\x00\x00\x00\x00\x00\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x008Value of contract node in InvokerContractAuthEntry tree.\x00\x00\x00\x00\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x07context\x00\x00\x00\x07\xd0\x00\x00\x00\x0fContractContext\x00\x00\x00\x00\x00\x00\x00\x00\x0fsub_invocations\x00\x00\x00\x03\xea\x00\x00\x07\xd0\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x02\x00\x00\x01/A node in the tree of authorizations performed on behalf of the current\ncontract as invoker of the contracts deeper in the call stack.\n\nThis is used as an argument of `authorize_as_current_contract` host function.\n\nThis tree corresponds `require_auth[_for_args]` calls on behalf of the\ncurrent contract.\x00\x00\x00\x00\x00\x00\x00\x00\x18InvokerContractAuthEntry\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12Invoke a contract.\x00\x00\x00\x00\x00\x08Contract\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x15SubContractInvocation\x00\x00\x00\x00\x00\x00\x01\x00\x00\x005Create a contract passing 0 arguments to constructor.\x00\x00\x00\x00\x00\x00\x14CreateContractHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x01\x00\x00\x00=Create a contract passing 0 or more arguments to constructor.\x00\x00\x00\x00\x00\x00\x1cCreateContractWithCtorHostFn\x00\x00\x00\x01\x00\x00\x07\xd0\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x01\x00\x00\x00vAuthorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1bCreateContractHostFnContext\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00\xd6Authorization context for `create_contract` host function that creates a\nnew contract on behalf of authorizer address.\nThis is the same as `CreateContractHostFnContext`, but also has\ncontract constructor arguments.\x00\x00\x00\x00\x00\x00\x00\x00\x00*CreateContractWithConstructorHostFnContext\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x10constructor_args\x00\x00\x03\xea\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nexecutable\x00\x00\x00\x00\x07\xd0\x00\x00\x00\x12ContractExecutable\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04salt\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\nExecutable\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04Wasm\x00\x00\x00\x01\x00\x00\x03\xee\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0cStellarAsset\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07Account\x00\x00\xd7\x05\x1ccontractspecv0.rssdk.graphv0SpGrV\x01\x00\x00\x03\xedX=\x97\x96\x0b\xa5*Y\x95;WYd\xcb\x0fF\x96\xdf>\xba\x07\x0f{\x08\xfb+dY\xb4,\x00\x00SpGrV\x01\x00\x02\xa3J\xcf\xf7D\x93\x0bB]\x95\xeb\xfe\x03y\x83e5\\\x16\xeb\x94Ne\xe6Xw\x1f&\xf7\xc0pT\x00\x03\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xec\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9ns\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xafSpGrV\x01\x00\x02\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xec\x00\x00SpGrV\x01\x00\x02\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xcc\x00\x00SpGrV\x01\x00\x02\x9e)H\x8e\xf0\x01{{\xce\x9fdO\x0eD\xc0,\x0f\xe8\xee\'\x845r\x9f\xeb`\xd0\x12H\x17\x96g\x00\x02\xf1\xf9\x90\x07E*e\xfd\x08\x8c\xc2\xb1\x10\xfd\xca\xae6T\x9e)[\xdb\xee_\xfa\xed\xcaE\x9bv\x98\xecULqD\xd3\xfa:\x1f\x0c\xa5\xb7\x04\xe5H\x8b\x91J\x9e\x0fe\x7f\x9f[\xdbG#\xc7o\xb0\xf4\xcf\xe6SpGrV\x01\x00\x02ULqD\xd3\xfa:\x1f\x0c\xa5\xb7\x04\xe5H\x8b\x91J\x9e\x0fe\x7f\x9f[\xdbG#\xc7o\xb0\xf4\xcf\xe6\x00\x03\x9e)H\x8e\xf0\x01{{\xce\x9fdO\x0eD\xc0,\x0f\xe8\xee\'\x845r\x9f\xeb`\xd0\x12H\x17\x96g\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9ns\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xafSpGrV\x01\x00\x02\x15\xe5\x1a,\xc0\xc7\xef\xd4\xe0\xda\x0e\x16\x87\x1a\xae\xe73X\x1d2\x8a\x8aP\xab\x81\xef\x18\x8e:\xa2\xa9n\x00\x01\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xccSpGrV\x01\x00\x02s\x94\x0c\x1926\x1d\x90\x19\xc8\x8b=\xbe\xaf\xd7n\xcd=z\t\xb27\xa5.\xde:>\x03\xcd7\x12\xaf\x00\x01\xb6\xb1Hy\xda\xca\xaf\xcc\x1f\x01\x07y\x9a#g}2\x1c.Q0Vf\xc7ze\xacm\x01\xf5Y\xccSpGrV\x01\x00\x02L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e\x00\x00\x00\x1e\x11contractenvmetav0\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x00\x00O\x0econtractmetav0\x00\x00\x00\x00\x00\x00\x00\x05rsver\x00\x00\x00\x00\x00\x00\x061.91.0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12rssdk_spec_shaking\x00\x00\x00\x00\x00\x012\x00\x00\x00";
    pub trait Contract {
        fn add(env: soroban_sdk::Env, a: u64, b: u64) -> u64;
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
    }
    ///Args is a type for building arg lists for functions defined in "Contract".
    pub struct Args;
    impl Args {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn add<'i>(a: &'i u64, b: &'i u64) -> (&'i u64, &'i u64) {
            (a, b)
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
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_TYPE_CONTRACTCONTEXT: [u8; 96usize] = ContractContext::spec_xdr();
    impl ContractContext {
        pub const fn spec_xdr() -> [u8; 96usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0fContractContext\0\0\0\0\x03\0\0\0\0\0\0\0\x04args\0\0\x03\xea\0\0\0\0\0\0\0\0\0\0\0\x08contract\0\0\0\x13\0\0\0\0\0\0\0\x07fn_name\0\0\0\0\x11"
        }
    }
    impl soroban_sdk::spec_shaking::SpecTypeId for ContractContext {
        const SPEC_TYPE_ID: [u8; 32] = *b"\x03\x04uN\xea\xd7[\x13V\x9f\xd4\xbd\xc1\x8a\xd6\x7f\xd8iD\xa5B\x89qT\x0b'\xad(\xb8\x9f\x8f\x19";
    }
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_TYPE_CONTEXT: [u8; 244usize] = Context::spec_xdr();
    impl Context {
        pub const fn spec_xdr() -> [u8; 244usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x07Context\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x08Contract\0\0\0\x01\0\0\x07\xd0\0\0\0\x0fContractContext\0\0\0\0\x01\0\0\0\0\0\0\0\x14CreateContractHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0\x1bCreateContractHostFnContext\0\0\0\0\x01\0\0\0\0\0\0\0\x1cCreateContractWithCtorHostFn\0\0\0\x01\0\0\x07\xd0\0\0\0*CreateContractWithConstructorHostFnContext\0\0"
        }
    }
    impl soroban_sdk::spec_shaking::SpecTypeId for Context {
        const SPEC_TYPE_ID: [u8; 32] = *b"\r\xb6\x0b\xec\x8f\xd04l1\xb3-\xa0{\x90\xa3\xc2\xab\x93\xd4\x82x\xe1_\x8a\xa8N?.\xcd\xc1\xfc\x08";
    }
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_TYPE_CONTRACTEXECUTABLE: [u8; 68usize] = ContractExecutable::spec_xdr();
    impl ContractExecutable {
        pub const fn spec_xdr() -> [u8; 68usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x12ContractExecutable\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 "
        }
    }
    impl soroban_sdk::spec_shaking::SpecTypeId for ContractExecutable {
        const SPEC_TYPE_ID: [u8; 32] = *b"^\xbe34\xd8\x99\x84\x91\x81\x9fu\x9fu\x05\xb8\xb4\x14\x95\xb7\x9d|\x06$\x04y\xe9\"\xb9\x14\xfc\xf9\x85";
    }
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
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
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_TYPE_EXECUTABLE: [u8; 104usize] = Executable::spec_xdr();
    impl Executable {
        pub const fn spec_xdr() -> [u8; 104usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\nExecutable\0\0\0\0\0\x03\0\0\0\x01\0\0\0\0\0\0\0\x04Wasm\0\0\0\x01\0\0\x03\xee\0\0\0 \0\0\0\0\0\0\0\0\0\0\0\x0cStellarAsset\0\0\0\0\0\0\0\0\0\0\0\x07Account\0"
        }
    }
    impl soroban_sdk::spec_shaking::SpecTypeId for Executable {
        const SPEC_TYPE_ID: [u8; 32] = *b"L|{\r\xf4\xf2\x1a\xa8\xf6\x981\xe2K\xcb\x824N\xe6\x97\xed\xdf\xc2\x1cck\xd6\xceW\x9cx\x10\x1e";
    }
    #[link_section = "contractspecv0.rssdk.graphv0"]
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
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, Executable>>::try_from_val(env, *val)
        }
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
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__add_with__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    #[link_section = "contractspecv0"]
    pub static __SPEC_XDR_FN_ADD_WITH: [u8; 88usize] = super::Contract::spec_xdr_add_with();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_add_with() -> [u8; 88usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x08add_with\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x01x\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\x01y\0\0\0\0\0\0\x06\0\0\0\x01\0\0\0\x06"
    }
}
#[link_section = "contractspecv0.rssdk.graphv0"]
#[allow(non_upper_case_globals)]
pub static __SPEC_GRAPH_FN_CONTRACT_ADD_WITH: [u8; 42usize] = soroban_sdk::spec_shaking::encode_graph_record::<
    42usize,
    0usize,
>(
    0,
    *b"\x01\x85\x97Gi\xe3\x86f\xe3\"\xb0+Y\xbf$\x98lN\x03\x1dk\x99\x17\xc6\x9a1\xfer\xafF\xa7\x0b",
    [],
);
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
