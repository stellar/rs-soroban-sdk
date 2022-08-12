use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use soroban_env_host::{
    xdr::{self, ScSpecEntry},
    Host, HostError, Vm,
};

use crate::parse;

#[derive(thiserror::Error, Debug)]
pub enum GetSpecError {
    #[error("reading contract spec from file")]
    Read(io::Error),
    #[error("loading contract into vm")]
    LoadContract(HostError),
    #[error("parsing contract spec")]
    Parse(xdr::Error),
    #[error("contract spec not found")]
    NotFound,
}

pub fn get_spec(path: &str) -> Result<Vec<ScSpecEntry>, GetSpecError> {
    let contents = fs::read(path).map_err(GetSpecError::Read)?;
    let h = Host::default();
    let vm = Vm::new(&h, [0; 32].into(), &contents).map_err(GetSpecError::LoadContract)?;
    if let Some(spec) = vm.custom_section("contractspecv0") {
        Ok(parse::parse_spec(spec).map_err(GetSpecError::Parse)?)
    } else {
        Err(GetSpecError::NotFound)
    }
}

/// Constructs a token stream containing variables for the WASM file.
pub fn generate_consts(path: &str) -> TokenStream {
    // TODO: Add variables for contract spec, and env meta.
    quote! {
        const WASM: &[u8] = include_bytes!(#path);
    }
}
