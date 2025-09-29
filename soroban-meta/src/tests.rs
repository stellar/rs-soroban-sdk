use crate::read;
use std::fs;
use stellar_xdr::curr::{Limits, ScMetaEntry, ScMetaV0, StringM, WriteXdr};

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
    assert_eq!(keys, ["rsver"]);
}

#[test]
fn test_from_wasm_no_metadata() {
    // Create a simple Wasm file without meta
    let wasm = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; // minimal Wasm header
    let result = read::from_wasm(&wasm).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_multiple_metadata_sections() {
    // Read the original test_zero.wasm
    let mut wasm = fs::read("../target/wasm32v1-none/release/test_zero.wasm").unwrap();

    // Add on an additional contract meta section
    let section_name = b"contractmetav0";
    let section_content = ScMetaEntry::ScMetaV0(ScMetaV0 {
        key: StringM::try_from("mykey").unwrap(),
        val: StringM::try_from("myval").unwrap(),
    })
    .to_xdr(Limits::none())
    .unwrap();

    // Encode custom section
    let mut custom_section = Vec::new();
    custom_section.push(0); // 0 = custom section
    custom_section.push((1 + section_name.len() + section_content.len()) as u8);
    custom_section.push(section_name.len() as u8);
    custom_section.extend_from_slice(section_name);
    custom_section.extend_from_slice(&section_content);

    // Append the custom section to the WASM file
    wasm.extend_from_slice(&custom_section);

    // Parse the new wasm
    let meta = read::from_wasm(&wasm).unwrap();

    // Should have the original entries plus the new one
    let keys = meta
        .iter()
        .map(|e| match e {
            ScMetaEntry::ScMetaV0(v0) => v0.key.to_string(),
        })
        .collect::<Vec<_>>();
    assert_eq!(keys, ["rsver", "mykey"]);
}
