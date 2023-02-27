pub mod types;
use std::{fs, io};

use sha2::{Digest, Sha256};
use stellar_xdr::{ScSpecEntry, ScSpecFunctionV0};

use crate::read::{from_wasm, FromWasmError};
use types::{generate_type_ident, generate_type_parser};

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
) -> Result<String, GenerateFromFileError> {
    // Read file.
    let wasm = fs::read(file).map_err(GenerateFromFileError::Io)?;

    // Produce hash for file.
    let sha256 = Sha256::digest(&wasm);
    let sha256 = format!("{:x}", sha256);

    if let Some(verify_sha256) = verify_sha256 {
        if verify_sha256 != sha256 {
            return Err(GenerateFromFileError::VerifySha256 { expected: sha256 });
        }
    }

    // Generate code.
    let code = generate_from_wasm(&wasm).map_err(GenerateFromFileError::GetSpec)?;
    Ok(code)
}

pub fn generate_from_wasm(wasm: &[u8]) -> Result<String, FromWasmError> {
    let spec = from_wasm(wasm)?;
    let code = generate(&spec);
    Ok(code)
}

pub fn generate(spec: &[ScSpecEntry]) -> String {
    let header = r#"import * as SorobanClient from "soroban-client";
let xdr = SorobanClient.xdr;"#
        .to_string();

    // TODO: Generate types for udts.
    let types: Vec<String> = Vec::new();

    let mut methods: Vec<String> = Vec::new();

    for entry in spec {
        match entry {
            ScSpecEntry::FunctionV0(f) => methods.push(generate_method(f, spec)),
            _ => todo!("generate_types"),
        };
    }

    let klass = format!(
        r#"export class Contract {{
  private _contract: SorobanClient.Contract;

  constructor(address: string) {{
    this._contract = new SorobanClient.Contract(address);
  }}

{}
}}"#,
        methods.join("\n\n")
    );

    vec![header, types.join("\n\n"), klass].join("\n\n")
}

fn generate_method(method: &ScSpecFunctionV0, _spec: &[ScSpecEntry]) -> String {
    let name = method.name.to_string().unwrap();
    let mut arg_names: Vec<String> = Vec::new();
    let mut arg_types: Vec<String> = Vec::new();
    let mut arg_parsers: Vec<String> = Vec::new();

    for input in method.inputs.iter() {
        let input_name = input.name.to_string().unwrap();
        arg_names.push(input_name.clone());
        arg_types.push(format!(
            "{}: {}",
            &input_name,
            generate_type_ident(&input.type_),
        ));
        arg_parsers.push(format!(
            "{} = {};",
            &input_name,
            generate_type_parser(&input.type_, &input_name)
        ));
    }

    let names = arg_names.join(", ");
    let types = arg_types.join(", ");
    let parsers = arg_parsers.join("\n");

    format!(
        r#"  {name}({types}): SorobanClient.Operation {{
        {parsers}
    return this._contract.call("{name}", {{ {names} }});
  }}"#,
    )
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::generate;

    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/test_hello.wasm");

    #[test]
    fn example() {
        let entries = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let code = generate(&entries).unwrap();
        assert_eq!(
            code,
            r#"import * as SorobanClient from "soroban-client";
let xdr = SorobanClient.xdr;

export class Contract {
  private _contract: SorobanClient.Contract;

  constructor(address: string) {
    this._contract = new SorobanClient.Contract(address);
  }

  hello(to: string): SorobanClient.Operation {
    to = xdr.ScVal.scvSymbol(to);
    return this._contract.call("hello", { to });
  
}"#,
        );
    }
}
