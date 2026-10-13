use std::io::Cursor;

use base64::Engine;
use stellar_xdr::{
    Limited, Limits, ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
};
use wasmparser::{
    BinaryReaderError, CompositeType, ExternalKind, Parser, Payload, TypeRef, ValType,
};

// TODO: Move these functions into stellar_xdr.

#[derive(thiserror::Error, Debug)]
pub enum ParseSpecBase64Error {
    #[error("parsing contract spec base64")]
    ParseBase64(base64::DecodeError),
    #[error("parsing contract spec xdr")]
    ParseXdr(stellar_xdr::Error),
}

pub fn parse_base64(spec: &[u8]) -> Result<Vec<ScSpecEntry>, ParseSpecBase64Error> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(spec)
        .map_err(ParseSpecBase64Error::ParseBase64)?;
    parse_raw(&decoded).map_err(ParseSpecBase64Error::ParseXdr)
}

pub fn parse_raw(spec: &[u8]) -> Result<Vec<ScSpecEntry>, stellar_xdr::Error> {
    let cursor = Cursor::new(spec);
    let entries = ScSpecEntry::read_xdr_iter(&mut Limited::new(
        cursor,
        Limits {
            depth: 500,
            len: 0x1000000,
        },
    ))
    .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

#[derive(thiserror::Error, Debug)]
pub enum FromWasmError {
    #[error("reading wasm")]
    Read(BinaryReaderError),
    #[error("parsing contract spec")]
    Parse(stellar_xdr::Error),
    #[error("contract spec not found")]
    NotFound,
}

pub fn raw_from_wasm(wasm: &[u8]) -> Result<Vec<u8>, FromWasmError> {
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.map_err(FromWasmError::Read)?;
        if let Payload::CustomSection(section) = payload {
            if section.name() == "contractspecv0" {
                return Ok(section.data().to_vec());
            }
        };
    }
    Err(FromWasmError::NotFound)
}

pub fn base64_from_wasm(wasm: &[u8]) -> Result<String, FromWasmError> {
    let raw = raw_from_wasm(wasm)?;
    let mut res = String::new();
    base64::engine::general_purpose::STANDARD.encode_string(raw, &mut res);
    Ok(res)
}

/// Reads the contract spec from the wasm, and appends a function entry for
/// every function callable in the wasm that the spec does not describe. See
/// [`exported_fn_entries`].
pub fn from_wasm(wasm: &[u8]) -> Result<Vec<ScSpecEntry>, FromWasmError> {
    let spec = raw_from_wasm(wasm)?;
    let mut entries = parse_raw(&spec).map_err(FromWasmError::Parse)?;
    let missing = exported_fn_entries(wasm)?
        .into_iter()
        .filter(|e| !entries.iter().any(|s| fn_name(s) == fn_name(e)))
        .collect::<Vec<_>>();
    entries.extend(missing);
    Ok(entries)
}

fn fn_name(entry: &ScSpecEntry) -> Option<&[u8]> {
    match entry {
        ScSpecEntry::FunctionV0(f) => Some(f.name.as_slice()),
        _ => None,
    }
}

/// Returns a function entry for every function exported by the wasm that is
/// callable as a contract function.
///
/// A function is callable if its name is a valid symbol not reserved by the
/// `__` prefix, and every parameter and its single result are `i64`, the wasm
/// representation of a `Val`. The real types are unknown, so each input is
/// named `argN` and every input and the output are typed `Val`.
pub fn exported_fn_entries(wasm: &[u8]) -> Result<Vec<ScSpecEntry>, FromWasmError> {
    // Whether each type in the type section is a callable function type, and
    // the number of params it has.
    let mut types: Vec<Option<usize>> = Vec::new();
    // The type index of each function in the function index space.
    let mut funcs: Vec<u32> = Vec::new();
    let mut exports: Vec<(String, u32)> = Vec::new();
    for payload in Parser::new(0).parse_all(wasm) {
        match payload.map_err(FromWasmError::Read)? {
            Payload::TypeSection(reader) => {
                for group in reader {
                    for ty in group.map_err(FromWasmError::Read)?.into_types() {
                        types.push(match ty.composite_type {
                            CompositeType::Func(f)
                                if f.params().iter().all(|p| *p == ValType::I64)
                                    && f.results() == [ValType::I64] =>
                            {
                                Some(f.params().len())
                            }
                            _ => None,
                        });
                    }
                }
            }
            Payload::ImportSection(reader) => {
                for import in reader {
                    if let TypeRef::Func(ty) = import.map_err(FromWasmError::Read)?.ty {
                        funcs.push(ty);
                    }
                }
            }
            Payload::FunctionSection(reader) => {
                for ty in reader {
                    funcs.push(ty.map_err(FromWasmError::Read)?);
                }
            }
            Payload::ExportSection(reader) => {
                for export in reader {
                    let export = export.map_err(FromWasmError::Read)?;
                    if export.kind == ExternalKind::Func {
                        exports.push((export.name.to_string(), export.index));
                    }
                }
            }
            _ => {}
        }
    }

    let entries = exports
        .into_iter()
        .filter(|(name, _)| !name.starts_with("__"))
        .filter(|(name, _)| name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
        .filter_map(|(name, index)| {
            let params = funcs
                .get(index as usize)
                .and_then(|ty| types.get(*ty as usize))
                .copied()
                .flatten()?;
            let inputs = (0..params)
                .map(|i| {
                    Some(ScSpecFunctionInputV0 {
                        doc: Default::default(),
                        name: format!("arg{i}").try_into().ok()?,
                        type_: ScSpecTypeDef::Val,
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            Some(ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: Default::default(),
                name: name.try_into().ok()?,
                inputs: inputs.try_into().ok()?,
                outputs: [ScSpecTypeDef::Val].try_into().ok()?,
            }))
        })
        .collect();
    Ok(entries)
}

#[cfg(test)]
mod test {
    use super::*;
    use stellar_xdr::{ScSpecFunctionV0, WriteXdr};

    fn leb(mut v: usize, out: &mut Vec<u8>) {
        loop {
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                out.push(b);
                return;
            }
            out.push(b | 0x80);
        }
    }

    fn bytes(b: &[u8], out: &mut Vec<u8>) {
        leb(b.len(), out);
        out.extend_from_slice(b);
    }

    fn section(id: u8, items: &[Vec<u8>], out: &mut Vec<u8>) {
        let mut contents = Vec::new();
        leb(items.len(), &mut contents);
        items.iter().for_each(|i| contents.extend_from_slice(i));
        out.push(id);
        bytes(&contents, out);
    }

    fn export(name: &str, kind: u8, index: u8) -> Vec<u8> {
        let mut e = Vec::new();
        bytes(name.as_bytes(), &mut e);
        e.extend_from_slice(&[kind, index]);
        e
    }

    fn spec_fn(name: &str, inputs: &[(&str, ScSpecTypeDef)], output: ScSpecTypeDef) -> ScSpecEntry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: Default::default(),
            name: name.try_into().unwrap(),
            inputs: inputs
                .iter()
                .map(|(n, t)| ScSpecFunctionInputV0 {
                    doc: Default::default(),
                    name: (*n).try_into().unwrap(),
                    type_: t.clone(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            outputs: [output].try_into().unwrap(),
        })
    }

    fn wasm(spec: &[ScSpecEntry]) -> Vec<u8> {
        const I32: u8 = 0x7f;
        const I64: u8 = 0x7e;
        let mut w = b"\0asm\x01\0\0\0".to_vec();
        // Types: 0 = (i64, i64) -> i64, 1 = (i32) -> i32, 2 = () -> i64.
        section(
            1,
            &[
                vec![0x60, 2, I64, I64, 1, I64],
                vec![0x60, 1, I32, 1, I32],
                vec![0x60, 0, 1, I64],
            ],
            &mut w,
        );
        // Import: func 0 = env.imported, type 0.
        let mut import = Vec::new();
        bytes(b"env", &mut import);
        bytes(b"imported", &mut import);
        import.extend_from_slice(&[0x00, 0]);
        section(2, &[import], &mut w);
        // Funcs: 1 = type 0, 2 = type 1, 3 = type 2.
        section(3, &[vec![0], vec![1], vec![2]], &mut w);
        // Memory: min 1 page.
        section(5, &[vec![0x00, 1]], &mut w);
        section(
            7,
            &[
                export("add", 0x00, 1),
                export("reexported", 0x00, 0),
                export("missing", 0x00, 1),
                export("no_args", 0x00, 3),
                export("not_callable", 0x00, 2),
                export("__check_auth", 0x00, 1),
                export("not-a-symbol", 0x00, 1),
                export("memory", 0x02, 0),
            ],
            &mut w,
        );
        section(
            10,
            &[
                vec![4, 0, 0x20, 0, 0x0b],
                vec![4, 0, 0x20, 0, 0x0b],
                vec![4, 0, 0x42, 0, 0x0b],
            ],
            &mut w,
        );
        let mut custom = Vec::new();
        bytes(b"contractspecv0", &mut custom);
        for e in spec {
            custom.extend(e.to_xdr(Limits::none()).unwrap());
        }
        w.push(0);
        bytes(&custom, &mut w);
        w
    }

    #[test]
    fn test_from_wasm_adds_missing_fns() {
        let add = spec_fn(
            "add",
            &[("a", ScSpecTypeDef::U64), ("b", ScSpecTypeDef::U64)],
            ScSpecTypeDef::U64,
        );
        let entries = from_wasm(&wasm(&[add.clone()])).unwrap();
        assert_eq!(
            entries,
            [
                add,
                spec_fn(
                    "reexported",
                    &[("arg0", ScSpecTypeDef::Val), ("arg1", ScSpecTypeDef::Val)],
                    ScSpecTypeDef::Val,
                ),
                spec_fn(
                    "missing",
                    &[("arg0", ScSpecTypeDef::Val), ("arg1", ScSpecTypeDef::Val)],
                    ScSpecTypeDef::Val,
                ),
                spec_fn("no_args", &[], ScSpecTypeDef::Val),
            ]
        );
    }

    #[test]
    fn test_from_wasm_raw_unchanged() {
        let add = spec_fn("add", &[], ScSpecTypeDef::U64);
        let raw = raw_from_wasm(&wasm(&[add.clone()])).unwrap();
        assert_eq!(parse_raw(&raw).unwrap(), [add]);
    }
}
