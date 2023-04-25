use soroban_env_host::TryIntoVal;

use crate::{
    xdr::{AccountId, Hash, PublicKey, ScAddress, Uint256},
    Address, BytesN, Env,
};

#[test]
fn test_account_address_conversions() {
    let env = Env::default();
    let account_address = Address::from_account_id(&BytesN::from_array(&env, &[222u8; 32]));
    assert_eq!(
        account_address.account_id(),
        Some(BytesN::from_array(&env, &[222u8; 32]))
    );
    assert_eq!(account_address.contract_id(), None);

    let scaddress: ScAddress = (&account_address).try_into().unwrap();
    assert_eq!(
        scaddress,
        ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
            [222u8; 32]
        ))))
    );

    let account_address_rt: Address = scaddress.try_into_val(&env).unwrap();
    assert_eq!(account_address_rt, account_address);
}

#[test]
fn test_contract_address_conversions() {
    let env = Env::default();
    let contract_address = Address::from_contract_id(&BytesN::from_array(&env, &[111u8; 32]));
    assert_eq!(
        contract_address.contract_id(),
        Some(BytesN::from_array(&env, &[111u8; 32]))
    );
    assert_eq!(contract_address.account_id(), None);

    let scaddress: ScAddress = (&contract_address).try_into().unwrap();
    assert_eq!(scaddress, ScAddress::Contract(Hash([111u8; 32])));

    let contract_address_rt: Address = scaddress.try_into_val(&env).unwrap();
    assert_eq!(contract_address_rt, contract_address);
}
