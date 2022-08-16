use std::io::Cursor;

use stellar_xdr::{self, ReadXdr, ScSpecEntry};
use wasmparser::{BinaryReaderError, Parser, Payload};

// TODO: Move these functions into stellar_xdr.

#[derive(thiserror::Error, Debug)]
pub enum ParseSpecBase64Error {
    #[error("parsing contract spec base64")]
    ParseBase64(base64::DecodeError),
    #[error("parsing contract spec xdr")]
    ParseXdr(stellar_xdr::Error),
}

pub fn parse_spec_base64(spec: &[u8]) -> Result<Vec<ScSpecEntry>, ParseSpecBase64Error> {
    let decoded = base64::decode(spec).map_err(ParseSpecBase64Error::ParseBase64)?;
    parse_spec(&decoded).map_err(ParseSpecBase64Error::ParseXdr)
}

pub fn parse_spec(spec: &[u8]) -> Result<Vec<ScSpecEntry>, stellar_xdr::Error> {
    let mut cursor = Cursor::new(spec);
    let entries = ScSpecEntry::read_xdr_iter(&mut cursor).collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

#[derive(thiserror::Error, Debug)]
pub enum GetSpecError {
    #[error("reading wasm")]
    Read(BinaryReaderError),
    #[error("parsing contract spec")]
    Parse(stellar_xdr::Error),
    #[error("contract spec not found")]
    NotFound,
}

pub fn read_spec_raw(wasm: &[u8]) -> Result<Vec<u8>, GetSpecError> {
    for payload in Parser::new(0).parse_all(wasm) {
        let payload = payload.map_err(GetSpecError::Read)?;
        if let Payload::CustomSection(section) = payload {
            if section.name() == "contractspecv0" {
                return Ok(section.data().to_vec());
            }
        };
    }
    Err(GetSpecError::NotFound)
}

pub fn read_spec(wasm: &[u8]) -> Result<Vec<ScSpecEntry>, GetSpecError> {
    let spec = read_spec_raw(wasm)?;
    parse_spec(&spec).map_err(GetSpecError::Parse)
}
