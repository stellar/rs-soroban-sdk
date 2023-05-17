use crate::{
    xdr::{AccountId, Hash, PublicKey, ScAddress, Uint256},
    Address, BytesN, Env, TryFromVal, TryIntoVal,
};

#[test]
fn test_account_address_conversions() {
    let env = Env::default();

    let scaddress = ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
        [222u8; 32],
    ))));

    let address = Address::try_from_val(&env, &scaddress).unwrap();

    let scaddress_roundtrip: ScAddress = (&address).try_into().unwrap();
    assert_eq!(scaddress, scaddress_roundtrip,);
}

#[test]
fn test_contract_address_conversions() {
    let env = Env::default();
    let contract_address = Address::from_contract_id(&BytesN::from_array(&env, &[111u8; 32]));
    assert_eq!(
        contract_address.contract_id(),
        BytesN::from_array(&env, &[111u8; 32])
    );

    let scaddress: ScAddress = (&contract_address).try_into().unwrap();
    assert_eq!(scaddress, ScAddress::Contract(Hash([111u8; 32])));

    let contract_address_rt: Address = scaddress.try_into_val(&env).unwrap();
    assert_eq!(contract_address_rt, contract_address);
}
