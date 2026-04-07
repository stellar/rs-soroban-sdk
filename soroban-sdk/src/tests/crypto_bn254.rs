use crate::{self as soroban_sdk};
use soroban_sdk::{
    bytes, bytesn,
    crypto::bn254::{Bn254, Bn254G1Affine, Fr},
    vec, Env, Vec, U256,
};

#[test]
fn test_g1_is_on_curve() {
    let env = Env::default();
    let bn254 = Bn254::new(&env);
    // infinity (64 zero bytes)
    let zero = Bn254G1Affine::from_bytes(bytesn!(
        &env,
        0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    ));
    // generator point (1, 2) - two 32-byte big-endian values
    let one = Bn254G1Affine::from_bytes(bytesn!(
        &env,
        0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002
    ));
    // (1, 1) is not on the curve: 1 != 1 + 3
    let not_on_curve = Bn254G1Affine::from_bytes(bytesn!(
        &env,
        0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001
    ));
    assert!(bn254.g1_is_on_curve(&zero));
    assert!(bn254.g1_is_on_curve(&one));
    assert!(!bn254.g1_is_on_curve(&not_on_curve));
}

#[test]
fn test_g1_msm() {
    let env = Env::default();
    let bn254 = Bn254::new(&env);
    let one = Bn254G1Affine::from_bytes(bytesn!(
        &env,
        0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002
    ));
    let zero = Bn254G1Affine::from_bytes(bytesn!(
        &env,
        0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    ));

    // 1*G + 0*G = G
    let vp: Vec<Bn254G1Affine> = vec![&env, one.clone(), one.clone()];
    let vs: Vec<Fr> = vec![
        &env,
        U256::from_u32(&env, 1).into(),
        U256::from_u32(&env, 0).into(),
    ];
    let res = bn254.g1_msm(vp, vs);
    assert_eq!(res, one);

    // 0*G + 0*G = 0
    let vp: Vec<Bn254G1Affine> = vec![&env, one.clone(), one.clone()];
    let vs: Vec<Fr> = vec![
        &env,
        U256::from_u32(&env, 0).into(),
        U256::from_u32(&env, 0).into(),
    ];
    let res = bn254.g1_msm(vp, vs);
    assert_eq!(res, zero);
}

#[test]
#[should_panic(expected = "HostError: Error(Crypto, InvalidInput)")]
fn test_g1_msm_mismatched_lengths() {
    let env = Env::default();
    let bn254 = Bn254::new(&env);
    let one = Bn254G1Affine::from_bytes(bytesn!(
        &env,
        0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002
    ));
    let vp: Vec<Bn254G1Affine> = vec![&env, one.clone(), one.clone()];
    let vs: Vec<Fr> = vec![&env, U256::from_u32(&env, 1).into()];
    bn254.g1_msm(vp, vs);
}

#[test]
#[should_panic(expected = "HostError: Error(Crypto, InvalidInput)")]
fn test_g1_msm_empty_vectors() {
    let env = Env::default();
    let bn254 = Bn254::new(&env);
    let vp: Vec<Bn254G1Affine> = vec![&env];
    let vs: Vec<Fr> = vec![&env];
    bn254.g1_msm(vp, vs);
}

#[test]
fn test_fr_arithmetic() {
    let env = Env::default();
    let bn254 = Bn254::new(&env);

    // BN254 scalar field modulus
    let modulus = U256::from_be_bytes(
        &env,
        &bytes!(
            &env,
            0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
        ),
    );

    // add
    assert_eq!(
        bn254.fr_add(
            &U256::from_u32(&env, 2).into(),
            &U256::from_u32(&env, 3).into()
        ),
        U256::from_u32(&env, 5).into()
    );

    // sub (wraps around modulus)
    assert_eq!(
        bn254.fr_sub(
            &U256::from_u32(&env, 2).into(),
            &U256::from_u32(&env, 3).into()
        ),
        modulus.sub(&U256::from_u32(&env, 1)).into()
    );

    // mul
    assert_eq!(
        bn254.fr_mul(
            &U256::from_u32(&env, 2).into(),
            &U256::from_u32(&env, 3).into()
        ),
        U256::from_u32(&env, 6).into()
    );

    // pow
    assert_eq!(
        bn254.fr_pow(&U256::from_u32(&env, 5).into(), 2),
        U256::from_u32(&env, 25).into()
    );

    // inv: inv(13) * 13 == 1
    let inverse_13 = bn254.fr_inv(&U256::from_u32(&env, 13).into());
    assert_eq!(
        bn254.fr_mul(&inverse_13, &U256::from_u32(&env, 13).into()),
        U256::from_u32(&env, 1).into()
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Crypto, InvalidInput)")]
fn test_fr_inv_zero() {
    let env = Env::default();
    let bn254 = Bn254::new(&env);
    bn254.fr_inv(&U256::from_u32(&env, 0).into());
}

#[test]
fn test_fr_operator_traits() {
    let env = Env::default();

    let a: Fr = U256::from_u32(&env, 10).into();
    let b: Fr = U256::from_u32(&env, 3).into();

    // Add
    assert_eq!(a.clone() + b.clone(), U256::from_u32(&env, 13).into());

    // Sub
    assert_eq!(a.clone() - b.clone(), U256::from_u32(&env, 7).into());

    // Mul
    assert_eq!(a.clone() * b.clone(), U256::from_u32(&env, 30).into());

    // pow convenience method
    assert_eq!(b.clone().pow(3), U256::from_u32(&env, 27).into());

    // inv convenience method: inv(a) * a == 1
    let inv_a = a.clone().inv();
    assert_eq!(inv_a * a, U256::from_u32(&env, 1).into());
}
