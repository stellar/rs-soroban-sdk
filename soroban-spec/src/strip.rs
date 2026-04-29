//! Wasm rewriting helpers for spec shaking.

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limits, WriteXdr};

use crate::{read, shaking};

const WASM_HEADER: &[u8; 8] = b"\0asm\x01\0\0\0";
const CUSTOM_SECTION_ID: u8 = 0;
const CONTRACT_SPEC_SECTION: &str = "contractspecv0";

#[derive(thiserror::Error, Debug)]
pub enum ShakeError {
    #[error("reading contract spec")]
    ReadSpec(read::FromWasmError),
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
/// This helper does not inspect `contractmetav0`; callers should use
/// [`shaking::spec_shaking_version_for_meta`] to decide whether a Wasm should be shaken before
/// calling it.
pub fn shake_contract_spec(wasm: &[u8]) -> Result<Vec<u8>, ShakeError> {
    let entries = read::from_wasm(wasm).map_err(ShakeError::ReadSpec)?;
    let markers = shaking::find_all(wasm);
    let graph = shaking::find_graph(wasm);
    let filtered = shaking::filter_with_graph(entries, &markers, &graph);

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
            shaking::GRAPH_SECTION => {}
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
    use super::{shake_contract_spec, write_custom_section, CONTRACT_SPEC_SECTION, WASM_HEADER};
    use crate::{read, shaking};
    use stellar_xdr::curr::{
        Limits, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeUdt,
        ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, WriteXdr,
    };

    fn function_with_udt(name: &str, udt_name: &str) -> ScSpecEntry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: name.try_into().unwrap(),
            inputs: vec![ScSpecFunctionInputV0 {
                doc: StringM::default(),
                name: "arg".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: udt_name.try_into().unwrap(),
                }),
            }]
            .try_into()
            .unwrap(),
            outputs: vec![].try_into().unwrap(),
        })
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

    fn graph_record(kind: u16, entry: &ScSpecEntry, refs: &[ScSpecEntry]) -> Vec<u8> {
        let mut record = Vec::new();
        record.extend_from_slice(b"SpGrV");
        record.push(1);
        record.extend_from_slice(&kind.to_be_bytes());
        record.extend_from_slice(&shaking::generate_spec_id_for_entry(entry));
        record.extend_from_slice(&(refs.len() as u16).to_be_bytes());
        for ref_entry in refs {
            record.extend_from_slice(&shaking::generate_spec_id_for_entry(ref_entry));
        }
        record
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
        let graph = graph_record(0, &function, std::slice::from_ref(&used));
        let wasm = minimal_wasm(&[
            (CONTRACT_SPEC_SECTION, &spec),
            (shaking::GRAPH_SECTION, &graph),
        ]);

        let shaken = shake_contract_spec(&wasm).unwrap();
        let filtered = read::from_wasm(&shaken).unwrap();

        assert_eq!(filtered, vec![function, used]);
        assert!(shaking::find_graph(&shaken).entries.is_empty());
        assert!(!shaken
            .windows(shaking::GRAPH_SECTION.len())
            .any(|bytes| bytes == shaking::GRAPH_SECTION.as_bytes()));
    }

    #[test]
    fn test_shake_contract_spec_keeps_other_custom_sections() {
        let function = function_with_udt("run", "Shared");
        let used = struct_entry("Shared", "used", ScSpecTypeDef::I32);
        let spec = spec_xdr(&[function.clone(), used.clone()]);
        let graph = graph_record(0, &function, &[used]);
        let other = b"kept";
        let wasm = minimal_wasm(&[
            ("before", other.as_slice()),
            (CONTRACT_SPEC_SECTION, &spec),
            (shaking::GRAPH_SECTION, &graph),
            ("after", other.as_slice()),
        ]);

        let shaken = shake_contract_spec(&wasm).unwrap();

        assert!(shaken.windows(other.len()).any(|bytes| bytes == other));
        assert!(shaken
            .windows("before".len())
            .any(|bytes| bytes == b"before"));
        assert!(shaken.windows("after".len()).any(|bytes| bytes == b"after"));
    }
}
