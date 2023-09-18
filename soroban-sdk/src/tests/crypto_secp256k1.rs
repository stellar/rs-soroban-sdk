use crate::{bytesn, Env};

#[test]
fn test_recover_key_ecdsa_secp256k1() {
    let env = Env::default();

    // From: https://github.com/ethereum/go-ethereum/blob/90d5bd85bcf2919ac2735a47fde675213348a0a6/crypto/secp256k1/secp256_test.go#L204-L217
    let message_digest = bytesn!(
        &env,
        0xce0677bb30baa8cf067c88db9811f4333d131bf8bcf12fe7065d211dce971008
    );
    let signature = bytesn!(
        &env,
        0x90f27b8b488db00b00606796d2987f6a5f59ae62ea05effe84fef5b8b0e549984a691139ad57a3f0b906637673aa2f63d1f55cb1a69199d4009eea23ceaddc93
    );
    let recovery_id = 1;
    let expected_public_key = bytesn!(
        &env,
        0x04e32df42865e97135acfb65f3bae71bdc86f4d49150ad6a440b6f15878109880a0a2b2667f7e725ceea70c673093bf67663e0312623c8e091b13cf2c0f11ef652
    );
    assert_eq!(
        env.crypto()
            .secp256k1_recover(&message_digest, &signature, recovery_id),
        expected_public_key
    );
}
