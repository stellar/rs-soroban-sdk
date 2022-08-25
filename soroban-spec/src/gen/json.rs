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
        let name = f.name.to_string()?;
        let value = SpecType::try_from(&f.type_)?;
        Ok(StructField {
            name: name,
            value: value,
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
        let name = f.name.to_string()?;
        let value = SpecType::try_from(&f.type_)?;
        Ok(FunctionInput {
            name: name,
            value: value,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnionCase {
    name: String,
    value: Option<SpecType>,
}

impl TryFrom<&ScSpecUdtUnionCaseV0> for UnionCase {
    type Error = Error;

    fn try_from(c: &ScSpecUdtUnionCaseV0) -> Result<Self, Self::Error> {
        let name = c.name.to_string()?;
        let value = c.type_.as_ref().map(SpecType::try_from);
        Ok(UnionCase {
            name: name,
            value: match value {
                None => None,
                Some(r) => Some(r?),
            },
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
            ScSpecTypeDef::Map(inner) => {
                let key = SpecType::try_from(inner.key_type.as_ref());
                let value = SpecType::try_from(inner.value_type.as_ref());
                Ok(SpecType::Map {
                    key: Box::new(key?),
                    value: Box::new(value?),
                })
            }
            ScSpecTypeDef::Option(inner) => {
                let value = SpecType::try_from(inner.value_type.as_ref());
                Ok(SpecType::Option {
                    value: Box::new(value?),
                })
            }
            ScSpecTypeDef::Result(inner) => {
                let value = SpecType::try_from(inner.ok_type.as_ref());
                let error = SpecType::try_from(inner.error_type.as_ref());
                Ok(SpecType::Result {
                    value: Box::new(value?),
                    error: Box::new(error?),
                })
            }
            ScSpecTypeDef::Set(inner) => {
                let element = SpecType::try_from(inner.element_type.as_ref());
                Ok(SpecType::Set {
                    element: Box::new(element?),
                })
            }
            ScSpecTypeDef::Tuple(inner) => {
                let elements: Result<Vec<SpecType>, Error> =
                    inner.value_types.iter().map(SpecType::try_from).collect();
                Ok(SpecType::Tuple {
                    elements: elements?,
                })
            }
            ScSpecTypeDef::Vec(inner) => {
                let element = SpecType::try_from(inner.element_type.as_ref());
                Ok(SpecType::Vec {
                    element: Box::new(element?),
                })
            }
            ScSpecTypeDef::Udt(inner) => Ok(SpecType::UserDefined {
                name: inner.name.to_string()?,
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
            ScSpecEntry::FunctionV0(f) => {
                let name = f.name.to_string()?;
                let inputs: Result<Vec<FunctionInput>, Error> =
                    f.inputs.iter().map(FunctionInput::try_from).collect();
                let outputs: Result<Vec<SpecType>, Error> =
                    f.outputs.iter().map(SpecType::try_from).collect();
                Ok(SpecType::Function {
                    name: name,
                    inputs: inputs?,
                    outputs: outputs?,
                })
            }
            ScSpecEntry::UdtStructV0(s) => {
                let name = s.name.to_string()?;
                let fields: Result<Vec<StructField>, Error> =
                    s.fields.iter().map(StructField::try_from).collect();
                Ok(SpecType::Struct {
                    name: name,
                    fields: fields?,
                })
            }
            ScSpecEntry::UdtUnionV0(u) => {
                let name = u.name.to_string()?;
                let cases: Result<Vec<UnionCase>, Error> =
                    u.cases.iter().map(UnionCase::try_from).collect();
                Ok(SpecType::Union {
                    name: name,
                    cases: cases?,
                })
            }
        }
    }
}
