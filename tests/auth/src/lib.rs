#![no_std]
use soroban_sdk::{contractimpl, Address};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Address) -> u64 {
        a.require_auth();
        2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        contracterror,
        testutils::{Address as _, MockAuth},
        vec,
        xdr::{
            AddressWithNonce, AuthorizedInvocation, ContractAuth, ScAddress, ScVal, StringM, VecM,
        },
        Address, Env, RawVal, Symbol,
    };
    extern crate std;

    #[test]
    fn test_with_mock_auth() {
        let e = Env::default();

        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::random(&e);

        let r = client
            .mock_auth(MockAuth {
                address: &a,
                nonce: 0,
                fn_name: "add",
                args: vec![&e, a.to_raw()],
                sub_invokes: &[],
            })
            .add(&a);

        assert_eq!(r, 2);
    }

    #[test]
    fn test_with_real_contract_auth_approve() {
        let e = Env::default();

        let auth_contract_id = e.register_contract(None, auth_approve::Contract);
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::from_contract_id(&auth_contract_id);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        e.set_auths(&[ContractAuth {
            address_with_nonce: Some(AddressWithNonce {
                address: a_xdr.clone(),
                nonce: 0,
            }),
            root_invocation: AuthorizedInvocation {
                contract_id: contract_id.to_array().into(),
                function_name: StringM::try_from("add").unwrap().into(),
                args: std::vec![ScVal::Address(a_xdr)].try_into().unwrap(),
                sub_invocations: VecM::default(),
            },
            signature_args: std::vec![].try_into().unwrap(),
        }]);

        let r = client.add(&a);
        assert_eq!(r, 2);
    }

    #[test]
    #[should_panic = "Status(ContractError(1))"]
    fn test_with_real_contract_auth_decline() {
        let e = Env::default();

        let auth_contract_id = e.register_contract(None, auth_decline::Contract);
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::from_contract_id(&auth_contract_id);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        e.set_auths(&[ContractAuth {
            address_with_nonce: Some(AddressWithNonce {
                address: a_xdr.clone(),
                nonce: 0,
            }),
            root_invocation: AuthorizedInvocation {
                contract_id: contract_id.to_array().into(),
                function_name: StringM::try_from("add").unwrap().into(),
                args: std::vec![ScVal::Address(a_xdr)].try_into().unwrap(),
                sub_invocations: VecM::default(),
            },
            signature_args: std::vec![].try_into().unwrap(),
        }]);

        client.add(&a);
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
