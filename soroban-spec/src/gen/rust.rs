pub mod r#trait;
pub mod types;

use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use stellar_xdr::{self, ScSpecEntry};

use crate::read::{from_wasm, FromWasmError};

use types::{generate_enum, generate_error_enum, generate_struct, generate_union};

#[derive(thiserror::Error, Debug)]
pub enum GenerateFromFileError {
    #[error("reading file: {0}")]
    Io(io::Error),
    #[error("sha256 does not match, expected: {expected}")]
    VerifySha256 { expected: String },
    #[error("parsing contract spec: {0}")]
    Parse(stellar_xdr::Error),
    #[error("getting contract spec: {0}")]
    GetSpec(FromWasmError),
}

pub fn generate_from_file(
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<TokenStream, GenerateFromFileError> {
    // Read file.
    let wasm = fs::read(file).map_err(GenerateFromFileError::Io)?;

    // Generate code.
    let code = generate_from_wasm(&wasm, file, verify_sha256)?;
    Ok(code)
}

pub fn generate_from_wasm(
    wasm: &[u8],
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<TokenStream, GenerateFromFileError> {
    let sha256 = Sha256::digest(&wasm);
    let sha256 = format!("{:x}", sha256);
    if let Some(verify_sha256) = verify_sha256 {
        if verify_sha256 != sha256 {
            return Err(GenerateFromFileError::VerifySha256 { expected: sha256 });
        }
    }

    let spec = from_wasm(wasm).map_err(GenerateFromFileError::GetSpec)?;
    let code = generate(&spec, file, &sha256);
    Ok(code)
}

pub fn generate(specs: &[ScSpecEntry], file: &str, sha256: &str) -> TokenStream {
    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    let mut spec_enums = Vec::new();
    let mut spec_error_enums = Vec::new();
    for s in specs {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
            ScSpecEntry::UdtEnumV0(e) => spec_enums.push(e),
            ScSpecEntry::UdtErrorEnumV0(e) => spec_error_enums.push(e),
        }
    }

    let trait_name = "Contract";
    let client_name = "Client";

    let trait_ = r#trait::generate_trait(trait_name, &spec_fns);
    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    let enums = spec_enums.iter().map(|s| generate_enum(s));
    let error_enums = spec_error_enums.iter().map(|s| generate_error_enum(s));

    quote! {
        pub const WASM: &[u8] = ::soroban_sdk::contractfile!(file = #file, sha256 = #sha256);

        #[::soroban_sdk::contractclient(name = #client_name)]
        #trait_

        #(#structs)*
        #(#unions)*
        #(#enums)*
        #(#error_enums)*
    }
}
