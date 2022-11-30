pub mod r#trait;
pub mod types;

use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use stellar_xdr::{self, ScSpecEntry};
use syn::Error;

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
    let sha256 = Sha256::digest(wasm);
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

    let trait_ = r#trait::generate_trait(trait_name, &spec_fns);
    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    let enums = spec_enums.iter().map(|s| generate_enum(s));
    let error_enums = spec_error_enums.iter().map(|s| generate_error_enum(s));

    quote! {
        pub const WASM: &[u8] = soroban_sdk::contractfile!(file = #file, sha256 = #sha256);

        #[soroban_sdk::contractclient(name = "Client")]
        #trait_

        #(#structs)*
        #(#unions)*
        #(#enums)*
        #(#error_enums)*
    }
}

/// Implemented by types that can be converted into pretty formatted Strings of
/// Rust code.
pub trait ToFormattedString {
    /// Converts the value to a String that is pretty formatted. If there is any
    /// error parsin the token stream the raw String version of the code is
    /// returned instead.
    fn to_formatted_string(&self) -> Result<String, Error>;
}

impl ToFormattedString for TokenStream {
    fn to_formatted_string(&self) -> Result<String, Error> {
        let file = syn::parse2(self.clone())?;
        Ok(prettyplease::unparse(&file))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::gen::rust::ToFormattedString;

    use super::generate;

    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/test_udt.wasm");

    #[test]
    fn example() {
        let entries = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let rust = generate(&entries, "<file>", "<sha256>")
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn add(env: soroban_sdk::Env, a: UdtEnum, b: UdtEnum) -> i64;
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtTuple(pub i64, pub soroban_sdk::Vec<i64>);
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtStruct {
    pub a: i64,
    pub b: i64,
    pub c: soroban_sdk::Vec<i64>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
    UdtC(UdtEnum2),
    UdtD(UdtTuple),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UdtEnum2 {
    A = 10,
    B = 15,
}
"#,
        );
    }
}
