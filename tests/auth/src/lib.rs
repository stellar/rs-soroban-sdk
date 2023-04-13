#![no_std]
use soroban_sdk::{contractimpl, vec, Address, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env, a: Address) -> u64 {
        a.require_auth_for_args(vec![&e]);
        2
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{
        testutils::Address as _,
        xdr::{AddressWithNonce, AuthorizedInvocation, ContractAuth, ScVec, StringM, VecM},
        Address, Env,
    };

    use crate::{Contract, ContractClient};
    extern crate std;

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::random(&e);

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
            signature_args: ScVec::default(),
        }]);

        let r = client.add(&a);
        assert_eq!(r, 2);
        // let auths = e.recorded_top_authorizations();
        // std::println!("auths: {:?}", auths);
    }
}
