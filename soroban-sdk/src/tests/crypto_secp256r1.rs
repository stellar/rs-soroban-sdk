use crate::{bytesn, crypto::Hash, Env};

#[test]
fn test_verify_sig_ecdsa_secp256r1() {
    let env = Env::default();

    // Test vector copied and adapted from
    // https://csrc.nist.gov/groups/STM/cavp/documents/dss/186-3ecdsatestvectors.zip
    // `SigVer.rsp` section [P-256,SHA-256]
    let message_digest = Hash::from_bytes(bytesn!(
        &env,
        0xd1b8ef21eb4182ee270638061063a3f3c16c114e33937f69fb232cc833965a94
    ));
    let signature = bytesn!(
        &env,
        0xbf96b99aa49c705c910be33142017c642ff540c76349b9dab72f981fd9347f4f17c55095819089c2e03b9cd415abdf12444e323075d98f31920b9e0f57ec871c
    );
    let public_key = bytesn!(
        &env,
        0x04e424dc61d4bb3cb7ef4344a7f8957a0c5134e16f7a67c074f82e6e12f49abf3c970eed7aa2bc48651545949de1dddaf0127e5965ac85d1243d6f60e7dfaee927
    );

    env.crypto()
        .secp256r1_verify(&public_key, &message_digest, &signature)
}
