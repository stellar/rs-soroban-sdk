pub mod parse;
pub mod r#trait;
pub mod types;
pub mod wasm;

use std::fs;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use sha2::{Digest, Sha256};
use soroban_env_host::xdr::ScSpecEntry;
use syn::Error;

use self::{
    types::{generate_struct, generate_union},
    wasm::GetSpecError,
};

// TODO: Return Result's in this crate, not TS.

// use sha2::{Digest, Sha256, Sha512};
// let mut hasher = Sha256::new();
// hasher.update(&wasm);
// let sha256 = hasher.finalize();

pub fn generate_from_file(file: &str) -> TokenStream {
    // Read file.
    let wasm = match fs::read(file) {
        Ok(wasm) => wasm,
        Err(e) => {
            return Error::new(Span::call_site(), e.to_string()).into_compile_error();
        }
    };

    // Produce hash for file.
    let sha256 = Sha256::digest(&wasm);
    let sha256 = format!("{:x}", sha256);

    // Read spec from file.
    let spec = match wasm::get_spec(&wasm) {
        Ok(spec) => spec,
        Err(e) => {
            let err_str = match e {
                GetSpecError::LoadContract(e) => e.to_string(),
                GetSpecError::Parse(e) => e.to_string(),
                GetSpecError::NotFound => "spec not found".to_string(),
            };
            return Error::new(Span::call_site(), err_str).into_compile_error();
        }
    };

    // Generate code.
    generate(&spec, file, &sha256)
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
