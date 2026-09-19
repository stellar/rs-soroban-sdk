//! Tests for wasm custom section reading and rewriting.

use super::{custom, module};
use crate::wasm::{custom_section, replace_custom_section, write_leb128_u32};

#[test]
fn leb128() {
    let enc = |n| {
        let mut v = Vec::new();
        write_leb128_u32(&mut v, n);
        v
    };
    assert_eq!(enc(0), vec![0x00]);
    assert_eq!(enc(127), vec![0x7f]);
    assert_eq!(enc(128), vec![0x80, 0x01]);
    assert_eq!(enc(624_485), vec![0xe5, 0x8e, 0x26]);
}

#[test]
fn custom_section_concatenates_all_with_the_name() {
    let wasm = module(&[
        (0, custom("contractspecv0", b"aaa")),
        (0, custom("other", b"xxx")),
        (0, custom("contractspecv0", b"bbb")),
    ]);
    assert_eq!(
        custom_section(&wasm, "contractspecv0").unwrap(),
        Some(b"aaabbb".to_vec()),
    );
    assert_eq!(custom_section(&wasm, "absent").unwrap(), None);
}

#[test]
fn replace_custom_section_replaces_all_and_keeps_the_rest() {
    let wasm = module(&[
        (0, custom("contractspecv0", b"old")),
        (0, custom("keep-me", b"xxx")),
        (0, custom("contractspecv0", b"also-old")),
    ]);
    let out = replace_custom_section(&wasm, "contractspecv0", b"new").unwrap();
    assert_eq!(
        custom_section(&out, "contractspecv0").unwrap(),
        Some(b"new".to_vec()),
    );
    assert_eq!(
        custom_section(&out, "keep-me").unwrap(),
        Some(b"xxx".to_vec()),
    );
}

#[test]
fn replace_custom_section_adds_when_absent_and_copies_non_custom_sections() {
    // A type section, which must survive untouched.
    let type_section = vec![0x01, 0x60, 0x00, 0x00];
    let wasm = module(&[(1, type_section.clone())]);
    let out = replace_custom_section(&wasm, "contractspecv0", b"new").unwrap();
    assert_eq!(
        custom_section(&out, "contractspecv0").unwrap(),
        Some(b"new".to_vec()),
    );
    assert_eq!(out[..8], *b"\0asm\x01\0\0\0");
    // The type section is still there, byte for byte.
    assert!(out.windows(type_section.len()).any(|w| w == type_section));
}

#[test]
fn replace_custom_section_round_trips_a_large_section() {
    // Longer than 127 bytes, so the length needs more than one leb128 byte.
    let big = vec![7u8; 1000];
    let wasm = module(&[(0, custom("contractspecv0", b"old"))]);
    let out = replace_custom_section(&wasm, "contractspecv0", &big).unwrap();
    assert_eq!(custom_section(&out, "contractspecv0").unwrap(), Some(big));
}
