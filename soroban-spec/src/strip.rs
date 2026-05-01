//! Wasm rewriting helpers for spec shaking.

use std::io::Cursor;

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limited, Limits, ReadXdr, ScMetaEntry, WriteXdr};
use wasmparser::{BinaryReaderError, Parser, Payload};

use crate::{read, shaking};
use soroban_spec_markers::GRAPH_SECTION;

const WASM_HEADER: &[u8; 8] = b"\0asm\x01\0\0\0";
const CUSTOM_SECTION_ID: u8 = 0;
const CONTRACT_SPEC_SECTION: &str = "contractspecv0";
const CONTRACT_META_SECTION: &str = "contractmetav0";

#[derive(thiserror::Error, Debug)]
pub enum ShakeError {
    #[error("reading contract spec")]
    ReadSpec(read::FromWasmError),
    #[error("reading contract meta")]
    ReadMeta(BinaryReaderError),
    #[error("parsing contract meta")]
    ParseMeta(stellar_xdr::Error),
    #[error("unsupported spec shaking version {0}, expected 2")]
    UnsupportedSpecShakingVersion(u32),
    #[error(transparent)]
    SpecShaking(#[from] shaking::SpecShakingError),
    #[error("encoding contract spec")]
    EncodeSpec(stellar_xdr::Error),
    #[error("rewriting wasm")]
    Rewrite(RewriteError),
}

#[derive(thiserror::Error, Debug)]
pub enum RewriteError {
    #[error("invalid wasm header")]
    InvalidHeader,
    #[error("invalid wasm varuint32")]
    InvalidVarUint32,
    #[error("invalid wasm section length")]
    InvalidSectionLength,
    #[error("invalid custom section name")]
    InvalidCustomSectionName(#[source] std::str::Utf8Error),
    #[error("contract spec section not found")]
    ContractSpecNotFound,
}

/// Filters `contractspecv0` using spec shaking v2 reachability and removes the sidecar graph.
///
/// This helper only operates on Wasms whose `contractmetav0` declares
/// `rssdk_spec_shaking = "2"` and whose graph, when needed for reachable UDT references, is valid
/// and complete. Passing a v1, non-Rust, or corrupted v2 contract would make marker scanning or
/// graph traversal ambiguous, so those inputs are rejected before rewriting.
pub fn shake_contract_spec(wasm: &[u8]) -> Result<Vec<u8>, ShakeError> {
    let entries = read::from_wasm(wasm).map_err(ShakeError::ReadSpec)?;
    let meta = contract_meta_from_wasm(wasm)?;
    let version = shaking::spec_shaking_version_for_meta(&meta);
    if version != 2 {
        return Err(ShakeError::UnsupportedSpecShakingVersion(version));
    }

    let markers = shaking::find_all(wasm);
    let graph = shaking::find_graph(wasm)?;
    let filtered = shaking::filter(entries, &markers, &graph)?;

    let mut spec_xdr = Vec::new();
    for entry in filtered {
        spec_xdr.extend(
            entry
                .to_xdr(Limits::none())
                .map_err(ShakeError::EncodeSpec)?,
        );
    }

    rewrite_contract_spec(wasm, &spec_xdr).map_err(ShakeError::Rewrite)
}

fn contract_meta_from_wasm(wasm: &[u8]) -> Result<Vec<ScMetaEntry>, ShakeError> {
    let mut meta = Vec::new();
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.map_err(ShakeError::ReadMeta)?;
        let Payload::CustomSection(section) = payload else {
            continue;
        };
        if section.name() != CONTRACT_META_SECTION {
            continue;
        }

        let cursor = Cursor::new(section.data());
        let entries = ScMetaEntry::read_xdr_iter(&mut Limited::new(
            cursor,
            Limits {
                depth: 500,
                len: 0x1000000,
            },
        ))
        .collect::<Result<Vec<_>, _>>()
        .map_err(ShakeError::ParseMeta)?;
        meta.extend(entries);
    }
    Ok(meta)
}

fn rewrite_contract_spec(wasm: &[u8], spec_xdr: &[u8]) -> Result<Vec<u8>, RewriteError> {
    if wasm.len() < WASM_HEADER.len() || &wasm[..WASM_HEADER.len()] != WASM_HEADER {
        return Err(RewriteError::InvalidHeader);
    }

    let mut out = Vec::with_capacity(wasm.len());
    out.extend_from_slice(WASM_HEADER);

    let mut offset = WASM_HEADER.len();
    let mut wrote_spec = false;
    while offset < wasm.len() {
        let section_start = offset;
        let section_id = wasm[offset];
        offset += 1;

        let (payload_len, payload_offset) = read_var_u32(wasm, offset)?;
        offset = payload_offset;
        let payload_len = payload_len as usize;
        let payload_end = offset
            .checked_add(payload_len)
            .ok_or(RewriteError::InvalidSectionLength)?;
        if payload_end > wasm.len() {
            return Err(RewriteError::InvalidSectionLength);
        }

        let payload = &wasm[offset..payload_end];
        offset = payload_end;

        if section_id != CUSTOM_SECTION_ID {
            out.extend_from_slice(&wasm[section_start..payload_end]);
            continue;
        }

        let Some(name) = custom_section_name(payload)? else {
            out.extend_from_slice(&wasm[section_start..payload_end]);
            continue;
        };

        match name {
            CONTRACT_SPEC_SECTION => {
                if !wrote_spec {
                    write_custom_section(&mut out, CONTRACT_SPEC_SECTION, spec_xdr)?;
                    wrote_spec = true;
                }
            }
            GRAPH_SECTION => {}
            _ => out.extend_from_slice(&wasm[section_start..payload_end]),
        }
    }

    if wrote_spec {
        Ok(out)
    } else {
        Err(RewriteError::ContractSpecNotFound)
    }
}

fn custom_section_name(payload: &[u8]) -> Result<Option<&str>, RewriteError> {
    let Ok((name_len, name_offset)) = read_var_u32(payload, 0) else {
        return Ok(None);
    };
    let name_len = name_len as usize;
    let name_end = name_offset
        .checked_add(name_len)
        .ok_or(RewriteError::InvalidSectionLength)?;
    if name_end > payload.len() {
        return Ok(None);
    }
    std::str::from_utf8(&payload[name_offset..name_end])
        .map(Some)
        .map_err(RewriteError::InvalidCustomSectionName)
}

fn write_custom_section(out: &mut Vec<u8>, name: &str, data: &[u8]) -> Result<(), RewriteError> {
    out.push(CUSTOM_SECTION_ID);

    let mut payload = Vec::with_capacity(name.len() + data.len() + 5);
    write_var_u32(&mut payload, checked_u32(name.len())?);
    payload.extend_from_slice(name.as_bytes());
    payload.extend_from_slice(data);

    write_var_u32(out, checked_u32(payload.len())?);
    out.extend_from_slice(&payload);
    Ok(())
}

fn checked_u32(n: usize) -> Result<u32, RewriteError> {
    n.try_into().map_err(|_| RewriteError::InvalidSectionLength)
}

fn read_var_u32(bytes: &[u8], mut offset: usize) -> Result<(u32, usize), RewriteError> {
    let mut value = 0u32;
    for shift in (0..35).step_by(7) {
        let byte = *bytes.get(offset).ok_or(RewriteError::InvalidVarUint32)?;
        offset += 1;

        let chunk = (byte & 0x7f) as u32;
        if shift == 28 && chunk > 0x0f {
            return Err(RewriteError::InvalidVarUint32);
        }
        value |= chunk << shift;

        if byte & 0x80 == 0 {
            return Ok((value, offset));
        }
    }
    Err(RewriteError::InvalidVarUint32)
}

fn write_var_u32(out: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        shake_contract_spec, write_custom_section, ShakeError, CONTRACT_META_SECTION,
        CONTRACT_SPEC_SECTION, WASM_HEADER,
    };
    use crate::{read, shaking};
    use soroban_spec_markers::{
        generate_graph_record, SpecGraphEntryKind, GRAPH_SECTION, META_KEY, META_VALUE_V2,
    };
    use stellar_xdr::curr::{
        Limits, ScMetaEntry, ScMetaV0, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0,
        ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, WriteXdr,
    };

    fn function_with_type(name: &str, type_: ScSpecTypeDef) -> ScSpecEntry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: name.try_into().unwrap(),
            inputs: vec![ScSpecFunctionInputV0 {
                doc: StringM::default(),
                name: "arg".try_into().unwrap(),
                type_,
            }]
            .try_into()
            .unwrap(),
            outputs: vec![].try_into().unwrap(),
        })
    }

    fn function_with_udt(name: &str, udt_name: &str) -> ScSpecEntry {
        function_with_type(
            name,
            ScSpecTypeDef::Udt(ScSpecTypeUdt {
                name: udt_name.try_into().unwrap(),
            }),
        )
    }

    fn struct_entry(name: &str, field_name: &str, type_: ScSpecTypeDef) -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: StringM::default(),
                name: field_name.try_into().unwrap(),
                type_,
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn spec_xdr(entries: &[ScSpecEntry]) -> Vec<u8> {
        let mut xdr = Vec::new();
        for entry in entries {
            xdr.extend(entry.to_xdr(Limits::none()).unwrap());
        }
        xdr
    }

    fn meta_xdr(value: &str) -> Vec<u8> {
        ScMetaEntry::ScMetaV0(ScMetaV0 {
            key: META_KEY.try_into().unwrap(),
            val: value.try_into().unwrap(),
        })
        .to_xdr(Limits::none())
        .unwrap()
    }

    fn v2_meta_xdr() -> Vec<u8> {
        meta_xdr(META_VALUE_V2)
    }

    fn graph_record(entry: &ScSpecEntry, refs: &[ScSpecEntry]) -> Vec<u8> {
        let ref_ids: Vec<_> = refs
            .iter()
            .map(shaking::generate_spec_id_for_entry)
            .collect();
        generate_graph_record(
            SpecGraphEntryKind::Function,
            shaking::generate_spec_id_for_entry(entry),
            &ref_ids,
        )
    }

    fn minimal_wasm(custom_sections: &[(&str, &[u8])]) -> Vec<u8> {
        let mut wasm = WASM_HEADER.to_vec();
        for (name, data) in custom_sections {
            write_custom_section(&mut wasm, name, data).unwrap();
        }
        wasm
    }

    #[test]
    fn test_shake_contract_spec_rewrites_spec_and_removes_sidecar() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let unused_duplicate = struct_entry("Shared", "unused", ScSpecTypeDef::I64);
        let entries = vec![function.clone(), used.clone(), unused_duplicate.clone()];

        let spec = spec_xdr(&entries);
        let meta = v2_meta_xdr();
        let graph = graph_record(&function, std::slice::from_ref(&used));
        let wasm = minimal_wasm(&[
            (CONTRACT_META_SECTION, &meta),
            (CONTRACT_SPEC_SECTION, &spec),
            (GRAPH_SECTION, &graph),
        ]);

        let shaken = shake_contract_spec(&wasm).unwrap();
        let filtered = read::from_wasm(&shaken).unwrap();

        assert_eq!(filtered, vec![function, used]);
        assert!(shaking::find_graph(&shaken).unwrap().entries.is_empty());
        assert!(!shaken
            .windows(GRAPH_SECTION.len())
            .any(|bytes| bytes == GRAPH_SECTION.as_bytes()));
    }

    #[test]
    fn test_shake_contract_spec_keeps_other_custom_sections() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let spec = spec_xdr(&[function.clone(), used.clone()]);
        let meta = v2_meta_xdr();
        let graph = graph_record(&function, &[used]);
        let other = b"kept";
        let wasm = minimal_wasm(&[
            ("before", other.as_slice()),
            (CONTRACT_META_SECTION, &meta),
            (CONTRACT_SPEC_SECTION, &spec),
            (GRAPH_SECTION, &graph),
            ("after", other.as_slice()),
        ]);

        let shaken = shake_contract_spec(&wasm).unwrap();

        assert!(shaken.windows(other.len()).any(|bytes| bytes == other));
        assert!(shaken
            .windows("before".len())
            .any(|bytes| bytes == b"before"));
        assert!(shaken.windows("after".len()).any(|bytes| bytes == b"after"));
    }

    #[test]
    fn test_shake_contract_spec_rejects_missing_v2_meta() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let spec = spec_xdr(&[function.clone(), used.clone()]);
        let graph = graph_record(&function, &[used]);
        let wasm = minimal_wasm(&[(CONTRACT_SPEC_SECTION, &spec), (GRAPH_SECTION, &graph)]);

        let err = shake_contract_spec(&wasm).unwrap_err();

        assert!(matches!(err, ShakeError::UnsupportedSpecShakingVersion(1)));
    }

    #[test]
    fn test_shake_contract_spec_rejects_non_v2_meta() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let spec = spec_xdr(&[function.clone(), used.clone()]);
        let meta = meta_xdr("1");
        let graph = graph_record(&function, &[used]);
        let wasm = minimal_wasm(&[
            (CONTRACT_META_SECTION, &meta),
            (CONTRACT_SPEC_SECTION, &spec),
            (GRAPH_SECTION, &graph),
        ]);

        let err = shake_contract_spec(&wasm).unwrap_err();

        assert!(matches!(err, ShakeError::UnsupportedSpecShakingVersion(1)));
    }

    #[test]
    fn test_shake_contract_spec_accepts_v2_meta_with_empty_graph_without_udts() {
        let function = function_with_type("run", ScSpecTypeDef::I32);
        let spec = spec_xdr(std::slice::from_ref(&function));
        let meta = v2_meta_xdr();
        let wasm = minimal_wasm(&[
            (CONTRACT_META_SECTION, &meta),
            (CONTRACT_SPEC_SECTION, &spec),
        ]);

        let shaken = shake_contract_spec(&wasm).unwrap();

        assert_eq!(read::from_wasm(&shaken).unwrap(), vec![function]);
        assert!(shaking::find_graph(&shaken).unwrap().entries.is_empty());
    }

    #[test]
    fn test_shake_contract_spec_rejects_v2_meta_with_missing_graph_entry() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let spec = spec_xdr(&[function, used]);
        let meta = v2_meta_xdr();
        let wasm = minimal_wasm(&[
            (CONTRACT_META_SECTION, &meta),
            (CONTRACT_SPEC_SECTION, &spec),
        ]);

        let err = shake_contract_spec(&wasm).unwrap_err();

        assert!(matches!(
            err,
            ShakeError::SpecShaking(shaking::SpecShakingError::MissingGraphEntry { .. })
        ));
    }

    #[test]
    fn test_shake_contract_spec_rejects_v2_meta_with_malformed_graph() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let spec = spec_xdr(&[function, used]);
        let meta = v2_meta_xdr();
        let truncated_graph = &b"SpGrV"[..];
        let wasm = minimal_wasm(&[
            (CONTRACT_META_SECTION, &meta),
            (CONTRACT_SPEC_SECTION, &spec),
            (GRAPH_SECTION, truncated_graph),
        ]);

        let err = shake_contract_spec(&wasm).unwrap_err();

        assert!(matches!(
            err,
            ShakeError::SpecShaking(shaking::SpecShakingError::TruncatedRecord { .. })
        ));
    }
}
