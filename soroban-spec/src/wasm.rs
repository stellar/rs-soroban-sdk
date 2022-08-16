use soroban_env_host::{
    xdr::{self, ScSpecEntry},
    Host, HostError, Vm,
};

use crate::parse;

#[derive(thiserror::Error, Debug)]
pub enum GetSpecError {
    #[error("loading contract into vm")]
    LoadContract(HostError),
    #[error("parsing contract spec")]
    Parse(xdr::Error),
    #[error("contract spec not found")]
    NotFound,
}

pub fn get_spec(wasm: &[u8]) -> Result<Vec<ScSpecEntry>, GetSpecError> {
    let h = Host::default();
    let vm = Vm::new(&h, [0; 32].into(), wasm).map_err(GetSpecError::LoadContract)?;
    if let Some(spec) = vm.custom_section("contractspecv0") {
        Ok(parse::parse_spec(spec).map_err(GetSpecError::Parse)?)
    } else {
        Err(GetSpecError::NotFound)
    }
}
