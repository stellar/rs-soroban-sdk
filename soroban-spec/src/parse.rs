use std::io::Cursor;

use soroban_env_host::xdr::{self, ReadXdr, ScSpecEntry};

// TODO: Move these functions into stellar_xdr.

#[derive(thiserror::Error, Debug)]
pub enum ParseSpecBase64Error {
    #[error("parsing contract spec base64")]
    ParseBase64(base64::DecodeError),
    #[error("parsing contract spec xdr")]
    ParseXdr(xdr::Error),
}

pub fn parse_spec_base64(spec: &[u8]) -> Result<Vec<ScSpecEntry>, ParseSpecBase64Error> {
    let decoded = base64::decode(spec).map_err(ParseSpecBase64Error::ParseBase64)?;
    parse_spec(&decoded).map_err(ParseSpecBase64Error::ParseXdr)
}

pub fn parse_spec(spec: &[u8]) -> Result<Vec<ScSpecEntry>, xdr::Error> {
    let mut cursor = Cursor::new(spec);
    let entries = ScSpecEntry::read_xdr_iter(&mut cursor).collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}
