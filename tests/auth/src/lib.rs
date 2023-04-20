#![no_std]
use soroban_sdk::{contracterror, contractimpl, vec, Address, Env, RawVal};

// --- THE CONTRACT

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env, a: Address) -> u64 {
        a.require_auth_for_args(vec![&e]);
        2
    }
}

// --- THE AUTHONTATOR

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

// --- THE TESTS

#[cfg(test)]
mod test {
    use soroban_sdk::{
        xdr::{AddressWithNonce, AuthorizedInvocation, ContractAuth, ScVec, StringM, VecM},
        Address, Env,
    };

    use crate::{Authonator, Contract, ContractClient};
    extern crate std;

    #[test]
    fn test_add() {
        let e = Env::default();
        let authonator_id = e.register_contract(None, Authonator);
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::from_contract_id(&authonator_id);

        e.set_auth(&[ContractAuth {
            address_with_nonce: Some(AddressWithNonce {
                address: (&a).try_into().unwrap(),
                nonce: 0,
            }),
            root_invocation: AuthorizedInvocation {
                contract_id: contract_id.to_array().into(),
                function_name: StringM::try_from("add").unwrap().into(),
                args: ScVec::default(),
                sub_invocations: VecM::default(),
            },
            signature_args: std::vec![].try_into().unwrap(),
        }]);

        let r = client.add(&a);
        assert_eq!(r, 2);
        // let auths = e.recorded_top_authorizations();
        // std::println!("auths: {:?}", auths);
    }
}
