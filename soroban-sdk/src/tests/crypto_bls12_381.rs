use crate::{
    bytes, bytesn,
    crypto::bls12_381::{Bls12_381, Fp, Fp2, Fr, G1Affine, G2Affine},
    vec, Bytes, Env, Vec, U256,
};

#[test]
fn test_bls_g1() {
    let env = Env::default();
    let bls12_381 = Bls12_381::new(&env);
    const DST_G1: &str = "QUUX-V01-CS02-with-BLS12381G1_XMD:SHA-256_SSWU_RO_";
    let zero = G1Affine::from_bytes(bytesn!(&env, 0x400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
    let one = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1));

    // subgroup check
    assert!(bls12_381.g1_is_in_subgroup(&zero));
    assert!(bls12_381.g1_is_in_subgroup(&one));

    // add
    let res = bls12_381.g1_add(&zero, &one);
    assert_eq!(res, one);

    // checked_add
    let res = bls12_381.g1_checked_add(&zero, &one);
    assert!(res.is_some_and(|v| v == one));

    // mul
    let res = bls12_381.g1_mul(&one, &U256::from_u32(&env, 0).into());
    assert_eq!(res, zero);

    // msm
    let vp: Vec<G1Affine> = vec![&env, one.clone(), one.clone()];
    let vs: Vec<Fr> = vec![
        &env,
        U256::from_u32(&env, 1).into(),
        U256::from_u32(&env, 0).into(),
    ];
    let res = bls12_381.g1_msm(vp, vs);
    assert_eq!(res, one);

    // map to curve (test case from https://datatracker.ietf.org/doc/html/rfc9380)
    let dst = Bytes::from_slice(&env, DST_G1.as_bytes());
    let fp = Fp::from_bytes(bytesn!(&env, 0x0d921c33f2bad966478a03ca35d05719bdf92d347557ea166e5bba579eea9b83e9afa5c088573c2281410369fbd32951));
    let expected = G1Affine::from_bytes(bytesn!(&env, 0x125435adce8e1cbd1c803e7123f45392dc6e326d292499c2c45c5865985fd74fe8f042ecdeeec5ecac80680d04317d800e8828948c989126595ee30e4f7c931cbd6f4570735624fd25aef2fa41d3f79cfb4b4ee7b7e55a8ce013af2a5ba20bf2));
    let res = bls12_381.map_fp_to_g1(&fp);
    assert_eq!(res, expected);

    // hash msg to curve (test case from https://datatracker.ietf.org/doc/html/rfc9380)
    let msg = Bytes::from_slice(&env, "abc".as_bytes());
    let expected = G1Affine::from_bytes(bytesn!(&env, 0x03567bc5ef9c690c2ab2ecdf6a96ef1c139cc0b2f284dca0a9a7943388a49a3aee664ba5379a7655d3c68900be2f69030b9c15f3fe6e5cf4211f346271d7b01c8f3b28be689c8429c85b67af215533311f0b8dfaaa154fa6b88176c229f2885d));
    let res = bls12_381.hash_to_g1(&msg, &dst);
    assert_eq!(res, expected);
}

#[test]
fn test_bls_g2() {
    let env = Env::default();
    let bls12_381 = Bls12_381::new(&env);
    const DST_G2: &str = "QUUX-V01-CS02-with-BLS12381G2_XMD:SHA-256_SSWU_RO_";
    let zero = G2Affine::from_bytes(bytesn!(&env, 0x400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
    let one = G2Affine::from_bytes(bytesn!(&env, 0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb80606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801));

    // subgroup check
    assert!(bls12_381.g2_is_in_subgroup(&zero));
    assert!(bls12_381.g2_is_in_subgroup(&one));

    // add
    let res = bls12_381.g2_add(&zero, &one);
    assert_eq!(res, one);

    // checked_add
    let res = bls12_381.g2_checked_add(&zero, &one);
    assert!(res.is_some_and(|v| v == one));

    // mul
    let res = bls12_381.g2_mul(&one, &U256::from_u32(&env, 0).into());
    assert_eq!(res, zero);

    // msm
    let vp: Vec<G2Affine> = vec![&env, one.clone(), one.clone()];
    let vs: Vec<Fr> = vec![
        &env,
        Fr::from_bytes(bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000001
        )),
        Fr::from_bytes(bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000000
        )),
    ];
    let res = bls12_381.g2_msm(vp.clone(), vs);
    assert_eq!(res, one);

    // map to curve (test case from https://datatracker.ietf.org/doc/html/rfc9380)
    let dst = Bytes::from_slice(&env, DST_G2.as_bytes());
    let fp2 = Fp2::from_bytes(bytesn!(&env, 0x01c8067bf4c0ba709aa8b9abc3d1cef589a4758e09ef53732d670fd8739a7274e111ba2fcaa71b3d33df2a3a0c8529dd15f7c0aa8f6b296ab5ff9c2c7581ade64f4ee6f1bf18f55179ff44a2cf355fa53dd2a2158c5ecb17d7c52f63e7195771));
    let expected = G2Affine::from_bytes(bytesn!(&env, 0x05d8a724db78e570e34100c0bc4a5fa84ad5839359b40398151f37cff5a51de945c563463c9efbdda569850ee5a53e7712b2e525281b5f4d2276954e84ac4f42cf4e13b6ac4228624e17760faf94ce5706d53f0ca1952f1c5ef75239aeed55ad04bbe48bfd5814648d0b9e30f0717b34015d45a861425fabc1ee06fdfce36384ae2c808185e693ae97dcde118f34de4102eacdc556d0bdb5d18d22f23dcb086dd106cad713777c7e6407943edbe0b3d1efe391eedf11e977fac55f9b94f2489c));
    let res = bls12_381.map_fp2_to_g2(&fp2);
    assert_eq!(res, expected);

    // hash msg to curve (test case from https://datatracker.ietf.org/doc/html/rfc9380)
    let msg = Bytes::from_slice(&env, "abc".as_bytes());
    let expected = G2Affine::from_bytes(bytesn!(&env, 0x139cddbccdc5e91b9623efd38c49f81a6f83f175e80b06fc374de9eb4b41dfe4ca3a230ed250fbe3a2acf73a41177fd802c2d18e033b960562aae3cab37a27ce00d80ccd5ba4b7fe0e7a210245129dbec7780ccc7954725f4168aff2787776e600aa65dae3c8d732d10ecd2c50f8a1baf3001578f71c694e03866e9f3d49ac1e1ce70dd94a733534f106d4cec0eddd161787327b68159716a37440985269cf584bcb1e621d3a7202be6ea05c4cfe244aeb197642555a0645fb87bf7466b2ba48));
    let res = bls12_381.hash_to_g2(&msg, &dst);
    assert_eq!(res, expected);
}

#[test]
fn test_pairing() {
    let env = Env::default();
    let bls12_381 = Bls12_381::new(&env);
    // test case from one of the ethereum tests "verify_valid_case_195246ee3bd3b6ec.json"
    const DST_ETHEREUM: &str = "BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";
    let dst = Bytes::from_slice(&env, DST_ETHEREUM.as_bytes());
    let neg_g1 = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca));
    let pk = G1Affine::from_bytes(bytesn!(&env, 0x153d21a4cfd562c469cc81514d4ce5a6b577d8403d32a394dc265dd190b47fa9f829fdd7963afdf972e5e77854051f6f14e22fd412a826a329fb40cbdc01b5e4e2f931ed84d8e45932ec62a039f9d61a9dbf2c6eedc5db6fa585b6e0bdde100c));
    let msg = bytes!(
        &env,
        0xabababababababababababababababababababababababababababababababab
    );
    let msg = bls12_381.hash_to_g2(&msg, &dst);
    let sig = G2Affine::from_bytes(bytesn!(&env, 0x0e82747ddeefe4fd64cf9cedb9b04ae3e8a43420cd255e3c7cd06a8d88b7c7f8638543719981c5d16fa3527c468c25f0026704a6951bde891360c7e8d12ddee0559004ccdbe6046b55bae1b257ee97f7cdb955773d7cf29adf3ccbb9975e4eb915e60d5b66a43e074b801a07df931a17505048f7f96dc80f857b638e505868dc008cc9c26ed5b8495e9c181b67dc4c2317d9d447337a9cc6d2956b9c6dd7c23c0bfb73855e902061bcb9cb9d40e43c38140091e638ffcffc7261366018900047));

    let vp1 = vec![&env, pk, neg_g1];
    let vp2 = vec![&env, msg, sig];
    assert!(bls12_381.pairing_check(vp1, vp2))
}

#[test]
fn test_fr_arithmetic() {
    let env = Env::default();
    let bls12_381 = Bls12_381::new(&env);
    let modulus = U256::from_be_bytes(
        &env,
        &bytes!(
            &env,
            0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
        ),
    );
    assert_eq!(
        bls12_381.fr_add(
            &U256::from_u32(&env, 2).into(),
            &U256::from_u32(&env, 3).into()
        ),
        U256::from_u32(&env, 5).into()
    );
    assert_eq!(
        bls12_381.fr_sub(
            &U256::from_u32(&env, 2).into(),
            &U256::from_u32(&env, 3).into()
        ),
        modulus.sub(&U256::from_u32(&env, 1)).into()
    );
    assert_eq!(
        bls12_381.fr_mul(
            &U256::from_u32(&env, 2).into(),
            &U256::from_u32(&env, 3).into()
        ),
        U256::from_u32(&env, 6).into()
    );
    assert_eq!(
        bls12_381.fr_pow(&U256::from_u32(&env, 5).into(), 2),
        U256::from_u32(&env, 25).into()
    );
    let inverse_13 = bls12_381.fr_inv(&U256::from_u32(&env, 13).into());
    assert_eq!(
        bls12_381.fr_mul(&inverse_13, &U256::from_u32(&env, 13).into()),
        U256::from_u32(&env, 1).into()
    );
}
