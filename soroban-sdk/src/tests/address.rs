use crate::{Address, Bytes, Env, String, TryIntoVal};

#[test]
fn test_account_address_conversions() {
    let env = Env::default();

    let strkey: String = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
        .try_into_val(&env)
        .unwrap();

    let address = Address::from_string(&strkey);
    assert_eq!(
        address.to_string().to_string(),
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn test_account_address_conversions_from_bytes() {
    let env = Env::default();

    let strkey: Bytes = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
        .as_bytes()
        .try_into_val(&env)
        .unwrap();

    let address = Address::from_string_bytes(&strkey);
    assert_eq!(
        address.to_string().to_string(),
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn test_contract_address_conversions() {
    let env = Env::default();

    let strkey: String = "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA"
        .try_into_val(&env)
        .unwrap();

    let address = Address::from_string(&strkey);
    assert_eq!(address.to_string(), strkey);
}
