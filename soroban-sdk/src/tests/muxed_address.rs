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

// Tests for MuxedAddress::from_str

#[test]
fn test_from_str_account() {
    let env = Env::default();
    let strkey = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
    let muxed = MuxedAddress::from_str(&env, strkey);
    let expected_address = Address::from_str(&env, strkey);
    assert_eq!(muxed.address(), expected_address);
    assert_eq!(muxed.id(), None);
}

#[test]
fn test_from_str_muxed_account() {
    let env = Env::default();
    let muxed_strkey = "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU";
    let base_strkey = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
    let muxed = MuxedAddress::from_str(&env, muxed_strkey);
    let expected_address = Address::from_str(&env, base_strkey);
    assert_eq!(muxed.address(), expected_address);
    assert_eq!(muxed.id(), Some(123456));
}

#[test]
fn test_from_str_contract() {
    let env = Env::default();
    let strkey = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
    let muxed = MuxedAddress::from_str(&env, strkey);
    let expected_address = Address::from_str(&env, strkey);
    assert_eq!(muxed.address(), expected_address);
    assert_eq!(muxed.id(), None);
}

// Debug roundtrip tests

#[test]
fn test_from_str_account_debug_roundtrip() {
    let env = Env::default();
    let strkey = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
    let muxed = MuxedAddress::from_str(&env, strkey);
    let debug_output = format!("{:?}", muxed);
    assert!(debug_output.contains(strkey));
}

#[test]
fn test_from_str_muxed_account_debug_roundtrip() {
    let env = Env::default();
    let strkey = "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU";
    let muxed = MuxedAddress::from_str(&env, strkey);
    let debug_output = format!("{:?}", muxed);
    assert!(debug_output.contains(strkey));
}

#[test]
fn test_from_str_contract_debug_roundtrip() {
    let env = Env::default();
    let strkey = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
    let muxed = MuxedAddress::from_str(&env, strkey);
    let debug_output = format!("{:?}", muxed);
    assert!(debug_output.contains(strkey));
}

// Error tests for unsupported strkey types

#[test]
#[should_panic(expected = "unsupported strkey type")]
fn test_from_str_private_key_panics() {
    let env = Env::default();
    // S... private key strkey
    let strkey = "SCZANGBA5YHTNYVVV3C7CAZMTQDBJHJQNE2M57SW7JEX6MRDBHWSKFPI";
    MuxedAddress::from_str(&env, strkey);
}

#[test]
#[should_panic(expected = "unsupported strkey type")]
fn test_from_str_preauth_tx_panics() {
    let env = Env::default();
    // T... pre-auth tx strkey
    let strkey = "TBU2RRGLXH3E5CQHTD3ODLDF2BWDCYUSSBLLZ5GNW7JXHDIRAT2IJDPN";
    MuxedAddress::from_str(&env, strkey);
}

#[test]
#[should_panic(expected = "unsupported strkey type")]
fn test_from_str_hash_x_panics() {
    let env = Env::default();
    // X... hash-x strkey
    let strkey = "XBU2RRGLXH3E5CQHTD3ODLDF2BWDCYUSSBLLZ5GNW7JXHDIRAT2IJDPN";
    MuxedAddress::from_str(&env, strkey);
}

#[test]
#[should_panic(expected = "unsupported strkey type")]
fn test_from_str_invalid_strkey_panics() {
    let env = Env::default();
    // Invalid strkey (random garbage)
    let strkey = "INVALID_NOT_A_REAL_STRKEY";
    MuxedAddress::from_str(&env, strkey);
}

// Tests for MuxedAddress::from_string_bytes

#[test]
fn test_from_string_bytes_account() {
    let env = Env::default();
    let strkey = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
    let strkey_bytes = crate::Bytes::from_slice(&env, strkey.as_bytes());
    let muxed = MuxedAddress::from_string_bytes(&strkey_bytes);
    let expected_address = Address::from_str(&env, strkey);
    assert_eq!(muxed.address(), expected_address);
    assert_eq!(muxed.id(), None);
}

#[test]
fn test_from_string_bytes_muxed_account() {
    let env = Env::default();
    let muxed_strkey = "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU";
    let base_strkey = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
    let strkey_bytes = crate::Bytes::from_slice(&env, muxed_strkey.as_bytes());
    let muxed = MuxedAddress::from_string_bytes(&strkey_bytes);
    let expected_address = Address::from_str(&env, base_strkey);
    assert_eq!(muxed.address(), expected_address);
    assert_eq!(muxed.id(), Some(123456));
}

#[test]
fn test_from_string_bytes_contract() {
    let env = Env::default();
    let strkey = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
    let strkey_bytes = crate::Bytes::from_slice(&env, strkey.as_bytes());
    let muxed = MuxedAddress::from_string_bytes(&strkey_bytes);
    let expected_address = Address::from_str(&env, strkey);
    assert_eq!(muxed.address(), expected_address);
    assert_eq!(muxed.id(), None);
}

#[test]
#[should_panic(expected = "strkey cannot be empty")]
fn test_from_string_bytes_empty_panics() {
    let env = Env::default();
    let strkey_bytes = crate::Bytes::from_slice(&env, &[]);
    MuxedAddress::from_string_bytes(&strkey_bytes);
}

#[test]
#[should_panic(expected = "strkey cannot be empty")]
fn test_from_string_empty_panics() {
    let env = Env::default();
    let strkey = crate::String::from_str(&env, "");
    MuxedAddress::from_string(&strkey);
}
