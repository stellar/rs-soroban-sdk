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
let xdr = SorobanClient.xdr;

// TODO: Move all the non-trivial conversions to js-stellar-base and use them them from there.
function bigintToI128(value: bigint): SorobanClient.xdr.ScVal {
  const buf = bigintToBuf(value);
  if (buf.length > 16) {
    throw new Error("value overflows i128");
  }

  if (value < BigInt(0)) {
    // Clear the top bit
    buf[0] &= 0x7f;
  }

  // left-pad with zeros up to 16 bytes
  let padded = Buffer.alloc(16);
  buf.copy(padded, padded.length-buf.length);

  if (value < BigInt(0)) {
    // Set the top bit
    padded[0] |= 0x80;
  }

  const hi = new xdr.Uint64(
    Number(bigintFromBytes(false, ...padded.slice(4, 8))),
    Number(bigintFromBytes(false, ...padded.slice(0, 4)))
  );
  const lo = new xdr.Uint64(
    Number(bigintFromBytes(false, ...padded.slice(12, 16))),
    Number(bigintFromBytes(false, ...padded.slice(8, 12)))
  );

  return xdr.ScVal.scvObject(xdr.ScObject.scoI128(new xdr.Int128Parts({lo, hi})));
}

function bigintToU128(value: bigint): SorobanClient.xdr.ScVal {
  const buf = bigintToBuf(value);
  if (buf.length > 16) {
    throw new Error("value overflows i128");
  }

  // left-pad with zeros up to 16 bytes
  let padded = Buffer.alloc(16);
  buf.copy(padded, padded.length-buf.length);

  const hi = new xdr.Uint64(
    Number(bigintFromBytes(false, ...padded.slice(4, 8))),
    Number(bigintFromBytes(false, ...padded.slice(0, 4)))
  );
  const lo = new xdr.Uint64(
    Number(bigintFromBytes(false, ...padded.slice(12, 16))),
    Number(bigintFromBytes(false, ...padded.slice(8, 12)))
  );

  return xdr.ScVal.scvObject(xdr.ScObject.scoU128(new xdr.Int128Parts({lo, hi})));
}

function bigintFromBytes(signed: boolean, ...bytes: (string | number | bigint)[]): bigint {
    let sign = 1;
    if (signed && bytes[0] === 0x80) {
      // top bit is set, negative number.
      sign = -1;
      bytes[0] &= 0x7f;
    }
    let b = BigInt(0);
    for (let byte of bytes) {
      b <<= BigInt(8);
      b |= BigInt(byte);
    }
    return BigInt(b.toString()) * BigInt(sign);
}

function bigintToBuf(bn: bigint): Buffer {
  var hex = BigInt(bn).toString(16).replace(/^-/, '');
  if (hex.length % 2) { hex = '0' + hex; }

  var len = hex.length / 2;
  var u8 = new Uint8Array(len);

  var i = 0;
  var j = 0;
  while (i < len) {
    u8[i] = parseInt(hex.slice(j, j+2), 16);
    i += 1;
    j += 2;
  }

  if (bn < BigInt(0)) {
    // Set the top bit
    u8[0] |= 0x80;
  }

  return Buffer.from(u8);
}
"#
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
    let mut arg_types: Vec<String> = Vec::new();
    let mut arg_names: Vec<String> = Vec::new();
    let mut arg_parsers: Vec<String> = Vec::new();

    for input in method.inputs.iter() {
        let input_name = input.name.to_string().unwrap();
        arg_types.push(format!(
            "{}: {}",
            &input_name,
            generate_type_ident(&input.type_),
        ));
        arg_names.push(format!("{}_xdr", &input_name,));
        arg_parsers.push(format!(
            "const {}_xdr = {};",
            &input_name,
            generate_type_parser(&input.type_, &input_name)
        ));
    }

    let types = arg_types.join(", ");
    let parsers = arg_parsers.join("\n");

    let mut args: Vec<String> = vec![format!("\"{name}\"")];
    args.extend(arg_names.iter().map(|x| format!("{x}")));
    let args_param = args.join(", ");

    format!(
        r#"  {name}({types}): SorobanClient.xdr.Operation {{
        {parsers}
    return this._contract.call({args_param});
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
