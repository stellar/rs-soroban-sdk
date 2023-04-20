#![no_std]
use soroban_sdk::{contracterror, contractimpl, Address, RawVal};

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
    use soroban_sdk::{
        testutils::Address as _,
        xdr::{
            AddressWithNonce, AuthorizedInvocation, ContractAuth, ScAddress, ScVal, StringM, VecM,
        },
        Address, Env,
    };

    use crate::{Authonator, Contract, ContractClient};
    extern crate std;

    #[test]
    fn test_with_mocked_auth() {
        let e = Env::default();
        e.record_auth();

        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::random(&e);

        let r = client.add(&a);
        assert_eq!(r, 2);
        let auths = e.recorded_top_authorizations();
        std::println!("auths: {:?}", auths);
    }

    #[test]
    fn test_with_real_contract_auth() {
        let e = Env::default();

        let authonator_id = e.register_contract(None, Authonator);
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::from_contract_id(&authonator_id);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        e.set_auth(&[ContractAuth {
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

    // TODO: This test is broken and doesn't work because the account doesn't exist.
    #[test]
    fn test_with_real_account_auth() {
        let e = Env::default();

        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::random_account(&e);
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        e.set_auth(&[ContractAuth {
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
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AuthonatorError {
    HellNo = 1,
}

pub struct Authonator;

#[contractimpl]
impl Authonator {
    #[allow(non_snake_case)]
    pub fn __check_auth(
        _signature_payload: RawVal,
        _signatures: RawVal,
        _auth_context: RawVal,
    ) -> Result<(), AuthonatorError> {
        Ok(())
    }
}
