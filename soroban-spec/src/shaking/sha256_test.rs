//! Tests for the const SHA-256 in `sha256.rs`.
//!
//! These live in their own file rather than in `sha256.rs` because
//! `soroban-spec/fuzz` includes `sha256.rs` by path, and that file must stay
//! free of anything beyond `core`.

use super::sha256::sha256;
use std::vec::Vec;

fn unhex(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}

/// The const SHA-256 must produce known digests, including across block
/// boundaries where the padding lands in a different block to the message.
/// Each input is the bytes `0, 1, 2, ...` wrapping at 251, and each expected
/// digest was computed independently of this crate.
#[test]
fn sha256_matches_known_digests() {
    // Lengths either side of the 64-byte block size and the 55/56-byte
    // boundary where the length field no longer fits in the final block.
    let cases: &[(&str, &str)] = &[
        // 0 bytes.
        (
            "",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        // 1 byte.
        (
            "00",
            "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d",
        ),
        // 54 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435",
            ),
            "675f28acc0b90a72d1c3a570fe83ac565555db358cf01826dc8eefb2bf7ca0f3",
        ),
        // 55 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "30313233343536",
            ),
            "463eb28e72f82e0a96c0a4cc53690c571281131f672aa229e0d45ae59b598b59",
        ),
        // 56 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "3031323334353637",
            ),
            "da2ae4d6b36748f2a318f23e7ab1dfdf45acdc9d049bd80e59de82a60895f562",
        ),
        // 57 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738",
            ),
            "2fe741af801cc238602ac0ec6a7b0c3a8a87c7fc7d7f02a3fe03d1c12eac4d8f",
        ),
        // 63 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e",
            ),
            "29af2686fd53374a36b0846694cc342177e428d1647515f078784d69cdb9e488",
        ),
        // 64 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f",
            ),
            "fdeab9acf3710362bd2658cdc9a29e8f9c757fcf9811603a8c447cd1d9151108",
        ),
        // 65 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f40",
            ),
            "4bfd2c8b6f1eec7a2afeb48b934ee4b2694182027e6d0fc075074f2fabb31781",
        ),
        // 119 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
                "606162636465666768696a6b6c6d6e6f70717273747576",
            ),
            "da18797ed7c3a777f0847f429724a2d8cd5138e6ed2895c3fa1a6d39d18f7ec6",
        ),
        // 120 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
                "606162636465666768696a6b6c6d6e6f7071727374757677",
            ),
            "f52b23db1fbb6ded89ef42a23ce0c8922c45f25c50b568a93bf1c075420bbb7c",
        ),
        // 127 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
                "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e",
            ),
            "92ca0fa6651ee2f97b884b7246a562fa71250fedefe5ebf270d31c546bfea976",
        ),
        // 128 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
                "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f",
            ),
            "471fb943aa23c511f6f72f8d1652d9c880cfa392ad80503120547703e56a2be5",
        ),
        // 129 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
                "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f80",
            ),
            "5099c6a56203f9687f7d33f4bfdf576d31dc91f6b695ecea38b2770c87631135",
        ),
        // 1000 bytes.
        (
            concat!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f",
                "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f",
                "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f",
                "909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf",
                "c0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeef",
                "f0f1f2f3f4f5f6f7f8f9fa000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f2021222324",
                "25262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f5051525354",
                "55565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f8081828384",
                "85868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4",
                "b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4",
                "e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fa000102030405060708090a0b0c0d0e0f10111213141516171819",
                "1a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f40414243444546474849",
                "4a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f70717273747576777879",
                "7a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9",
                "aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9",
                "dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fa000102030405060708090a0b0c0d0e",
                "0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e",
                "3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e",
                "6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e",
                "9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdce",
                "cfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6",
            ),
            "4e4c294b331f7a2099a379bec34b9f9fc03dc46ab465d998f4d683da53487e6d",
        ),
    ];
    for &(input, expected) in cases {
        let input = unhex(input);
        assert_eq!(
            sha256(&input),
            unhex(expected).as_slice(),
            "mismatch at len {}",
            input.len()
        );
    }
}

/// The two samples from NIST's SHA-256 example document, a message that
/// fits in one block and a message that spans two.
/// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/SHA256.pdf
#[test]
fn sha256_matches_nist_examples() {
    // One Block Message Sample.
    assert_eq!(
        sha256(b"abc"),
        unhex("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").as_slice(),
    );
    // Two Block Message Sample.
    assert_eq!(
        sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        unhex("248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1").as_slice(),
    );
}

// The SHA-256 test vectors from NIST's Cryptographic Algorithm Validation
// Program, byte-oriented set, from
// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/shabytetestvectors.zip
mod long_msg;
mod monte;
mod short_msg;

fn check_msg_vectors(vectors: &[(&str, &str)]) {
    for &(msg, md) in vectors {
        let msg = unhex(msg);
        assert_eq!(
            sha256(&msg),
            unhex(md).as_slice(),
            "mismatch at len {}",
            msg.len()
        );
    }
}

#[test]
fn sha256_matches_nist_short_msg_vectors() {
    check_msg_vectors(short_msg::VECTORS);
}

#[test]
fn sha256_matches_nist_long_msg_vectors() {
    check_msg_vectors(long_msg::VECTORS);
}

/// The Monte Carlo test from the SHA Validation System: each checkpoint is
/// the result of 1000 chained hashes of the previous three digests, starting
/// from the seed.
/// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/SHAVS.pdf
#[test]
fn sha256_matches_nist_monte_carlo_vectors() {
    let mut seed: [u8; 32] = unhex(monte::SEED).try_into().unwrap();
    for (i, expected) in monte::CHECKPOINTS.iter().enumerate() {
        let mut md = [seed; 3];
        for _ in 0..1000 {
            let msg = [md[0], md[1], md[2]].concat();
            md = [md[1], md[2], sha256(&msg)];
        }
        seed = md[2];
        assert_eq!(seed, unhex(expected).as_slice(), "mismatch at COUNT = {i}");
    }
}
