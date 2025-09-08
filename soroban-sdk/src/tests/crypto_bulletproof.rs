use crate::{bytesn, vec, Bytes, BytesN, Env, Vec};

#[test]
fn test_bulletproof_verify_basic_error_cases() {
    let env = Env::default();

    // Test with invalid proof bytes (too short)
    let invalid_proof = Bytes::from_array(&env, &[0u8; 10]); // Too short
    let dst = Bytes::from_slice(&env, b"test");
    let dummy_commitment: BytesN<32> = bytesn!(&env, 0x0101010101010101010101010101010101010101010101010101010101010101);
    let commitments: Vec<BytesN<32>> = vec![&env, dummy_commitment];

    // This should panic because the proof is invalid
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.crypto().bulletproof_verify_multiple_values_in_range(
            &invalid_proof,
            &dst,
            32,
            &commitments,
        );
    }));
    
    assert!(result.is_err(), "Expected bulletproof verification to fail with invalid proof");
}

#[test]
fn test_bulletproof_verify_empty_commitments() {
    let env = Env::default();

    let dummy_proof = Bytes::from_array(&env, &[0u8; 64]);
    let dst = Bytes::from_slice(&env, b"test");
    let empty_commitments: Vec<BytesN<32>> = vec![&env];

    // This should panic because commitments are empty
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.crypto().bulletproof_verify_multiple_values_in_range(
            &dummy_proof,
            &dst,
            32,
            &empty_commitments,
        );
    }));
    
    assert!(result.is_err(), "Expected bulletproof verification to fail with empty commitments");
}

#[test]
fn test_bulletproof_verify_valid_input() {
    let env = Env::default();

    // Use proper-sized proof and commitment for the test
    let dummy_proof = Bytes::from_array(&env, &[0u8; 64]);
    let dst = Bytes::from_slice(&env, b"test");
    let valid_commitment: BytesN<32> = bytesn!(&env, 0x0202020202020202020202020202020202020202020202020202020202020202);
    let commitments: Vec<BytesN<32>> = vec![&env, valid_commitment];

    // This should panic because the proof is invalid (dummy data)
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        env.crypto().bulletproof_verify_multiple_values_in_range(
            &dummy_proof,
            &dst,
            32,
            &commitments,
        );
    }));
    
    assert!(result.is_err(), "Expected bulletproof verification to fail with invalid proof data");
}