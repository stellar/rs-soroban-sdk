use std::io::Cursor;

use soroban_env_host::{
    xdr::{self, ReadXdr, ScSpecEntry},
    Host, HostError, Vm,
};

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

#[derive(thiserror::Error, Debug)]
pub enum GetSpecError {
    #[error("loading contract into vm")]
    LoadContract(HostError),
    #[error("parsing contract spec")]
    Parse(xdr::Error),
    #[error("contract spec not found")]
    NotFound,
}

pub fn read_spec(wasm: &[u8]) -> Result<Vec<ScSpecEntry>, GetSpecError> {
    let h = Host::default();
    let vm = Vm::new(&h, [0; 32].into(), wasm).map_err(GetSpecError::LoadContract)?;
    if let Some(spec) = vm.custom_section("contractspecv0") {
        Ok(parse_spec(spec).map_err(GetSpecError::Parse)?)
    } else {
        Err(GetSpecError::NotFound)
    }
}
