use crate::{Address, BytesN, Env};

#[test]
fn test_account_address_conversions() {
    let env = Env::default();
    let account_address = Address::from_account_id(&env, &BytesN::from_array(&env, &[222u8; 32]));
    assert_eq!(
        account_address.account_id(),
        Some(BytesN::from_array(&env, &[222u8; 32]))
    );
    assert_eq!(account_address.contract_id(), None);
}

#[test]
fn test_contract_address_conversions() {
    let env = Env::default();
    let contract_address = Address::from_contract_id(&env, &BytesN::from_array(&env, &[111u8; 32]));
    assert_eq!(
        contract_address.contract_id(),
        Some(BytesN::from_array(&env, &[111u8; 32]))
    );
    assert_eq!(contract_address.account_id(), None);
}
