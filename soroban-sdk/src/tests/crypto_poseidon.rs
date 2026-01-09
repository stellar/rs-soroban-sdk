use crate::crypto::CryptoHazmat;
use crate::{bytesn, vec, Env, Symbol, U256};

// This test uses dummy MDS matrix and round constants with a minimal number of
// rounds (t=2, rounds_f=2, rounds_p=1) as a sanity check against the
// poseidon_permutation host function.
#[test]
fn test_poseidon_permutation() {
    let env = Env::default();

    let mds = vec![
        &env,
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x066f6f85d6f68a85ec10345351a23a3aaf07f38af8c952a7bceca70bd2af7ad5
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x2b9d4b4110c9ae997782e1509b1d0fdb20a7c02bbd8bea7305462b9f8125b1e8
                )
                .into(),
            ),
        ],
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x0cc57cdbb08507d62bf67a4493cc262fb6c09d557013fff1f573f431221f8ff9
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x1274e649a32ed355a31a6ed69724e1adade857e86eb5c3a121bcd147943203c8
                )
                .into(),
            ),
        ],
    ];

    let rc = vec![
        &env,
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x09c46e9ec68e9bd4fe1faaba294cba38a71aa177534cdd1b6c7dc0dbd0abd7a7
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x0c0356530896eec42a97ed937f3135cfc5142b3ae405b8343c1d83ffa604cb81
                )
                .into(),
            ),
        ],
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x1e28a1d935698ad1142e51182bb54cf4a00ea5aabd6268bd317ea977cc154a30
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x27af2d831a9d2748080965db30e298e40e5757c3e008db964cf9e2b12b91251f
                )
                .into(),
            ),
        ],
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x1e6f11ce60fc8f513a6a3cfe16ae175a41291462f214cd0879aaf43545b74e03
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x2a67384d3bbd5e438541819cb681f0be04462ed14c3613d8f719206268d142d3
                )
                .into(),
            ),
        ],
    ];

    let input1 = vec![&env, U256::from_u32(&env, 0), U256::from_u32(&env, 1)];

    // Parameters: t=2, d=5 (sbox exponent), rounds_f=2 (full, must be even), rounds_p=1 (partial)
    let field = Symbol::new(&env, "BN254");
    let t = 2;
    let d = 5;
    let rounds_f = 2;
    let rounds_p = 1;

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result1 =
        hazmat.poseidon_permutation(&input1, field.clone(), t, d, rounds_f, rounds_p, &mds, &rc);
    assert_eq!(result1.len(), 2);

    let input2 = vec![&env, U256::from_u32(&env, 0), U256::from_u32(&env, 2)];
    let result2 = hazmat.poseidon_permutation(&input2, field, t, d, rounds_f, rounds_p, &mds, &rc);

    // Different inputs should produce different outputs
    assert_ne!(result1, result2);
}

// This test uses dummy internal matrix diagonal and round constants with a
// minimal number of rounds (t=2, rounds_f=2, rounds_p=1) as a sanity check
// against the poseidon2_permutation host function.
#[test]
fn test_poseidon2_permutation() {
    let env = Env::default();

    let m_diag = vec![
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

    let rc = vec![
        &env,
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x09c46e9ec68e9bd4fe1faaba294cba38a71aa177534cdd1b6c7dc0dbd0abd7a7
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x0c0356530896eec42a97ed937f3135cfc5142b3ae405b8343c1d83ffa604cb81
                )
                .into(),
            ),
        ],
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x1e28a1d935698ad1142e51182bb54cf4a00ea5aabd6268bd317ea977cc154a30
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x27af2d831a9d2748080965db30e298e40e5757c3e008db964cf9e2b12b91251f
                )
                .into(),
            ),
        ],
        vec![
            &env,
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x1e6f11ce60fc8f513a6a3cfe16ae175a41291462f214cd0879aaf43545b74e03
                )
                .into(),
            ),
            U256::from_be_bytes(
                &env,
                &bytesn!(
                    &env,
                    0x2a67384d3bbd5e438541819cb681f0be04462ed14c3613d8f719206268d142d3
                )
                .into(),
            ),
        ],
    ];

    let input1 = vec![&env, U256::from_u32(&env, 0), U256::from_u32(&env, 1)];

    // Parameters: t=2, d=5 (sbox exponent), rounds_f=2 (full, must be even), rounds_p=1 (partial)
    let field = Symbol::new(&env, "BN254");
    let t = 2;
    let d = 5;
    let rounds_f = 2;
    let rounds_p = 1;

    // Call the permutation
    let hazmat = CryptoHazmat::new(&env);
    let result1 = hazmat.poseidon2_permutation(
        &input1,
        field.clone(),
        t,
        d,
        rounds_f,
        rounds_p,
        &m_diag,
        &rc,
    );
    assert_eq!(result1.len(), 2);

    let input2 = vec![&env, U256::from_u32(&env, 0), U256::from_u32(&env, 2)];
    let result2 =
        hazmat.poseidon2_permutation(&input2, field, t, d, rounds_f, rounds_p, &m_diag, &rc);

    // Different inputs should produce different outputs
    assert_ne!(result1, result2);
}
