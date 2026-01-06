use crate::crypto::poseidon_params::{get_rounds_f_bn254, get_rounds_p_bn254};
use crate::{bytesn, vec, Env, Symbol, U256};

// Poseidon tests

// This test case matches circom hash([1, 2]) with t=3: https://github.com/iden3/circomlib/blob/35e54ea21da3e8762557234298dbb553c175ea8d/test/poseidoncircuit.js#L47
#[test]
fn test_poseidon_bn254_hash_1_2() {
    let env = Env::default();

    // Input: [1, 2]
    let inputs = [
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
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches circom hash([3, 4]) with t=3: https://github.com/iden3/circomlib/blob/35e54ea21da3e8762557234298dbb553c175ea8d/test/poseidoncircuit.js#L57
#[test]
fn test_poseidon_bn254_hash_3_4() {
    let env = Env::default();

    // Input: [3, 4]
    let inputs = [
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
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches circom hash([1]) with t=2 (N=1)
#[test]
fn test_poseidon_bn254_hash_1() {
    let env = Env::default();

    // Input: [1]
    let inputs = [U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000001
        )
        .into(),
    )];

    // Expected output from circomlibjs
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x29176100eaa962bdc1fe6c654d6a3c130e96a4d1168b33848b897dc502820133
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches circom hash([1, 2, 3]) with t=4 (N=3)
#[test]
fn test_poseidon_bn254_hash_1_2_3() {
    let env = Env::default();

    // Input: [1, 2, 3]
    let inputs = [
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
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000003
            )
            .into(),
        ),
    ];

    // Expected output from circomlibjs
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x0e7732d89e6939c0ff03d5e58dab6302f3230e269dc5b968f725df34ab36d732
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches circom hash([1, 2, 3, 4]) with t=5 (N=4)
#[test]
fn test_poseidon_bn254_hash_1_2_3_4() {
    let env = Env::default();

    // Input: [1, 2, 3, 4]
    let inputs = [
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

    // Expected output from circomlibjs
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x299c867db6c1fdd79dcefa40e4510b9837e60ebb1ce0663dbaa525df65250465
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches circom hash([1, 2, 3, 4, 5]) with t=6 (N=5)
#[test]
fn test_poseidon_bn254_hash_1_2_3_4_5() {
    let env = Env::default();

    // Input: [1, 2, 3, 4, 5]
    let inputs = [
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
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000005
            )
            .into(),
        ),
    ];

    // Expected output from circomlibjs
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x0dab9449e4a1398a15224c0b15a49d598b2174d305a316c918125f8feeb123c0
        )
        .into(),
    );

    let field = Symbol::new(&env, "BN254");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon_bls12_381_hash_1_2() {
    let env = Env::default();

    // Input: [1, 2]
    let inputs = [
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

    // Expected output
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x3fb8310b0e962b75bffec5f9cfcbf3f965a7b1d2dcac8d95ccb13d434e08e5fa
        )
        .into(),
    );

    let field = Symbol::new(&env, "BLS12_381");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches poseidon-bls12381-circom hash([1]) with t=2 (N=1)
#[test]
fn test_poseidon_bls12_381_hash_1() {
    let env = Env::default();

    // Input: [1]
    let inputs = [U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x0000000000000000000000000000000000000000000000000000000000000001
        )
        .into(),
    )];

    // Expected output from poseidon-bls12381-circom
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x49a66f6b01dbc6440d1a5f920e027b94429916f2c821a920cf6203ad3de56cea
        )
        .into(),
    );

    let field = Symbol::new(&env, "BLS12_381");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches poseidon-bls12381-circom hash([1, 2, 3]) with t=4 (N=3)
#[test]
fn test_poseidon_bls12_381_hash_1_2_3() {
    let env = Env::default();

    // Input: [1, 2, 3]
    let inputs = [
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
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000003
            )
            .into(),
        ),
    ];

    // Expected output from poseidon-bls12381-circom
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x5ad8bcfa9754b5bc043cc74dea65ae15e3fdb0c2295970aaacfc116c802d9895
        )
        .into(),
    );

    let field = Symbol::new(&env, "BLS12_381");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches poseidon-bls12381-circom hash([1, 2, 3, 4]) with t=5 (N=4)
#[test]
fn test_poseidon_bls12_381_hash_1_2_3_4() {
    let env = Env::default();

    // Input: [1, 2, 3, 4]
    let inputs = [
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

    // Expected output from poseidon-bls12381-circom
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x2ebfd520dd8b5f26dfdc74e4ca0861495e119e6b43f7df3369dbb2f190cd5866
        )
        .into(),
    );

    let field = Symbol::new(&env, "BLS12_381");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

// This test case matches poseidon-bls12381-circom hash([1, 2, 3, 4, 5]) with t=6 (N=5)
#[test]
fn test_poseidon_bls12_381_hash_1_2_3_4_5() {
    let env = Env::default();

    // Input: [1, 2, 3, 4, 5]
    let inputs = [
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
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000005
            )
            .into(),
        ),
    ];

    // Expected output from poseidon-bls12381-circom
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x2c0507691a38c8c109572be56878c10a34a741fafe3e6d04c3d1e0be60ddd781
        )
        .into(),
    );

    let field = Symbol::new(&env, "BLS12_381");
    let result = env.crypto().poseidon_hash(field, &inputs);

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon_permutation() {
    use crate::crypto::poseidon_params::{get_mds_bn254, get_rc_bn254, SBOX_D, T};
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
    let t = T as u32;
    let mds = get_mds_bn254(&env, t);
    let rc = get_rc_bn254(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f_bn254(t);
    let rounds_p = get_rounds_p_bn254(t);

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

// Poseidon2 tests

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
    let inputs = [
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
    let result = env.crypto().poseidon2_hash(field, &inputs);

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bn254, get_rc_bn254, get_rounds_f, get_rounds_p, SBOX_D,
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
    let t = 4;
    let m_diag = get_mat_diag_m_1_bn254(&env, t);
    let rc = get_rc_bn254(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

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
fn test_poseidon2_permutation_bn254_t4() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bn254, get_rc_bn254, get_rounds_f, get_rounds_p, SBOX_D,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1, 2, 3]
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
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000003
            )
            .into(),
        ),
    ];

    // Get parameters
    let field = Symbol::new(&env, "BN254");
    let t = 4u32;
    let m_diag = get_mat_diag_m_1_bn254(&env, t);
    let rc = get_rc_bn254(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x239b62e7db98aa3a2a8f6a0d2fa1709e7a35959aa6c7034814d9daa90cbac662
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x04cbb44c61d928ed06808456bf758cbf0c18d1e15a7b6dbc8245fa7515d5e3cb
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x2e11c5cff2a22c64d01304b778d78f6998eff1ab73163a35603f54794c30847a
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation_bls12_381_t4() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bls12_381, get_rc_bls12_381, get_rounds_f, get_rounds_p, SBOX_D,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1, 2, 3]
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
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000003
            )
            .into(),
        ),
    ];

    // Get parameters
    let field = Symbol::new(&env, "BLS12_381");
    let t = 4u32;
    let m_diag = get_mat_diag_m_1_bls12_381(&env, t);
    let rc = get_rc_bls12_381(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x28ff6c4edf9768c08ae26290487e93449cc8bc155fc2fad92a344adceb3ada6d
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0e56f2b6fad25075aa93560185b70e2b180ed7e269159c507c288b6747a0db2d
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x6d8196f28da6006bb89b3df94600acdc03d0ba7c2b0f3f4409a54c1db6bf30d0
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x07cfb49540ee456cce38b8a7d1a930a57ffc6660737f6589ef184c5e15334e36
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation_bn254_t2() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bn254, get_rc_bn254, get_rounds_f, get_rounds_p, SBOX_D,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1]
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
    ];

    // Get parameters
    let field = Symbol::new(&env, "BN254");
    let t = 2u32;
    let m_diag = get_mat_diag_m_1_bn254(&env, t);
    let rc = get_rc_bn254(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x1d01e56f49579cec72319e145f06f6177f6c5253206e78c2689781452a31878b
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0d189ec589c41b8cffa88cfc523618a055abe8192c70f75aa72fc514560f6c61
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation_bn254_t3() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bn254, get_rc_bn254, get_rounds_f, get_rounds_p, SBOX_D,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1, 2]
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
    let t = 3u32;
    let m_diag = get_mat_diag_m_1_bn254(&env, t);
    let rc = get_rc_bn254(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0bb61d24daca55eebcb1929a82650f328134334da98ea4f847f760054f4a3033
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x303b6f7c86d043bfcbcc80214f26a30277a15d3f74ca654992defe7ff8d03570
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x1ed25194542b12eef8617361c3ba7c52e660b145994427cc86296242cf766ec8
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation_bls12_381_t2() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bls12_381, get_rc_bls12_381, get_rounds_f, get_rounds_p, SBOX_D,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1]
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
    ];

    // Get parameters
    let field = Symbol::new(&env, "BLS12_381");
    let t = 2u32;
    let m_diag = get_mat_diag_m_1_bls12_381(&env, t);
    let rc = get_rc_bls12_381(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x73c46dd530e248a87b61d19e67fa1b4ed30fc3d09f16531fe189fb945a15ce4e
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x1f0e305ee21c9366d5793b80251405032a3fee32b9dd0b5f4578262891b043b4
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_poseidon2_permutation_bls12_381_t3() {
    use crate::crypto::poseidon2_params::{
        get_mat_diag_m_1_bls12_381, get_rc_bls12_381, get_rounds_f, get_rounds_p, SBOX_D,
    };
    use crate::crypto::CryptoHazmat;

    let env = Env::default();

    // Input: [0, 1, 2]
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
    let field = Symbol::new(&env, "BLS12_381");
    let t = 3u32;
    let m_diag = get_mat_diag_m_1_bls12_381(&env, t);
    let rc = get_rc_bls12_381(&env, t);
    let d = SBOX_D;
    let rounds_f = get_rounds_f(t);
    let rounds_p = get_rounds_p(t);

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result =
        hazmat.poseidon2_permutation(&input, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Expected output
    let expected = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x1b152349b1950b6a8ca75ee4407b6e26ca5cca5650534e56ef3fd45761fbf5f0
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x4c5793c87d51bdc2c08a32108437dc0000bd0275868f09ebc5f36919af5b3891
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x1fc8ed171e67902ca49863159fe5ba6325318843d13976143b8125f08b50dc6b
            )
            .into(),
        ),
    ];

    assert_eq!(result, expected);
}
