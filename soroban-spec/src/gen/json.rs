use std::{fs, io};

pub mod types;

use sha2::{Digest, Sha256};
use stellar_xdr::ScSpecEntry;

use types::Entry;

use crate::read::{from_wasm, FromWasmError};

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
    let json = generate_from_wasm(&wasm).map_err(GenerateFromFileError::GetSpec)?;
    Ok(json)
}

pub fn generate_from_wasm(wasm: &[u8]) -> Result<String, FromWasmError> {
    let spec = from_wasm(wasm)?;
    let json = generate(&spec);
    Ok(json)
}

pub fn generate(spec: &[ScSpecEntry]) -> String {
    let collected: Vec<_> = spec.iter().map(Entry::from).collect();
    serde_json::to_string_pretty(&collected).expect("serialization of the spec entries should not have any failure cases as all keys are strings and the serialize implementations are derived")
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::generate;

    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/test_udt.wasm");

    #[test]
    fn example() {
        let entries = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let json = generate(&entries);
        assert_eq!(
            json,
            r#"[
  {
    "type": "enum",
    "name": "UdtEnum2",
    "cases": [
      {
        "name": "A",
        "value": 10
      },
      {
        "name": "B",
        "value": 15
      }
    ]
  },
  {
    "type": "union",
    "name": "UdtEnum",
    "cases": [
      {
        "name": "UdtA",
        "values": []
      },
      {
        "name": "UdtB",
        "values": [
          {
            "type": "custom",
            "name": "UdtStruct"
          }
        ]
      },
      {
        "name": "UdtC",
        "values": [
          {
            "type": "custom",
            "name": "UdtEnum2"
          }
        ]
      },
      {
        "name": "UdtD",
        "values": [
          {
            "type": "custom",
            "name": "UdtTuple"
          }
        ]
      }
    ]
  },
  {
    "type": "struct",
    "name": "UdtTuple",
    "fields": [
      {
        "name": "0",
        "value": {
          "type": "i64"
        }
      },
      {
        "name": "1",
        "value": {
          "type": "vec",
          "element": {
            "type": "i64"
          }
        }
      }
    ]
  },
  {
    "type": "struct",
    "name": "UdtStruct",
    "fields": [
      {
        "name": "a",
        "value": {
          "type": "i64"
        }
      },
      {
        "name": "b",
        "value": {
          "type": "i64"
        }
      },
      {
        "name": "c",
        "value": {
          "type": "vec",
          "element": {
            "type": "i64"
          }
        }
      }
    ]
  },
  {
    "type": "function",
    "name": "add",
    "inputs": [
      {
        "name": "a",
        "value": {
          "type": "custom",
          "name": "UdtEnum"
        }
      },
      {
        "name": "b",
        "value": {
          "type": "custom",
          "name": "UdtEnum"
        }
      }
    ],
    "outputs": [
      {
        "type": "i64"
      }
    ]
  }
]"#,
        );
    }
}
