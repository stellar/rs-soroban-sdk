use soroban_sdk_macros::{contract, contractimpl};

use crate::testutils::{Address as _, MuxedAddress as _};
use crate::{self as soroban_sdk};
use crate::{
    env::xdr::{AccountId, ScAddress, Uint256},
    Address, Env, MuxedAddress, TryFromVal,
};

#[contract]
pub struct MuxedAddressContract;

#[contractimpl]
impl MuxedAddressContract {
    pub fn get_muxed_ids(
        _e: Env,
        a: MuxedAddress,
        b: soroban_sdk::MuxedAddress,
    ) -> (Option<u64>, Option<u64>) {
        (a.id(), b.id())
    }
}

#[test]
fn test_account_address_to_muxed_address_conversion() {
    let env = Env::default();
    let strkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
    let address = Address::from_str(&env, &strkey);

    let muxed_address: MuxedAddress = address.clone().into();
    assert_eq!(muxed_address.address(), address);
    assert_eq!(muxed_address.id(), None);
}

#[test]
fn test_contract_address_to_muxed_address_conversion() {
    let env = Env::default();
    let strkey = "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA";
    let address = Address::from_str(&env, &strkey);

    let muxed_address: MuxedAddress = address.clone().into();
    assert_eq!(muxed_address.address(), address);
    assert_eq!(muxed_address.id(), None);
}

#[test]
fn test_muxed_address_component_getters() {
    let env = Env::default();
    let muxed_address = MuxedAddress::generate(&env);
    let mut expected_id = [0_u8; 32];
    expected_id[31] = 1;
    let expected_address = Address::try_from_val(
        &env,
        &ScAddress::Account(AccountId(
            soroban_env_host::xdr::PublicKey::PublicKeyTypeEd25519(Uint256(expected_id)),
        )),
    )
    .unwrap();
    assert_eq!(muxed_address.address(), expected_address);

    let muxed_address_with_another_id = MuxedAddress::new(muxed_address, u64::MAX);
    assert_eq!(muxed_address_with_another_id.address(), expected_address);
    assert_eq!(muxed_address_with_another_id.id(), Some(u64::MAX));

    let muxed_address_from_address = MuxedAddress::new(muxed_address_with_another_id.address(), 0);
    assert_eq!(muxed_address_from_address.address(), expected_address);
    assert_eq!(muxed_address_from_address.id(), Some(0));
}

#[test]
fn test_accept_muxed_address_argument_in_contract() {
    let env = Env::default();
    let client = MuxedAddressContractClient::new(&env, &env.register(MuxedAddressContract, ()));

    let muxed_address = MuxedAddress::generate(&env);
    let muxed_address = MuxedAddress::new(muxed_address, 1);
    let muxed_address2 = MuxedAddress::generate(&env);
    let muxed_address2 = MuxedAddress::new(muxed_address2, 2);
    let non_muxed_address = Address::generate(&env);
    let non_muxed_address2 = Address::generate(&env);

    assert_eq!(
        client.get_muxed_ids(&muxed_address, &muxed_address2),
        (Some(1), Some(2))
    );
    assert_eq!(
        client.get_muxed_ids(&muxed_address, &muxed_address),
        (Some(1), Some(1))
    );
    assert_eq!(
        client
            .try_get_muxed_ids(&muxed_address, &non_muxed_address)
            .unwrap(),
        Ok((Some(1), None))
    );
    assert_eq!(
        client.get_muxed_ids(&non_muxed_address, &muxed_address2),
        (None, Some(2))
    );
    assert_eq!(
        client
            .try_get_muxed_ids(&non_muxed_address, &non_muxed_address2)
            .unwrap(),
        Ok((None, None))
    );
    assert_eq!(
        client.get_muxed_ids(non_muxed_address, muxed_address2),
        (None, Some(2))
    );
}
