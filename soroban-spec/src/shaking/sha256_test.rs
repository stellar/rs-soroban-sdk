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
// Program, byte-oriented set, unmodified from
// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/shabytetestvectors.zip
const SHORT_MSG: &str = include_str!("sha256_test_vectors/SHA256ShortMsg.rsp");
const LONG_MSG: &str = include_str!("sha256_test_vectors/SHA256LongMsg.rsp");
const MONTE: &str = include_str!("sha256_test_vectors/SHA256Monte.rsp");

/// The `key = value` pairs of a CAVP response file, in order, skipping
/// comments and section headers.
fn rsp_fields(rsp: &str) -> impl Iterator<Item = (&str, &str)> {
    rsp.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('['))
        .map(|l| l.split_once(" = ").unwrap())
}

/// Checks every `Len`/`Msg`/`MD` record of a message test file, returning the
/// number checked. `Len` is in bits, and a zero-length message is written as
/// `Msg = 00`, so the message is the first `Len / 8` bytes of `Msg`.
fn check_msg_vectors(rsp: &str) -> usize {
    let mut fields = rsp_fields(rsp);
    let mut count = 0;
    while let Some(("Len", len)) = fields.next() {
        let len: usize = len.parse().unwrap();
        let (Some(("Msg", msg)), Some(("MD", md))) = (fields.next(), fields.next()) else {
            panic!("malformed record at Len = {len}");
        };
        let msg = &unhex(msg)[..len / 8];
        assert_eq!(sha256(msg), unhex(md).as_slice(), "mismatch at Len = {len}");
        count += 1;
    }
    count
}

#[test]
fn sha256_matches_nist_short_msg_vectors() {
    assert_eq!(check_msg_vectors(SHORT_MSG), 65);
}

#[test]
fn sha256_matches_nist_long_msg_vectors() {
    assert_eq!(check_msg_vectors(LONG_MSG), 64);
}

/// The Monte Carlo test from the SHA Validation System: each checkpoint is
/// the result of 1000 chained hashes of the previous three digests, starting
/// from the seed.
/// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/SHAVS.pdf
#[test]
fn sha256_matches_nist_monte_carlo_vectors() {
    let mut fields = rsp_fields(MONTE);
    let Some(("Seed", seed)) = fields.next() else {
        panic!("missing seed");
    };
    let mut seed: [u8; 32] = unhex(seed).try_into().unwrap();
    let mut count = 0;
    while let (Some(("COUNT", c)), Some(("MD", expected))) = (fields.next(), fields.next()) {
        let mut md = [seed; 3];
        for _ in 0..1000 {
            let msg = [md[0], md[1], md[2]].concat();
            md = [md[1], md[2], sha256(&msg)];
        }
        seed = md[2];
        assert_eq!(seed, unhex(expected).as_slice(), "mismatch at COUNT = {c}");
        count += 1;
    }
    assert_eq!(count, 100);
}
