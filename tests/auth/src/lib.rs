#![no_std]
use soroban_sdk::{contractimpl, Address, Env, IntoVal};

pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn fn1(a: Address) -> u64 {
        a.require_auth();
        2
    }
}

#[cfg(test)]
mod test_a {
    use super::*;
    use soroban_sdk::{
        contracterror,
        testutils::{
            Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke,
        },
        xdr::{
            ScAddress, ScVal, SorobanAddressCredentials, SorobanAuthorizationEntry,
            SorobanAuthorizedContractFunction, SorobanAuthorizedFunction,
            SorobanAuthorizedInvocation, SorobanCredentials, StringM, VecM,
        },
        Address, Env, Error, RawVal, Symbol,
    };
    extern crate std;

    #[test]
    fn test_with_mock_all_auth() {
        let e = Env::default();

        let contract_id = e.register_contract(None, ContractA);
        let client = ContractAClient::new(&e, &contract_id);

        let a = Address::random(&e);

        let r = client.mock_all_auths().fn1(&a);
        assert_eq!(r, 2);
        assert_eq!(
            e.auths(),
            std::vec![(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_id.clone(),
                        Symbol::new(&e, "fn1"),
                        (&a,).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }
            )]
        );
    }

    #[test]
    fn test_with_mock_auth() {
        let e = Env::default();

        let contract_id = e.register_contract(None, ContractA);
        let client = ContractAClient::new(&e, &contract_id);

        let a = Address::random(&e);

        let r = client
            .mock_auths(&[MockAuth {
                address: &a,
                nonce: 0,
                invoke: &MockAuthInvoke {
                    contract: &contract_id,
                    fn_name: "fn1",
                    args: (&a,).into_val(&e),
                    sub_invokes: &[],
                },
            }])
            .fn1(&a);
        assert_eq!(r, 2);
        assert_eq!(
            e.auths(),
            std::vec![(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_id.clone(),
                        Symbol::new(&e, "fn1"),
                        (&a,).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }
            )]
        );
    }

    #[test]
    fn test_with_real_contract_auth_approve() {
        let e = Env::default();

        let contract_id = e.register_contract(None, ContractA);
        let client = ContractAClient::new(&e, &contract_id);

        let a = e.register_contract(None, auth_approve::Contract);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 0,
                    signature_args: Default::default(),
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(
                        SorobanAuthorizedContractFunction {
                            contract_address: contract_id.clone().try_into().unwrap(),
                            function_name: StringM::try_from("fn1").unwrap().into(),
                            args: std::vec![ScVal::Address(a_xdr.clone())].try_into().unwrap(),
                        },
                    ),
                    sub_invocations: VecM::default(),
                },
            }])
            .fn1(&a);

        assert_eq!(r, 2);
        assert_eq!(
            e.auths(),
            std::vec![(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_id.clone(),
                        Symbol::new(&e, "fn1"),
                        (&a,).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }
            )]
        );
    }

    #[test]
    fn test_with_real_contract_auth_decline() {
        let e = Env::default();

        let contract_id = e.register_contract(None, ContractA);
        let client = ContractAClient::new(&e, &contract_id);

        let a = e.register_contract(None, auth_decline::Contract);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 0,
                    signature_args: Default::default(),
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(
                        SorobanAuthorizedContractFunction {
                            contract_address: contract_id.try_into().unwrap(),
                            function_name: StringM::try_from("fn1").unwrap().into(),
                            args: std::vec![ScVal::Address(a_xdr.clone())].try_into().unwrap(),
                        },
                    ),
                    sub_invocations: VecM::default(),
                },
            }])
            .try_fn1(&a);

        // TODO: Update this test to assert that a general panic/trap occurred
        // once https://github.com/stellar/rs-soroban-env/issues/771 is fixed.
        // The ContractError(1) being captured here is from the
        // auth_decline::Contract defined at the bottom of this file. The auth
        // contract's error is leaking into the contract being called and
        // propogating as its own contract, which should not be happening.
        assert_eq!(r, Err(Ok(Error::from_contract_error(1))));

        assert_eq!(e.auths(), []);
    }

    mod auth_approve {
        use super::*;

        pub struct Contract;

        #[contractimpl]
        impl Contract {
            #[allow(non_snake_case)]
            pub fn __check_auth(
                _signature_payload: RawVal,
                _signatures: RawVal,
                _auth_context: RawVal,
            ) {
            }
        }
    }

    mod auth_decline {
        use super::*;

        pub struct Contract;

        #[contracterror]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
        #[repr(u32)]
        pub enum Error {
            Decline = 1,
        }

        #[contractimpl]
        impl Contract {
            #[allow(non_snake_case)]
            pub fn __check_auth(
                _signature_payload: RawVal,
                _signatures: RawVal,
                _auth_context: RawVal,
            ) -> Result<(), Error> {
                Err(Error::Decline)
            }
        }
    }
}

pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn fn2(e: Env, a: Address, sub: Address) -> u64 {
        a.require_auth_for_args((1, 2).into_val(&e));
        let client = ContractAClient::new(&e, &sub);
        client.fn1(&a)
    }
}

#[cfg(test)]
mod test_b {
    use super::*;
    use soroban_sdk::{
        contracterror,
        testutils::{
            Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke,
        },
        xdr::{
            ScAddress, ScVal, SorobanAddressCredentials, SorobanAuthorizationEntry,
            SorobanAuthorizedContractFunction, SorobanAuthorizedFunction,
            SorobanAuthorizedInvocation, SorobanCredentials, StringM,
        },
        Address, Env, Error, RawVal, Symbol,
    };
    extern crate std;

    #[test]
    fn test_with_mock_all_auth() {
        let e = Env::default();

        let contract_a_id = e.register_contract(None, ContractA);
        let contract_b_id = e.register_contract(None, ContractB);
        let client = ContractBClient::new(&e, &contract_b_id);

        let a = Address::random(&e);

        let r = client.mock_all_auths().fn2(&a, &contract_a_id);
        assert_eq!(r, 2);
        assert_eq!(
            e.auths(),
            [(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_b_id.clone(),
                        Symbol::short("fn2"),
                        (1, 2).into_val(&e),
                    )),
                    sub_invocations: std::vec![AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            contract_a_id.clone(),
                            Symbol::short("fn1"),
                            (&a,).into_val(&e),
                        )),
                        sub_invocations: std::vec![]
                    }]
                }
            ),],
        );
    }

    #[test]
    fn test_with_mock_auth() {
        let e = Env::default();

        let contract_a_id = e.register_contract(None, ContractA);
        let contract_b_id = e.register_contract(None, ContractB);
        let client = ContractBClient::new(&e, &contract_b_id);

        let a = Address::random(&e);

        let r = client
            .mock_auths(&[MockAuth {
                address: &a,
                nonce: 0,
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
        assert_eq!(r, 2);
        assert_eq!(
            e.auths(),
            [(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_b_id.clone(),
                        Symbol::short("fn2"),
                        (1, 2).into_val(&e),
                    )),
                    sub_invocations: std::vec![AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            contract_a_id.clone(),
                            Symbol::short("fn1"),
                            (&a,).into_val(&e),
                        )),
                        sub_invocations: std::vec![]
                    }]
                }
            ),],
        );
    }

    #[test]
    fn test_with_real_contract_auth_approve() {
        let e = Env::default();

        let contract_a_id = e.register_contract(None, ContractA);
        let contract_b_id = e.register_contract(None, ContractB);
        let client = ContractBClient::new(&e, &contract_b_id);

        let a = e.register_contract(None, auth_approve::Contract);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 0,
                    signature_args: Default::default(),
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(
                        SorobanAuthorizedContractFunction {
                            contract_address: contract_b_id.clone().try_into().unwrap(),
                            function_name: StringM::try_from("fn2").unwrap().into(),
                            args: std::vec![ScVal::I32(1), ScVal::I32(2),].try_into().unwrap(),
                        },
                    ),
                    sub_invocations: std::vec![SorobanAuthorizedInvocation {
                        function: SorobanAuthorizedFunction::ContractFn(
                            SorobanAuthorizedContractFunction {
                                contract_address: contract_a_id.clone().try_into().unwrap(),
                                function_name: StringM::try_from("fn1").unwrap().into(),
                                args: std::vec![ScVal::Address(a_xdr.clone())].try_into().unwrap(),
                            },
                        ),
                        sub_invocations: Default::default()
                    }]
                    .try_into()
                    .unwrap(),
                },
            }])
            .fn2(&a, &contract_a_id);

        assert_eq!(r, 2);
        assert_eq!(
            e.auths(),
            [(
                a.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_b_id.clone(),
                        Symbol::short("fn2"),
                        (1, 2).into_val(&e),
                    )),
                    sub_invocations: std::vec![AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            contract_a_id.clone(),
                            Symbol::short("fn1"),
                            (&a,).into_val(&e),
                        )),
                        sub_invocations: std::vec![]
                    }]
                }
            ),],
        );
    }

    #[test]
    fn test_with_real_contract_auth_decline() {
        let e = Env::default();

        let contract_a_id = e.register_contract(None, ContractA);
        let contract_b_id = e.register_contract(None, ContractB);
        let client = ContractBClient::new(&e, &contract_b_id);

        let a = e.register_contract(None, auth_decline::Contract);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        let r = client
            .set_auths(&[SorobanAuthorizationEntry {
                credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                    address: a_xdr.clone(),
                    nonce: 0,
                    signature_args: Default::default(),
                }),
                root_invocation: SorobanAuthorizedInvocation {
                    function: SorobanAuthorizedFunction::ContractFn(
                        SorobanAuthorizedContractFunction {
                            contract_address: contract_b_id.try_into().unwrap(),
                            function_name: StringM::try_from("fn2").unwrap().into(),
                            args: std::vec![ScVal::I32(1), ScVal::I32(2),].try_into().unwrap(),
                        },
                    ),
                    sub_invocations: std::vec![SorobanAuthorizedInvocation {
                        function: SorobanAuthorizedFunction::ContractFn(
                            SorobanAuthorizedContractFunction {
                                contract_address: contract_a_id.clone().try_into().unwrap(),
                                function_name: StringM::try_from("fn1").unwrap().into(),
                                args: std::vec![ScVal::Address(a_xdr.clone())].try_into().unwrap(),
                            },
                        ),
                        sub_invocations: Default::default()
                    }]
                    .try_into()
                    .unwrap(),
                },
            }])
            .try_fn2(&a, &contract_a_id);

        // TODO: Update this test to assert that a general panic/trap occurred
        // once https://github.com/stellar/rs-soroban-env/issues/771 is fixed.
        // The ContractError(1) being captured here is from the
        // auth_decline::Contract defined at the bottom of this file. The auth
        // contract's error is leaking into the contract being called and
        // propogating as its own contract, which should not be happening.
        assert_eq!(r, Err(Ok(Error::from_contract_error(1))));

        assert_eq!(e.auths(), []);
    }

    mod auth_approve {
        use super::*;

        pub struct Contract;

        #[contractimpl]
        impl Contract {
            #[allow(non_snake_case)]
            pub fn __check_auth(
                _signature_payload: RawVal,
                _signatures: RawVal,
                _auth_context: RawVal,
            ) {
            }
        }
    }

    mod auth_decline {
        use super::*;

        pub struct Contract;

        #[contracterror]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
        #[repr(u32)]
        pub enum Error {
            Decline = 1,
        }

        #[contractimpl]
        impl Contract {
            #[allow(non_snake_case)]
            pub fn __check_auth(
                _signature_payload: RawVal,
                _signatures: RawVal,
                _auth_context: RawVal,
            ) -> Result<(), Error> {
                Err(Error::Decline)
            }
        }
    }
}
