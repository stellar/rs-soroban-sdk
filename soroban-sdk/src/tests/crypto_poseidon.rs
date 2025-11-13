use crate::{bytesn, vec, Env, Symbol, U256};

// This test matches barretenberg test case for hashing 4 inputs: https://github.com/AztecProtocol/aztec-packages/blob/b95e36c6c1a5a84ba488c720189102ecbb052d2c/barretenberg/cpp/src/barretenberg/crypto/poseidon2/poseidon2.test.cpp#L34
#[test]
fn test_poseidon2_hash() {
    let env = Env::default();

    // Input: 4 identical field elements
    let input_value = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x9a807b615c4d3e2fa0b1c2d3e4f56789fedcba9876543210abcdef0123456789
        )
        .into(),
    );
    let inputs = vec![
        &env,
        input_value.clone(),
        input_value.clone(),
        input_value.clone(),
        input_value,
    ];

    // Expected output from Aztec's implementation
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x2f43a0f83b51a6f5fc839dea0ecec74947637802a579fa9841930a25a0bcec11
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon2_hash(&inputs, field);

    assert_eq!(result, expected);
}

// This test case matches circom hash([1, 2]) with t=3: https://github.com/iden3/circomlib/blob/35e54ea21da3e8762557234298dbb553c175ea8d/test/poseidoncircuit.js#L47
#[test]
fn test_poseidon_hash_1_2() {
    let env = Env::default();

    // Input: [1, 2]
    let inputs = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000001
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000002
            )
            .into(),
        ),
    ];

    // Expected output: 7853200120776062878684798364095072458815029376092732009249414926327459813530
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x115cc0f5e7d690413df64c6b9662e9cf2a3617f2743245519e19607a4417189a
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon_hash(&inputs, field);

    assert_eq!(result, expected);
}

// This test case matches circom hash([3, 4]) with t=3: https://github.com/iden3/circomlib/blob/35e54ea21da3e8762557234298dbb553c175ea8d/test/poseidoncircuit.js#L57
#[test]
fn test_poseidon_hash_3_4() {
    let env = Env::default();

    // Input: [3, 4]
    let inputs = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000003
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000004
            )
            .into(),
        ),
    ];

    // Expected output: 14763215145315200506921711489642608356394854266165572616578112107564877678998
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x20a3af0435914ccd84b806164531b0cd36e37d4efb93efab76913a93e1f30996
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon_hash(&inputs, field);

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag4_m_1, get_rc4, ROUNDS_F, ROUNDS_P, SBOX_D, T,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: 4 identical field elements
    let input_value = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x9a807b615c4d3e2fa0b1c2d3e4f56789fedcba9876543210abcdef0123456789
        )
        .into(),
    );
    let input = vec![
        &env,
        input_value.clone(),
        input_value.clone(),
        input_value.clone(),
        input_value,
    ];

    // Get parameters
    let field = Symbol::new(&env, "BN254");
    let m_diag = get_mat_diag4_m_1(&env);
    let rc = get_rc4(&env);
    let t = T as u32;
    let d = SBOX_D;
    let rounds_f = ROUNDS_F;
    let rounds_p = ROUNDS_P;

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output (full state after permutation)
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x2bf1eaf87f7d27e8dc4056e9af975985bccc89077a21891d6c7b6ccce0631f95
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0c01fa1b8d0748becafbe452c0cb0231c38224ea824554c9362518eebdd5701f
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x018555a8eb50cf07f64b019ebaf3af3c925c93e631f3ecd455db07bbb52bbdd3
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0cbea457c91c22c6c31fd89afd2541efc2edf31736b9f721e823b2165c90fd41
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon_permutation() {
    use crate::crypto::poseidon_params::{get_mds3, get_rc3, ROUNDS_F, ROUNDS_P, SBOX_D, T};
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1, 2] (capacity=0, rate=[1,2])
    let input = vec![
        &env,
        U256::from_u32(&env, 0),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000001
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000002
            )
            .into(),
        ),
    ];

    // Get parameters
    let field = Symbol::new(&env, "BN254");
    let mds = get_mds3(&env);
    let rc = get_rc3(&env);
    let t = T as u32;
    let d = SBOX_D;
    let rounds_f = ROUNDS_F;
    let rounds_p = ROUNDS_P;

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result = hazmat.poseidon_permutation(&input, field, t, d, rounds_f, rounds_p, &mds, &rc);

    // Expected output[0] = 7853200120776062878684798364095072458815029376092732009249414926327459813530
    let expected_0 = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x115cc0f5e7d690413df64c6b9662e9cf2a3617f2743245519e19607a4417189a
        )
        .into(),
    );

    assert_eq!(result.len(), 3);
    assert_eq!(result.get_unchecked(0), expected_0);
}
