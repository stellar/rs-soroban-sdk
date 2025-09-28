#![cfg(test)]
use crate::read;
use std::fs;
use stellar_xdr::curr::ScMetaEntry;

#[test]
fn test_from_wasm() {
    let wasm = fs::read("../target/wasm32v1-none/release/test_zero.wasm").unwrap();
    let meta = read::from_wasm(&wasm).unwrap();
    let keys = meta
        .iter()
        .map(|e| match e {
            ScMetaEntry::ScMetaV0(v0) => v0.key.to_string(),
        })
        .collect::<Vec<_>>();
    assert_eq!(keys, ["rsver", "rssdkver"]);
}

#[test]
fn test_raw_from_wasm() {
    let wasm = fs::read("../target/wasm32v1-none/release/test_zero.wasm").unwrap();
    let raw_meta = read::raw_from_wasm(&wasm).unwrap();
    assert!(!raw_meta.is_empty());
}

#[test]
fn test_parse_raw() {
    let wasm = fs::read("../target/wasm32v1-none/release/test_zero.wasm").unwrap();
    let raw_meta = read::raw_from_wasm(&wasm).unwrap();
    let meta = read::parse_raw(&raw_meta).unwrap();
    assert!(!meta.is_empty());
}

#[test]
fn test_from_wasm_no_metadata() {
    // Create a simple Wasm file without metadata
    let wasm = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; // minimal Wasm header
    let result = read::from_wasm(&wasm);
    assert!(matches!(result, Err(read::FromWasmError::NotFound)));
}

#[test]
fn test_raw_from_wasm_no_metadata() {
    let wasm = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let result = read::raw_from_wasm(&wasm);
    assert!(matches!(result, Err(read::FromWasmError::NotFound)));
}

#[test]
fn test_parse_raw_invalid() {
    let result = read::parse_raw(b"invalid xdr data");
    assert!(matches!(result, Err(_)));
}
