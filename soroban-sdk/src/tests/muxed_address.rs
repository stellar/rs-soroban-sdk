use crate::testutils::MuxedAddress as _;
use crate::{
    env::xdr::{AccountId, ScAddress, Uint256},
    Address, Env, MuxedAddress, TryFromVal,
};

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
    let muxed_address = MuxedAddress::from_account_id(&env, &[1; 32], 123_456);
    let expected_address = Address::try_from_val(
        &env,
        &ScAddress::Account(AccountId(
            soroban_env_host::xdr::PublicKey::PublicKeyTypeEd25519(Uint256([1; 32])),
        )),
    )
    .unwrap();
    assert_eq!(muxed_address.address(), expected_address);
    assert_eq!(muxed_address.id(), Some(123_456));
}
