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
        testutils::{Accounts, Address as _},
        vec,
        xdr::{
            AddressWithNonce, AuthorizedInvocation, ContractAuth, Hash, HashIdPreimage,
            HashIdPreimageContractAuth, ScAddress, ScMap, ScMapEntry, ScSymbol, ScVal, ScVec,
            StringM, VecM, WriteXdr,
        },
        Address, BytesN, Env, RawVal, Symbol,
    };
    extern crate std;

    #[test]
    fn test_with_mock_all_auth() {
        let e = Env::default();
        e.mock_all_auths();

        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let a = Address::random(&e);

        let r = client.add(&a);
        assert_eq!(r, 2);
        assert_eq!(
            e.mocked_auths(),
            [(
                a.clone(),
                contract_id,
                Symbol::short("add"),
                vec![&e, a.to_raw()]
            )],
        );
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

    #[test]
    fn test_with_real_account_auth() {
        let e = Env::default();

        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        use ed25519_dalek::Signer;
        let account_sk = ed25519_dalek::SecretKey::from_bytes(
            &hex::decode("5acc7253295dfc356c046297925a369f3d2762d00afdf2583ecbe92180b07c37")
                .unwrap(),
        )
        .unwrap();
        let account_pk = ed25519_dalek::PublicKey::from(&account_sk);
        let account_kp = ed25519_dalek::Keypair {
            secret: account_sk,
            public: account_pk,
        };

        let a = Address::from_account_id(&BytesN::from_array(&e, account_pk.as_bytes()));
        e.accounts().create(&a);
        let a_bytes = a.account_id().unwrap();
        let a_xdr: ScAddress = (&a).try_into().unwrap();

        let auth = AuthorizedInvocation {
            contract_id: contract_id.to_array().into(),
            function_name: StringM::try_from("add").unwrap().into(),
            args: std::vec![ScVal::Address(a_xdr.clone())].try_into().unwrap(),
            sub_invocations: VecM::default(),
        };

        let sig_payload = {
            let preimage = HashIdPreimage::ContractAuth(HashIdPreimageContractAuth {
                network_id: Hash(e.ledger().network_id().to_array()),
                invocation: auth.clone(),
                nonce: 0,
            });

            use sha2::Digest;
            let mut hasher = sha2::Sha256::new();
            hasher.update(preimage.to_xdr().unwrap());
            hasher.finalize()
        };
        let sig = account_kp.sign(sig_payload.as_slice()).to_bytes();

        e.set_auths(&[ContractAuth {
            address_with_nonce: Some(AddressWithNonce {
                address: a_xdr.clone(),
                nonce: 0,
            }),
            root_invocation: auth,
            signature_args: std::vec![ScVal::Vec(Some(ScVec(
                [ScVal::Map(Some(ScMap(
                    std::vec![
                        ScMapEntry {
                            key: ScVal::Symbol(ScSymbol("public_key".try_into().unwrap())),
                            val: a_bytes.try_into().unwrap(),
                        },
                        ScMapEntry {
                            key: ScVal::Symbol(ScSymbol("signature".try_into().unwrap())),
                            val: sig.try_into().unwrap(),
                        }
                    ]
                    .try_into()
                    .unwrap()
                )))]
                .try_into()
                .unwrap()
            )))]
            .try_into()
            .unwrap(),
        }]);

        let r = client.add(&a);
        assert_eq!(r, 2);
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
