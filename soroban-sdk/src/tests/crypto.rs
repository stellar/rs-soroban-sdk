use crate::{self as soroban_sdk};
use crate::{bytes, bytesn, env::internal::U32Val, Bytes, BytesN, Env, IntoVal, Val};
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct TestCryptoContract;

#[contractimpl]
impl TestCryptoContract {
    pub fn keccak256(env: Env, bytes: Bytes) -> BytesN<32> {
        env.crypto().keccak256(&bytes)
    }

    pub fn recover_key_ecdsa_secp256k1(
        env: Env,
        message: Bytes,
        signature: BytesN<64>,
        recovery_id: U32Val,
    ) -> Bytes {
        env.crypto()
            .recover_key_ecdsa_secp256k1(&message, &signature, recovery_id)
    }
}

#[test]
fn test_keccak256() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);

    let bytes = b"test vector for soroban".into_val(&env);

    assert_eq!(
        client.keccak256(&bytes),
        bytesn!(
            &env,
            0x352fe2eaddf44eb02eb3eab1f8d6ff4ba426df4f1734b1e3f210d621ee8853d9
        )
    );
}

#[test]
fn test_recover_key_ecdsa_secp256k1() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TestCryptoContract);
    let client = TestCryptoContractClient::new(&env, &contract_id);

    // From ethereum: https://github.com/ethereum/go-ethereum/blob/master/crypto/secp256k1/secp256_test.go

    let public_key = bytesn!(
        &env,
        0x04e32df42865e97135acfb65f3bae71bdc86f4d49150ad6a440b6f15878109880a0a2b2667f7e725ceea70c673093bf67663e0312623c8e091b13cf2c0f11ef652
    )
    .try_into()
    .unwrap();
    let signature = bytesn!(
        &env,
        0x90f27b8b488db00b00606796d2987f6a5f59ae62ea05effe84fef5b8b0e549984a691139ad57a3f0b906637673aa2f63d1f55cb1a69199d4009eea23ceaddc93
    );
    let message_digest = bytes!(
        &env,
        0xce0677bb30baa8cf067c88db9811f4333d131bf8bcf12fe7065d211dce971008
    );
    let recovery_id = Val::from_u32(1);
    assert_eq!(
        client.recover_key_ecdsa_secp256k1(&message_digest, &signature, &recovery_id),
        public_key
    );
}
