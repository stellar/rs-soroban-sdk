use crate::{bytes, bytesn, Env};

#[test]
fn test_verify_sig_ed25519() {
    let env = Env::default();

    // From https://datatracker.ietf.org/doc/html/rfc8032#section-7.1 TEST 2
    let public_key = bytesn!(
        &env,
        0x3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c
    );
    let signature = bytesn!(
        &env,
        0x92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00
    );
    let message = bytes!(&env, 0x72);

    env.crypto()
        .ed25519_verify(&public_key, &message, &signature);
}

#[test]
#[should_panic(expected = "HostError: Error(Crypto, InvalidInput)")]
fn test_verify_sig_ed25519_invalid_sig() {
    let env = Env::default();

    // From https://datatracker.ietf.org/doc/html/rfc8032#section-7.1 TEST 2, message modified from 0x72 to 0x73
    let public_key = bytesn!(
        &env,
        0x3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c
    )
    .try_into()
    .unwrap();
    let signature = bytesn!(
        &env,
        0x92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00
    );
    let message = bytes!(&env, 0x73);

    env.crypto()
        .ed25519_verify(&public_key, &message, &signature);
}
