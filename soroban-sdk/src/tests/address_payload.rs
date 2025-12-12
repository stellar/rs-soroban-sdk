use crate::{self as soroban_sdk};
use soroban_sdk::{address_payload::AddressPayload, Address, BytesN, Env};

#[test]
fn test_contract_address_payload() {
    let env = Env::default();

    // Contract address: CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
    // Payload: d7928b72c2703ccfeaf7eb9ff4ef4d504a55a8b979fc9b450ea2c842b4d1ce61
    let strkey = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
    let address = Address::from_str(&env, strkey);

    let expected_payload = BytesN::from_array(
        &env,
        &[
            0xd7, 0x92, 0x8b, 0x72, 0xc2, 0x70, 0x3c, 0xcf, 0xea, 0xf7, 0xeb, 0x9f, 0xf4, 0xef,
            0x4d, 0x50, 0x4a, 0x55, 0xa8, 0xb9, 0x79, 0xfc, 0x9b, 0x45, 0x0e, 0xa2, 0xc8, 0x42,
            0xb4, 0xd1, 0xce, 0x61,
        ],
    );
    assert_eq!(
        address.to_payload(),
        Some(AddressPayload::ContractIDHash(expected_payload))
    );
}

#[test]
fn test_account_address_payload() {
    let env = Env::default();

    // Account address: GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ
    // Payload: 899b2840ed5636c56ddc5f14b23975f79f1ba2388d2694e4c56ecdddc960e5ef
    let strkey = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
    let address = Address::from_str(&env, strkey);

    let expected_payload = BytesN::from_array(
        &env,
        &[
            0x89, 0x9b, 0x28, 0x40, 0xed, 0x56, 0x36, 0xc5, 0x6d, 0xdc, 0x5f, 0x14, 0xb2, 0x39,
            0x75, 0xf7, 0x9f, 0x1b, 0xa2, 0x38, 0x8d, 0x26, 0x94, 0xe4, 0xc5, 0x6e, 0xcd, 0xdd,
            0xc9, 0x60, 0xe5, 0xef,
        ],
    );
    assert_eq!(
        address.to_payload(),
        Some(AddressPayload::AccountIdPublicKeyEd25519(expected_payload))
    );
}

#[test]
fn test_contract_address_from_payload() {
    let env = Env::default();

    let payload = BytesN::from_array(
        &env,
        &[
            0xd7, 0x92, 0x8b, 0x72, 0xc2, 0x70, 0x3c, 0xcf, 0xea, 0xf7, 0xeb, 0x9f, 0xf4, 0xef,
            0x4d, 0x50, 0x4a, 0x55, 0xa8, 0xb9, 0x79, 0xfc, 0x9b, 0x45, 0x0e, 0xa2, 0xc8, 0x42,
            0xb4, 0xd1, 0xce, 0x61,
        ],
    );

    let address = Address::from_payload(&env, AddressPayload::ContractIDHash(payload));
    assert_eq!(
        address.to_string().to_string(),
        "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
    );
}

#[test]
fn test_account_address_from_payload() {
    let env = Env::default();

    let payload = BytesN::from_array(
        &env,
        &[
            0x89, 0x9b, 0x28, 0x40, 0xed, 0x56, 0x36, 0xc5, 0x6d, 0xdc, 0x5f, 0x14, 0xb2, 0x39,
            0x75, 0xf7, 0x9f, 0x1b, 0xa2, 0x38, 0x8d, 0x26, 0x94, 0xe4, 0xc5, 0x6e, 0xcd, 0xdd,
            0xc9, 0x60, 0xe5, 0xef,
        ],
    );

    let address = Address::from_payload(&env, AddressPayload::AccountIdPublicKeyEd25519(payload));
    assert_eq!(
        address.to_string().to_string(),
        "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ"
    );
}
