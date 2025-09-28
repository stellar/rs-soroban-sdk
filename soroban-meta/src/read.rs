use std::io::Cursor;

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limited, Limits, ReadXdr, ScMetaEntry};
use wasmparser::{BinaryReaderError, Parser, Payload};

pub fn parse_raw(meta: &[u8]) -> Result<Vec<ScMetaEntry>, stellar_xdr::Error> {
    let cursor = Cursor::new(meta);
    let entries = ScMetaEntry::read_xdr_iter(&mut Limited::new(
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
    #[error("parsing contract meta")]
    Parse(stellar_xdr::Error),
    #[error("contract meta not found")]
    NotFound,
}

pub fn raw_from_wasm(wasm: &[u8]) -> Result<Vec<u8>, FromWasmError> {
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.map_err(FromWasmError::Read)?;
        if let Payload::CustomSection(section) = payload {
            if section.name() == "contractmetav0" {
                return Ok(section.data().to_vec());
            }
        };
    }
    Err(FromWasmError::NotFound)
}

pub fn from_wasm(wasm: &[u8]) -> Result<Vec<ScMetaEntry>, FromWasmError> {
    let meta = raw_from_wasm(wasm)?;
    parse_raw(&meta).map_err(FromWasmError::Parse)
}
