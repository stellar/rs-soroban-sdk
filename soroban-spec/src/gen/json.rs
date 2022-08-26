use serde::Serialize;
use soroban_env_host::xdr::{
    Error, ScSpecEntry, ScSpecFunctionInputV0, ScSpecTypeDef, ScSpecUdtStructFieldV0,
    ScSpecUdtUnionCaseV0,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    name: String,
    value: SpecType,
}

impl TryFrom<&ScSpecUdtStructFieldV0> for StructField {
    type Error = Error;

    fn try_from(f: &ScSpecUdtStructFieldV0) -> Result<Self, Self::Error> {
        Ok(StructField {
            name: f.name.to_string()?,
            value: SpecType::try_from(&f.type_)?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInput {
    name: String,
    value: SpecType,
}

impl TryFrom<&ScSpecFunctionInputV0> for FunctionInput {
    type Error = Error;

    fn try_from(f: &ScSpecFunctionInputV0) -> Result<Self, Self::Error> {
        Ok(FunctionInput {
            name: f.name.to_string()?,
            value: SpecType::try_from(&f.type_)?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnionCase {
    name: String,
    values: Vec<SpecType>,
}

impl TryFrom<&ScSpecUdtUnionCaseV0> for UnionCase {
    type Error = Error;

    fn try_from(c: &ScSpecUdtUnionCaseV0) -> Result<Self, Self::Error> {
        Ok(UnionCase {
            name: c.name.to_string()?,
            values: c
                .type_
                .as_ref()
                .map(SpecType::try_from)
                .transpose()?
                .into_iter()
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SpecType {
    U64,
    I64,
    U32,
    I32,
    Bool,
    Symbol,
    Bitset,
    Status,
    Bytes,
    BigInt,
    Function {
        name: String,
        inputs: Vec<FunctionInput>,
        outputs: Vec<SpecType>,
    },
    Map {
        key: Box<SpecType>,
        value: Box<SpecType>,
    },
    Option {
        value: Box<SpecType>,
    },
    Result {
        value: Box<SpecType>,
        error: Box<SpecType>,
    },
    Set {
        element: Box<SpecType>,
    },
    Vec {
        element: Box<SpecType>,
    },
    Tuple {
        elements: Vec<SpecType>,
    },
    UserDefined {
        name: String,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Union {
        name: String,
        cases: Vec<UnionCase>,
    },
}

impl TryFrom<&ScSpecTypeDef> for SpecType {
    type Error = Error;

    fn try_from(spec: &ScSpecTypeDef) -> Result<Self, Self::Error> {
        match spec {
            ScSpecTypeDef::Map(map) => Ok(SpecType::Map {
                key: Box::new(SpecType::try_from(map.key_type.as_ref())?),
                value: Box::new(SpecType::try_from(map.value_type.as_ref())?),
            }),
            ScSpecTypeDef::Option(opt) => Ok(SpecType::Option {
                value: Box::new(SpecType::try_from(opt.value_type.as_ref())?),
            }),
            ScSpecTypeDef::Result(res) => Ok(SpecType::Result {
                value: Box::new(SpecType::try_from(res.ok_type.as_ref())?),
                error: Box::new(SpecType::try_from(res.error_type.as_ref())?),
            }),
            ScSpecTypeDef::Set(set) => Ok(SpecType::Set {
                element: Box::new(SpecType::try_from(set.element_type.as_ref())?),
            }),
            ScSpecTypeDef::Tuple(tuple) => Ok(SpecType::Tuple {
                elements: tuple
                    .value_types
                    .iter()
                    .map(SpecType::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
            ScSpecTypeDef::Vec(vec) => Ok(SpecType::Vec {
                element: Box::new(SpecType::try_from(vec.element_type.as_ref())?),
            }),
            ScSpecTypeDef::Udt(udt) => Ok(SpecType::UserDefined {
                name: udt.name.to_string()?,
            }),
            ScSpecTypeDef::U64 => Ok(SpecType::U64),
            ScSpecTypeDef::I64 => Ok(SpecType::I64),
            ScSpecTypeDef::U32 => Ok(SpecType::U32),
            ScSpecTypeDef::I32 => Ok(SpecType::I32),
            ScSpecTypeDef::Bool => Ok(SpecType::Bool),
            ScSpecTypeDef::Symbol => Ok(SpecType::Symbol),
            ScSpecTypeDef::Bitset => Ok(SpecType::Bitset),
            ScSpecTypeDef::Status => Ok(SpecType::Status),
            ScSpecTypeDef::Bytes => Ok(SpecType::Bytes),
            ScSpecTypeDef::BigInt => Ok(SpecType::BigInt),
        }
    }
}

impl TryFrom<&ScSpecEntry> for SpecType {
    type Error = Error;

    fn try_from(spec: &ScSpecEntry) -> Result<Self, Self::Error> {
        match spec {
            ScSpecEntry::FunctionV0(f) => Ok(SpecType::Function {
                name: f.name.to_string()?,
                inputs: f
                    .inputs
                    .iter()
                    .map(FunctionInput::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
                outputs: f
                    .outputs
                    .iter()
                    .map(SpecType::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
            ScSpecEntry::UdtStructV0(s) => Ok(SpecType::Struct {
                name: s.name.to_string()?,
                fields: s
                    .fields
                    .iter()
                    .map(StructField::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
            ScSpecEntry::UdtUnionV0(u) => Ok(SpecType::Union {
                name: u.name.to_string()?,
                cases: u
                    .cases
                    .iter()
                    .map(UnionCase::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::SpecType;

    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/example_udt.wasm");

    #[test]
    fn example() {
        let entries = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let mut json = "".to_string();
        for entry in &entries {
            let json_entry: SpecType = entry.try_into().unwrap();
            json.push_str(&serde_json::to_string_pretty(&json_entry).unwrap());
        }
        assert_eq!(
            json,
            r#"{
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
          "type": "userDefined",
          "name": "UdtStruct"
        }
      ]
    }
  ]
}{
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
}{
  "type": "function",
  "name": "add",
  "inputs": [
    {
      "name": "a",
      "value": {
        "type": "userDefined",
        "name": "UdtEnum"
      }
    },
    {
      "name": "b",
      "value": {
        "type": "userDefined",
        "name": "UdtEnum"
      }
    }
  ],
  "outputs": [
    {
      "type": "i64"
    }
  ]
}"#,
        );
    }
}
