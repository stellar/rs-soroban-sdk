pub mod r#trait;
pub mod types;

use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use soroban_env_host::xdr::{self, ScSpecEntry};

use self::types::{generate_struct, generate_union};
use crate::wasm::{get_spec, GetSpecError};

#[derive(thiserror::Error, Debug)]
pub enum GenerateFromFileError {
    #[error("loading contract into vm")]
    Io(io::Error),
    #[error("verify sha256")]
    VerifySha256,
    #[error("parsing contract spec")]
    Parse(xdr::Error),
    #[error("getting contract spec")]
    GetSpec(GetSpecError),
}

pub fn generate_from_file(
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<TokenStream, GenerateFromFileError> {
    // Read file.
    let wasm = fs::read(file).map_err(GenerateFromFileError::Io)?;

    // Produce hash for file.
    let sha256 = Sha256::digest(&wasm);
    let sha256 = format!("{:x}", sha256);

    if let Some(verify_sha256) = verify_sha256 {
        if verify_sha256 != sha256 {
            return Err(GenerateFromFileError::VerifySha256);
        }
    }

    // Read spec from file.
    let spec = get_spec(&wasm).map_err(GenerateFromFileError::GetSpec)?;

    // Generate code.
    let code = generate(&spec, file, &sha256);
    Ok(code)
}

pub fn generate(specs: &[ScSpecEntry], file: &str, sha256: &str) -> TokenStream {
    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    for s in specs {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
        }
    }
    let trait_ = r#trait::generate_trait("Contract", &spec_fns);
    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    quote! {
        #[::soroban_sdk::contractfile(file = #file, sha256 = #sha256)]

        #[::soroban_sdk::contractclient(name = "Client")]
        #trait_

        #(#structs)*
        #(#unions)*
    }
}
